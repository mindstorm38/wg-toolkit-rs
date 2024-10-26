//! A special proxy application that can be used to generically forward calls it receives
//! to another application while providing debugging capabilities to inspect the network
//! without being blocking by blowfish cipher.

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::HashMap;
use std::time::Duration;
use std::{io, thread};
use std::sync::Arc;

use crossbeam_channel::{Receiver, Sender};
use blowfish::Blowfish;

use crate::net::channel::{ChannelIndex, ChannelTracker};
use crate::net::socket::{PacketSocket, decrypt_packet};
use crate::net::packet::Packet;
use crate::net::bundle::Bundle;
use super::io_invalid_data;


/// The unspecified address used to let the socket allocate its own address.
const UNSPECIFIED_ADDR: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0));
/// The receive timeout on socket, used to ensure that we check that the thread can 
/// continue running.
const RECV_TIMEOUT: Duration = Duration::from_secs(5);


/// The proxy application.
#[derive(Debug)]
pub struct App {
    /// Thread poll for socket result.
    socket_poll: ThreadPoll<SocketPollRet>,
    /// The main socket receiving peer packets.
    socket: PacketSocket,
    /// The address of the real application where we proxy all packets.
    real_addr: SocketAddr,
    /// Channel tracker for out packets.
    out_channel: ChannelTracker,
    /// Channel tracker for in packets.
    in_channel: ChannelTracker,
    /// Each peer connected and forwarded. Using an index map because we use the peer's
    /// index as the mio token (-1).
    peers: HashMap<SocketAddr, Arc<Peer>>,
}

/// A registered peer that can forward and receive packets from the real application.
#[derive(Debug)]
struct Peer {
    /// The socket represent this peer for the real application.
    socket: PacketSocket,
    /// The address to send packets to the peer when receiving from real application.
    addr: SocketAddr,
    /// Encryption key for this peer.
    blowfish: Arc<Blowfish>,
}

/// Type of return value for our socket poll. 
#[derive(Debug)]
struct SocketPollRet {
    /// The raw I/O result containing the packet if successful.
    res: io::Result<(Box<Packet>, SocketAddr)>,
    /// The peer address if this is the result of a peer socket.
    peer: Option<Arc<Peer>>,
}

impl App {

    /// Create a new proxy application with the given listening address and the address
    /// to proxy to and from.
    pub fn new(addr: SocketAddr, real_addr: SocketAddr) -> io::Result<Self> {
        
        let socket_poll = ThreadPoll::new();

        let socket = PacketSocket::bind(addr)?;
        socket.set_recv_timeout(Some(RECV_TIMEOUT))?;

        let thread_socket = socket.try_clone()?;
        socket_poll.spawn(move || SocketPollRet {
            peer: None,
            res: thread_socket.recv_without_encryption(),
        });

        Ok(Self {
            socket_poll,
            socket,
            real_addr,
            out_channel: ChannelTracker::new(),
            in_channel: ChannelTracker::new(),
            peers: HashMap::new(),
        })

    }

    pub fn bind_peer(&mut self, addr: SocketAddr, blowfish: Arc<Blowfish>) -> io::Result<()> {
        
        let socket = PacketSocket::bind(UNSPECIFIED_ADDR)?;
        socket.set_recv_timeout(Some(RECV_TIMEOUT))?;

        let thread_socket = socket.try_clone()?;

        let peer = Arc::new(Peer {
            socket,
            addr,
            blowfish,
        });

        let thread_peer = Arc::clone(&peer);
        self.socket_poll.spawn(move || SocketPollRet {
            peer: Some(Arc::clone(&thread_peer)),
            res: thread_socket.recv_without_encryption(),
        });

        self.peers.insert(addr, peer);

        Ok(())

    }

    /// Poll for the next event of this login app, blocking.
    pub fn poll(&mut self) -> Event {
        loop {

            let socket_poll_ret = self.socket_poll.poll();

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
                res = self.socket.send_without_encryption(cipher_packet.raw(), peer.addr);
            } else if let Some(peer_) = self.peers.get(&addr) {
                peer = &**peer_;
                direction = PacketDirection::Out;
                res = peer.socket.send_without_encryption(cipher_packet.raw(), self.real_addr);
            } else {
                return Event::Rejection(RejectionEvent {
                    addr,
                });
            }

            if let Err(e) = res {
                return Event::IoError(IoErrorEvent {
                    error: e,
                    addr: Some(peer.addr),
                });
            }

            let packet = match decrypt_packet(cipher_packet, &peer.blowfish) {
                Ok(ret) => ret,
                Err(_) => {
                    return Event::IoError(IoErrorEvent {
                        error: io_invalid_data(format_args!("invalid packet encryption")),
                        addr: Some(addr),
                    });
                }
            };

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

/// The given peer has been rejected because it has not been registered before.
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

/// This structure is made to block on multiple thread at the same time and repeatedly
/// in order to aggregate the value they are returning.
#[derive(Debug)]
pub struct ThreadPoll<T> {
    tx: Sender<T>,
    rx: Receiver<T>,
    alive: Arc<AtomicBool>,
}

impl<T: Send + 'static> ThreadPoll<T> {

    pub fn new() -> Self {
        let (tx, rx) = crossbeam_channel::bounded(2);
        Self {
            tx, rx,
            alive: Arc::new(AtomicBool::new(true)),
        }
    }

    /// Spawn a new value producer that will be continuously polled and its result will
    /// be added to the internal queue that can be retrieved with [`Self::poll`], this
    /// producer's thread terminates when this aggregator is dropped. In order for this
    /// to properly work you should be using so kind of timeout on the producer.
    pub fn spawn<F>(&self, mut producer: F) -> ThreadPollHandle
    where 
        F: FnMut() -> T,
        F: Send + 'static,
    {

        let tx = self.tx.clone();
        let alive = Arc::clone(&self.alive);
        let handle_alive = Arc::new(AtomicBool::new(true));
        let handle = ThreadPollHandle {
            alive: Arc::clone(&handle_alive)
        };

        thread::Builder::new()
            .name(format!("Aggregator"))
            .spawn(move || {
                while alive.load(Ordering::Relaxed) && handle_alive.load(Ordering::Relaxed) {
                    // Deliberately ignoring potential error if the channel has been closed
                    // since we .
                    let _ = tx.send(producer());
                }
            })
            .unwrap();
        
        handle

    }

    /// Block until a new value is available.
    pub fn poll(&self) -> T {
        // Unwrap because we own both ends so it should not disconnect.
        self.rx.recv().unwrap()
    }

    /// Non-blocking poll.
    pub fn try_poll(&self) -> Option<T> {
        // Don't care of the "disconnected" error because it should not happen.
        self.rx.try_recv().ok()
    }

}

impl<T> Drop for ThreadPoll<T> {
    fn drop(&mut self) {
        self.alive.store(false, Ordering::Relaxed);
    }
}

#[derive(Debug, Clone)]
pub struct ThreadPollHandle {
    alive: Arc<AtomicBool>,
}

impl ThreadPollHandle {
    
    pub fn terminate(&self) {
        self.alive.store(false, Ordering::Relaxed);
    }

}
