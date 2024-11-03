//! A special proxy application that can be used to generically forward calls it receives
//! to another application while providing debugging capabilities to inspect the network
//! without being blocking by blowfish cipher.

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::collections::HashMap;
use std::time::Duration;
use std::sync::Arc;
use std::io;

use blowfish::Blowfish;

use tracing::trace;

use crate::net::packet::Packet;
use crate::util::thread::ThreadPoll;
use crate::net::channel::{ChannelIndex, ChannelTracker};
use crate::net::socket::{PacketSocket, decrypt_packet};
use crate::net::bundle::Bundle;
use super::io_invalid_data;


/// The unspecified address used to let the socket allocate its own address.
pub(crate) const UNSPECIFIED_ADDR: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0));

/// The receive timeout on socket, used to ensure that we check that the thread can 
/// continue running.
pub(crate) const RECV_TIMEOUT: Duration = Duration::from_secs(5);


/// The generic proxy application.
#[derive(Debug)]
pub struct App {
    /// Thread poll for socket result.
    socket_poll: ThreadPoll<SocketPollRet>,
    /// The main socket receiving peer packets.
    socket: PacketSocket,
    /// Channel tracker for out packets.
    out_channel: ChannelTracker,
    /// Channel tracker for in packets.
    in_channel: ChannelTracker,
    /// Each peer connected and forwarded. Using an index map because we use the peer's
    /// index as the mio token (-1).
    peers: HashMap<SocketAddr, Arc<Peer>>,
    /// Filled when a peer is rejected and a Rejection event is returned, it allows the
    /// handler of that event to bind the missing peer and allow it to be accepted on
    /// next poll. 
    last_rejection: Option<(Packet, SocketAddr)>,
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
            out_channel: ChannelTracker::new(),
            in_channel: ChannelTracker::new(),
            peers: HashMap::new(),
            last_rejection: None,
        })

    }

    /// Get the address this app is bound to.
    pub fn addr(&self) -> io::Result<SocketAddr> {
        self.socket.addr()
    }

    pub fn bind_peer(&mut self, 
        addr: SocketAddr, 
        real_addr: SocketAddr, 
        blowfish: Option<Arc<Blowfish>>,
        socket: Option<PacketSocket>, 
    ) -> io::Result<()> {

        let socket = match socket {
            Some(socket) => socket,
            None => PacketSocket::bind(UNSPECIFIED_ADDR)?
        };

        socket.set_recv_timeout(Some(RECV_TIMEOUT))?;

        let peer = Arc::new(Peer {
            socket,
            addr,
            real_addr,
            blowfish,
        });

        let thread_peer = Arc::clone(&peer);
        self.socket_poll.spawn(move || Some(SocketPollRet {
            peer: Some(Arc::clone(&thread_peer)),
            res: thread_peer.socket.recv_without_encryption(),
        }));

        self.peers.insert(addr, peer);

        Ok(())
        
    }

    /// Poll for the next event of this login app, blocking.
    pub fn poll(&mut self) -> Event {
        loop {

            let ignore_rejection;
            let socket_poll_ret;
            if let Some((packet, addr)) = self.last_rejection.take() {
                ignore_rejection = true;  // To avoid infinite rejection.
                socket_poll_ret = SocketPollRet {
                    res: Ok((packet, addr)),
                    peer: None,
                };
            } else {
                ignore_rejection = false;
                socket_poll_ret = self.socket_poll.poll();
            }

            let (cipher_packet, addr) = match socket_poll_ret.res {
                Ok(ret) => ret,
                Err(e) if matches!(e.kind(), io::ErrorKind::TimedOut | io::ErrorKind::WouldBlock) => continue,
                Err(e) => {
                    return Event::IoError(IoErrorEvent {
                        error: e,
                        addr: None,
                    });
                }
            };

            let peer;
            let direction;
            let res;
            if let Some(peer_) = &socket_poll_ret.peer {
                peer = &**peer_;
                direction = PacketDirection::In;
                res = self.socket.send_without_encryption(&cipher_packet, peer.addr);
            } else if let Some(peer_) = self.peers.get(&addr) {
                peer = &**peer_;
                direction = PacketDirection::Out;
                res = peer.socket.send_without_encryption(&cipher_packet, peer.real_addr);
            } else {
                if ignore_rejection {
                    continue;
                } else {
                    self.last_rejection = Some((cipher_packet, addr));
                    return Event::Rejection(RejectionEvent {
                        addr,
                    });
                }
            }

            if let Err(e) = res {
                return Event::IoError(IoErrorEvent {
                    error: e,
                    addr: Some(peer.addr),
                });
            }

            let packet;
            if let Some(blowfish) = peer.blowfish.as_deref() {
                packet = match decrypt_packet(cipher_packet, blowfish) {
                    Ok(ret) => ret,
                    Err(_cipher_packet) => {
                        // warn!("invalid encryption, continuing without it...");
                        // cipher_packet
                        // warn!(direction = ?direction, "Cipher packet: {:?}", cipher_packet.raw());
                        return Event::IoError(IoErrorEvent {
                            error: io_invalid_data(format_args!("invalid packet encryption")),
                            addr: Some(addr),
                        });
                    }
                };
            } else {
                packet = cipher_packet;
            }

            match direction {
                PacketDirection::Out => trace!(peer_addr = %peer.addr, real_addr = %peer.real_addr, "> {:?}", packet),
                PacketDirection::In => trace!(peer_addr = %peer.addr, real_addr = %peer.real_addr, "< {:?}", packet),
            }

            let channel = match direction {
                PacketDirection::Out => &mut self.out_channel,
                PacketDirection::In => &mut self.in_channel,
            };

            if let Some((bundle, channel)) = channel.accept(packet, peer.addr) {
                return Event::Bundle(BundleEvent {
                    addr: peer.addr,
                    bundle,
                    direction,
                    channel: channel.is_on().then(|| PacketChannel {
                        index: channel.index(),
                    }),
                })
            }

        }

    }

}

/// An event that happened in the login app regarding the login process.
#[derive(Debug)]
pub enum Event {
    IoError(IoErrorEvent),
    Rejection(RejectionEvent),
    Bundle(BundleEvent),
}

/// The given peer has been rejected because it has not been registered before. Using
/// [`App::bind_peer`] you can fix this rejection and allow the peer to be proxied on 
/// next poll.
#[derive(Debug)]
pub struct RejectionEvent {
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
