//! Providing an bundle-oriented socket, backed by an UDP socket.

use std::net::{SocketAddr, UdpSocket};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::io::{self, Cursor};
use std::time::Duration;

use blowfish::Blowfish;

use tracing::trace;

use super::filter::{BlowfishReader, BlowfishWriter, blowfish::BLOCK_SIZE};
use super::packet::{self, Packet};
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
#[derive(Debug, Clone)]
pub struct PacketSocket {
    /// Internal sharable state.
    inner: Arc<Inner>,
}

#[derive(Debug)]
struct Inner {
    /// The inner socket.
    socket: UdpSocket,
    /// Possible symmetric encryption on given socket addresses. Behind a shared 
    /// read/write lock because most of the time we don't modify it.
    encryption: RwLock<HashMap<SocketAddr, Arc<Blowfish>>>,
    total_send_size: AtomicUsize,
    total_send_count: AtomicUsize,
    total_recv_size: AtomicUsize,
    total_recv_count: AtomicUsize,
}

impl PacketSocket {

    pub fn bind(addr: SocketAddr) -> io::Result<Self> {
        Ok(Self {
            inner: Arc::new(Inner {
                socket: UdpSocket::bind(addr)?,
                encryption: RwLock::new(HashMap::new()),
                total_send_size: AtomicUsize::new(0),
                total_send_count: AtomicUsize::new(0),
                total_recv_size: AtomicUsize::new(0),
                total_recv_count: AtomicUsize::new(0),
            }),
        })
    }
    
    pub fn addr(&self) -> io::Result<SocketAddr> {
        self.inner.socket.local_addr()
    }

    pub fn set_recv_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.inner.socket.set_read_timeout(dur)
    }

    pub fn set_send_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.inner.socket.set_write_timeout(dur)
    }

    #[inline]
    pub fn set_encryption(&mut self, addr: SocketAddr, blowfish: Arc<Blowfish>) {
        self.inner.encryption.write().unwrap().insert(addr, blowfish);
    }

    #[inline]
    pub fn remove_encryption(&mut self, addr: SocketAddr) {
        self.inner.encryption.write().unwrap().remove(&addr);
    }

    /// Get a snapshot of this socket's statistics.
    pub fn stat(&self) -> PacketSocketStat {
        PacketSocketStat {
            total_send_size: self.inner.total_send_size.load(Ordering::Relaxed),
            total_send_count: self.inner.total_send_count.load(Ordering::Relaxed),
            total_recv_size: self.inner.total_recv_size.load(Ordering::Relaxed),
            total_recv_count: self.inner.total_recv_count.load(Ordering::Relaxed),
        }
    }

    /// Receive a packet from some peer, without encryption if set for the address.
    pub fn recv_without_encryption(&self) -> io::Result<(Packet, SocketAddr)> {
        
        let mut packet = Packet::new();
        let (len, addr) = self.inner.socket.recv_from(packet.buf_mut())?;

        // Adjust the data length depending on what have been received.
        packet.set_len(len);

        // Here we use the release ordering on count to ensure that any 
        self.inner.total_recv_size.fetch_add(len, Ordering::Relaxed);
        self.inner.total_recv_count.fetch_add(1, Ordering::Relaxed);

        Ok((packet, addr))

    }

    /// Receive a packet from some peer.
    pub fn recv(&self) -> io::Result<(Packet, SocketAddr)> {
        
        let (mut packet, addr) = self.recv_without_encryption()?;

        if let Some(blowfish) = self.inner.encryption.read().unwrap().get(&addr) {
            packet = decrypt_packet(packet, &blowfish)
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid encryption"))?;
        }
    
        Ok((packet, addr))

    }

    /// Send a packet to the given peer, without encryption if set for the address.
    pub fn send_without_encryption(&self, packet: &Packet, addr: SocketAddr) -> io::Result<usize> {
        self.inner.total_send_size.fetch_add(packet.len(), Ordering::Relaxed);
        self.inner.total_send_count.fetch_add(1, Ordering::Relaxed);
        self.inner.socket.send_to(packet.slice(), addr)
    }

    /// Send a packet to the given peer.
    pub fn send(&self, packet: &Packet, addr: SocketAddr) -> io::Result<usize> {

        let size;

        if let Some(blowfish) = self.inner.encryption.read().unwrap().get(&addr) {
            let mut dst_packet = encryption_packet::take();
            encrypt_packet_raw(packet, &blowfish, &mut dst_packet);
            size = self.send_without_encryption(&dst_packet, addr)?;
            encryption_packet::put(dst_packet);
        } else {
            size = self.send_without_encryption(packet, addr)?;
        }

        Ok(size)

    }

    /// Send all packets in a bundle to the given peer, without encryption if set for the address.
    pub fn send_bundle_without_encryption(&self, bundle: &Bundle, addr: SocketAddr) -> io::Result<usize> {
        let mut size = 0;
        for packet in bundle.iter() {
            size += self.send_without_encryption(packet, addr)?;
        }
        Ok(size)
    }

    /// Send all packets in a bundle to the given peer.
    pub fn send_bundle(&self, bundle: &Bundle, addr: SocketAddr) -> io::Result<usize> {
        if let Some(blowfish) = self.inner.encryption.read().unwrap().get(&addr) {
            
            let mut dst_packet = encryption_packet::take();
            let mut size = 0;

            for packet in bundle.iter() {
                dst_packet.reset();
                encrypt_packet_raw(packet, &blowfish, &mut dst_packet);
                size += self.send_without_encryption(&dst_packet, addr)?;
            }

            encryption_packet::put(dst_packet);

            Ok(size)

        } else {
            self.send_bundle_without_encryption(bundle, addr)
        }
    }

}

/// A snapshot of packet socket statistics.
#[derive(Debug)]
pub struct PacketSocketStat {
    pub total_send_size: usize,
    pub total_send_count: usize,
    pub total_recv_size: usize,
    pub total_recv_count: usize,
}

/// Decrypt a packet of a given length with a blowfish key. Note that the destination 
/// packet will be completely erased, so the inner data is not relevant.
fn decrypt_packet_raw(src_packet: &Packet, bf: &Blowfish, dst_packet: &mut Packet) -> bool {

    let len = src_packet.len();

    dst_packet.set_len(len);

    // Decrypt the incoming packet into the new clear packet.
    // We don't need to set the length yet because this packet will be synchronized just 
    // after. We don't encrypt the prefix.
    let src = &src_packet.slice()[packet::PACKET_PREFIX_LEN..];
    let dst = &mut dst_packet.slice_mut()[packet::PACKET_PREFIX_LEN..];
    
    // Note that src and dst have the same length, thanks to blowfish encryption.
    // Then we can already check the length and ensures that it is a multiple of
    // blowfish block size *and* can contain the wastage and encryption magic.
    if src.len() % BLOCK_SIZE != 0 {
        trace!("Invalid source body length: {}, block size: {BLOCK_SIZE}", src.len());
        return false;
    } else if src.len() < ENCRYPTION_FOOTER_LEN {
        trace!("Invalid source body length: {}, min len: {ENCRYPTION_FOOTER_LEN}", src.len());
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
        trace!("Invalid destination packet magic: {:X}, expected: {:X}", 
            crate::util::BytesFmt(&dst[magic_begin..wastage_begin]),
            crate::util::BytesFmt(&ENCRYPTION_MAGIC));
        return false;
    }

    // Get the wastage count and compute the packet's length.
    // Note that wastage count also it self length.
    let wastage = dst[wastage_begin];
    assert!(wastage <= BLOCK_SIZE as u8, "temporary check that wastage is not greater than block size");

    dst_packet.set_len(len - wastage as usize - ENCRYPTION_MAGIC.len());
    // Copy the prefix directly because it is clear.
    dst_packet.write_prefix(src_packet.read_prefix());

    true

}

/// Encrypt source packet with the given blowfish key and write it to the destination
/// raw packet. Everything except the packet prefix is encrypted, and the destination
/// packet will have a size that is a multiple of blowfish's block size (8). The clear
/// data is also padded to block size, but with additional data at the end: encryption
/// signature (0xDEADBEEF in little endian) and the wastage count + 1 on the last byte.
fn encrypt_packet_raw(src_packet: &Packet, bf: &Blowfish, dst_packet: &mut Packet) {
    
    // Get the minimum, unpadded length of this packet with encryption footer appended to it.
    let mut len = src_packet.len() - packet::PACKET_PREFIX_LEN + ENCRYPTION_FOOTER_LEN;

    // The wastage amount is basically the padding + 1 for the wastage itself.
    let padding = (BLOCK_SIZE - (len % BLOCK_SIZE)) % BLOCK_SIZE;
    len += padding;

    // Clone the packet data into a new vec and append the padding and the footer.
    let mut clear_data = src_packet.slice()[packet::PACKET_PREFIX_LEN..].to_vec();
    clear_data.reserve_exact(padding + ENCRYPTION_FOOTER_LEN);
    clear_data.extend_from_slice(&[0u8; BLOCK_SIZE - 1][..padding]); // Padding
    clear_data.extend_from_slice(&ENCRYPTION_MAGIC); // Magic
    clear_data.push(padding as u8 + 1); // Wastage count (+1 for it self size)

    debug_assert_eq!(clear_data.len(), len, "incoherent length");
    debug_assert_eq!(clear_data.len() % 8, 0, "data not padded as expected");
    
    // +4 for the prefix.
    dst_packet.set_len(clear_data.len() + 4);

    // Unwrapping because we know that source/destination have the same length.
    io::copy(
        &mut Cursor::new(&clear_data[..]), 
        &mut BlowfishWriter::new(Cursor::new(&mut dst_packet.slice_mut()[packet::PACKET_PREFIX_LEN..]), bf),
    ).unwrap();
    
    // Copy the prefix directly because it is clear.
    dst_packet.write_prefix(src_packet.read_prefix());

}

/// Decrypt a source packet given a blowfish key, return the clear packet if success,
/// if the decryption fails it return the source packet not touched.
pub fn decrypt_packet(src_packet: Packet, bf: &Blowfish) -> Result<Packet, Packet> {
    let mut dst_packet = encryption_packet::take();
    if decrypt_packet_raw(&src_packet, bf, &mut dst_packet) {
        encryption_packet::put(src_packet);
        Ok(dst_packet)
    } else {
        encryption_packet::put(dst_packet);
        Err(src_packet)
    }
}

/// Encrypt a source packet given a blowfish key, return the encrypted packet.
pub fn encrypt_packet(src_packet: Packet, bf: &Blowfish) -> Packet {
    let mut dst_packet = encryption_packet::take();
    encrypt_packet_raw(&src_packet, bf, &mut dst_packet);
    encryption_packet::put(src_packet);
    dst_packet
}

/// Internal module to isolate implementation detail, the goal is just to avoid wasting
/// already allocated packets by keeping an encryption packet around.
mod encryption_packet {

    use crate::net::packet::Packet;
    use std::cell::Cell;
    
    thread_local! {
        static ENCRYPTION_PACKET: Cell<Option<Packet>> = Cell::new(None);
    }

    pub fn take() -> Packet {
        ENCRYPTION_PACKET.take().unwrap_or_else(|| Packet::new())
    }

    pub fn put(mut packet: Packet) {
        packet.reset();
        ENCRYPTION_PACKET.set(Some(packet));
    }

}
