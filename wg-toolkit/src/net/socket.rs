//! Providing an bundle-oriented socket, backed by an UDP socket.

use std::collections::{HashMap, hash_map};
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::io::{self, Cursor};

use blowfish::Blowfish;

use super::filter::{BlowfishReader, BlowfishWriter, blowfish::BLOCK_SIZE};
use super::packet::{Packet, RawPacket, PacketConfig, PacketConfigError};
use super::bundle::Bundle;


/// The (currently hardcoded) timeout on bundle fragments.
const FRAGMENT_TIMEOUT: Duration = Duration::from_secs(10);


/// A socket providing interface for sending and receiving bundles of elements, backed by
/// an UDP server with support for blowfish channel encryption. This socket is blocking
/// on sends and receives.
/// 
/// It also provides fragmentation support when sending bundles that contains more than 
/// one packet, channel blowfish encryption and packet acknowledgment.
/// 
/// This socket handle is actually just a shared pointer to shared data, it can be cloned
/// as needed and used in multiple threads at the same time.
#[derive(Debug, Clone)]
pub struct BundleSocket {
    /// Shared data.
    shared: Arc<Shared>,
}

/// A reference counted shared underlying socket data.
#[derive(Debug)]
struct Shared {
    /// Bound address for UDP server.
    addr: SocketAddr,
    /// The socket used for sending and receiving UDP packets.
    socket: UdpSocket,
    /// The mutable part of the shared data, behind a mutex lock.
    mutable: Mutex<SharedMutable>,
}

/// Mutable shared socket data.
#[derive(Debug)]
struct SharedMutable {
    /// Bundle fragments tracking.
    fragments: HashMap<(SocketAddr, u32), BundleFragments>,
    /// The next sequence ID to use for bundles.
    next_sequence_num: u32,
    /// Registered channels on the app that defines a particular blowfish
    /// key for packet encryption and decryption.
    channels: HashMap<SocketAddr, Channel>,
    /// A raw packet used as a temporary buffer for blowfish encryption.
    encryption_packet: Box<RawPacket>,
    /// List of rejected packets.
    rejected_packets: Vec<(SocketAddr, Box<Packet>, PacketRejectionError)>,
}

impl BundleSocket {

    /// Create a new socket bound to the given address.
    pub fn new(addr: SocketAddr) -> io::Result<Self> {

        let socket = UdpSocket::bind(addr)?;
        
        Ok(Self {
            shared: Arc::new(Shared { 
                addr, 
                socket, 
                mutable: Mutex::new(SharedMutable {
                    fragments: HashMap::new(),
                    next_sequence_num: 0,
                    channels: HashMap::new(),
                    encryption_packet: Box::new(RawPacket::new()),
                    rejected_packets: Vec::new(),
                }),
            }),
        })

    }

    /// Get the bind address of this socket.
    #[inline]
    pub fn addr(&self) -> SocketAddr {
        self.shared.addr
    }

    /// Associate a new channel to the given address with the given blowfish encryption.
    /// This blowfish encryption will be used for all transaction to come with this given
    /// socket address.
    pub fn set_channel(&mut self, addr: SocketAddr, blowfish: Arc<Blowfish>) {
        self.shared.mutable.lock().unwrap()
            .channels.insert(addr, Channel::new(blowfish));
    }

    /// Set the send timeout for later [`Self::send()`].
    pub fn set_send_timeout(&mut self, dur: Option<Duration>) -> io::Result<()> {
        self.shared.socket.set_write_timeout(dur)
    }

    /// Set the receive timeout for later [`Self::recv()`].
    pub fn set_recv_timeout(&mut self, dur: Option<Duration>) -> io::Result<()> {
        self.shared.socket.set_read_timeout(dur)
    }

    /// Send a bundle to a given address. Note that the bundle is finalized by
    /// this method with the internal sequence id.
    /// 
    /// Note that the net data is guaranteed to be untouched by this function,
    /// this includes prefix, flags up to the footer. Bytes beyond this limit
    /// might be modified in case of channel encryption.
    /// 
    /// This function forwards the IO error from the UDP socket's `send_to` call.
    pub fn send(&mut self, bundle: &mut Bundle, to: SocketAddr) -> io::Result<usize> {

        // Do nothing if bundle is empty.
        if bundle.is_empty() {
            return Ok(0)
        }

        // NOTE: This may potentially block is a received packet is being processed.
        let mut mutable = self.shared.mutable.lock().unwrap();
        let SharedMutable {
            next_sequence_num,
            channels,
            encryption_packet,
            ..
        } = &mut *mutable;

        // Get a potential reference to the channel this address is linked to.
        let mut channel = channels.get_mut(&to);

        // Compute first and last sequence num and directly update next sequence num.
        let sequence_first_num = *next_sequence_num;
        *next_sequence_num = next_sequence_num.checked_add(bundle.len() as u32).expect("sequence num overflow");
        let sequence_last_num = *next_sequence_num - 1;

        // Create a common packet config for all the bundle.
        let mut packet_config = PacketConfig::new();

        // When on channel, we set appropriate flags and values.
        if let Some(channel) = channel.as_deref_mut() {

            packet_config.set_on_channel(true);
            packet_config.set_reliable(true);
            
            // If we send a cumulative ack, just take auto ack to avoid resending 
            // it automatically.
            if packet_config.cumulative_ack().is_some() {
                channel.take_auto_ack();
            }

        }

        // If multi-packet bundle, set sequence range.
        if sequence_last_num > sequence_first_num {
            packet_config.set_sequence_range(sequence_first_num, sequence_last_num);
        }

        let mut size = 0;
        let mut sequence_num = sequence_first_num;

        for packet in bundle.packets_mut() {

            // Only the last packet has a cumulative ack.
            if sequence_num == sequence_last_num {
                if let Some(channel) = channel.as_mut() {
                    if let Some(num) = channel.get_cumulative_ack_exclusive() {
                        packet_config.set_cumulative_ack(num);
                    } else {
                        packet_config.clear_cumulative_ack();
                    }
                }
            }

            // Set sequence number and sync data.
            packet_config.set_sequence_num(sequence_num);
            packet.write_config(&mut packet_config);

            // Reference to the actual raw packet to send.
            let raw_packet;

            if let Some(channel) = channel.as_deref_mut() {

                channel.add_sent_ack(sequence_num);

                encrypt_packet(packet.raw(), &channel.blowfish, &mut **encryption_packet);
                raw_packet = &**encryption_packet;

            } else {
                raw_packet = packet.raw()
            }
            
            // println!("Sending {:X}", BytesFmt(raw_packet.data()));

            size += self.shared.socket.send_to(raw_packet.data(), to)?;
            sequence_num += 1;

        }

        Ok(size)

    }

    /// Blocking receive of a packet, if a bundle can be constructed it is returned, if
    /// not, none is returned instead. If the packet is rejected for any reason listed
    /// in [`PacketRejectionError`], none is also returned but the packet is internally
    /// queued and can later be retrieved with the error using 
    /// [`Self::take_rejected_packets()`].
    /// 
    /// This function forwards the IO error from the UDP socket's `recv_from` call.
    pub fn recv(&mut self) -> io::Result<Option<(SocketAddr, Bundle)>> {

        let mut packet = Packet::new_boxed();
        let (len, addr) = self.shared.socket.recv_from(packet.raw_mut().raw_data_mut())?;

        // Adjust the data length depending on what have been received.
        packet.raw_mut().set_data_len(len);

        // NOTE: We lock only once we received the packet, so it's not blocking any other
        // handle to this socket that want to send bundles.
        let mut mutable = self.shared.mutable.lock().unwrap();
        let SharedMutable {
            channels,
            fragments,
            rejected_packets,
            ..
        } = &mut *mutable;

        // Get a potential reference to the channel this address is linked to.
        let mut channel = channels.get_mut(&addr);

        // If the address is linked to a channel, we need to decrypt it according the 
        // channel's blowfish key.
        if let Some(channel) = channel.as_deref_mut() {
            match decrypt_packet(&packet, &channel.blowfish) {
                Ok(clear_packet) => packet = clear_packet,
                Err(()) => {
                    mutable.rejected_packets.push((addr, packet, PacketRejectionError::InvalidEncryption));
                    return Ok(None);
                }
            }
        }

        // Retrieve the real clear-text length after a potential decryption.
        let len = packet.raw().data_len();

        // TODO: Use thread-local for packet config?
        let mut packet_config = PacketConfig::new();
        if let Err(error) = packet.read_config(len, &mut packet_config) {
            mutable.rejected_packets.push((addr, packet, PacketRejectionError::Config(error)));
            return Ok(None);
        }

        // Again, if we are in a channel, we handle packet acknowledgment.
        if let Some(channel) = channel.as_deref_mut() {

            // If packet is reliable, take its ack number and store it for future acknowledging.
            if packet_config.reliable() {
                channel.add_received_ack(packet_config.sequence_num());
                channel.set_auto_ack();
            }

            if let Some(ack) = packet_config.cumulative_ack() {
                channel.remove_cumulative_ack(ack);
            }

        }

        // We can observe that packets with the flag 0x1000 are only used
        // for auto acking with sometimes duplicated data that is sent
        // just after. If it become a problem this check can be removed. 
        if packet_config.unk_1000().is_none() {
            
            let instant = Instant::now();

            match packet_config.sequence_range() {
                // Only if there is a range and this range is not a single num.
                Some((first_num, last_num)) if last_num > first_num => {

                    let num = packet_config.sequence_num();

                    match fragments.entry((addr, first_num)) {
                        hash_map::Entry::Occupied(mut o) => {

                            // If this fragments is too old, timeout every packet in it
                            // and start again with the packet.
                            // FIXME: Maybe dumb?
                            if o.get().is_old(instant, FRAGMENT_TIMEOUT) {
                                rejected_packets.extend(o.get_mut().drain()
                                    .map(|packet| (addr, packet, PacketRejectionError::TimedOut)));
                            }

                            o.get_mut().set(num, packet);

                            // When all fragments are collected, remove entry and return.
                            if o.get().is_full() {
                                return Ok(Some((addr, o.remove().into_bundle())));
                            }

                        },
                        hash_map::Entry::Vacant(v) => {
                            let mut fragments = BundleFragments::new(last_num - first_num + 1);
                            fragments.set(num, packet);
                            v.insert(fragments);
                        }
                    }

                }
                // Not sequence range in the packet, create a bundle only with it.
                _ => {
                    return Ok(Some((addr, Bundle::with_single(packet))));
                }
            }

        }

        // No error but no full bundle received.
        Ok(None)

    }

    /// Send all auto packet acknowledgments.
    pub fn send_auto_ack(&mut self) {

        let mut mutable = self.shared.mutable.lock().unwrap();
        let SharedMutable {
            channels,
            encryption_packet,
            ..
        } = &mut *mutable;

        for (addr, channel) in channels {
            if channel.take_auto_ack() {
                
                let mut packet_config = PacketConfig::new();
                let ack = channel.get_cumulative_ack_exclusive().expect("incoherent");
                packet_config.set_sequence_num(ack);
                packet_config.set_cumulative_ack(ack);
                packet_config.set_on_channel(true);
                packet_config.set_unk_1000(0);

                let mut packet = Packet::new_boxed();
                packet.write_config(&mut packet_config);

                encrypt_packet(packet.raw(), &channel.blowfish, &mut **encryption_packet);

                // println!("Sending auto ack {:X}", BytesFmt(self.encryption_packet.data()));
                self.shared.socket.send_to(encryption_packet.data(), *addr).unwrap();

            }
        }

    }

    /// Take the vector of all rejected packets and the rejection reason.
    pub fn take_rejected_packets(&mut self) -> Vec<(SocketAddr, Box<Packet>, PacketRejectionError)> {

        let mut mutable = self.shared.mutable.lock().unwrap();
        let SharedMutable {
            fragments,
            rejected_packets,
            ..
        } = &mut *mutable;

        // Before returning the vector, take all timed out fragments.
        let instant = Instant::now();
        fragments.retain(|(addr, _), fragments| {
            if fragments.is_old(instant, FRAGMENT_TIMEOUT) {
                rejected_packets.extend(fragments.drain()
                    .map(|packet| (*addr, packet, PacketRejectionError::TimedOut)));
                false
            } else {
                true
            }
        });

        // NOTE: We just take the vector, so mutable data is not locked for too long.
        std::mem::take(rejected_packets)

    }

}


/// Encryption magic, 0xDEADBEEF in little endian.
const ENCRYPTION_MAGIC: [u8; 4] = 0xDEADBEEFu32.to_le_bytes();
/// Encryption footer length, 1 byte for wastage count + 4 bytes magic.
const ENCRYPTION_FOOTER_LEN: usize = ENCRYPTION_MAGIC.len() + 1;


/// Decrypt a packet of a given length with a blowfish key.
/// 
/// This returns an empty error if the encryption is invalid.
/// If successful the clear packet is returned with its size, the size can then
/// be used to synchronize the packet's state to its data.
fn decrypt_packet(packet: &Packet, bf: &Blowfish) -> Result<Box<Packet>, ()> {

    let len = packet.raw().data_len();

    // Create a packet that have the same length as input packet.
    let mut clear_packet = Packet::new_boxed();
    clear_packet.raw_mut().set_data_len(len);

    // Decrypt the incoming packet into the new clear packet.
    // We don't need to set the length yet because this packet 
    // will be synchronized just after.
    let src = packet.raw().body();
    let dst = clear_packet.raw_mut().body_mut();
    
    // Note that src and dst have the same length, thanks to blowfish encryption.
    // Then we can already check the length and ensures that it is a multiple of
    // blowfish block size *and* can contain the wastage and encryption magic.
    if src.len() % BLOCK_SIZE != 0 || src.len() < ENCRYPTION_FOOTER_LEN {
        return Err(())
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
        return Err(())
    }

    // Get the wastage count and compute the packet's length.
    // Note that wastage count also it self length.
    let wastage = dst[wastage_begin];
    assert!(wastage <= BLOCK_SIZE as u8, "temporary check that wastage is not greater than block size");

    clear_packet.raw_mut().set_data_len(len - wastage as usize - ENCRYPTION_MAGIC.len());
    // Copy the prefix directly because it is clear.
    clear_packet.raw_mut().write_prefix(packet.raw().read_prefix());

    Ok(clear_packet)

}


/// Encrypt source packet with the given blowfish key and write it to the destination
/// raw packet. Everything except the packet prefix is encrypted, and the destination
/// packet will have a size that is a multiple of blowfish's block size (8). The clear
/// data is also padded to block size, but with additional data at the end: encryption
/// signature (0xDEADBEEF in little endian) and the wastage count + 1 on the last byte.
fn encrypt_packet(src_packet: &RawPacket, bf: &Blowfish, dst_packet: &mut RawPacket) {
    
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


/// Represent a channel between the app and a client with specific socket address.
#[derive(Debug)]
pub struct Channel {
    /// The blowfish key used for encryption of this channel.
    blowfish: Arc<Blowfish>,
    /// The list of acks that are pending for completion. They should be ordered
    /// in the vector, so a simple binary search is enough.
    sent_acks: Vec<u32>,
    /// The list of received acks, it's used for sending. 
    received_acks: Vec<u32>,
    // /// Set to true when an ack should be sent even if not bundle is set.
    auto_ack: bool,
}

impl Channel {

    fn new(blowfish: Arc<Blowfish>) -> Self {
        Self {
            blowfish,
            sent_acks: Vec::new(),
            received_acks: Vec::new(),
            auto_ack: false,
        }
    }

    fn add_sent_ack(&mut self, sequence_num: u32) {

        debug_assert!(
            self.sent_acks.is_empty() || *self.sent_acks.last().unwrap() < sequence_num,
            "sequence number is not ordered"
        );

        self.sent_acks.push(sequence_num);
        println!("[AFTER ADD] sent_acks: {:?}", self.sent_acks);

    }

    fn remove_cumulative_ack(&mut self, ack: u32) {
        
        let discard_offset = match self.sent_acks.binary_search(&ack) {
            Ok(index) => index,
            Err(index) => index,
        };

        self.sent_acks.drain(..discard_offset);
        println!("[AFTER REM] sent_acks: {:?}", self.sent_acks);

    }

    fn add_received_ack(&mut self, sequence_num: u32) {

        match self.received_acks.binary_search(&sequence_num) {
            Ok(_) => {
                // Maybe an error to receive the same ack twice?
            }
            Err(index) => {
                self.received_acks.insert(index, sequence_num);
            }
        }

        println!("[AFTER ADD] received_acks: {:?}", self.received_acks);

    }

    #[inline]
    fn set_auto_ack(&mut self) {
        self.auto_ack = true;
    }

    /// Take the auto ack and disable it anyway.
    #[inline]
    fn take_auto_ack(&mut self) -> bool {
        std::mem::replace(&mut self.auto_ack, false)
    }

    /// Return the last ack that is part of an chain.
    fn get_cumulative_ack(&mut self) -> Option<u32> {

        let first_ack = *self.received_acks.get(0)?;
        let mut cumulative_ack = first_ack;

        for &sequence_num in &self.received_acks[1..] {
            if sequence_num == cumulative_ack + 1 {
                cumulative_ack += 1;
            } else {
                break
            }
        }

        if cumulative_ack > first_ack {
            let diff = cumulative_ack - first_ack;
            self.received_acks.drain(..diff as usize);
        }

        Some(cumulative_ack)

    }

    #[inline]
    fn get_cumulative_ack_exclusive(&mut self) -> Option<u32> {
        self.get_cumulative_ack().map(|n| n + 1)
    }

}

/// Internal structure to keep fragments from a given sequence.
#[derive(Debug)]
struct BundleFragments {
    fragments: Vec<Option<Box<Packet>>>,  // Using boxes to avoid moving huge structures.
    seq_count: u32,
    last_update: Instant,
}

impl BundleFragments {

    /// Create from sequence length.
    fn new(seq_len: u32) -> Self {
        Self {
            fragments: (0..seq_len).map(|_| None).collect(),
            seq_count: 0,
            last_update: Instant::now()
        }
    }

    /// This this fragments packets and reset internal count to zero.
    fn drain(&mut self) -> impl Iterator<Item = Box<Packet>> {
        self.seq_count = 0;
        std::mem::take(&mut self.fragments)
            .into_iter()
            .filter_map(|slot| slot)
    }

    /// Set a fragment.
    fn set(&mut self, num: u32, packet: Box<Packet>) {
        let frag = &mut self.fragments[num as usize];
        if frag.is_none() {
            self.seq_count += 1;
        }
        self.last_update = Instant::now();
        *frag = Some(packet);
    }

    #[inline]
    fn is_old(&self, instant: Instant, timeout: Duration) -> bool {
        instant - self.last_update > timeout
    }

    #[inline]
    fn is_full(&self) -> bool {
        self.seq_count as usize == self.fragments.len()
    }

    /// Convert this structure to a bundle, **safe to call only if `is_full() == true`**.
    #[inline]
    fn into_bundle(self) -> Bundle {
        assert!(self.is_full());
        let packets = self.fragments.into_iter()
            .map(|o| o.unwrap())
            .collect();
        Bundle::with_multiple(packets)
    }

}


///  Kind of error that caused a packet to be rejected from this socket and not received.
#[derive(Debug, Clone, thiserror::Error)]
pub enum PacketRejectionError {
    /// The packet is part of a sequence but no other packets of the sequence have been
    /// found and therefore no bundle can be reconstructed.
    #[error("timed out")]
    TimedOut,
    /// The packet should be decrypted but it failed.
    #[error("invalid encryption")]
    InvalidEncryption,
    /// The packet could not be synchronized from its data.
    #[error("sync error: {0}")]
    Config(#[from] PacketConfigError),
}
