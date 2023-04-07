//! This module provides a callback-based interface for sending and
//! receiving bundles' elements.

use std::net::{SocketAddrV4, SocketAddr};
use std::io;

use super::bundle::TopElementReader;
use super::element::TopElement;
use super::socket::WgSocket;


/// A callback-based interface for sending and receiving elements,
/// backed by a [`WgSocket`], itself backed by an [`UdpSocket`].
/// 
/// This interface take a generic data type that will be stored
/// internally and passed to callbacks when called, this usually
/// is the shared state of the application. The shared data will
/// be passed by mutable reference because this interface is
/// single-threaded.
pub struct Interface<S> {
    /// The inner socket that provides interface for sending and
    /// receiving bundles of packets (themselves containing
    /// elements).
    socket: WgSocket,
    /// The shared data that is passed by mutable reference to
    /// all callbacks as the first parameter (allowing methods).
    shared: S,
    /// The array of callbacks for each element's id (except for
    /// the reply element's id, 0xFF).
    callbacks: Box<[Option<fn(&mut S, TopElementReader, SocketAddr)>; 255]>,
}

impl<S> Interface<S> {

    pub fn new(addr: SocketAddrV4, shared: S) -> io::Result<Self> {
        Ok(Self {
            socket: WgSocket::new(addr)?,
            shared,
            callbacks: Box::new([None; 255]),
        })
    }

    pub fn register<E>(&mut self, id: u8, callback: fn(&mut S, E, SocketAddr))
    where
        E: TopElement,
    {

        assert_ne!(id, 0xFF, "id reserved for reply elements");

        self.callbacks[id as usize] = Some(move |state: &mut S, reader: TopElementReader, addr: SocketAddr| {
            
            reader.read(&())

        })

    }

}
