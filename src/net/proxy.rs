//! A proxy implementation for intercepting and dumping packets passing
//! from client to server and vice versa. Also allowing filters for
//! rewriting packets (e.g. for login app decryption/encryption).


use std::net::SocketAddr;

use mio::net::UdpSocket;
use mio::{Events, Interest, Poll, Token};

use crate::net::bundle::{Bundle, BundleAssembler};
use crate::net::packet::Packet;


const CLIENT_AVAIL: Token = Token(0);
const SERVER_AVAIL: Token = Token(1);


/// A special proxy for intercepting, decoding and resending packets
/// from server to client and from client to server.
pub struct Proxy<CF, SF> {
    client: ProxySide<ProxyClientHandler, CF>,
    server: ProxySide<ProxyServerHandler, SF>,
    poll: Poll,
    events: Events
}

impl<CF, SF> Proxy<CF, SF>
where
    CF: ProxySideFilter,
    SF: ProxySideFilter
{

    /// Bind a proxy to a local address to which a client can connect.
    /// - The client bind address to give is the one to which the client must connect.
    /// - The server bind address is the local endpoint where the server will send data.
    /// - The server address is the server to which the proxy will be connected to send data.
    pub fn bind(client_bind_addr: SocketAddr, server_bind_addr: SocketAddr, server_addr: SocketAddr, client_filter: CF, server_filter: SF) -> std::io::Result<Self> {

        let mut client = ProxySide::new("CLIENT", client_bind_addr, ProxyClientHandler::new(), client_filter)?;
        let mut server = ProxySide::new("SERVER", server_bind_addr, ProxyServerHandler::new(server_addr), server_filter)?;

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

    pub fn poll(&mut self) -> std::io::Result<()> {

        println!("[POLL]");
        self.poll.poll(&mut self.events, None)?;

        for event in self.events.iter() {
            let res = match event.token() {
                CLIENT_AVAIL => {
                    self.client.transfer_to(&mut self.server)
                }
                SERVER_AVAIL => {
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
struct ProxySide<H, F> {
    name: &'static str,
    sock: UdpSocket,
    asm: BundleAssembler,
    handler: H,
    filter: F
}

impl<H, F> ProxySide<H, F>
where
    H: ProxySideHandler,
    F: ProxySideFilter
{
    
    fn new(name: &'static str, bind_addr: SocketAddr, mut handler: H, filter: F) -> std::io::Result<Self> {
        let mut sock = UdpSocket::bind(bind_addr)?;
        handler.setup(&mut sock)?;
        Ok(Self {
            name,
            sock,
            asm: BundleAssembler::new(true),
            handler,
            filter
        })
    }

    fn send_finalized_bundle(&mut self, bundle: &mut Bundle) -> std::io::Result<usize> {
        let mut total_len = 0;
        for packet in bundle.get_packets() {
            total_len += self.handler.send(&self.sock, packet.get_valid_data())?;
        }
        Ok(total_len)
    }
    
    /// Transfer from this side to another while possible. Every filter is applied.
    fn transfer_to<TH, TF>(&mut self, to: &mut ProxySide<TH, TF>) -> std::io::Result<()>
    where
        TH: ProxySideHandler,
        TF: ProxySideFilter
    {

        macro_rules! tlog {
            ($format:tt, $($arg:tt)*) => {
                println!(concat!("[{} -> {}] ", $format), self.name, to.name, $($arg)*);
            };
        }

        loop {

            let mut packet = Packet::new_boxed(true);

            match self.handler.recv(&self.sock, &mut packet.data[..]) {
                Ok(len) => {

                    if self.filter.immediate_transfer() {
                        to.handler.send(&to.sock, &packet.data[..len])?;
                    }

                    if let Err(e) = packet.sync_state(len, true) {
                        tlog!("Failed to sync packet data: {:?}", e);
                    } else {

                        tlog!("Accumulating packet: {:?}", packet);
                        self.filter.received_packet(&packet);

                        if let Some(bundle) = self.asm.try_assemble((), packet) {
                            tlog!("Received bundle of length: {}", bundle.len());
                            self.filter.received_bundle(&bundle);
                        }

                    }

                },
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(e) => return Err(e)
            }

        }

        Ok(())

    }
    
}


/// An internal handler for specific transfer from one side to another.
trait ProxySideHandler {
    fn setup(&mut self, sock: &mut UdpSocket) -> std::io::Result<()>;
    fn recv(&mut self, from: &UdpSocket, buf: &mut [u8]) -> std::io::Result<usize>;
    fn send(&mut self, to: &UdpSocket, buf: &[u8]) -> std::io::Result<usize>;
}


/// A public filter trait for packets received by a proxy side and sent to another.
pub trait ProxySideFilter {

    /// Return `true` to immediately transfer the received packet's data
    /// to the opposite side.
    fn immediate_transfer(&mut self) -> bool;

    /// Called when a packet is successfully synced to its received data.
    fn received_packet(&mut self, packet: &Packet);

    /// Called when a bundle is successfully assembled from received packets.
    fn received_bundle(&mut self, bundle: &Bundle);

}

// Blank impl if you don't want any filter.
impl ProxySideFilter for () {
    fn immediate_transfer(&mut self) -> bool { true }
    fn received_packet(&mut self, _packet: &Packet) { }
    fn received_bundle(&mut self, _bundle: &Bundle) {}
}


/// A handler for the client side of a proxy, with a dynamic address.
struct ProxyClientHandler {
    addr: Option<SocketAddr>
}

impl ProxyClientHandler {
    fn new() -> Self {
        Self { addr: None }
    }
}

impl ProxySideHandler for ProxyClientHandler {

    fn setup(&mut self, _sock: &mut UdpSocket) -> std::io::Result<()> {
        Ok(())
    }

    fn recv(&mut self, from: &UdpSocket, buf: &mut [u8]) -> std::io::Result<usize> {
        let (len, orig) = from.recv_from(buf)?;
        self.addr = Some(orig);
        Ok(len)
    }

    fn send(&mut self, to: &UdpSocket, buf: &[u8]) -> std::io::Result<usize> {
        if let Some(addr) = self.addr {
            to.send_to(buf, addr)
        } else {
            Ok(0)
        }
    }

}

/// A handler for the server side of a proxy, with a fixed connected address.
struct ProxyServerHandler {
    addr: SocketAddr
}

impl ProxyServerHandler {
    fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }
}

impl ProxySideHandler for ProxyServerHandler {

    fn setup(&mut self, sock: &mut UdpSocket) -> std::io::Result<()> {
        sock.connect(self.addr)
    }

    fn recv(&mut self, from: &UdpSocket, buf: &mut [u8]) -> std::io::Result<usize> {
        from.recv(buf)
    }

    fn send(&mut self, to: &UdpSocket, buf: &[u8]) -> std::io::Result<usize> {
        to.send(buf)
    }

}
