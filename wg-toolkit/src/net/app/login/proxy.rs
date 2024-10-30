use std::collections::{hash_map, HashMap, VecDeque};
use std::net::{SocketAddr, SocketAddrV4};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::io;

use rsa::{RsaPrivateKey, RsaPublicKey};
use crypto_common::KeyInit;
use blowfish::Blowfish;

use tracing::debug;

use crate::net::app::login::element::{ChallengeResponse, CuckooCycleResponse};
use crate::net::bundle::{Bundle, ElementReader, ReplyElementReader, TopElementReader};
use crate::net::app::proxy::{UNSPECIFIED_ADDR, RECV_TIMEOUT};
use crate::net::channel::ChannelTracker;
use crate::net::socket::PacketSocket;
use crate::util::thread::ThreadPoll;
use crate::net::packet::Packet;

use super::element::{LoginError, LoginRequest, LoginRequestEncryption, LoginResponse, LoginResponseEncryption, Ping};
use super::io_invalid_data;
use super::id;


/// The login application.
#[derive(Debug)]
pub struct App {
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
    peers: HashMap<SocketAddr, Arc<Peer>>,
}

#[derive(Debug)]
struct Peer {
    /// The socket represent this peer for the real application.
    socket: PacketSocket,
    /// The address to send packets to the peer when receiving from real application.
    addr: SocketAddr,
    mutable: Mutex<PeerMut>,
}

#[derive(Debug)]
struct PeerMut {
    last_ping: Option<PeerLastPing>,
    last_login: Option<PeerLastLogin>,
}

#[derive(Debug)]
struct PeerLastPing {
    request_id: u32,
    time: Instant,
}

#[derive(Debug)]
struct PeerLastLogin {
    request_id: u32,
    blowfish: Arc<Blowfish>,
    username: String,
    context: String,
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

    pub fn new(addr: SocketAddr, real_addr: SocketAddr, real_encryption_key: Option<Arc<RsaPublicKey>>) -> io::Result<Self> {
        
        let socket_poll = ThreadPoll::new();

        let socket = PacketSocket::bind(addr)?;
        socket.set_recv_timeout(Some(RECV_TIMEOUT))?;

        let thread_socket = socket.try_clone()?;
        socket_poll.spawn(move || SocketPollRet {
            peer: None,
            res: thread_socket.recv_without_encryption(),
        });

        Ok(Self {
            events: VecDeque::new(),
            socket_poll,
            socket,
            encryption_key: None,
            real_addr,
            real_encryption_key,
            out_channel: ChannelTracker::new(),
            in_channel: ChannelTracker::new(),
            bundle: Bundle::new(),
            peers: HashMap::new(),
        })

    }

    /// Get the address this app is bound to.
    pub fn addr(&self) -> io::Result<SocketAddr> {
        self.socket.addr()
    }

    /// Enable encryption on login app, given a RSA private key, the client should use 
    /// the matching public key in order to validate this server.
    pub fn set_encryption(&mut self, key: Arc<RsaPrivateKey>) {
        self.encryption_key = Some(key);
    }

    /// As opposed to [`Self::set_private_key`], disable encryption on login app.
    pub fn remove_encryption(&mut self) {
        self.encryption_key = None;
    }

    /// Return true if encryption is enabled on this login app.
    pub fn has_encryption(&self) -> bool {
        self.encryption_key.is_some()
    }

    /// Poll for the next event of this login app, blocking.
    pub fn poll(&mut self) -> Event {
        loop {

            while let Some(event) = self.events.pop_front() {
                return event;
            }
            
            let socket_poll_ret = self.socket_poll.poll();

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
            
            debug!("<{}: [{:08X}] {:?}", addr, packet.raw().read_prefix(), packet.raw());

            let peer;
            let channel;
            if let Some(peer_) = &socket_poll_ret.peer {
                peer = Arc::clone(&peer_);
                channel = &mut self.in_channel;
            } else {
                
                let peer_ = match self.peers.entry(addr) {
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

                        let peer = Arc::new(Peer {
                            socket,
                            addr,
                            mutable: Mutex::new(PeerMut {
                                last_ping: None,
                                last_login: None,
                            }),
                        });

                        let thread_peer = Arc::clone(&peer);
                        self.socket_poll.spawn(move || SocketPollRet {
                            peer: Some(Arc::clone(&thread_peer)),
                            res: thread_peer.socket.recv_without_encryption(),
                        });

                        v.insert(peer)

                    }
                };

                peer = Arc::clone(peer_);
                channel = &mut self.out_channel;

            }
            
            let Some((bundle, _)) = channel.accept(packet, peer.addr) else {
                continue;
            };

            if socket_poll_ret.peer.is_some() {
                self.handle_in(bundle, &peer).unwrap();
            } else {
                self.handle_out(bundle, &peer).unwrap();
            }

        }
    }

    fn handle_out(&mut self, bundle: Bundle, peer: &Peer) -> io::Result<()> {
        
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
            for packet in self.bundle.packets() {
                debug!(">{}: [{:08X}] {:?}", self.real_addr, packet.raw().read_prefix(), packet.raw());
            }
            peer.socket.send_bundle_without_encryption(&self.bundle, self.real_addr)?;
        }

        Ok(())

    }

    fn handle_out_element(&mut self, reader: TopElementReader, peer: &Peer) -> io::Result<()> {
        match reader.id() {
            id::PING => self.handle_out_ping(reader, peer),
            id::LOGIN_REQUEST => self.handle_login_request(reader, peer),
            id::CHALLENGE_RESPONSE => self.handle_challenge_response(reader, peer),
            id => Err(io_invalid_data(format_args!("unexpected element #{id}"))),
        }
    }

    /// Handle a ping request to the login node, we answer as fast as possible.
    fn handle_out_ping(&mut self, elt: TopElementReader, peer: &Peer) -> io::Result<()> {

        let ping = elt.read_simple::<Ping>()?;
        let request_id = ping.request_id
            .ok_or_else(|| io_invalid_data(format_args!("ping should be a request")))?;

        let mut peer_mut = peer.mutable.lock().unwrap();
        peer_mut.last_ping = Some(PeerLastPing { 
            request_id,
            time: Instant::now(),
        });
        
        self.bundle.element_writer().write_simple_request(id::PING, ping.element, request_id);

        Ok(())

    }

    /// Handle a login request to the login node.
    fn handle_login_request(&mut self, elt: TopElementReader, peer: &Peer) -> io::Result<()> {
        
        let recv_encryption = self.encryption_key.as_ref()
            .map(|key| LoginRequestEncryption::Server(Arc::clone(&key)))
            .unwrap_or(LoginRequestEncryption::Clear);

        let login = elt.read::<LoginRequest>(&recv_encryption)?;
        
        let request_id = login.request_id
            .ok_or_else(|| io_invalid_data(format_args!("login should be a request")))?;

        let blowfish = Arc::new(Blowfish::new_from_slice(&login.element.blowfish_key)
            .map_err(|_| io_invalid_data(format_args!("login has invalid blowfish key: {:?}", login.element.blowfish_key)))?);

        let mut peer_mut = peer.mutable.lock().unwrap();
        peer_mut.last_login = Some(PeerLastLogin { 
            request_id, 
            blowfish,
            username: login.element.username.clone(),
            context: login.element.context.clone(),
        });

        let send_encryption = self.real_encryption_key.as_ref()
            .map(|key| LoginRequestEncryption::Client(Arc::clone(&key)))
            .unwrap_or(LoginRequestEncryption::Clear);

        self.bundle.element_writer().write_request(id::LOGIN_REQUEST, login.element.clone(), &send_encryption, request_id);

        Ok(())

    }

    fn handle_challenge_response(&mut self, elt: TopElementReader, _peer: &Peer) -> io::Result<()> {
        let challenge = elt.read_simple::<ChallengeResponse<CuckooCycleResponse>>()?;
        self.bundle.element_writer().write_simple(id::CHALLENGE_RESPONSE, challenge.element);
        Ok(())
    }

    fn handle_in(&mut self, bundle: Bundle, peer: &Peer) -> io::Result<()> {

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
                debug!(">{}: [{:08X}] {:?}", peer.addr, packet.raw().read_prefix(), packet.raw());
            }
            self.socket.send_bundle_without_encryption(&self.bundle, peer.addr)?;
        }

        Ok(())

    }

    fn handle_in_reply(&mut self, elt: ReplyElementReader, peer: &Peer, inherit_prefix: &mut bool) -> io::Result<()> {
        
        let request_id = elt.request_id();
        let mut peer_mut = peer.mutable.lock().unwrap();

        if peer_mut.last_ping.as_ref().map(|l| l.request_id) == Some(request_id) {

            let last_ping = peer_mut.last_ping.take().unwrap();
            let latency = last_ping.time.elapsed();

            self.events.push_back(Event::Ping(PingEvent {
                addr: peer.addr,
                latency,
            }));

            let ping = elt.read_simple::<Ping>()?;
            self.bundle.element_writer().write_simple_reply(ping.element, request_id);

        } else if peer_mut.last_login.as_ref().map(|l| l.request_id) == Some(request_id) {
            
            let last_login = peer_mut.last_login.take().unwrap();

            let encryption = LoginResponseEncryption::Encrypted(Arc::clone(&last_login.blowfish));
            let login = elt.read::<LoginResponse>(&encryption)?;
            
            if let LoginResponse::Success(success) = &login.element {
                *inherit_prefix = true;
                self.events.push_back(Event::LoginSuccess(LoginSuccessEvent {
                    addr: peer.addr,
                    username: last_login.username,
                    context: last_login.context,
                    blowfish: last_login.blowfish,
                    base_app_addr: success.addr,
                    login_key: success.login_key,
                    server_message: success.server_message.clone(),
                }));
            } else if let LoginResponse::Error(error, data) = &login.element {
                self.events.push_back(Event::LoginError(LoginErrorEvent {
                    addr: peer.addr,
                    username: last_login.username,
                    context: last_login.context,
                    error: *error,
                    data: data.clone(),
                }));
            }

            self.bundle.element_writer().write_reply(login.element, &encryption, request_id);

        } else {
            return Err(io_invalid_data(format_args!("unexpected reply #{}", request_id)));
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
    pub username: String,
    pub context: String,
    pub blowfish: Arc<Blowfish>,
    pub base_app_addr: SocketAddrV4,
    pub login_key: u32,
    pub server_message: String,
}

#[derive(Debug)]
pub struct LoginErrorEvent {
    /// The address of the client that successfully logged in.
    pub addr: SocketAddr,
    pub username: String,
    pub context: String,
    pub error: LoginError,
    pub data: String,
}
