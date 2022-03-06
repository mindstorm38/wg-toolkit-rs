use std::net::{ToSocketAddrs, UdpSocket};

use super::element::ElementCodec;


pub struct Interface {
    sock: UdpSocket
}

impl Interface {

    pub fn new<A: ToSocketAddrs>(addr: A) -> std::io::Result<Self> {
        Ok(Self {
            sock: UdpSocket::bind(addr)?
        })
    }

    pub fn add_element<E: ElementCodec>(&mut self) {

    }

    pub fn add_element_callback<E: ElementCodec>(&mut self) {

    }

}
