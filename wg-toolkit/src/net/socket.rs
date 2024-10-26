//! Providing an bundle-oriented socket, backed by an UDP socket.

use std::net::{SocketAddr, UdpSocket};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::io::{self, Cursor};
use std::time::Duration;

use blowfish::Blowfish;

use super::filter::{BlowfishReader, BlowfishWriter, blowfish::BLOCK_SIZE};
use super::packet::{Packet, RawPacket};
use super::bundle::Bundle;


/// Encryption magic, 0xDEADBEEF in little endian.
const ENCRYPTION_MAGIC: [u8; 4] = 0xDEADBEEFu32.to_le_bytes();
/// Encryption footer length, 1 byte for wastage count + 4 bytes magic.
const ENCRYPTION_FOOTER_LEN: usize = ENCRYPTION_MAGIC.len() + 1;


/// A tiny wrapper around UDP socket that allows sending and receiving raw packets, with
/// support for encryption of specific socket addresses.
/// 
/// This can be used as a MIO source to know when to receive and send packets, because
/// it is non-blocking by default and it cannot be changed.
#[derive(Debug)]
pub struct PacketSocket {
    /// The inner socket.
    socket: UdpSocket,
    /// Possible symmetric encryption on given socket addresses. Behind a shared 
    /// read/write lock because most of the time we don't modify it.
    encryption: Arc<RwLock<HashMap<SocketAddr, Arc<Blowfish>>>>,
}

impl PacketSocket {

    pub fn bind(addr: SocketAddr) -> io::Result<Self> {
        Ok(Self {
            socket: UdpSocket::bind(addr)?,
            encryption: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    pub fn addr(&self) -> io::Result<SocketAddr> {
        self.socket.local_addr()
    }

    pub fn try_clone(&self) -> io::Result<PacketSocket> {
        let socket = self.socket.try_clone()?;
        Ok(Self {
            socket,
            encryption: Arc::clone(&self.encryption),
        })
    }

    pub fn set_recv_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.socket.set_read_timeout(dur)
    }

    pub fn set_send_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.socket.set_write_timeout(dur)
    }

    #[inline]
    pub fn set_encryption(&mut self, addr: SocketAddr, blowfish: Arc<Blowfish>) {
        self.encryption.write().unwrap().insert(addr, blowfish);
    }

    #[inline]
    pub fn remove_encryption(&mut self, addr: SocketAddr) {
        self.encryption.write().unwrap().remove(&addr);
    }

    /// Receive a packet from some peer, without encryption if set for the address.
    pub fn recv_without_encryption(&self) -> io::Result<(Box<Packet>, SocketAddr)> {
        
        let mut packet = Packet::new_boxed();
        let (len, addr) = self.socket.recv_from(packet.raw_mut().raw_data_mut())?;

        // Adjust the data length depending on what have been received.
        packet.raw_mut().set_data_len(len);

        Ok((packet, addr))

    }

    /// Receive a packet from some peer.
    pub fn recv(&self) -> io::Result<(Box<Packet>, SocketAddr)> {
        
        let (mut packet, addr) = self.recv_without_encryption()?;

        if let Some(blowfish) = self.encryption.read().unwrap().get(&addr) {
            packet = decrypt_packet(packet, &blowfish)
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid encryption"))?;
        }
    
        Ok((packet, addr))

    }

    /// Send a packet to the given peer, without encryption if set for the address.
    pub fn send_without_encryption(&self, packet: &RawPacket, addr: SocketAddr) -> io::Result<usize> {
        self.socket.send_to(packet.data(), addr)
    }

    /// Send a packet to the given peer.
    pub fn send(&self, packet: &RawPacket, addr: SocketAddr) -> io::Result<usize> {

        let size;

        if let Some(blowfish) = self.encryption.read().unwrap().get(&addr) {
            let mut dst_packet = encryption_packet::take();
            encrypt_packet_raw(packet, &blowfish, dst_packet.raw_mut());
            size = self.send_without_encryption(dst_packet.raw(), addr)?;
            encryption_packet::put(dst_packet);
        } else {
            size = self.send_without_encryption(packet, addr)?;
        }

        Ok(size)

    }

    /// Send all packets in a bundle to the given peer, without encryption if set for the address.
    pub fn send_bundle_without_encryption(&self, bundle: &Bundle, addr: SocketAddr) -> io::Result<usize> {
        let mut size = 0;
        for packet in bundle.packets() {
            size += self.send_without_encryption(packet.raw(), addr)?;
        }
        Ok(size)
    }

    /// Send all packets in a bundle to the given peer.
    pub fn send_bundle(&self, bundle: &Bundle, addr: SocketAddr) -> io::Result<usize> {
        if let Some(blowfish) = self.encryption.read().unwrap().get(&addr) {
            
            let mut dst_packet = encryption_packet::take();
            let mut size = 0;

            for packet in bundle.packets() {
                dst_packet.reset();
                encrypt_packet_raw(packet.raw(), &blowfish, dst_packet.raw_mut());
                size += self.send_without_encryption(dst_packet.raw(), addr)?;
            }

            encryption_packet::put(dst_packet);

            Ok(size)

        } else {
            self.send_bundle_without_encryption(bundle, addr)
        }
    }

}

/// Decrypt a packet of a given length with a blowfish key. Note that the destination 
/// packet will be completely erased, so the inner data is not relevant.
fn decrypt_packet_raw(src_packet: &RawPacket, bf: &Blowfish, dst_packet: &mut RawPacket) -> bool {

    let len = src_packet.data_len();

    dst_packet.set_data_len(len);

    // Decrypt the incoming packet into the new clear packet.
    // We don't need to set the length yet because this packet 
    // will be synchronized just after.
    let src = src_packet.body();
    let dst = dst_packet.body_mut();
    
    // Note that src and dst have the same length, thanks to blowfish encryption.
    // Then we can already check the length and ensures that it is a multiple of
    // blowfish block size *and* can contain the wastage and encryption magic.
    if src.len() % BLOCK_SIZE != 0 || src.len() < ENCRYPTION_FOOTER_LEN {
        return false;
    }

    // Unwrapping because we know that source/destination have the same length.
    io::copy(
        &mut BlowfishReader::new(Cursor::new(src), &bf), 
        &mut Cursor::new(&mut *dst),
    ).unwrap();

    let wastage_begin = src.len() - 1;
    let magic_begin = wastage_begin - 4;

    // Check invalid magic.
    if &dst[magic_begin..wastage_begin] != &ENCRYPTION_MAGIC {
        return false;
    }

    // Get the wastage count and compute the packet's length.
    // Note that wastage count also it self length.
    let wastage = dst[wastage_begin];
    assert!(wastage <= BLOCK_SIZE as u8, "temporary check that wastage is not greater than block size");

    dst_packet.set_data_len(len - wastage as usize - ENCRYPTION_MAGIC.len());
    // Copy the prefix directly because it is clear.
    dst_packet.write_prefix(src_packet.read_prefix());

    true

}

/// Encrypt source packet with the given blowfish key and write it to the destination
/// raw packet. Everything except the packet prefix is encrypted, and the destination
/// packet will have a size that is a multiple of blowfish's block size (8). The clear
/// data is also padded to block size, but with additional data at the end: encryption
/// signature (0xDEADBEEF in little endian) and the wastage count + 1 on the last byte.
fn encrypt_packet_raw(src_packet: &RawPacket, bf: &Blowfish, dst_packet: &mut RawPacket) {
    
    // Get the minimum, unpadded length of this packet with encryption footer appended to it.
    let mut len = src_packet.body_len() + ENCRYPTION_FOOTER_LEN;

    // The wastage amount is basically the padding + 1 for the wastage itself.
    let padding = (BLOCK_SIZE - (len % BLOCK_SIZE)) % BLOCK_SIZE;
    len += padding;

    // Clone the packet data into a new vec and append the padding and the footer.
    let mut clear_data = Vec::from(src_packet.body());
    clear_data.reserve_exact(padding + ENCRYPTION_FOOTER_LEN);
    clear_data.extend_from_slice(&[0u8; BLOCK_SIZE - 1][..padding]); // Padding
    clear_data.extend_from_slice(&ENCRYPTION_MAGIC); // Magic
    clear_data.push(padding as u8 + 1); // Wastage count (+1 for it self size)

    debug_assert_eq!(clear_data.len(), len, "incoherent length");
    debug_assert_eq!(clear_data.len() % 8, 0, "data not padded as expected");
    
    // +4 for the prefix.
    dst_packet.set_data_len(clear_data.len() + 4);

    // Unwrapping because we know that source/destination have the same length.
    io::copy(
        &mut Cursor::new(&clear_data[..]), 
        &mut BlowfishWriter::new(Cursor::new(dst_packet.body_mut()), bf),
    ).unwrap();
    
    // Copy the prefix directly because it is clear.
    dst_packet.write_prefix(src_packet.read_prefix());

}

/// Decrypt a source packet given a blowfish key, return the clear packet if success,
/// if the decryption fails it return the source packet not touched.
pub fn decrypt_packet(src_packet: Box<Packet>, bf: &Blowfish) -> Result<Box<Packet>, Box<Packet>> {
    let mut dst_packet = encryption_packet::take();
    if decrypt_packet_raw(src_packet.raw(), bf, dst_packet.raw_mut()) {
        encryption_packet::put(src_packet);
        Ok(dst_packet)
    } else {
        encryption_packet::put(dst_packet);
        Err(src_packet)
    }
}

/// Encrypt a source packet given a blowfish key, return the encrypted packet.
pub fn encrypt_packet(src_packet: Box<Packet>, bf: &Blowfish) -> Box<Packet> {
    let mut dst_packet = encryption_packet::take();
    encrypt_packet_raw(src_packet.raw(), bf, dst_packet.raw_mut());
    encryption_packet::put(src_packet);
    dst_packet
}

/// Internal module to isolate implementation detail, the goal is just to avoid wasting
/// already allocated packets by keeping an encryption packet around.
mod encryption_packet {

    use std::cell::Cell;
    use crate::net::packet::Packet;
    
    thread_local! {
        static ENCRYPTION_PACKET: Cell<Option<Box<Packet>>> = Cell::new(None);
    }

    pub fn take() -> Box<Packet> {
        ENCRYPTION_PACKET.take().unwrap_or_else(|| Packet::new_boxed())
    }

    pub fn put(mut packet: Box<Packet>) {
        packet.reset();
        ENCRYPTION_PACKET.set(Some(packet));
    }

}
