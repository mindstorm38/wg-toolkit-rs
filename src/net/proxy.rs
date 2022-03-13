//! A proxy implementation for intercepting and dumping packets passing
//! from client to server or vice versa.


use std::time::{Duration, Instant};
use std::net::SocketAddr;
use std::str::FromStr;

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
    client_addr: SocketAddr,
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
            client_addr: SocketAddr::from_str("0.0.0.0:0").unwrap(),
            poll,
            events: Events::with_capacity(128)
        })

    }

    pub fn poll(&mut self) -> std::io::Result<()> {

        println!("[POLL]");
        self.poll.poll(&mut self.events, None)?;

        for event in self.events.iter() {
            let res = match event.token() {
                CLIENT_AVAIL => {
                    Self::transfer(&self.client_sock, &self.server_sock, &mut self.client_assembler, ClientToServer {
                        client_addr: &mut self.client_addr
                    })
                }
                SERVER_AVAIL => {
                    Self::transfer(&self.server_sock, &self.client_sock, &mut self.server_assembler, ServerToClient {
                        client_addr: &self.client_addr
                    })
                }
                _ => unreachable!()
            };
            if let Err(e) = res {
                println!("Unexpected error: {:?}", e);
            }
        }

        Ok(())

    }

    #[inline(always)]
    fn transfer<T: Transfer>(from: &UdpSocket, to: &UdpSocket, asm: &mut BundleAssembler, mut transfer: T) -> std::io::Result<()> {
        loop {
            let mut packet = Packet::new_boxed(true);
            match transfer.recv(from, &mut packet.data[..]) {
                Ok(len) => {
                    transfer.send(to, &packet.data[..len])?;
                    if let Err(e) = packet.sync_state(len, true) {
                        println!("[{}] Failed to sync packet data: {:?}", T::DISPLAY, e);
                    } else {
                        println!("[{}] Accumulating packet ({})...", T::DISPLAY, packet.len());
                        if let Some(bundle) = asm.try_assemble((), packet) {
                            println!("[{}] Received bundle of length: {}", T::DISPLAY, bundle.len());
                        }
                    }
                },
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    println!("[WOULD BLOCK]");
                    break
                },
                Err(e) => return Err(e)
            }
        }
        Ok(())
    }

}


trait Transfer {
    const DISPLAY: &'static str;
    fn recv(&mut self, from: &UdpSocket, buf: &mut [u8]) -> std::io::Result<usize>;
    fn send(&mut self, to: &UdpSocket, buf: &[u8]) -> std::io::Result<usize>;
}


struct ClientToServer<'a> {
    client_addr: &'a mut SocketAddr
}

impl<'a> Transfer for ClientToServer<'a> {

    const DISPLAY: &'static str = "CLIENT -> SERVER";

    fn recv(&mut self, from: &UdpSocket, buf: &mut [u8]) -> std::io::Result<usize> {
        let (len, orig) = from.recv_from(buf)?;
        if &orig != self.client_addr {
            println!("[CLIENT -> SERVER] New address: {}", orig);
            self.client_addr.clone_from(&orig);
        }
        Ok(len)
    }

    fn send(&mut self, to: &UdpSocket, buf: &[u8]) -> std::io::Result<usize> {
        to.send(buf)
    }

}


struct ServerToClient<'a> {
    client_addr: &'a SocketAddr
}

impl<'a> Transfer for ServerToClient<'a> {

    const DISPLAY: &'static str = "SERVER -> CLIENT";

    fn recv(&mut self, from: &UdpSocket, buf: &mut [u8]) -> std::io::Result<usize> {
        from.recv(buf)
    }

    fn send(&mut self, to: &UdpSocket, buf: &[u8]) -> std::io::Result<usize> {
        to.send_to(buf, self.client_addr.clone())
    }

}
