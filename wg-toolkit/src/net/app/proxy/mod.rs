//! A special proxy application that can be used to generically forward calls it receives
//! to another application while providing debugging capabilities to inspect the network
//! without being blocking by blowfish cipher.

use std::collections::VecDeque;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;
use std::io;

use blowfish::Blowfish;

use indexmap::IndexMap;

use mio::{Events, Interest, Poll, Token};

use crate::net::channel::{ChannelIndex, ChannelTracker};
use crate::net::socket::{PacketSocket, decrypt_packet};
use crate::net::packet::Packet;
use crate::net::bundle::Bundle;
use super::io_invalid_data;


const UNSPECIFIED_ADDR: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0));
const MAIN_TOKEN: Token = Token(0);


/// The proxy application.
#[derive(Debug)]
pub struct App {
    /// The mio poll used to wait on all 
    poll: Poll,
    /// The mio poll events.
    poll_events: Option<Events>,
    /// The main socket
    main: Main,
    /// Each peer connected and forwarded. Using an index map because we use the peer's
    /// index as the mio token (-1).
    peers: IndexMap<SocketAddr, Peer>,
}

#[derive(Debug)]
struct Main {
    /// Internal socket for this application.
    socket: PacketSocket,
    /// The address to proxy to and from.
    real_addr: SocketAddr,
    /// Queue of events that are waiting to be returned.
    events: VecDeque<Event>,
}

#[derive(Debug)]
struct Peer {
    addr: SocketAddr,
    socket: PacketSocket,
    blowfish: Arc<Blowfish>,
    channel: ChannelTracker,
    /// Pending packets if the peer is not yet writable, should be used rarely at the
    /// very beginning and for a short time.
    pending_packets: Option<Vec<Box<Packet>>>
}

impl App {

    /// Create a new proxy application with the given listening address and the address
    /// to proxy to and from.
    pub fn new(addr: SocketAddr, real_addr: SocketAddr) -> io::Result<Self> {
        
        let poll = Poll::new()?;
        let mut socket = PacketSocket::bind(addr)?;

        poll.registry().register(&mut socket, MAIN_TOKEN, Interest::WRITABLE)?;

        Ok(Self {
            poll,
            poll_events: Some(Events::with_capacity(128)),
            main: Main {
                socket,
                real_addr,
                events: VecDeque::new(),
            },
            peers: IndexMap::new(),
        })

    }

    pub fn register_peer(&mut self, addr: SocketAddr, blowfish: Arc<Blowfish>) -> io::Result<()> {
        
        // Let the UDP socket create a new address.
        let socket = PacketSocket::bind(UNSPECIFIED_ADDR)?;
        
        let (index, _) = self.peers.insert_full(addr, Peer {
            addr,
            socket,
            blowfish,
            channel: ChannelTracker::new(),
            pending_packets: Some(Vec::new()),
        });

        // Don't use the WRITABLE interest, if send would block we add the packet to 
        // pending ones.
        self.poll.registry().register(&mut self.peers[index].socket, Token(index + 1), Interest::READABLE)?;
        Ok(())

    }

    /// Poll for the next event of this login app, blocking.
    pub fn poll(&mut self) -> Event {
        loop {

            // Empty the events before.
            while let Some(event) = self.main.events.pop_front() {
                return event;
            }

            // To avoid borrowing issues.
            let mut events = self.poll_events.take().unwrap();
            match self.poll.poll(&mut events, None) {
                Ok(()) => (),
                Err(error) => return Event::IoError(IoErrorEvent { error, addr: None }),
            }

            for event in &events {
                self.handle_event(event);
            }

            self.poll_events = Some(events);

        }
    }

    fn handle_event(&mut self, event: &mio::event::Event) {
        
        match event.token() {
            MAIN_TOKEN if event.is_readable() || event.is_writable() => {
                if event.is_writable() {
                    self.poll.registry().reregister(&mut self.main.socket, MAIN_TOKEN, Interest::READABLE).unwrap();
                }
                self.handle_main_readable();
            }
            Token(index) if event.is_readable() || event.is_writable() => {
                self.handle_peer_readable(index - 1);
            }
            _ => {}
        }

    }

    fn handle_main_readable(&mut self) {

        loop {

            let (packet, peer_addr) = match self.main.socket.recv_without_encryption() {
                Ok(ret) => ret,
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => break,
                Err(error) => {
                    self.main.events.push_back(Event::IoError(IoErrorEvent {
                        error,
                        addr: None,
                    }));
                    break;
                }
            };
    
            let Some(peer) = self.peers.get_mut(&peer_addr) else {
                self.main.events.push_back(Event::PeerRejection(PeerRejectionEvent { 
                    addr: peer_addr
                }));
                continue;
            };
    
            match peer.handle_out_packet(&mut self.main, packet) {
                Ok(()) => (),
                Err(error) => {
                    self.main.events.push_back(Event::IoError(IoErrorEvent {
                        error,
                        addr: Some(peer_addr),
                    }));
                    continue;
                }
            }

        }

    }

    fn handle_peer_readable(&mut self, index: usize) {

        loop {

            let Some((&peer_addr, peer)) = self.peers.get_index_mut(index - 1) else {
                self.main.events.push_back(Event::IoError(IoErrorEvent { 
                    error: io_invalid_data(format_args!("invalid peer token")), 
                    addr: None,
                }));
                break;
            };
    
            let (packet, app_addr) = match peer.socket.recv_without_encryption() {
                Ok(ret) => ret,
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => break,
                Err(error) => {
                    self.main.events.push_back(Event::IoError(IoErrorEvent {
                        error,
                        addr: Some(peer_addr),
                    }));
                    break;
                }
            };
    
            if app_addr != self.main.real_addr {
                self.main.events.push_back(Event::IoError(IoErrorEvent { 
                    error: io_invalid_data(format_args!("peer received packet from wrong address")), 
                    addr: None,
                }));
                continue;
            }
    
            match peer.handle_in_packet(&mut self.main, packet) {
                Ok(()) => (),
                Err(error) => {
                    self.main.events.push_back(Event::IoError(IoErrorEvent {
                        error,
                        addr: Some(peer_addr),
                    }));
                    continue;
                }
            }

        }

    }

}

impl Peer {

    fn handle_out_packet(&mut self, main: &mut Main, cipher_packet: Box<Packet>) -> io::Result<()> {
        
        // All this machinery to handle WouldBlock...
        if let Some(pending_packets) = &mut self.pending_packets {
            if let Some(pending_cipher_packet) = pending_packets.first() {

                match self.socket.send_without_encryption(pending_cipher_packet.raw(), main.real_addr) {
                    Ok(_) => {}
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                        // Don't try to send the new one, just make it pending.
                        pending_packets.push(cipher_packet);
                        return Ok(());
                    }
                    Err(e) => return Err(e)
                }

                // If sending the first pending packet is successful then all should be 
                // successful, the socket should be ready now. DONT RESEND THE FIRST ONE.
                let pending_packets = self.pending_packets.take().unwrap();
                for (i, cipher_packet) in pending_packets.into_iter().enumerate() {
                    if i != 0 {
                        self.socket.send_without_encryption(cipher_packet.raw(), main.real_addr)?;
                    }
                    self.accept_packet(main, cipher_packet, PacketDirection::Out);
                }
                
            }
        }

        match self.socket.send_without_encryption(cipher_packet.raw(), main.real_addr) {
            Ok(_) => {}
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                if let Some(pending_packets) = &mut self.pending_packets {
                    pending_packets.push(cipher_packet);
                    return Ok(());
                } else {
                    return Err(e);
                }
            }
            Err(e) => return Err(e)
        }

        self.accept_packet(main, cipher_packet, PacketDirection::Out);

        Ok(())

    }

    fn handle_in_packet(&mut self, main: &mut Main, cipher_packet: Box<Packet>) -> io::Result<()> {
        // This should not return WouldBlock because we wait for a WRITABLE interest
        // before actually doing anything with the proxy.
        main.socket.send_without_encryption(cipher_packet.raw(), self.addr)?;
        self.accept_packet(main, cipher_packet, PacketDirection::In);
        Ok(())
    }

    fn accept_packet(&mut self, main: &mut Main, cipher_packet: Box<Packet>, direction: PacketDirection) {

        const FAKE_ADDR_OUT: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 1), 0));
        const FAKE_ADDR_IN: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 2), 0));

        let packet = match decrypt_packet(cipher_packet, &self.blowfish) {
            Ok(packet) => packet,
            Err(_) => {
                main.events.push_back(Event::IoError(IoErrorEvent {
                    error: io_invalid_data(format_args!("invalid packet encryption ({direction:?})")),
                    addr: Some(self.addr),
                }));
                return;
            }
        };

        // Because we are using the same channel (just for reducing memory footprint),
        // we use two fake addresses, one for out-packets and one for in-packets, these
        // addresses have nothing to do with the actual peers or main addresses.
        let fake_addr = match direction {
            PacketDirection::Out => FAKE_ADDR_OUT,
            PacketDirection::In => FAKE_ADDR_IN,
        };

        if let Some((bundle, channel)) = self.channel.accept(packet, fake_addr) {
            main.events.push_back(Event::Bundle(BundleEvent {
                addr: self.addr,
                bundle,
                direction,
                channel: channel.is_on().then(|| PacketChannel {
                    index: channel.index(),
                }),
            }));
        }

    }

}


/// An event that happened in the login app regarding the login process.
#[derive(Debug)]
pub enum Event {
    IoError(IoErrorEvent),
    PeerRejection(PeerRejectionEvent),
    Bundle(BundleEvent),
}

/// The given peer has been rejected because it has not been registered before.
#[derive(Debug)]
pub struct PeerRejectionEvent {
    /// Address of the client that sent a packet.
    pub addr: SocketAddr,
}

/// Some IO error happened internally and optionally related to a client.
#[derive(Debug)]
pub struct IoErrorEvent {
    /// The IO error.
    pub error: io::Error,
    /// An optional client address related to the error.
    pub addr: Option<SocketAddr>,
}

#[derive(Debug)]
pub struct BundleEvent {
    /// Address of the client that sent this bundle.
    pub addr: SocketAddr,
    /// The bundle that has been reconstructed.
    pub bundle: Bundle,
    /// The direction this bundle was intercepted.
    pub direction: PacketDirection,
    /// If the bundle has passed through a channel.
    pub channel: Option<PacketChannel>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketDirection {
    Out,
    In,
}

#[derive(Debug)]
pub struct PacketChannel {
    pub index: Option<ChannelIndex>,
}
