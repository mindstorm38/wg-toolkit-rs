//! Base application where clients send all their requests.

pub mod element;

use std::collections::{HashMap, VecDeque};
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::num::Wrapping;
use std::sync::Arc;
use std::fmt;
use std::io;

use blowfish::Blowfish;

use rand::rngs::OsRng;
use rand::RngCore;

use crate::net::bundle::{Bundle, NextElementReader, ElementReader};
use crate::net::element::SimpleElement;
use crate::net::socket::PacketSocket;
use crate::net::proto::Protocol;

use super::common::entity::Entity;
use super::io_invalid_data;

use element::{LoginKey, SessionKey};


/// The base application.
#[derive(Debug)]
pub struct App {
    /// Internal socket for this application.
    socket: PacketSocket,
    /// The channel tracker.
    protocol: Protocol,
    /// Queue of events that are waiting to be returned.
    events: VecDeque<Event>,
    /// A temporary bundle for sending.
    bundle: Bundle,
    /// Clients that have made an initial client connection, associated to the request id.
    pending_clients: HashMap<SocketAddr, u32>,
    /// Map of clients.
    clients: HashMap<SocketAddr, Client>,
    /// Map of all currently alive entities.
    entities: HashMap<u32, EntityGeneric>,
    /// The next id for entities, this is wrapping around and we ensure that the same id
    /// isn't used twice!
    entities_next_id: Wrapping<u32>,
}

impl App {

    pub fn new(addr: SocketAddr) -> io::Result<Self> {
        Ok(Self {
            socket: PacketSocket::bind(addr)?,
            protocol: Protocol::new(),
            events: VecDeque::new(),
            bundle: Bundle::new(),
            pending_clients: HashMap::new(),
            clients: HashMap::new(),
            entities: HashMap::new(),
            entities_next_id: Wrapping(OsRng.next_u32()),
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

            let Ok(mut channel) = self.protocol.accept(packet, addr) else {
                continue;
            };

            let Some(bundle) = channel.next_bundle() else {
                continue;
            };

            // Fully read the bundle to determine how to handle that client.
            let mut reader = bundle.element_reader();
            while let Some(reader) = reader.next() {
                match reader {
                    NextElementReader::Element(elt) => {
                        if let Err(error) = self.handle_element(addr, elt) {
                            return Event::IoError(IoErrorEvent { error, addr: Some(addr) });
                        }
                    }
                    NextElementReader::Reply(reply) => {
                        return Event::IoError(IoErrorEvent {
                            error: io_invalid_data(format_args!("unexpected reply #{}", reply.request_id())),
                            addr: Some(addr),
                        });
                    }
                }
            }

        }
    }

    /// Handle an element read from the given address.
    fn handle_element(&mut self, addr: SocketAddr, reader: ElementReader) -> io::Result<()> {
        match reader.id() {
            LoginKey::ID => self.handle_client_auth(addr, reader),
            SessionKey::ID => self.handle_client_session_key(addr, reader),
            id => Err(io_invalid_data(format_args!("unexpected element #{id}"))),
        }
    }

    fn handle_client_auth(&mut self, addr: SocketAddr, reader: ElementReader) -> io::Result<()> {
        
        let auth = reader.read_simple::<LoginKey>()?;
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

    fn handle_client_session_key(&mut self, addr: SocketAddr, reader: ElementReader) -> io::Result<()> {
        let _ = (addr, reader);
        Ok(())
    }

    /// Accept the login of the given user, in response to [`Event::Login`], giving the
    /// blowfish key that will be used for encryption.
    /// 
    /// This returns true if the client hasn't been answered yet.
    pub fn answer_login_success(&mut self, addr: SocketAddr, _blowfish: Arc<Blowfish>) -> bool {
        
        let Some(_request_id) = self.pending_clients.remove(&addr) else {
            return false;
        };


        true

    }

    /// Create an entity and return the handle to manage it.
    pub fn create_entity<E: Entity + 'static>(&mut self, entity: E) -> Handle<E> {

        // Generate a new unique entity id.
        let entity_id = loop {
            let id = self.entities_next_id.0;
            self.entities_next_id += 1;
            if !self.entities.contains_key(&id) {
                break id;
            }
        };

        // self.entities.insert(entity_id, EntityGeneric {
        //     wrapper: Box::new(EntityWrapperImpl {
        //         inner: entity,
        //     })
        // });

        todo!()

    }

    /// Call a method on an entity present on the given client address and its handle.
    pub fn call_method<E: Entity>(&mut self, addr: SocketAddr, handle: Handle<E>, method: E::ClientMethod) {
        let _ = (addr, handle, method);
        todo!()
    }

}

/// An event that happened in the login app regarding the login process.
#[derive(Debug)]
pub enum Event {
    IoError(IoErrorEvent),
    Login(LoginEvent),
    // BaseMethod(BaseMethodEvent),
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

#[derive(Debug)]
pub struct BaseMethodEvent {
    pub addr: SocketAddr,
    pub entity_id: u32,

}

/// A typed handle to an entity in the base app, potentially present on client side.
#[derive(Debug, Clone, Copy)]
pub struct Handle<E> {
    entity_id: u32,
    _phantom: PhantomData<*const E>,
}

/// A untyped handle to an entity in the base app, potentially present on client side.
#[derive(Debug, Clone, Copy)]
pub struct GenericHandle {
    entity_id: u32,
}

/// An active logged in client in the base application.
#[derive(Debug)]
struct Client {
    /// The session key for this client.
    session_key: u32,
    /// The blowfish key for encryption of this client's packets.
    blowfish: Arc<Blowfish>,
}



struct EntityGeneric {
    // wrapper: Box<dyn EntityWrapper>,
}

impl fmt::Debug for EntityGeneric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EntityGeneric").finish()
    }
}

trait EntityWrapper {

}

struct EntityWrapperImpl<E: Entity> {
    inner: E,
}

impl<E: Entity> EntityWrapper for EntityWrapperImpl<E> {

}
