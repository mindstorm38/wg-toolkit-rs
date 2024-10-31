use std::collections::{hash_map, HashMap, VecDeque};
use std::net::{SocketAddr, SocketAddrV4};
use std::time::{Duration, Instant};
use std::sync::Arc;
use std::io;

use rsa::{RsaPrivateKey, RsaPublicKey};
use crypto_common::KeyInit;
use blowfish::Blowfish;

use tracing::trace;

use crate::net::bundle::{Bundle, ElementReader, ReplyElementReader, TopElementReader};
use crate::net::app::login::element::{ChallengeResponse, CuckooCycleResponse};
use crate::net::app::proxy::{UNSPECIFIED_ADDR, RECV_TIMEOUT};
use crate::net::channel::ChannelTracker;
use crate::net::socket::PacketSocket;
use crate::net::packet::Packet;

use crate::util::thread::{ThreadPoll, ThreadPollHandle};

use super::element::{LoginError, LoginRequest, LoginRequestEncryption, LoginResponse, LoginResponseEncryption, Ping};
use super::io_invalid_data;
use super::id;


const DEAD_PEER_TIMEOUT: Duration = Duration::from_secs(10);


/// The login application.
#[derive(Debug)]
pub struct App {
    /// Internal state.
    inner: Inner,
    /// Map of all peers in the ping or login process.
    peers: HashMap<SocketAddr, Peer>,
}

#[derive(Debug)]
struct Inner {
    /// Pending events.
    events: VecDeque<Event>,
    /// Thread poll for socket result.
    socket_poll: ThreadPoll<SocketPollRet>,
    /// Internal socket for this application.
    socket: PacketSocket,
    /// Optional private key to set if encryption is enabled on the login app. This 
    /// implies that the client should use the matching public key when logging in in
    /// order to validate.
    encryption_key: Option<Arc<RsaPrivateKey>>,
    /// Allows modifying the base app address returned to the client.
    forced_base_app_addr: Option<SocketAddrV4>,
    /// The address of the real application where we proxy all packets.
    real_addr: SocketAddr,
    /// Encryption key for sending to the real login application.
    real_encryption_key: Option<Arc<RsaPublicKey>>,
    /// Channel tracker for accepting out packets and preparing in packets.
    out_channel: ChannelTracker,
    /// Channel tracker for accepting in packets and preparing out packets.
    in_channel: ChannelTracker,
    /// A temporary bundle for sending.
    bundle: Bundle,
}

#[derive(Debug)]
struct Peer {
    /// Handle for drop-destruction of the poll thread worker, only used for drop.
    #[allow(unused)]
    socket_poll_handle: ThreadPollHandle,
    /// The socket represent this peer for the real application.
    socket: PacketSocket,
    /// The address to send packets to the peer when receiving from real application.
    addr: SocketAddr,
    /// The last instant this peer has received any packet.
    last_time: Instant,
    /// Information about the last request made by the client, if any.
    last_request: Option<PeerLastRequest>,
}

#[derive(Debug)]
struct PeerLastRequest {
    request_id: u32,
    kind: PeerLastRequestKind,
}

#[derive(Debug)]
enum PeerLastRequestKind {
    Ping {},
    Login { blowfish: Arc<Blowfish>, },
}

/// Type of return value for our socket poll. 
#[derive(Debug)]
struct SocketPollRet {
    /// The raw I/O result containing the packet if successful.
    res: io::Result<(Box<Packet>, SocketAddr)>,
    /// The peer address if this is the result of a peer socket.
    peer: Option<SocketAddr>,
}

impl App {

    pub fn new(addr: SocketAddr, real_addr: SocketAddr, real_encryption_key: Option<Arc<RsaPublicKey>>) -> io::Result<Self> {
        
        let socket_poll = ThreadPoll::new();

        let socket = PacketSocket::bind(addr)?;
        socket.set_recv_timeout(Some(RECV_TIMEOUT))?;

        let thread_socket = socket.clone();
        socket_poll.spawn(move || Some(SocketPollRet {
            res: thread_socket.recv_without_encryption(),
            peer: None,
        }));

        Ok(Self {
            inner: Inner {
                events: VecDeque::new(),
                socket_poll,
                socket,
                encryption_key: None,
                forced_base_app_addr: None,
                real_addr,
                real_encryption_key,
                out_channel: ChannelTracker::new(),
                in_channel: ChannelTracker::new(),
                bundle: Bundle::new(),
            },
            peers: HashMap::new(),
        })

    }

    /// Get the address this app is bound to.
    pub fn addr(&self) -> io::Result<SocketAddr> {
        self.inner.socket.addr()
    }

    /// Enable encryption on login app, given a RSA private key, the client should use 
    /// the matching public key in order to validate this server.
    pub fn set_encryption(&mut self, key: Arc<RsaPrivateKey>) {
        self.inner.encryption_key = Some(key);
    }

    /// As opposed to [`Self::set_private_key`], disable encryption on login app.
    pub fn remove_encryption(&mut self) {
        self.inner.encryption_key = None;
    }

    /// Return true if encryption is enabled on this login app.
    pub fn has_encryption(&self) -> bool {
        self.inner.encryption_key.is_some()
    }

    pub fn set_forced_base_app_addr(&mut self, addr: SocketAddrV4) {
        self.inner.forced_base_app_addr = Some(addr);
    }

    pub fn remove_forced_base_app_addr(&mut self) {
        self.inner.forced_base_app_addr = None;
    }

    /// Poll for the next event of this login app, blocking.
    pub fn poll(&mut self) -> Event {
        loop {

            // Dropping dead peers, this will also terminate poll threads.
            if !self.peers.is_empty() {
                let now = Instant::now();
                self.peers.retain(|addr, peer| {
                    if now - peer.last_time >= DEAD_PEER_TIMEOUT {
                        trace!("Dropped dead peer: {addr}");
                        false
                    } else {
                        true
                    }
                });
            }

            while let Some(event) = self.inner.events.pop_front() {
                return event;
            }
            
            let socket_poll_ret = self.inner.socket_poll.poll();

            let (packet, addr) = match socket_poll_ret.res {
                Ok(ret) => ret,
                Err(e) if matches!(e.kind(), io::ErrorKind::TimedOut | io::ErrorKind::WouldBlock) => continue,
                Err(e) => {
                    return Event::IoError(IoErrorEvent {
                        error: e,
                        addr: None,
                    });
                }
            };
            
            // debug!("<{}: [{:08X}] {:?}", addr, packet.raw().read_prefix(), packet.raw());

            let now = Instant::now();

            let channel;
            let peer;
            if let Some(peer_addr) = &socket_poll_ret.peer {
                channel = &mut self.inner.in_channel;
                peer = match self.peers.get_mut(peer_addr) {
                    Some(peer) => peer,
                    None => continue, // Ignore if we received an event from a dead peer.
                }
            } else {
                channel = &mut self.inner.out_channel;
                peer = match self.peers.entry(addr) {
                    hash_map::Entry::Occupied(o) => o.into_mut(),
                    hash_map::Entry::Vacant(v) => {
                        
                        fn new_peer_socket() -> io::Result<PacketSocket> {
                            let socket = PacketSocket::bind(UNSPECIFIED_ADDR)?;
                            socket.set_recv_timeout(Some(RECV_TIMEOUT))?;
                            Ok(socket)
                        }

                        let socket = match new_peer_socket() {
                            Ok(socket) => socket,
                            Err(e) => {
                                return Event::IoError(IoErrorEvent {
                                    error: e,
                                    addr: None,
                                });
                            }
                        };

                        let thread_socket = socket.clone();
                        let socket_poll_handle = self.inner.socket_poll.spawn_with_handle(move || Some(SocketPollRet {
                            res: thread_socket.recv_without_encryption(),
                            peer: Some(addr),
                        }));

                        v.insert(Peer {
                            socket_poll_handle,
                            socket,
                            addr,
                            last_time: now,
                            last_request: None,
                        })

                    }
                };
            }

            peer.last_time = now;

            let Some((bundle, _)) = channel.accept(packet, peer.addr) else {
                continue;
            };

            if socket_poll_ret.peer.is_some() {
                self.inner.handle_in(bundle, peer).unwrap();
            } else {
                self.inner.handle_out(bundle, peer).unwrap();
            }

        }
    }

}

impl Inner {

    fn handle_out(&mut self, bundle: Bundle, peer: &mut Peer) -> io::Result<()> {
        
        self.bundle.clear();

        let mut reader = bundle.element_reader();
        while let Some(reader) = reader.next_element() {
            match reader {
                ElementReader::Top(reader) => 
                    self.handle_out_element(reader, peer)?,
                ElementReader::Reply(reader) => 
                    return Err(io_invalid_data(format_args!("unexpected reply #{}", reader.request_id()))),
            }
        }

        if !self.bundle.is_empty() {
            self.in_channel.off_channel(peer.addr).prepare(&mut self.bundle, false);
            // for packet in self.bundle.packets() {
            //     debug!(">{}: [{:08X}] {:?}", self.real_addr, packet.raw().read_prefix(), packet.raw());
            // }
            peer.socket.send_bundle_without_encryption(&self.bundle, self.real_addr)?;
        }

        Ok(())

    }

    fn handle_out_element(&mut self, reader: TopElementReader, peer: &mut Peer) -> io::Result<()> {
        match reader.id() {
            id::PING => self.handle_out_ping(reader, peer),
            id::LOGIN_REQUEST => self.handle_login_request(reader, peer),
            id::CHALLENGE_RESPONSE => self.handle_challenge_response(reader, peer),
            id => Err(io_invalid_data(format_args!("unexpected element #{id}"))),
        }
    }

    /// Handle a ping request to the login node, we answer as fast as possible.
    fn handle_out_ping(&mut self, elt: TopElementReader, peer: &mut Peer) -> io::Result<()> {

        let ping = elt.read_simple::<Ping>()?;
        let request_id = ping.request_id
            .ok_or_else(|| io_invalid_data(format_args!("ping should be a request")))?;

        peer.last_request = Some(PeerLastRequest {
            request_id,
            kind: PeerLastRequestKind::Ping {  },
        });
        
        self.bundle.element_writer().write_simple_request(id::PING, ping.element, request_id);

        Ok(())

    }

    /// Handle a login request to the login node.
    fn handle_login_request(&mut self, elt: TopElementReader, peer: &mut Peer) -> io::Result<()> {
        
        let recv_encryption = self.encryption_key.as_ref()
            .map(|key| LoginRequestEncryption::Server(Arc::clone(&key)))
            .unwrap_or(LoginRequestEncryption::Clear);

        let login = elt.read::<LoginRequest>(&recv_encryption)?;
        
        let request_id = login.request_id
            .ok_or_else(|| io_invalid_data(format_args!("login should be a request")))?;

        let blowfish = Arc::new(Blowfish::new_from_slice(&login.element.blowfish_key)
            .map_err(|_| io_invalid_data(format_args!("login has invalid blowfish key: {:?}", login.element.blowfish_key)))?);

        peer.last_request = Some(PeerLastRequest {
            request_id,
            kind: PeerLastRequestKind::Login { blowfish },
        });

        let send_encryption = self.real_encryption_key.as_ref()
            .map(|key| LoginRequestEncryption::Client(Arc::clone(&key)))
            .unwrap_or(LoginRequestEncryption::Clear);

        self.bundle.element_writer().write_request(id::LOGIN_REQUEST, login.element.clone(), &send_encryption, request_id);

        Ok(())

    }

    fn handle_challenge_response(&mut self, elt: TopElementReader, _peer: &mut Peer) -> io::Result<()> {
        let challenge = elt.read_simple::<ChallengeResponse<CuckooCycleResponse>>()?;
        self.bundle.element_writer().write_simple(id::CHALLENGE_RESPONSE, challenge.element);
        Ok(())
    }

    fn handle_in(&mut self, bundle: Bundle, peer: &mut Peer) -> io::Result<()> {

        self.bundle.clear();
        
        // We currently know how to compute the prefix of packets, but the official login
        // application is returning a slightly wrong (from our POV) prefix ONLY on 
        // successful login responses, and we don't know yet how it produces it.
        // This prefix is also always the same from what have been observed (64C20486).
        // From the client decompilation attempt it looks like the client is aware of 
        // that and in case of successful login it use the latest received prefix (the
        // one previously mentioned) as the prefix offset for the rest of the 
        // communications with the base app.
        //
        // As a temporary fix, we set this flag to true only on successful login, and
        // in this case we brainlessly inherit the prefix.
        let mut inherit_prefix = false;

        let mut reader = bundle.element_reader();
        while let Some(reader) = reader.next_element() {
            match reader {
                ElementReader::Top(reader) => 
                    return Err(io_invalid_data(format_args!("unexpected element #{}", reader.id()))),
                ElementReader::Reply(reader) => 
                    self.handle_in_reply(reader, peer, &mut inherit_prefix)?,
            }
        }

        if !self.bundle.is_empty() {
            self.out_channel.off_channel(peer.addr).prepare(&mut self.bundle, false);
            for packet in self.bundle.packets_mut() {
                if inherit_prefix {
                    packet.raw_mut().write_prefix(self.in_channel.last_accepted_prefix());
                }
                // debug!(">{}: [{:08X}] {:?}", peer.addr, packet.raw().read_prefix(), packet.raw());
            }
            self.socket.send_bundle_without_encryption(&self.bundle, peer.addr)?;
        }

        Ok(())

    }

    fn handle_in_reply(&mut self, elt: ReplyElementReader, peer: &mut Peer, inherit_prefix: &mut bool) -> io::Result<()> {
        
        let request_id = elt.request_id();
        if peer.last_request.as_ref().map(|l| l.request_id) != Some(request_id) {
            return Err(io_invalid_data(format_args!("unexpected reply #{}", request_id)));
        }

        let last_request = peer.last_request.take().unwrap();
        let latency = peer.last_time.elapsed();

        match last_request.kind {
            PeerLastRequestKind::Ping {  } => {

                self.events.push_back(Event::Ping(PingEvent {
                    addr: peer.addr,
                    latency,
                }));

                let ping = elt.read_simple::<Ping>()?;
                self.bundle.element_writer().write_simple_reply(ping.element, request_id);
                
            }
            PeerLastRequestKind::Login { blowfish } => {

                let encryption = LoginResponseEncryption::Encrypted(Arc::clone(&blowfish));
                let mut login = elt.read::<LoginResponse>(&encryption)?;
                
                if let LoginResponse::Success(success) = &mut login.element {

                    *inherit_prefix = true;
                    self.events.push_back(Event::LoginSuccess(LoginSuccessEvent {
                        addr: peer.addr,
                        blowfish,
                        real_base_app_addr: success.addr,
                        login_key: success.login_key,
                        server_message: success.server_message.clone(),
                    }));

                    // Change the base app just after the event, so the event still get the
                    // non-forced address.
                    if let Some(base_app_addr) = self.forced_base_app_addr {
                        success.addr = base_app_addr;
                    }
                    
                } else if let LoginResponse::Error(error, data) = &login.element {
                    
                    self.events.push_back(Event::LoginError(LoginErrorEvent {
                        addr: peer.addr,
                        error: *error,
                        data: data.clone(),
                    }));

                }

                self.bundle.element_writer().write_reply(login.element, &encryption, request_id);
                
            }
        }

        Ok(())

    }

}

/// An event that happened in the proxy login app regarding the login process.
#[derive(Debug)]
pub enum Event {
    IoError(IoErrorEvent),
    Ping(PingEvent),
    LoginSuccess(LoginSuccessEvent),
    LoginError(LoginErrorEvent),
}

/// Some IO error happened internally and optionally related to a client.
#[derive(Debug)]
pub struct IoErrorEvent {
    /// The IO error.
    pub error: io::Error,
    /// An optional client address related to the error.
    pub addr: Option<SocketAddr>,
}

/// A client has pinged the login app.
#[derive(Debug)]
pub struct PingEvent {
    /// The address of the client that pinged the login app.
    pub addr: SocketAddr,
    /// Duration between proxy forwarding the ping packet to the real login application
    /// and the response being received, this is basically the latency of the real login
    /// application with a bit of internal latency of this proxy.
    pub latency: Duration,
}

/// A client has successfully logged in the real login application.
#[derive(Debug)]
pub struct LoginSuccessEvent {
    /// The address of the client that successfully logged in.
    pub addr: SocketAddr,
    /// The blowfish key the client sent with its login request and used to decode any
    /// successful response, but also for any input/output packet with the base app.
    pub blowfish: Arc<Blowfish>,
    /// The address of the base app that was answered by the real server, if any base
    /// app address is forced then this value is still the value of the real server.
    pub real_base_app_addr: SocketAddrV4,
    /// The login key returned, used to authenticate to the base app.
    pub login_key: u32,
    /// The server message returned with the login success, usually a stringified JSON.
    pub server_message: String,
}

#[derive(Debug)]
pub struct LoginErrorEvent {
    /// The address of the client that successfully logged in.
    pub addr: SocketAddr,
    pub error: LoginError,
    pub data: String,
}
