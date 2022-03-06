use std::net::{ToSocketAddrs, UdpSocket, SocketAddr};

use super::bundle::BundleAssembler;
use super::element::ElementCodec;
use super::packet::Packet;


pub struct Interface {
    sock: UdpSocket,
    bundle_assembler: BundleAssembler<SocketAddr>,
    has_prefix: bool
}

impl Interface {

    pub fn new<A: ToSocketAddrs>(addr: A, has_prefix: bool) -> std::io::Result<Self> {
        Ok(Self {
            sock: UdpSocket::bind(addr)?,
            bundle_assembler: BundleAssembler::new(has_prefix),
            has_prefix
        })
    }

    pub fn receive(&mut self) {

        let mut packet = Packet::new_boxed(self.has_prefix);
        let (len, addr) = self.sock.recv_from(&mut packet.data).unwrap();

        if let Err(e) = packet.load(len, true) {
            todo!("{:?}", e)
        } else if let Some(bundle) = self.bundle_assembler.try_assemble(addr, packet) {
            bundle.iter_elements()
        }

    }

    pub fn add_element<E: ElementCodec>(&mut self) {

    }

    pub fn add_element_callback<E, C>(&mut self, callback: C)
    where
        E: ElementCodec,
        C: FnMut(E)
    {



    }

}
