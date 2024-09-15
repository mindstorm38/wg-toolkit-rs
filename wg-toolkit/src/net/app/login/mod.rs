//! Login application where clients ping and initiate connection in order to be routed
//! to the base application afterward.

pub mod element;

use std::collections::{HashMap, VecDeque};
use std::net::{SocketAddr, SocketAddrV4};
use std::sync::Arc;
use std::io;
use std::time::Instant;

use crypto_common::KeyInit;
use blowfish::Blowfish;
use rsa::RsaPrivateKey;

use rand::rngs::OsRng;
use rand::RngCore;

use crate::net::bundle::{Bundle, ElementReader, TopElementReader};
use crate::net::cuckoo::CuckooContext;
use crate::net::socket::BundleSocket;
use super::io_invalid_data;

use element::{
    Ping,
    LoginRequest, LoginRequestEncryption,
    LoginResponse, LoginResponseEncryption, LoginChallenge,
    LoginSuccess, 
    ChallengeResponse, CuckooCycleResponse,
};


/// This modules defines numerical identifiers for login app elements.
mod id {
    pub const LOGIN_REQUEST: u8         = 0x00;
    pub const PING: u8                  = 0x02;
    pub const CHALLENGE_RESPONSE: u8    = 0x03;
}

/// The login application.
#[derive(Debug)]
pub struct App {
    /// Internal socket for this application.
    socket: BundleSocket,
    /// Queue of events that are waiting to be returned.
    events: VecDeque<Event>,
    /// A temporary bundle for sending.
    bundle: Bundle,
    /// Optional private key to set if encryption is enabled on the login app. This 
    /// implies that the client should use the matching public key when logging in in
    /// order to validate.
    priv_key: Option<Arc<RsaPrivateKey>>,
    /// Login requests of each client in process with the login app.
    pending_requests: HashMap<SocketAddr, PendingRequest>,
    /// Responses to be sent in response to login or challenge requests.
    pending_responses: VecDeque<PendingResponse>,
    /// Issued and pending challenges.
    pending_challenges: HashMap<SocketAddr, PendingChallenge>,
    /// Used for benchmarking performance.
    received_instant: Option<Instant>,
}

impl App {

    pub fn new(addr: SocketAddr) -> io::Result<Self> {
        Ok(Self {
            socket: BundleSocket::new(addr)?,
            events: VecDeque::new(),
            bundle: Bundle::new(),
            priv_key: None,
            pending_requests: HashMap::new(),
            pending_responses: VecDeque::new(),
            pending_challenges: HashMap::new(),
            received_instant: None,
        })
    }

    /// Get the address this app is bound to.
    pub fn addr(&self) -> SocketAddr {
        self.socket.addr()
    }

    /// Enable encryption on login app, given a RSA private key, the client should use 
    /// the matching public key in order to validate this server.
    pub fn set_private_key(&mut self, key: Arc<RsaPrivateKey>) {
        self.priv_key = Some(key);
    }

    /// As opposed to [`Self::set_private_key`], disable encryption on login app.
    pub fn unset_private_key(&mut self) {
        self.priv_key = None;
    }

    /// Poll for the next event of this login app, blocking.
    pub fn poll(&mut self) -> Event {
        loop {

            // Empty the events before.
            while let Some(event) = self.events.pop_front() {
                return event;
            }

            // Then send pending login responses.
            while let Some(res) = self.pending_responses.pop_front() {
                let addr = res.addr;
                if let Err(error) = self.send_response(res) {
                    return Event::IoError(IoErrorEvent { error, addr: Some(addr) });
                }
            }

            // Wait for a bundle to be fully received.
            let (addr, bundle) = loop {
                match self.socket.recv() {
                    Ok(Some(ret)) => break ret,
                    Ok(None) => continue,
                    Err(error) => return Event::IoError(IoErrorEvent { error, addr: None }),
                }
            };

            self.received_instant = Some(Instant::now());

            // Fully read the bundle to determine how to handle that client.
            let mut reader = bundle.element_reader();
            while let Some(reader) = reader.next_element() {
                match reader {
                    ElementReader::Top(reader) => {
                        if let Err(error) = self.handle_element(addr, reader) {
                            return Event::IoError(IoErrorEvent { error, addr: Some(addr) });
                        }
                    }
                    ElementReader::Reply(reader) => {
                        return Event::IoError(IoErrorEvent {
                            error: io_invalid_data(format_args!("unexpected reply #{}", reader.request_id())),
                            addr: Some(addr),
                        });
                    }
                }
            }

        }
    }

    /// Handle an element read from the given address.
    fn handle_element(&mut self, addr: SocketAddr, reader: TopElementReader) -> io::Result<()> {
        match reader.id() {
            id::PING => self.handle_ping(addr, reader),
            id::LOGIN_REQUEST => self.handle_login_request(addr, reader),
            id::CHALLENGE_RESPONSE => self.handle_challenge_response(addr, reader),
            id => Err(io_invalid_data(format_args!("unexpected element #{id}"))),
        }
    }

    /// Handle a ping request to the login node, we answer as fast as possible.
    fn handle_ping(&mut self, addr: SocketAddr, elt: TopElementReader) -> io::Result<()> {

        let ping = elt.read_simple::<Ping>()?;
        let request_id = ping.request_id
            .ok_or_else(|| io_invalid_data(format_args!("ping should be a request")))?;

        self.events.push_back(Event::Ping(PingEvent { addr }));

        self.bundle.clear();
        self.bundle.element_writer().write_simple_reply(ping.element, request_id);
        self.socket.send(&mut self.bundle, addr)?;

        if let Some(received_instant) = self.received_instant {
            let _ping_internal_duration = received_instant.elapsed();
            // println!("ping_internal_duration: {ping_internal_duration:?}");
        }

        Ok(())

    }

    /// Handle a login request to the login node.
    fn handle_login_request(&mut self, addr: SocketAddr, elt: TopElementReader) -> io::Result<()> {
        
        let req_encryption = self.priv_key.as_ref()
            .map(|key| LoginRequestEncryption::Server(Arc::clone(&key)))
            .unwrap_or(LoginRequestEncryption::Clear);

        let login = elt.read::<LoginRequest>(&req_encryption)?;
        let request_id = login.request_id
            .ok_or_else(|| io_invalid_data(format_args!("login should be a request")))?;
        let blowfish = Arc::new(Blowfish::new_from_slice(&login.element.blowfish_key)
            .map_err(|_| io_invalid_data(format_args!("login has invalid blowfish key: {:?}", login.element.blowfish_key)))?);

        // Update or insert the login tracker... 
        self.pending_requests.insert(addr, PendingRequest {
            blowfish,
            request_id,
        });

        self.events.push_back(Event::Login(LoginEvent {
            addr,
            request: login.element,
        }));

        Ok(())

    }

    fn handle_challenge_response(&mut self, addr: SocketAddr, elt: TopElementReader) -> io::Result<()> {

        let Some(pending_challenge) = self.pending_challenges.remove(&addr) else {
            return Err(io_invalid_data(format_args!("unexpected challenge")));
        };

        let challenge = elt.read_simple::<ChallengeResponse<CuckooCycleResponse>>()?;
        
        // Start by checking coherency.
        if !challenge.element.data.key.starts_with(&pending_challenge.key_prefix) {
            return Err(io_invalid_data(format_args!("challenge has invalid key prefix")));
        }

        let cuckoo = CuckooContext::new(pending_challenge.max_nonce, &challenge.element.data.key);
        if !cuckoo.verify(&challenge.element.data.solution) {
            return Err(io_invalid_data(format_args!("challenge has invalid solution")));
        }
        
        self.events.push_back(Event::Challenge(ChallengeEvent {
            addr,
        }));

        Ok(())

    }

    /// In response to a [`LoginRequestEvent`], authorize a client to log into the base
    /// application, giving them its address and a login key that will be used to 
    /// register itself.
    pub fn answer_login_success(&mut self, 
        addr: SocketAddr, 
        app_addr: SocketAddrV4, 
        login_key: u32,
        server_message: String
    ) -> bool {
        self.answer_login_response(addr, LoginResponse::Success(LoginSuccess {
            addr: app_addr,
            login_key,
            server_message,
        }))
    }

    /// In response to a [`LoginRequestEvent`], send a client the challenge it should
    /// complete. This implementation issue a Cuckoo Cycle challenge, but that's a detail.
    pub fn answer_login_challenge(&mut self,
        addr: SocketAddr,
    ) -> bool {

        let easiness = 0.9;

        let key_prefix_value = OsRng.next_u64();
        let key_prefix = format!("{key_prefix_value:>02X}").into_bytes();
        let max_nonce = ((1 << 20) as f32 * easiness) as u32;

        self.pending_challenges.insert(addr, PendingChallenge {
            key_prefix: key_prefix.clone(),
            max_nonce,
        });

        self.answer_login_response(addr, LoginResponse::Challenge(LoginChallenge::CuckooCycle { 
            key_prefix, 
            max_nonce,
        }))

    }

    /// Internal wrapper for answering a login response.
    fn answer_login_response(&mut self, addr: SocketAddr, response: LoginResponse) -> bool {

        let Some(request) = self.pending_requests.remove(&addr) else {
            return false
        };

        self.pending_responses.push_back(PendingResponse {
            request,
            addr,
            inner: response,
        });

        true

    }

    /// Send a challenge to the given client, this only works if a tracker exists.
    fn send_response(&mut self, response: PendingResponse) -> io::Result<()> {

        let res_encryption = LoginResponseEncryption::Encrypted(response.request.blowfish);
        self.bundle.clear();
        self.bundle.element_writer().write_reply(response.inner, &res_encryption, response.request.request_id);
        self.socket.send(&mut self.bundle, response.addr)?;

        Ok(())

    }

}

/// An event that happened in the login app regarding the login process.
#[derive(Debug)]
pub enum Event {
    IoError(IoErrorEvent),
    Ping(PingEvent),
    Login(LoginEvent),
    Challenge(ChallengeEvent),
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
}

/// A client has made a request to login, this request can be answered with the app.
#[derive(Debug)]
pub struct LoginEvent {
    /// The address of the client that request a login.
    pub addr: SocketAddr,
    /// The request received.
    pub request: LoginRequest,
}

/// A challenge has been answered by the client.
#[derive(Debug)]
pub struct ChallengeEvent {
    /// The address of the client that request a login.
    pub addr: SocketAddr,
}

/// Describe a client trying to log into the server.
#[derive(Debug)]
struct PendingRequest {
    /// This is the blowfish key as sent by the client when requesting login.
    blowfish: Arc<Blowfish>,
    /// Id of the last request the client sent and where replies should be sent.
    request_id: u32,
}

/// Describe a response pending to be sent to an address.
#[derive(Debug)]
struct PendingResponse {
    /// Initial request leading to this response.
    request: PendingRequest,
    /// The address of the client.
    addr: SocketAddr,
    /// Inner login response.
    inner: LoginResponse,
}

/// Describe a challenge that have been issued, this is currently about a Cuckoo Cycle.
#[derive(Debug)]
struct PendingChallenge {
    /// The key prefix expected for the answered key.
    key_prefix: Vec<u8>,
    /// The configured max nonce.
    max_nonce: u32,
}

#[cfg(test)]
mod tests {

    use std::net::{Ipv4Addr, SocketAddr};

    use super::{App, Event};

    fn test() {
    
        let mut app = App::new(SocketAddr::new(Ipv4Addr::UNSPECIFIED, 4123)).unwrap();

        loop {
            match app.poll() {
                Event::IoError(_) => todo!(),
                Event::Ping(ping) => {
                    println!("[{}] Ping...", ping.addr);
                }
                Event::Login(login) => {
                    println!("[{}] Login...", login.addr);
                    app.answer_login_challenge(login.addr);
                }
                Event::Challenge(challenge) => {
                    println!("[{}] Challenge...", challenge.addr);
                    // app.answer_login_success(challenge.addr, app_addr, login_key, server_message)
                }
            }
        }
    
    }

}
