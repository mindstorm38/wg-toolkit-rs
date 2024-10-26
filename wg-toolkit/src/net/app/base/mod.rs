//! Base application where clients send all their requests.

pub mod element;

use std::collections::{HashMap, VecDeque};
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::Arc;
use std::io;

use blowfish::Blowfish;

use crate::net::bundle::{Bundle, ElementReader, TopElementReader};
use crate::net::channel::ChannelTracker;
// use crate::net::element::ElementIdRange;
use crate::net::socket::PacketSocket;

use super::common::element::{Entity, Method};
use super::io_invalid_data;

use element::ClientAuth;


/// This modules defines numerical identifiers for base app elements.
pub mod id {

    // use super::ElementIdRange;

    pub const CLIENT_AUTH: u8           = 0x00;
    pub const CLIENT_SESSION_KEY: u8    = 0x01;

    // pub const CELL_ENTITY_METHOD: ElementIdRange = ElementIdRange::new(0x0F, 0x87);
    // pub const BASE_ENTITY_METHOD: ElementIdRange = ElementIdRange::new(0x88, 0xFE);

}

/// The base application.
#[derive(Debug)]
pub struct App {
    /// Internal socket for this application.
    socket: PacketSocket,
    /// The channel tracker.
    channel: ChannelTracker,
    /// Queue of events that are waiting to be returned.
    events: VecDeque<Event>,
    /// A temporary bundle for sending.
    bundle: Bundle,
    /// Clients that have made an initial client connection, associated to the request id.
    pending_clients: HashMap<SocketAddr, u32>,
    /// Map of clients.
    clients: HashMap<SocketAddr, Client>,
}

impl App {

    pub fn new(addr: SocketAddr) -> io::Result<Self> {
        Ok(Self {
            socket: PacketSocket::bind(addr)?,
            channel: ChannelTracker::new(),
            events: VecDeque::new(),
            bundle: Bundle::new(),
            pending_clients: HashMap::new(),
            clients: HashMap::new(),
        })
    }

    /// Get the address this app is bound to.
    pub fn addr(&self) -> io::Result<SocketAddr> {
        self.socket.addr()
    }

    /// Poll for the next event of this login app, blocking.
    pub fn poll(&mut self) -> Event {
        loop {

            // Empty the events before.
            while let Some(event) = self.events.pop_front() {
                return event;
            }

            let (packet, addr) = match self.socket.recv() {
                Ok(ret) => ret,
                Err(error) => return Event::IoError(IoErrorEvent { error, addr: None }),
            };

            let Some((bundle, _)) = self.channel.accept(packet, addr) else {
                continue;
            };

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
            id::CLIENT_AUTH => self.handle_client_auth(addr, reader),
            id::CLIENT_SESSION_KEY => self.handle_client_session_key(addr, reader),
            id => Err(io_invalid_data(format_args!("unexpected element #{id}"))),
        }
    }

    fn handle_client_auth(&mut self, addr: SocketAddr, reader: TopElementReader) -> io::Result<()> {
        
        let auth = reader.read_simple::<ClientAuth>()?;
        let request_id = auth.request_id
            .ok_or_else(|| io_invalid_data(format_args!("auth should be a request")))?;

        self.events.push_back(Event::Login(LoginEvent {
            addr,
            login_key: auth.element.login_key,
            attempt_num: auth.element.attempt_num,
        }));

        self.pending_clients.insert(addr, request_id);

        Ok(())

    }

    fn handle_client_session_key(&mut self, addr: SocketAddr, reader: TopElementReader) -> io::Result<()> {
        let _ = (addr, reader);
        Ok(())
    }

    /// Accept the login of the given user, in response to [`Event::Login`], giving the
    /// blowfish key that will be used for encryption.
    /// 
    /// This returns true if the client hasn't been answered yet.
    pub fn answer_login_success(&mut self, addr: SocketAddr, blowfish: Arc<Blowfish>) -> bool {
        
        let Some(request_id) = self.pending_clients.remove(&addr) else {
            return false;
        };


        true

    }

    pub fn create_entity<E: Entity>(&mut self, addr: SocketAddr, entity: E) -> Handle<E> {
        todo!()
    }

    pub fn call_method<E: Entity>(&mut self, addr: SocketAddr, handle: Handle<E>, method: E::ClientMethod) {
        todo!()
    }

}

/// An event that happened in the login app regarding the login process.
#[derive(Debug)]
pub enum Event {
    IoError(IoErrorEvent),
    Login(LoginEvent),
}

/// Some IO error happened internally and optionally related to a client.
#[derive(Debug)]
pub struct IoErrorEvent {
    /// The IO error.
    pub error: io::Error,
    /// An optional client address related to the error.
    pub addr: Option<SocketAddr>,
}

/// A client is trying to connect.
#[derive(Debug)]
pub struct LoginEvent {
    /// The address of the client that pinged the login app.
    pub addr: SocketAddr,
    /// The given client from the given address
    pub login_key: u32,
    /// The attempt number.
    pub attempt_num: u8,
}

/// A typed handle to an arbitrary entity in the base app.
pub struct Handle<E> {
    entity_id: u32,
    _phantom: PhantomData<*const E>,
}

/// An active logged in client in the base application.
#[derive(Debug)]
struct Client {
    /// The session key for this client.
    session_key: u32,
    /// The blowfish key for encryption of this client's packets.
    blowfish: Arc<Blowfish>,
}
