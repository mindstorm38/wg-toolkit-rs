//! This module is common to both base and client applications, it contains protocol 
//! definition of how to encode entities, their components/interfaces and the method 
//! calls.

// pub mod data;
pub mod entity;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::io;

use crate::net::bundle::{Bundle, ElementReader, TopElementReader};
use crate::net::socket::PacketSocket;
use crate::net::proto::Protocol;


/// Common abstract application that handle bundle messages.
#[derive(Debug)]
pub struct App {
    /// Internal socket.
    socket: PacketSocket,
    /// The protocol.
    protocol: Protocol,
    /// The bundle used to encode a peer's elements.
    bundle: Bundle,
    /// Mapping of each known peer to its remote address.
    peers: HashMap<SocketAddr, InternalPeer>,
}

impl App {

    pub fn new(addr: SocketAddr) -> io::Result<Self> {
        Ok(Self {
            socket: PacketSocket::bind(addr)?,
            protocol: Protocol::new(),
            bundle: Bundle::new(),
            peers: HashMap::new(),
        })
    }

    pub fn poll<H: Handler>(&mut self, mut handler: H) -> io::Result<()> {

        let (packet, addr) = self.socket.recv()?;
        
        let peer = self.peers.entry(addr)
            .or_insert_with(|| InternalPeer {
                addr,
            });

        let Some(mut channel) = self.protocol.accept(packet, addr) else {
            return Ok(());
        };

        while let Some(bundle) = channel.next_bundle() {
            let mut reader = bundle.element_reader();
            while let Some(elt) = reader.next_element() {
                match elt {
                    ElementReader::Top(elt) => {
                        handler.handle_element(Peer {
                            internal: &mut *peer,
                            bundle: &mut self.bundle,
                        }, elt)?;
                    }
                    ElementReader::Reply(_elt) => {

                    }
                }
            }
        }

        Ok(())

    }

}

/// The main handle trait for this application.
pub trait Handler {

    /// Handle an incoming top element from the given peer.
    fn handle_element(&mut self, peer: Peer, elt: TopElementReader) -> io::Result<()>;

    fn handle_reply(&mut self, peer: Peer);

}

/// A handle to a peer.
#[derive(Debug)]
pub struct Peer<'a> {
    internal: &'a mut InternalPeer,
    bundle: &'a mut Bundle,
}

impl<'a> Peer<'a> {

    /// Get the address of this peer.
    #[inline]
    pub fn addr(&self) -> SocketAddr {
        self.internal.addr
    }

}

/// Internal peer data that is forwarded via the peer handle given to handler.
#[derive(Debug)]
struct InternalPeer {
    addr: SocketAddr,
}
