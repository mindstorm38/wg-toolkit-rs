//! Base application where clients send all their requests.

pub mod element;

use std::collections::{hash_map, HashMap, VecDeque};
use std::net::SocketAddr;
use std::io;

use rand::rngs::OsRng;
use rand::RngCore;

use crate::net::bundle::{Bundle, ElementReader, TopElementReader};
// use crate::net::element::ElementIdRange;
use crate::net::socket::BundleSocket;
use super::io_invalid_data;


/// This modules defines numerical identifiers for base app elements.
mod id {

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
    socket: BundleSocket,
    /// Queue of events that are waiting to be returned.
    events: VecDeque<Event>,
    /// A temporary bundle for sending.
    bundle: Bundle,
    /// Clients pending to log into the application.
    pending_clients: HashMap<u32, PendingClient>,
}

impl App {

    pub fn new(addr: SocketAddr) -> io::Result<Self> {
        Ok(Self {
            socket: BundleSocket::new(addr)?,
            events: VecDeque::new(),
            bundle: Bundle::new(),
            pending_clients: HashMap::new(),
        })
    }

    /// Get the address this app is bound to.
    pub fn addr(&self) -> SocketAddr {
        self.socket.addr()
    }

    /// Poll for the next event of this login app, blocking.
    pub fn poll(&mut self) -> Event {
        loop {

            // Empty the events before.
            while let Some(event) = self.events.pop_front() {
                return event;
            }

            // Wait for a bundle to be fully received.
            let (addr, bundle) = loop {
                match self.socket.recv() {
                    Ok(Some(ret)) => break ret,
                    Ok(None) => continue,
                    Err(error) => return Event::IoError(IoErrorEvent { error, addr: None }),
                }
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
        let _ = (addr, reader);
        Ok(())
    }

    fn handle_client_session_key(&mut self, addr: SocketAddr, reader: TopElementReader) -> io::Result<()> {
        let _ = (addr, reader);
        Ok(())
    }

    /// Use this function to reserve a unique login key (unique regarding this app 
    /// instance only) for a client that will be authorized to connect by the login app.
    pub fn reserve_pending_client(&mut self, addr: SocketAddr) -> u32 {
        loop {
            let key = OsRng.next_u32();
            match self.pending_clients.entry(key) {
                hash_map::Entry::Vacant(v) => {
                    v.insert(PendingClient {
                        addr,
                    });
                    break key
                }
                _ => continue
            }
        }
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

/// A client has been successfully connected to the base app using the session key given
/// by the login app.
#[derive(Debug)]
pub struct LoginEvent {
    /// The address of the client that pinged the login app.
    pub addr: SocketAddr,
}

/// Describe a client that is pending to 
#[derive(Debug)]
struct PendingClient {
    /// The socket address we expect from this client, as a best effort to avoid spoofing.
    addr: SocketAddr,
}
