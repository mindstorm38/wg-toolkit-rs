//! A proxy implementation for intercepting and dumping packets passing
//! from client to server and vice versa. Also allowing filters for
//! rewriting packets (e.g. for login app decryption/encryption).


use std::net::SocketAddr;
use std::io;

use mio::net::UdpSocket;
use mio::{Events, Interest, Poll, Token};

use crate::net::bundle::Bundle;
use crate::net::packet::Packet;


const CLIENT_AVAIL: Token = Token(0);
const SERVER_AVAIL: Token = Token(1);


/// A special proxy for intercepting, decoding and resending packets
/// from server to client and from client to server.
pub struct Proxy<CL, SL> {
    client: ProxySide<ProxyClientHandler, CL>,
    server: ProxySide<ProxyServerHandler, SL>,
    poll: Poll,
    events: Events
}

impl<CL, SL> Proxy<CL, SL>
where
    CL: ProxyListener,
    SL: ProxyListener
{

    /// Bind a proxy to a local address to which a client can connect.
    /// - The client bind address to give is the one to which the client must connect.
    /// - The server bind address is the local endpoint where the server will send data.
    /// - The server address is the server to which the proxy will be connected to send data.
    pub fn bind(
        client_bind_addr: SocketAddr,
        server_bind_addr: SocketAddr,
        server_addr: SocketAddr,
        client_listener: CL,
        server_listener: SL
    ) -> io::Result<Self> {

        let mut client = ProxySide::new(client_bind_addr, ProxyClientHandler::new(), client_listener)?;
        let mut server = ProxySide::new(server_bind_addr, ProxyServerHandler::new(server_addr), server_listener)?;

        let poll = Poll::new()?;
        poll.registry().register(&mut client.sock, CLIENT_AVAIL, Interest::READABLE)?;
        poll.registry().register(&mut server.sock, SERVER_AVAIL, Interest::READABLE)?;

        Ok(Self {
            client,
            server,
            poll,
            events: Events::with_capacity(128)
        })

    }

    pub fn poll(&mut self) -> io::Result<()> {

        println!("[POLL]");
        self.poll.poll(&mut self.events, None)?;

        for event in self.events.iter() {
            let res = match event.token() {
                CLIENT_AVAIL => {
                    println!("[CLIENT -> SERVER]");
                    self.client.transfer_to(&mut self.server)
                }
                SERVER_AVAIL => {
                    println!("[SERVER -> CLIENT]");
                    self.server.transfer_to(&mut self.client)
                }
                _ => unreachable!()
            };
            if let Err(e) = res {
                println!("Unexpected error: {:?}", e);
            }
        }

        Ok(())

    }

}


/// Internal structure for defining a proxy side, usually client or server.
/// It also contains the bundle assembler for received packets.
struct ProxySide<H, L> {
    sock: UdpSocket,
    handler: H,
    listener: L
}

impl<H, L> ProxySide<H, L>
where
    H: ProxySideConnector,
    L: ProxyListener
{
    
    fn new(bind_addr: SocketAddr, mut handler: H, listener: L) -> io::Result<Self> {
        let mut sock = UdpSocket::bind(bind_addr)?;
        handler.setup(&mut sock)?;
        Ok(Self {
            sock,
            handler,
            listener
        })
    }
    
    /// Transfer from this side to another while possible. Every filter is applied.
    fn transfer_to<TH, TL>(&mut self, to: &mut ProxySide<TH, TL>) -> io::Result<()>
    where
        TH: ProxySideConnector,
        TL: ProxyListener
    {
        loop {
            let mut packet = Packet::new_boxed(true);
            match self.handler.recv(&self.sock, &mut packet.data[..]) {
                Ok(len) => {
                    self.listener.received(packet, len, to)?;
                },
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) => return Err(e)
            }
        }
        Ok(())
    }
    
}


/// An abstract view to a proxy side's output. It is passed to the filter
pub trait ProxySideOutput {

    /// Send raw data to this side.
    fn send_data(&mut self, data: &[u8]) -> io::Result<()>;

    fn send_synced_packet(&mut self, packet: &Packet) -> io::Result<()> {
        self.send_data(packet.get_valid_data())
    }

    fn send_finalized_bundle(&mut self, bundle: &Bundle) -> io::Result<()> {
        for packet in bundle.get_packets() {
            self.send_synced_packet(&**packet)?;
        }
        Ok(())
    }

}

/// Implement the trait for proxy side.
impl<H, L> ProxySideOutput for ProxySide<H, L>
where
    H: ProxySideConnector,
    L: ProxyListener
{
    fn send_data(&mut self, data: &[u8]) -> io::Result<()> {
        self.handler.send(&self.sock, data).map(|_| ())
    }
}


/// A listener trait responsible of raw and not synced packets received from
/// a proxy side. With this you can do anything of the received packet.
pub trait ProxyListener {

    /// Called when packet's data is received, the implementor is responsible
    /// of transmitting data to the output side if needed. **Note that** the
    /// given packet is not synced, only its data is valid for the given len.
    fn received<O: ProxySideOutput>(&mut self, packet: Box<Packet>, len: usize, out: &mut O) -> io::Result<()>;

}

/// A simple common side listener that just redirect incoming datagram to output.
pub struct ProxyDirectTransfer;
impl ProxyListener for ProxyDirectTransfer {
    fn received<O: ProxySideOutput>(&mut self, packet: Box<Packet>, len: usize, out: &mut O) -> io::Result<()> {
        out.send_data(&packet.data[..len])
    }
}


// Side handlers (internal only)

/// An internal handler for specific transfer from one side to another.
trait ProxySideConnector {
    fn setup(&mut self, sock: &mut UdpSocket) -> io::Result<()>;
    fn recv(&mut self, from: &UdpSocket, buf: &mut [u8]) -> io::Result<usize>;
    fn send(&mut self, to: &UdpSocket, buf: &[u8]) -> io::Result<usize>;
}

/// A handler for the client side of a proxy, with a dynamic address.
/// Because we are using UDP, the client peer address may often change from one
/// datagram to another. To solve that the client socket is never directly
/// connected to the peer address because it would break MIO event detection,
/// but we save the peer address when receiving a datagram and use this address
/// when sending datagrams. *This means that no datagram can be sent if no
/// one was received before, because we can't know the client peer address.*
struct ProxyClientHandler {
    addr: Option<SocketAddr>
}

impl ProxyClientHandler {
    fn new() -> Self {
        Self { addr: None }
    }
}

impl ProxySideConnector for ProxyClientHandler {

    fn setup(&mut self, _sock: &mut UdpSocket) -> io::Result<()> {
        Ok(())
    }

    fn recv(&mut self, from: &UdpSocket, buf: &mut [u8]) -> io::Result<usize> {
        let (len, orig) = from.recv_from(buf)?;
        self.addr = Some(orig);
        Ok(len)
    }

    fn send(&mut self, to: &UdpSocket, buf: &[u8]) -> io::Result<usize> {
        if let Some(addr) = self.addr {
            to.send_to(buf, addr)
        } else {
            Ok(0)
        }
    }

}

/// A handler for the server side of a proxy, with a fixed connected address.
/// The server socket is connected to the server's address at setup and
/// data is received from and sent to this address without passing the address
/// again to `recv` and `send` functions.
struct ProxyServerHandler {
    addr: SocketAddr
}

impl ProxyServerHandler {
    fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }
}

impl ProxySideConnector for ProxyServerHandler {

    fn setup(&mut self, sock: &mut UdpSocket) -> io::Result<()> {
        sock.connect(self.addr)
    }

    fn recv(&mut self, from: &UdpSocket, buf: &mut [u8]) -> io::Result<usize> {
        from.recv(buf)
    }

    fn send(&mut self, to: &UdpSocket, buf: &[u8]) -> io::Result<usize> {
        to.send(buf)
    }

}
