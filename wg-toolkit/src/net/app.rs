//! Providing basic app structure.

use std::collections::HashMap;
use std::net::{SocketAddr, SocketAddrV4};
use std::sync::Arc;
use std::time::Duration;
use std::io::{self, Cursor};

use blowfish::Blowfish;
use mio::{Events, Poll, Interest, Token};
use mio::net::UdpSocket;

use super::packet::{Packet, PacketSyncError, PACKET_FLAGS_OFFSET};
use super::bundle::{BundleAssembler, Bundle};
use super::filter::blowfish::{BlowfishReader, BlowfishWriter, BLOCK_SIZE};

use crate::util::BytesFmt;


const COMMON_EVENT: Token = Token(0);


/// A base structure for network applications. This basically provides
/// a practical interface for the UDP server where packets are automatically
/// built into bundles, with de-fragmentation if needed. 
/// 
/// It also provides fragmentation support when sending bundles that contains 
/// more than one packet.
/// 
/// It optionally allow filtering sent and received packets through blowfish
/// depending on the remote socket address.
pub struct App {
    /// Bound address for UDP server.
    addr: SocketAddrV4,
    /// The socket used for sending and receiving UDP packets.
    socket: UdpSocket,
    /// Socket poll.
    socket_poll: Poll,
    /// Socket events.
    socket_events: Events,
    /// The structure used to re-assemble bundles from received packets.
    /// We associate a socket address used as packet origin.
    bundle_assembler: BundleAssembler<SocketAddr>,
    /// The next sequence ID to use for bundles.
    next_seq_id: u32,
    /// Registered channels on the app that defines a particular blowfish
    /// key for packet encryption and decryption.
    channels: HashMap<SocketAddr, Arc<Blowfish>>,
}

impl App {

    pub fn new(addr: SocketAddrV4) -> io::Result<Self> {

        let mut socket = UdpSocket::bind(SocketAddr::V4(addr))?;
        let socket_poll = Poll::new()?;

        socket_poll.registry().register(&mut socket, COMMON_EVENT, Interest::READABLE)?;

        Ok(Self {
            addr,
            socket,
            socket_poll,
            socket_events: Events::with_capacity(128),
            bundle_assembler: BundleAssembler::new(),
            next_seq_id: 0,
            channels: HashMap::new(),
        })

    }

    #[inline]
    pub fn addr(&self) -> SocketAddrV4 {
        self.addr
    }

    pub fn set_channel(&mut self, addr: SocketAddr, bf: Arc<Blowfish>) {
        self.channels.insert(addr, bf);
    }

    /// Send a bundle to a given address. Note that the bundle is finalized by
    /// this method with the internal sequence id.
    /// 
    /// Note that the net data is guaranteed to be untouched by this function,
    /// this includes prefix, flags up to the footer. Bytes beyond this limit
    /// might be modified in case of channel encryption.
    pub fn send(&mut self, bundle: &mut Bundle, to: SocketAddr) -> io::Result<usize> {

        bundle.finalize(&mut self.next_seq_id);

        let bf = self.channels.get(&to);

        let mut size = 0;
        for packet in bundle.packets() {

            if let Some(bf) = &bf {
                let data = encrypt_packet(&packet, &bf);
                size += self.socket.send_to(&data, to)?;
            } else {

            }
            // TODO: Remove once debugged
            println!("Sending {:X}", crate::util::BytesFmt(packet.net_data()));
            size += self.socket.send_to(packet.net_data(), to)?;

        }

        Ok(size)

    }

    /// Poll events from this application.
    /// 
    /// *Note that* the list of events is cleared internally prior to polling.
    pub fn poll(&mut self, events: &mut Vec<Event>, timeout: Option<Duration>) -> io::Result<()> {

        self.socket_poll.poll(&mut self.socket_events, timeout)?;

        events.clear();
        
        for event in self.socket_events.iter() {
            if event.token() == COMMON_EVENT && event.is_readable() {

                loop {

                    let mut packet = Packet::new_boxed();
                    
                    let (mut len, addr) = match self.socket.recv_from(packet.raw_mut().raw_data_mut()) {
                        Ok(t) => t,
                        Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                        Err(e) => return Err(e),
                    };

                    if let Some(bf) = self.channels.get(&addr) {
                        match decrypt_packet(&packet, len, &bf) {
                            Ok((clear_packet, clear_len)) => {
                                packet = clear_packet;
                                len = clear_len;
                            }
                            Err(()) => {
                                events.push(Event::new(addr, EventKind::PacketError(packet, PacketError::InvalidEncryption)));
                                continue
                            }
                        }
                    }

                    if let Err(error) = packet.sync_state(len) {
                        events.push(Event::new(addr, EventKind::PacketError(packet, PacketError::Sync(error))));
                    } else if let Some(bundle) = self.bundle_assembler.try_assemble(addr, packet) {
                        events.push(Event::new(addr, EventKind::Bundle(bundle)));
                    }

                }

            }
        }

        events.extend(self.bundle_assembler.drain_old()
            .into_iter()
            .map(|(addr, p)| Event::new(addr, EventKind::PacketError(p, PacketError::BundleTimeout))));

        Ok(())

    }

}


/// Encryption magic, 0xDEADBEEF in little endian.
const ENCRYPTION_MAGIC: [u8; 4] = 0xDEADBEEFu32.to_le_bytes();
/// Encryption footer length, 1 byte for wastage count + 4 bytes magic.
const ENCRYPTION_FOOTER_LEN: usize = ENCRYPTION_MAGIC.len() + 1;


/// Decrypt a packet of a given length with a blowfish key.
/// 
/// This returns an empty error if the encryption is invalid.
/// If successful the clear packet is returned with its size, the size can then
/// be used to synchronize the packet's state to its data.
fn decrypt_packet(packet: &Packet, len: usize, bf: &Blowfish) -> Result<(Box<Packet>, usize), ()> {

    let mut clear_packet = Packet::new_boxed();

    // Decrypt the incoming packet into the new clear packet.
    // We don't need to set the length yet because this packet 
    // will be synchronized just after.
    let src = &packet.raw_data()[PACKET_FLAGS_OFFSET..len];
    let dst = &mut clear_packet.raw_data_mut()[PACKET_FLAGS_OFFSET..len];
    
    // Note that src and dst have the same length, thanks to blowfish encryption.
    // Then we can already check the length and ensures that it is a multiple of
    // blowfish block size *and* can contain the wastage and encryption magic.
    if src.len() % BLOCK_SIZE != 0 || src.len() < ENCRYPTION_FOOTER_LEN {
        return Err(())
    }

    // Unwrapping because we know that source/destination have the same length.
    io::copy(
        &mut BlowfishReader::new(Cursor::new(src), &bf), 
        &mut Cursor::new(&mut *dst),
    ).unwrap();

    let wastage_begin = src.len() - 1;
    let magic_begin = wastage_begin - 4;

    // Check invalid magic.
    if &dst[magic_begin..wastage_begin] != &ENCRYPTION_MAGIC {
        return Err(())
    }

    // Get the wastage count and compute the packet's length.
    // Note that wastage count also it self length.
    let wastage = dst[wastage_begin];
    assert!(wastage <= BLOCK_SIZE as u8, "temporary check that wastage is not greater than block size");

    let new_len = len - wastage as usize - ENCRYPTION_MAGIC.len();

    // Copy the prefix directly because it is clear.
    clear_packet.raw_data_mut()[..PACKET_FLAGS_OFFSET]
        .copy_from_slice(&packet.raw_data()[..PACKET_FLAGS_OFFSET]);

    Ok((clear_packet, new_len))

}


/// Encrypt packet with the given blowfish key and returns the vector holding
/// the whole encrypted data to be sent.
fn encrypt_packet(packet: &Packet, bf: &Blowfish) -> Vec<u8> {
    
    // Get the minimum, unpadded length of this packet with encryption footer appended to it.
    let mut len = packet.len() + ENCRYPTION_FOOTER_LEN;

    // The wastage amount is basically the padding + 1 for the wastage itself.
    let padding = BLOCK_SIZE - (len % BLOCK_SIZE);
    len += padding;

    // Clone the packet data into a new vec and append the padding and the footer.
    let mut clear_data = Vec::from(packet.data());
    clear_data.reserve_exact(padding + ENCRYPTION_FOOTER_LEN);
    clear_data.extend_from_slice(&[0u8; BLOCK_SIZE - 1][..padding]); // Padding
    clear_data.extend_from_slice(&ENCRYPTION_MAGIC); // Magic
    clear_data.push(padding as u8 + 1); // Wastage count (+1 for it self size)

    debug_assert_eq!(clear_data.len(), len, "incoherent length");
    debug_assert_eq!(clear_data.len() % 8, 0, "data not padded as expected");
    
    // +4 for the prefix.
    let mut raw_data = Vec::with_capacity(clear_data.len() + 4);

    // Unwrapping because we know that source/destination have the same length.
    io::copy(
        &mut Cursor::new(&clear_data[..]), 
        &mut BlowfishWriter::new(Cursor::new(&mut raw_data[4..]), bf),
    ).unwrap();
    
    // Copy the prefix directly because it is clear.
    raw_data[..PACKET_FLAGS_OFFSET].copy_from_slice(&packet.prefix_data());

    raw_data

}


/// An event of the application.
#[derive(Debug)]
pub struct Event {
    /// The source address of the event.
    pub addr: SocketAddr,
    /// The kind of event.
    pub kind: EventKind
}

impl Event {

    #[inline]
    fn new(from: SocketAddr, kind: EventKind) -> Self {
        Self { addr: from, kind }
    }

}

/// The kind of event.
#[derive(Debug)]
pub enum EventKind {
    /// A fully received bundle.
    Bundle(Bundle),
    /// An error happened with a packet and it cannot be recovered.
    PacketError(Box<Packet>, PacketError)
}

/// Kind of packet errors.
#[derive(Debug)]
pub enum PacketError {
    /// The packet could not be synchronized from its data.
    Sync(PacketSyncError),
    /// The packet waited too much time to create a bundle.
    BundleTimeout,
    /// The packet should be decrypted but it failed.
    InvalidEncryption,
}
