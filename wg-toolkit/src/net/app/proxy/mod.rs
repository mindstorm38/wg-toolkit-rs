//! A special proxy application that can be used to generically forward calls it receives
//! to another application while providing debugging capabilities to inspect the network
//! without being blocking by blowfish cipher.

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::collections::{hash_map, HashMap};
use std::num::NonZero;
use std::time::Duration;
use std::sync::Arc;
use std::io;

use blowfish::Blowfish;

use tracing::{trace, trace_span};

use crate::util::thread::{ThreadPoll, ThreadWorker};
use crate::net::proto::{Channel, ChannelIndex, Protocol};
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
    /// Inner state.
    inner: AppInner,
    /// Each peer connected and forwarded..
    peers: HashMap<SocketAddr, PeerInner>,
}

/// Inner application structure to split from peer.
#[derive(Debug)]
struct AppInner {
    /// Thread poll for socket result.
    socket_poll: ThreadPoll<SocketPollRet>,
    /// The main socket receiving peer packets.
    socket: PacketSocket,
    /// Channel tracker for out packets.
    out_protocol: Protocol,
    /// Channel tracker for in packets.
    in_protocol: Protocol,
}

/// A registered peer that can forward and receive packets from the real application.
#[derive(Debug)]
struct PeerInner {
    /// Handle for drop-destruction of the poll thread worker, only used for drop.
    _socket_worker: ThreadWorker,
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
    peer: Option<SocketAddr>,
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
            res: thread_socket.recv_without_encryption(),
            peer: None,
        }));

        Ok(Self {
            inner: AppInner {
                socket_poll,
                socket,
                out_protocol: Protocol::new(),
                in_protocol: Protocol::new(),
            },
            peers: HashMap::new(),
        })

    }

    /// Get the address this app is bound to.
    pub fn addr(&self) -> io::Result<SocketAddr> {
        self.inner.socket.addr()
    }

    /// Blocking poll of this application with the given handler.
    pub fn poll<H: Handler>(&mut self, handler: &mut H) -> Result<(), H::Error> {

        let socket_poll_ret = self.inner.socket_poll.poll();
        let (cipher_packet, addr) = match socket_poll_ret.res {
            Ok(ret) => ret,
            Err(e) if matches!(e.kind(), io::ErrorKind::TimedOut | io::ErrorKind::WouldBlock) => return Ok(()),
            Err(e) => return Err(e.into()),
        };

        let peer;
        let direction;
        let res;

        if let Some(peer_addr) = &socket_poll_ret.peer {

            // The packet has been received from the real application and should be
            // forwarded to the peer.
            peer = match self.peers.get_mut(peer_addr) {
                Some(peer) => peer,
                None => return Ok(()),  // Receiving event from a no longer existing peer.
            };
            direction = PacketDirection::In;
            res = self.inner.socket.send_without_encryption(&cipher_packet, peer.addr);

        } else {

            // The packet has been received from the peer and should be forwarded to the 
            // real login application.
            peer = match self.peers.entry(addr) {
                hash_map::Entry::Occupied(o) => o.into_mut(),
                hash_map::Entry::Vacant(v) => {

                    // If we have no peer for this, we ask the handler the initial
                    // configuration for this peer. If the handle don't accept the peer
                    // then we just return and ignore the packet.
                    let Some(peer_config) = handler.accept_peer(addr)? else {
                        return Ok(());
                    };

                    let socket = PacketSocket::bind(UNSPECIFIED_ADDR)?;
                    socket.set_recv_timeout(Some(RECV_TIMEOUT))?;

                    let thread_socket = socket.clone();
                    let _socket_worker = self.inner.socket_poll.spawn_with_handle(move || Some(SocketPollRet {
                        res: thread_socket.recv_without_encryption(),
                        peer: Some(addr),
                    }));

                    v.insert(PeerInner {
                        _socket_worker,
                        socket,
                        addr,
                        real_addr: peer_config.real_addr,
                        blowfish: peer_config.blowfish,
                    })

                }
            };
            direction = PacketDirection::Out;
            res = peer.socket.send_without_encryption(&cipher_packet, peer.real_addr);

        }

        // Just ignore the length sent...
        match res {
            Ok(_len) => {}
            Err(e) if e.kind() == io::ErrorKind::ConnectionReset => {
                // When forwarding to a local peer, this means that the local client no
                // longer has a listening socket to send to! Ignore this peer.
                trace!("Dropped peer due to connection reset: {addr}");
                // Unwrap because we have this peer and it exists.
                self.peers.remove(&addr).unwrap();
                return Ok(());
            }
            Err(e) => {
                return Err(e.into());
            }
        }

        // Now decrypt packet if the peer has symmetric encryption...
        let packet;
        if let Some(blowfish) = peer.blowfish.as_deref() {
            packet = match decrypt_packet(cipher_packet, blowfish) {
                Ok(ret) => ret,
                Err(cipher_packet) => {
                    let peer_ref = Peer { 
                        app: &mut self.inner,
                        peer: &mut *peer,
                    };
                    handler.receive_invalid_packet_encryption(peer_ref, cipher_packet, direction)?;
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
            PacketDirection::Out => (
                &mut self.inner.out_protocol, trace_span!("out"), 
                &mut self.inner.in_protocol, trace_span!("in"),
            ),
            PacketDirection::In => (
                &mut self.inner.in_protocol, trace_span!("in"), 
                &mut self.inner.out_protocol, trace_span!("out"),
            ),
        };

        {
            let _span = accept_protocol_span.enter();
            trace!(real_addr = %peer.real_addr, "{:width$?}", packet, width = 0);
        }
        
        {
            let _span = accept_out_protocol_span.enter();
            if !accept_out_protocol.accept_out(&packet, peer.addr) {
                return Ok(());
            }
        }
        
        let _span = accept_protocol_span.enter();
        
        let mut channel = match accept_protocol.accept(packet, peer.addr) {
            Ok(channel) => channel,
            Err(_packet) => return Ok(()),
        };

        let packet_channel = channel.is_on().then(|| PacketChannel {
            index: channel.index(),
        });

        for bundle in channel.pop_bundles() {
            let peer_ref = Peer { 
                app: &mut self.inner,
                peer: &mut *peer,
            };
            handler.receive_bundle(peer_ref, bundle, direction, packet_channel.clone())?;
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
        peer: Peer,
        packet: Packet,
        direction: PacketDirection, 
    ) -> Result<(), Self::Error> {
        let _ = (peer, packet, direction);
        Ok(())
    }

    /// A bundle of elements has been transferred to or from the given peer.
    /// 
    /// The default implementation does nothing.
    fn receive_bundle(&mut self, 
        peer: Peer, 
        bundle: Bundle, 
        direction: PacketDirection, 
        channel: Option<PacketChannel>,
    ) -> Result<(), Self::Error> {
        let _ = (peer, bundle, direction, channel);
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

/// A reference to an internal peer that allows its owner to enable or disable the 
/// encryption, modify the real address and send bundles.
#[derive(Debug)]
pub struct Peer<'a> {
    app: &'a mut AppInner,
    peer: &'a mut PeerInner,
}

impl<'a> Peer<'a> {

    /// Get the address of this peer.
    #[inline]
    pub fn addr(&self) -> SocketAddr {
        self.peer.addr
    }

    /// Get the real address this peer is currently being forwarded to and from.
    #[inline]
    pub fn real_addr(&self) -> SocketAddr {
        self.peer.real_addr
    }

    /// Set the real address this peer will be forwarded to and from.
    #[inline]
    pub fn set_real_addr(&mut self, real_addr: SocketAddr) {
        self.peer.real_addr = real_addr;
    }

    /// Get the currently configured blowfish encryption key for this peer, none if no
    /// encryption is currently used.
    #[inline]
    pub fn blowfish(&self) -> Option<&Blowfish> {
        self.peer.blowfish.as_deref()
    }

    /// Set the blowfish encryption key to be used for this peer, none if encryption
    /// should be disabled.
    #[inline]
    pub fn set_blowfish(&mut self, blowfish: Option<Arc<Blowfish>>) {
        self.peer.blowfish = blowfish;
    }

    /// Get the protocol off-channel.
    pub fn off_channel(&mut self, direction: PacketDirection) -> Channel<'_> {
        match direction {
            PacketDirection::Out => self.app.out_protocol.off_channel(self.peer.addr),
            PacketDirection::In => self.app.in_protocol.off_channel(self.peer.addr),
        }
    }

    /// Get the protocol channel.
    pub fn channel(&mut self, direction: PacketDirection, index: Option<NonZero<u32>>) -> Channel<'_> {
        match direction {
            PacketDirection::Out => self.app.out_protocol.channel(self.peer.addr, index),
            PacketDirection::In => self.app.in_protocol.channel(self.peer.addr, index),
        }
    }

    /// Immediately send a bundle to the given direction.
    pub fn send_bundle(&self, direction: PacketDirection, bundle: &Bundle) -> io::Result<usize> {
        
        let (socket, addr) = match direction {
            PacketDirection::Out => (&self.peer.socket, self.peer.real_addr),
            PacketDirection::In => (&self.app.socket, self.peer.addr),
        };

        if let Some(blowfish) = self.peer.blowfish.as_deref() {
            socket.send_bundle_with_encryption(bundle, addr, blowfish)
        } else {
            socket.send_bundle_without_encryption(bundle, addr)
        }

    }

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
