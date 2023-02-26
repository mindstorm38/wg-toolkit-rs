//! Providing basic app structure.

use std::collections::HashMap;
use std::net::{SocketAddr, SocketAddrV4};
use std::sync::Arc;
use std::time::Duration;
use std::io::{self, Cursor};

use blowfish::Blowfish;
use mio::{Events, Poll, Interest, Token};
use mio::net::UdpSocket;

use super::packet::{Packet, PacketSyncError, PACKET_BODY_OFFSET};
use super::bundle::{BundleAssembler, Bundle};
use super::filter::blowfish::BlowfishReader;

use crate::util::BytesFmt;


const COMMON_EVENT: Token = Token(0);


/// A base structure for network applications. This basically provides
/// a practical interface for the UDP server where packets are automatically
/// built into bundles. It also provides sequence id when sending bundles
/// that contains more than one packet.
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

    /// Send a bundle to a given address. Note that the bundle is finilized by
    /// this method with the internal sequence id.
    pub fn send(&mut self, bundle: &mut Bundle, to: SocketAddr) -> io::Result<usize> {

        bundle.finalize(&mut self.next_seq_id);

        let mut size = 0;
        for packet in bundle.packets() {
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
                    
                    let (len, addr) = match self.socket.recv_from(packet.raw_data_mut()) {
                        Ok(t) => t,
                        Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                        Err(e) => return Err(e),
                    };

                    if let Some(bf) = self.channels.get(&addr) {
                        
                        let mut clear_packet = Packet::new_boxed();

                        // Decrypt the incoming packet into the new clear packet.
                        // We don't need to set the length yet because this packet 
                        // will be synchronized just after
                        let src = &packet.raw_data()[PACKET_BODY_OFFSET..len];
                        let dst = &mut clear_packet.raw_data_mut()[PACKET_BODY_OFFSET..len];
                        
                        assert!(src.len() % 8 == 0);

                        io::copy(
                            &mut BlowfishReader::new(Cursor::new(src), &bf), 
                            &mut Cursor::new(dst),
                        ).unwrap();

                        // Copy the prefix and flags directly because they are clear.
                        clear_packet.raw_data_mut()[..PACKET_BODY_OFFSET]
                            .copy_from_slice(&packet.raw_data()[..PACKET_BODY_OFFSET]);

                        println!("raw:   {:X}", BytesFmt(&packet.raw_data()[..len]));
                        println!("clear: {:X}", BytesFmt(&clear_packet.raw_data()[..len]));

                        // The packet is now clear, replace the old encrypted one.
                        packet = clear_packet;

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
}
