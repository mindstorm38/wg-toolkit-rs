//! Providing basic app structure.

use std::net::SocketAddr;
use std::time::Duration;
use std::io;

use mio::{Events, Poll, Interest, Token};
use mio::net::UdpSocket;

use super::packet::{Packet, PacketSyncError};
use super::bundle::{BundleAssembler, Bundle};


const COMMON_EVENT: Token = Token(0);


/// A base structure for network applications.
pub struct App {
    /// The socket used for sending and receiving UDP packets.
    socket: UdpSocket,
    /// Socket poll.
    socket_poll: Poll,
    /// Socket events.
    socket_events: Events,
    /// The structure used to re-assemble bundles from received packets.
    /// We associate a socket address used as packet origin.
    bundle_assembler: BundleAssembler<SocketAddr>,
    /// If all bundles and packets should have the 4-bytes prefix.
    has_prefix: bool,
    /// The next sequence ID to use for bundles.
    next_seq_id: u32,
}

impl App {

    pub fn new(addr: SocketAddr, has_prefix: bool) -> io::Result<Self> {

        let mut socket = UdpSocket::bind(addr)?;
        let socket_poll = Poll::new()?;

        socket_poll.registry().register(&mut socket, COMMON_EVENT, Interest::READABLE)?;

        Ok(Self {
            socket,
            socket_poll,
            socket_events: Events::with_capacity(128),
            bundle_assembler: BundleAssembler::new(has_prefix),
            has_prefix,
            next_seq_id: 0,
        })

    }

    /// Send a bundle to a given address. Note that the bundle is finilized by
    /// this method with the internal sequence id.
    pub fn send(&mut self, bundle: &mut Bundle, to: SocketAddr) -> io::Result<usize> {

        bundle.finalize(&mut self.next_seq_id);

        let mut size = 0;
        for packet in bundle.get_packets() {
            size += self.socket.send_to(packet.get_raw_data(), to)?;
        }

        Ok(size)

    }

    /// Poll events from this application.
    pub fn poll(&mut self, events: &mut Vec<Event>, timeout: Option<Duration>) -> io::Result<()> {

        self.socket_poll.poll(&mut self.socket_events, timeout)?;

        events.clear();
        
        for event in self.socket_events.iter() {
            if event.token() == COMMON_EVENT && event.is_readable() {

                loop {

                    let mut packet = Packet::new_boxed(self.has_prefix);
                    
                    let (len, addr) = match self.socket.recv_from(packet.get_raw_data_mut()) {
                        Ok(t) => t,
                        Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                        Err(e) => return Err(e),
                    };

                    if let Err(error) = packet.sync_state(len) {
                        events.push(Event::new(addr, EventKind::InvalidPacket { 
                            error,
                            packet,
                        }));
                    } else if let Some(bundle) = self.bundle_assembler.try_assemble(addr, packet) {
                        events.push(Event::new(addr, EventKind::Bundle(bundle)));
                    }

                }

            }
        }

        events.extend(self.bundle_assembler.drain_old()
            .into_iter()
            .map(|(addr, p)| Event::new(addr, EventKind::DiscardedPacket(p))));

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
    /// A received packet is invalid and therefore the packet cannot
    /// be de-fragmented to a bundle.
    InvalidPacket {
        /// The synchronization error.
        error: PacketSyncError,
        /// The raw packet that could not be synchronized.
        packet: Box<Packet>,
    },
    /// A packet that should have made a bundle but have timed out.
    DiscardedPacket(Box<Packet>)
}
