//! A special proxy application that can be used to generically forward calls it receives
//! to another application while providing debugging capabilities to inspect the network
//! without being blocking by blowfish cipher.

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::collections::HashMap;
use std::time::Duration;
use std::sync::Arc;
use std::io;

use blowfish::Blowfish;

use tracing::{trace, trace_span};

use crate::util::thread::ThreadPoll;
use crate::net::proto::{ChannelIndex, Protocol};
use crate::net::socket::{PacketSocket, decrypt_packet};
use crate::net::packet::Packet;
use crate::net::bundle::Bundle;


/// The unspecified address used to let the socket allocate its own address.
pub(super) const UNSPECIFIED_ADDR: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0));

/// The receive timeout on socket, used to ensure that we check that the thread can 
/// continue running.
pub(super) const RECV_TIMEOUT: Duration = Duration::from_secs(5);


/// The generic proxy application.
#[derive(Debug)]
pub struct App {
    /// Thread poll for socket result.
    socket_poll: ThreadPoll<SocketPollRet>,
    /// The main socket receiving peer packets.
    socket: PacketSocket,
    /// Channel tracker for out packets.
    out_protocol: Protocol,
    /// Channel tracker for in packets.
    in_protocol: Protocol,
    /// Each peer connected and forwarded..
    peers: HashMap<SocketAddr, Arc<Peer>>,
}

/// A registered peer that can forward and receive packets from the real application.
#[derive(Debug)]
struct Peer {
    /// The socket represent this peer for the real application.
    socket: PacketSocket,
    /// The address to send packets to the peer when receiving from real application.
    addr: SocketAddr,
    /// Real address of the base server to communicate with.
    real_addr: SocketAddr,
    /// Encryption key for this peer.
    blowfish: Option<Arc<Blowfish>>,
}

/// Type of return value for our socket poll. 
#[derive(Debug)]
struct SocketPollRet {
    /// The raw I/O result containing the packet if successful.
    res: io::Result<(Packet, SocketAddr)>,
    /// The peer address if this is the result of a peer socket.
    peer: Option<Arc<Peer>>,
}

impl App {

    /// Create a new proxy application with the given listening address and the address
    /// to proxy to and from.
    pub fn new(addr: SocketAddr) -> io::Result<Self> {
        
        let socket_poll = ThreadPoll::new();

        let socket = PacketSocket::bind(addr)?;
        socket.set_recv_timeout(Some(RECV_TIMEOUT))?;

        let thread_socket = socket.clone();
        socket_poll.spawn(move || Some(SocketPollRet {
            peer: None,
            res: thread_socket.recv_without_encryption(),
        }));

        Ok(Self {
            socket_poll,
            socket,
            out_protocol: Protocol::new(),
            in_protocol: Protocol::new(),
            peers: HashMap::new(),
        })

    }

    /// Get the address this app is bound to.
    pub fn addr(&self) -> io::Result<SocketAddr> {
        self.socket.addr()
    }

    /// Blocking poll of this application with the given handler.
    pub fn poll<H: Handler>(&mut self, handler: &mut H) -> Result<(), H::Error> {

        let socket_poll_ret = self.socket_poll.poll();
        let (cipher_packet, addr) = match socket_poll_ret.res {
            Ok(ret) => ret,
            Err(e) if matches!(e.kind(), io::ErrorKind::TimedOut | io::ErrorKind::WouldBlock) => return Ok(()),
            Err(e) => return Err(e.into()),
        };

        let peer;
        let direction;
        let res;

        if let Some(peer_) = &socket_poll_ret.peer {

            // The packet has been received from the real application and should be
            // forwarded to the peer.
            peer = &**peer_;
            direction = PacketDirection::In;
            res = self.socket.send_without_encryption(&cipher_packet, peer.addr);

        } else if let Some(peer_) = self.peers.get(&addr) {

            // The packet has been received from the peer and should be forwarded to the 
            // real application.
            peer = &**peer_;
            direction = PacketDirection::Out;
            res = peer.socket.send_without_encryption(&cipher_packet, peer.real_addr);

        } else if let Some(new_peer) = handler.accept_peer(addr)? {
            
            // The packet has been received from a peer that we don't know yet, but
            // the handler has accepted it and to we register it!
            
            let socket = PacketSocket::bind(UNSPECIFIED_ADDR)?;
            socket.set_recv_timeout(Some(RECV_TIMEOUT))?;
            
            let peer_ = Arc::new(Peer {
                socket,
                addr,
                real_addr: new_peer.real_addr,
                blowfish: new_peer.blowfish,
            });
            
            let thread_peer = Arc::clone(&peer_);
            self.socket_poll.spawn(move || Some(SocketPollRet {
                peer: Some(Arc::clone(&thread_peer)),
                res: thread_peer.socket.recv_without_encryption(),
            }));

            let peer_ = self.peers.entry(addr).insert_entry(peer_).into_mut();
            peer = &**peer_;
            direction = PacketDirection::Out;
            res = peer.socket.send_without_encryption(&cipher_packet, peer.real_addr);

        } else {
            // The peer has been rejected! Just ignore it because the handler should
            // already be aware because it has rejected it.
            return Ok(());
        }

        // Just ignore the length sent...
        let _len = res?;

        // Now decrypt packet if the peer has symmetric encryption...
        let packet;
        if let Some(blowfish) = peer.blowfish.as_deref() {
            packet = match decrypt_packet(cipher_packet, blowfish) {
                Ok(ret) => ret,
                Err(cipher_packet) => {
                    handler.receive_invalid_packet_encryption(addr, cipher_packet, direction)?;
                    return Ok(());
                }
            };
        } else {
            packet = cipher_packet;
        }

        let (
            accept_protocol, 
            accept_protocol_span,
            accept_out_protocol,
            accept_out_protocol_span,
        ) = match direction {
            PacketDirection::Out => (&mut self.out_protocol, trace_span!("out"), &mut self.in_protocol, trace_span!("in")),
            PacketDirection::In => (&mut self.in_protocol, trace_span!("in"), &mut self.out_protocol, trace_span!("out")),
        };

        let span = accept_protocol_span.enter();
        trace!(real_addr = %peer.real_addr, "{:width$?}", packet, width = 0);
        drop(span);
        
        let span = accept_out_protocol_span.enter();
        if !accept_out_protocol.accept_out(&packet, peer.addr) {
            return Ok(());
        }
        drop(span);
        
        let _span = accept_protocol_span.enter();
        let Some(mut channel) = accept_protocol.accept(packet, peer.addr) else {
            return Ok(());
        };

        let packet_channel = channel.is_on().then(|| PacketChannel {
            index: channel.index(),
        });

        while let Some(bundle) = channel.next_bundle() {
            handler.receive_bundle(addr, bundle, direction, packet_channel.clone())?;
        }

        Ok(())

    }

    /// Same as [`Self::loop`] but indefinitely looping until an error is returned.
    pub fn run<H: Handler>(&mut self, mut handler: H) -> Result<(), H::Error> {
        loop {
            self.poll(&mut handler)?;
        }
    }

}

/// A handler for events when polling the application.
pub trait Handler {

    /// The error type that should be able to be constructed from I/O error.
    type Error: From<io::Error>;

    /// The given peer is currently unknown and should be configured. This handler should
    /// return none to ignore that new peer.
    /// 
    /// The default implementation reject all peers.
    fn accept_peer(&mut self, 
        addr: SocketAddr,
    ) -> Result<Option<PeerConfig>, Self::Error> {
        let _ = (addr,);
        Ok(None)
    }

    /// The given peer has received or sent a packet with invalid encryption, and so it
    /// cannot be handled by protocol to be made into a bundle.
    /// 
    /// The default implementation does nothing.
    fn receive_invalid_packet_encryption(&mut self,
        addr: SocketAddr,
        packet: Packet,
        direction: PacketDirection, 
    ) -> Result<(), Self::Error> {
        let _ = (addr, packet, direction);
        Ok(())
    }

    /// A bundle of elements has been transferred to or from the given peer.
    /// 
    /// The default implementation does nothing.
    fn receive_bundle(&mut self, 
        addr: SocketAddr, 
        bundle: Bundle, 
        direction: PacketDirection, 
        channel: Option<PacketChannel>,
    ) -> Result<(), Self::Error> {
        let _ = (addr, bundle, direction, channel);
        Ok(())
    }

}

/// Blanket impl.
impl Handler for () {
    type Error = io::Error;
}

/// The type returned for new peers when accepted by the handler.
#[derive(Debug)]
pub struct PeerConfig {
    /// The real server address to forward the packets to after being received by proxy.
    pub real_addr: SocketAddr, 
    /// If this peer should use symmetric blowfish encryption for its packets, this will
    /// be used to intercept and read the clear packets before building the bundles.
    pub blowfish: Option<Arc<Blowfish>>,
}

/// Represent the forwarding direction of a packet or bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketDirection {
    /// From the peer to the real application.
    Out,
    /// From the real application to the peer.
    In,
}

/// Represent the optional channel used by a packet.
#[derive(Debug, Clone)]
pub struct PacketChannel {
    /// If the channel is indexed, this represent the index and the version of it.
    pub index: Option<ChannelIndex>,
}
