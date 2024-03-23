//! Providing an bundle-oriented socket, backed by an UDP socket.

use std::collections::HashMap;
use std::net::{SocketAddr, SocketAddrV4};
use std::sync::Arc;
use std::time::Duration;
use std::io::{self, Cursor};

use blowfish::Blowfish;
use mio::{Events, Poll, Interest, Token};
use mio::net::UdpSocket;

use super::packet::{Packet, RawPacket, PacketConfig, PacketSyncError};
use super::bundle::{BundleAssembler, Bundle};
use super::filter::{BlowfishReader, BlowfishWriter, blowfish::BLOCK_SIZE};


const COMMON_EVENT: Token = Token(0);


/// A socket providing interface for sending and receiving bundles of elements, backed by
/// an UDP server with support for blowfish channel encryption. This socket is event
/// oriented, using a poll function that will wait for incoming network datagram.
/// 
/// It also provides fragmentation support when sending bundles that contains 
/// more than one packet.
pub struct WgSocket {
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
    next_sequence_num: u32,
    /// Registered channels on the app that defines a particular blowfish
    /// key for packet encryption and decryption.
    channels: HashMap<SocketAddr, Channel>,
    /// A raw packet used as a temporary buffer for blowfish encryption.
    encryption_packet: Box<RawPacket>,
}

impl WgSocket {

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
            next_sequence_num: 0,
            channels: HashMap::new(),
            encryption_packet: Box::new(RawPacket::new()),
        })

    }

    #[inline]
    pub fn addr(&self) -> SocketAddrV4 {
        self.addr
    }

    /// Associate a new channel to the given address with the given blowfish
    /// encryption. This blowfish encryption will be used for all 
    /// transaction to come with this given socket address.
    pub fn set_channel(&mut self, addr: SocketAddr, blowfish: Arc<Blowfish>) {
        self.channels.insert(addr, Channel::new(blowfish));
    }

    /// Send a bundle to a given address. Note that the bundle is finalized by
    /// this method with the internal sequence id.
    /// 
    /// Note that the net data is guaranteed to be untouched by this function,
    /// this includes prefix, flags up to the footer. Bytes beyond this limit
    /// might be modified in case of channel encryption.
    pub fn send(&mut self, bundle: &mut Bundle, to: SocketAddr) -> io::Result<usize> {

        // Do nothing if bundle is empty.
        if bundle.is_empty() {
            return Ok(0)
        }

        let mut channel = self.channels.get_mut(&to);

        // Compute first and last sequence num and directly update next sequence num.
        let sequence_first_num = self.next_sequence_num;
        self.next_sequence_num += bundle.len() as u32;
        let sequence_last_num = self.next_sequence_num - 1;

        let mut packet_config = PacketConfig::new();

        // When on channel, we set appropriate flags and values.
        if let Some(channel) = channel.as_deref_mut() {

            packet_config.set_on_channel(true);
            packet_config.set_reliable(true);
            
            // If we send a cumulative ack, just take auto ack to avoid resending 
            // it automatically.
            if packet_config.cumulative_ack().is_some() {
                channel.take_auto_ack();
            }

        }

        // If multi-packet bundle, set sequence range.
        if sequence_last_num > sequence_first_num {
            packet_config.set_sequence_range(sequence_first_num, sequence_last_num);
        }

        let mut size = 0;
        let mut sequence_num = sequence_first_num;

        for packet in bundle.packets_mut() {

            // Only the last packet has a cumulative ack.
            if sequence_num == sequence_last_num {
                if let Some(channel) = channel.as_mut() {
                    // FIXME: Do not set 0 as cumulative ack.
                    packet_config.set_cumulative_ack(channel.get_cumulative_ack_exclusive().unwrap_or(0));
                }
            }

            // Set sequence number and sync data.
            packet_config.set_sequence_num(sequence_num);
            packet.sync_data(&mut packet_config);

            // Reference to the actual raw packet to send.
            let raw_packet;

            if let Some(channel) = channel.as_deref_mut() {

                channel.add_sent_ack(sequence_num);

                encrypt_packet(packet.raw(), &channel.blowfish, &mut self.encryption_packet);
                raw_packet = &*self.encryption_packet;

            } else {
                raw_packet = packet.raw()
            }
            
            // println!("Sending {:X}", BytesFmt(raw_packet.data()));

            size += self.socket.send_to(raw_packet.data(), to)?;
            sequence_num += 1;

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
                    
                    let (len, addr) = match self.socket.recv_from(packet.raw_mut().raw_data_mut()) {
                        Ok(t) => t,
                        Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                        Err(e) => return Err(e),
                    };

                    let mut channel = self.channels.get_mut(&addr);

                    // Set the raw data length here, it's just temporary and will 
                    // be overwritten by 'sync_state'.
                    packet.raw_mut().set_data_len(len);

                    if let Some(channel) = channel.as_deref_mut() {
                        match decrypt_packet(&packet, &channel.blowfish) {
                            Ok(clear_packet) => packet = clear_packet,
                            Err(()) => {
                                events.push(Event::new(addr, EventKind::PacketError(packet, PacketError::InvalidEncryption)));
                                continue
                            }
                        }
                    }

                    // println!("Received {:X}", BytesFmt(packet.raw().data()));

                    // Get length again because it might be modified by decrypt.
                    let len = packet.raw().data_len();
                    let mut packet_config = PacketConfig::new();

                    if let Err(error) = packet.sync_state(len, &mut packet_config) {
                        events.push(Event::new(addr, EventKind::PacketError(packet, PacketError::Sync(error))));
                        continue
                    }

                    if let Some(channel) = channel.as_deref_mut() {

                        // If packet is reliable, take its ack number and store it for future acknowledging.
                        if packet_config.reliable() {
                            channel.add_received_ack(packet_config.sequence_num());
                            channel.set_auto_ack();
                        }

                        if let Some(ack) = packet_config.cumulative_ack() {
                            channel.remove_cumulative_ack(ack);
                        }

                    }

                    // We can observe that packets with the flag 0x1000 are only used
                    // for auto acking with sometimes duplicated data that is sent
                    // just after. If it become a problem this check can be removed. 
                    if packet_config.unk_1000().is_none() {
                        if let Some(bundle) = self.bundle_assembler.try_assemble(addr, packet, &packet_config) {
                            events.push(Event::new(addr, EventKind::Bundle(bundle)));
                        }
                    }

                }

            }
        }

        events.extend(self.bundle_assembler.drain_old()
            .into_iter()
            .map(|(addr, p)| Event::new(addr, EventKind::PacketError(p, PacketError::BundleTimeout))));

        // Send auto acks.
        for (addr, channel) in &mut self.channels {
            if channel.take_auto_ack() {
                
                let mut packet_config = PacketConfig::new();
                let ack = channel.get_cumulative_ack_exclusive().expect("incoherent");
                packet_config.set_sequence_num(ack);
                packet_config.set_cumulative_ack(ack);
                packet_config.set_on_channel(true);
                packet_config.set_unk_1000(0);

                let mut packet = Packet::new_boxed();
                packet.sync_data(&mut packet_config);

                encrypt_packet(packet.raw(), &channel.blowfish, &mut self.encryption_packet);

                // println!("Sending auto ack {:X}", BytesFmt(self.encryption_packet.data()));
                self.socket.send_to(self.encryption_packet.data(), *addr).unwrap();

            }
        }

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
fn decrypt_packet(packet: &Packet, bf: &Blowfish) -> Result<Box<Packet>, ()> {

    let len = packet.raw().data_len();

    // Create a packet that have the same length as input packet.
    let mut clear_packet = Packet::new_boxed();
    clear_packet.raw_mut().set_data_len(len);

    // Decrypt the incoming packet into the new clear packet.
    // We don't need to set the length yet because this packet 
    // will be synchronized just after.
    let src = packet.raw().body();
    let dst = clear_packet.raw_mut().body_mut();
    
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

    clear_packet.raw_mut().set_data_len(len - wastage as usize - ENCRYPTION_MAGIC.len());
    // Copy the prefix directly because it is clear.
    clear_packet.raw_mut().write_prefix(packet.raw().read_prefix());

    Ok(clear_packet)

}


/// Encrypt source packet with the given blowfish key and write it to the destination
/// raw packet. Everything except the packet prefix is encrypted, and the destination
/// packet will have a size that is a multiple of blowfish's block size (8). The clear
/// data is also padded to block size, but with additional data at the end: encryption
/// signature (0xDEADBEEF in little endian) and the wastage count + 1 on the last byte.
fn encrypt_packet(src_packet: &RawPacket, bf: &Blowfish, dst_packet: &mut RawPacket) {
    
    // Get the minimum, unpadded length of this packet with encryption footer appended to it.
    let mut len = src_packet.body_len() + ENCRYPTION_FOOTER_LEN;

    // The wastage amount is basically the padding + 1 for the wastage itself.
    let padding = (BLOCK_SIZE - (len % BLOCK_SIZE)) % BLOCK_SIZE;
    len += padding;

    // Clone the packet data into a new vec and append the padding and the footer.
    let mut clear_data = Vec::from(src_packet.body());
    clear_data.reserve_exact(padding + ENCRYPTION_FOOTER_LEN);
    clear_data.extend_from_slice(&[0u8; BLOCK_SIZE - 1][..padding]); // Padding
    clear_data.extend_from_slice(&ENCRYPTION_MAGIC); // Magic
    clear_data.push(padding as u8 + 1); // Wastage count (+1 for it self size)

    debug_assert_eq!(clear_data.len(), len, "incoherent length");
    debug_assert_eq!(clear_data.len() % 8, 0, "data not padded as expected");
    
    // +4 for the prefix.
    dst_packet.set_data_len(clear_data.len() + 4);

    // Unwrapping because we know that source/destination have the same length.
    io::copy(
        &mut Cursor::new(&clear_data[..]), 
        &mut BlowfishWriter::new(Cursor::new(dst_packet.body_mut()), bf),
    ).unwrap();
    
    // Copy the prefix directly because it is clear.
    dst_packet.write_prefix(src_packet.read_prefix());

}


/// Represent a channel between the app and a client with specific socket address.
#[derive(Debug)]
pub struct Channel {
    /// The blowfish key used for encryption of this channel.
    blowfish: Arc<Blowfish>,
    /// The list of acks that are pending for completion. They should be ordered
    /// in the vector, so a simple binary search is enough.
    sent_acks: Vec<u32>,
    /// The list of received acks, it's used for sending. 
    received_acks: Vec<u32>,
    // /// Set to true when an ack should be sent even if not bundle is set.
    auto_ack: bool,
}

impl Channel {

    fn new(blowfish: Arc<Blowfish>) -> Self {
        Self {
            blowfish,
            sent_acks: Vec::new(),
            received_acks: Vec::new(),
            auto_ack: false,
        }
    }

    fn add_sent_ack(&mut self, sequence_num: u32) {

        debug_assert!(
            self.sent_acks.is_empty() || *self.sent_acks.last().unwrap() < sequence_num,
            "sequence number is not ordered"
        );

        self.sent_acks.push(sequence_num);
        println!("[AFTER ADD] sent_acks: {:?}", self.sent_acks);

    }

    fn remove_cumulative_ack(&mut self, ack: u32) {
        
        let discard_offset = match self.sent_acks.binary_search(&ack) {
            Ok(index) => index,
            Err(index) => index,
        };

        self.sent_acks.drain(..discard_offset);
        println!("[AFTER REM] sent_acks: {:?}", self.sent_acks);

    }

    fn add_received_ack(&mut self, sequence_num: u32) {

        match self.received_acks.binary_search(&sequence_num) {
            Ok(_) => {
                // Maybe an error to receive the same ack twice?
            }
            Err(index) => {
                self.received_acks.insert(index, sequence_num);
            }
        }

        println!("[AFTER ADD] received_acks: {:?}", self.received_acks);

    }

    #[inline]
    fn set_auto_ack(&mut self) {
        self.auto_ack = true;
    }

    /// Take the auto ack and disable it anyway.
    #[inline]
    fn take_auto_ack(&mut self) -> bool {
        std::mem::replace(&mut self.auto_ack, false)
    }

    /// Return the last ack that is part of an chain.
    fn get_cumulative_ack(&mut self) -> Option<u32> {

        let first_ack = *self.received_acks.get(0)?;
        let mut cumulative_ack = first_ack;

        for &sequence_num in &self.received_acks[1..] {
            if sequence_num == cumulative_ack + 1 {
                cumulative_ack += 1;
            } else {
                break
            }
        }

        if cumulative_ack > first_ack {
            let diff = cumulative_ack - first_ack;
            self.received_acks.drain(..diff as usize);
        }

        Some(cumulative_ack)

    }

    #[inline]
    fn get_cumulative_ack_exclusive(&mut self) -> Option<u32> {
        self.get_cumulative_ack().map(|n| n + 1)
    }

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
#[derive(Debug, Clone)]
pub enum PacketError {
    /// The packet could not be synchronized from its data.
    Sync(PacketSyncError),
    /// The packet waited too much time to create a bundle.
    BundleTimeout,
    /// The packet should be decrypted but it failed.
    InvalidEncryption,
}
