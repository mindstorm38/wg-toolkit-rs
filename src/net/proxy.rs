//! A proxy implementation for intercepting and dumping packets passing
//! from client to server or vice versa.


use std::net::SocketAddr;
use std::time::Duration;

use mio::net::UdpSocket;
use mio::{Events, Interest, Poll, Token};

use crate::net::bundle::BundleAssembler;
use crate::net::packet::Packet;


const CLIENT_AVAIL: Token = Token(0);
const SERVER_AVAIL: Token = Token(1);


pub struct Proxy {
    client_sock: UdpSocket,
    server_sock: UdpSocket,
    client_assembler: BundleAssembler,
    server_assembler: BundleAssembler,
    poll: Poll,
    events: Events
}

impl Proxy {

    /// Bind a proxy to a local address to which a client can connect.
    /// - The client bind address to give is the one to which the client must connect.
    /// - The server bind address is the local endpoint where the server will send data.
    /// - The server address is the server to which the proxy will be connected to send data.
    pub fn bind(client_bind_addr: SocketAddr, server_bind_addr: SocketAddr, server_addr: SocketAddr) -> std::io::Result<Self> {

        let mut client_sock = UdpSocket::bind(client_bind_addr)?;
        let mut server_sock = UdpSocket::bind(server_bind_addr)?;
        server_sock.connect(server_addr)?;

        let poll = Poll::new()?;
        poll.registry().register(&mut client_sock, CLIENT_AVAIL, Interest::READABLE)?;
        poll.registry().register(&mut server_sock, SERVER_AVAIL, Interest::READABLE)?;

        Ok(Self {
            client_sock,
            server_sock,
            client_assembler: BundleAssembler::new(true),
            server_assembler: BundleAssembler::new(true),
            poll,
            events: Events::with_capacity(128)
        })

    }

    pub fn poll(&mut self) -> std::io::Result<()> {
        self.poll.poll(&mut self.events, Some(Duration::from_millis(100)))?;
        for event in self.events.iter() {
            match event.token() {
                CLIENT_AVAIL => {
                    Self::transfer("CLIENT -> SERVER", &self.client_sock, &self.server_sock, &mut self.client_assembler, true);
                }
                SERVER_AVAIL => {
                    Self::transfer("SERVER -> CLIENT", &self.server_sock, &self.client_sock, &mut self.server_assembler, false);
                }
                _ => unreachable!()
            }
        }
        Ok(())
    }

    fn transfer(display: &str, from: &UdpSocket, to: &UdpSocket, asm: &mut BundleAssembler, connect: bool) -> std::io::Result<()> {
        let mut packet = Packet::new_boxed(true);
        let (len, orig) = from.recv_from(&mut packet.data[..])?;
        if connect {
            from.connect(orig)?;
        }
        to.send(&packet.data[..len])?;
        if let Err(e) = packet.sync_state(len, true) {
            println!("[{}] Failed to sync packet data: {:?}", display, e);
        } else if let Some(bundle) = asm.try_assemble((), packet) {
            println!("[{}] Received bundle of length: {}", display, bundle.len());
        } else {
            println!("[{}] Accumulated packet on bundle...", display);
        }
        Ok(())
    }

}
