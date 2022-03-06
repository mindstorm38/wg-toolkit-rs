use std::collections::hash_map::Entry;
use std::io::{Write, Cursor};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::hash::Hash;

use super::element::ElementCodec;
use super::packet::Packet;


pub const BUNDLE_FRAGMENT_MAX_AGE: Duration = Duration::from_secs(10);


/// A elements bundle, used to pack elements and encode them.
pub struct Bundle {
    /// Chain of packets.
    packets: Vec<Box<Packet>>,
    /// Available length on the last packet.
    available_len: usize,
}

impl Bundle {

    pub fn new() -> Bundle {
        Bundle {
            packets: Vec::new(),
            available_len: 0
        }
    }

    /// Create a new bundle with one packet.
    pub fn from_single(packet: Box<Packet>) -> Self {
        Bundle {
            available_len: packet.available_len(),
            packets: vec![packet]
        }
    }

    pub fn from_packets(packets: Vec<Box<Packet>>) -> Self {
        debug_assert!(!packets.is_empty());
        Bundle {
            available_len: packets.last().unwrap().available_len(),
            packets
        }
    }

    /// Add a new element to this bundle, everything is manager for the caller,
    /// new packets are created if needed and the message can be a request.
    pub fn add_element<E: ElementCodec>(&mut self, elt: E, request: bool) {

        if self.packets.is_empty() {
            self.add_packet();
        }

        // Allocate element's header, +1 for element's ID, +6 reply_id and link offset.
        let header_len = E::LEN.header_len() + 1 + if request { 6 } else { 0 };
        self.reserve_exact(header_len)[0] = E::ID;

        // Update the current packet's cursor and header length.
        let first_packet_idx = self.packets.len() - 1;
        let first_packet = &mut self.packets[first_packet_idx];
        if request {
            let cursor = first_packet.len;
            // -2 because link offset is encoded on two bytes (u16).
            first_packet.add_request(cursor, cursor + header_len - 2);
        }
        let first_packet_elt_offset = first_packet.len;
        first_packet.len += header_len;

        // Write the actual element's content.
        let mut writer = BundleWriter::new(self);
        // For now we just unwrap the encode result, because no IO error should be produced by a BundleWriter.
        elt.encode(&mut writer).unwrap();
        let length = writer.len as u32;

        // Finally write length.
        let first_packet = &mut self.packets[first_packet_idx];
        let first_len_slice = &mut first_packet.data[first_packet_elt_offset + 1..];
        // Unwrap because we now there is enough space at the given position.
        E::LEN.write(&mut Cursor::new(first_len_slice), length).unwrap();

    }

    /// Finalize the bundle by finalizing all packets in it and setting their sequence id.
    /// This can be called multiple times, the result is stable.
    pub fn finalize(&mut self, seq_id: &mut u32) {

        let multi_packet = self.packets.len() > 1;
        let seq_first = *seq_id;
        let seq_last = seq_first + self.packets.len() as u32;

        for packet in &mut self.packets {
            if multi_packet {
                packet.set_seq(seq_first, seq_last, *seq_id);
                *seq_id += 1;
            } else {
                packet.clear_seq();
            }
            packet.finalize();
        }

    }

    /// Internal method to add a new packet at the end of the chain.
    fn add_packet(&mut self) {
        self.packets.push(Box::new(Packet::new()));
    }

    /// Reserve exactly the given length in the current packet or a new one if
    /// this such space is not available in the current packet. **An exact
    /// reservation must not exceed maximum packet size.**
    fn reserve_exact(&mut self, len: usize) -> &mut [u8] {
        let new_packet = self.available_len < len;
        if new_packet {
            self.add_packet();
        }
        let packet = self.packets.last_mut().unwrap();
        if new_packet {
            self.available_len = packet.available_len();
        }
        self.available_len -= len;
        packet.reserve_unchecked(len)
    }

    /// Reserve up to the given length in the current packet, if 0 byte is
    /// available in the current packet, a new packet is created. The final
    /// reserved length is the size of the returned slice.
    fn reserve(&mut self, len: usize) -> &mut [u8] {
        let new_packet = self.available_len == 0;
        if new_packet {
            self.add_packet();
        }
        let packet = self.packets.last_mut().unwrap();
        if new_packet {
            self.available_len = packet.available_len();
        }
        let len = len.min(self.available_len);
        self.available_len -= len;
        packet.reserve_unchecked(len)
    }

}


/// A temporary writer implementation used to write on a bundle.
pub struct BundleWriter<'a> {
    bundle: &'a mut Bundle,
    len: usize
}

impl<'a> BundleWriter<'a> {

    /// Construct a new bundle writer, must be constructed only if at least one packet
    /// is already existing in the bundle.
    fn new(bundle: &'a mut Bundle) -> Self {
        Self { bundle, len: 0 }
    }

}

impl<'a> Write for BundleWriter<'a> {

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let slice = self.bundle.reserve(buf.len());
        slice.copy_from_slice(&buf[..slice.len()]);
        self.len += slice.len();
        Ok(slice.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }

}


/// A structure that reassemble received bundles' fragments. You can provide an
/// additional key type `O` to be used to identify fragments' origin. For example
/// it can be a client address.
pub struct BundleAssembler<O = ()> {
    fragments: HashMap<(O, u32), BundleFragments>
}

impl<O> BundleAssembler<O>
where
    O: Hash + Eq
{

    pub fn new() -> Self {
        Self {
            fragments: HashMap::new()
        }
    }

    /// Add the given packet to internal fragments and try to make a bundle if all fragments
    /// were received. *Special case for packet with no sequence number, in such case a bundle
    /// with this single packet is returned.*
    pub fn try_assemble(&mut self, from: O, packet: Box<Packet>) -> Option<Bundle> {
        if packet.has_seq() {
            let (seq_first, seq_last, seq) = packet.get_seq();
            match self.fragments.entry((from, seq_first)) {
                Entry::Occupied(mut o) => {
                    if o.get().is_old() {
                        o.get_mut().reset();
                    }
                    o.get_mut().set(seq, packet);
                    if o.get().is_full() {
                        Some(o.remove().into_bundle())
                    } else {
                        None
                    }
                },
                Entry::Vacant(v) => {
                    v.insert(BundleFragments::new(seq_last - seq_first + 1));
                    None
                }
            }
        } else {
            Some(Bundle::from_single(packet))
        }
    }

    /// Clean all incomplete outdated fragments.
    pub fn cleanup(&mut self) {
        self.fragments.retain(|_, v| !v.is_old());
    }

}


/// Internal structure to keep fragments from a given sequence.
struct BundleFragments {
    fragments: Vec<Option<Box<Packet>>>,  // Using boxes to avoid moving huge structures.
    seq_count: u32,
    last_update: Instant
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

    /// Reset all fragments.
    fn reset(&mut self) {
        self.fragments.iter_mut().for_each(|o| *o = None);
        self.seq_count = 0;
    }

    /// Set a fragment.
    fn set(&mut self, seq: u32, packet: Box<Packet>) {
        let frag = &mut self.fragments[seq as usize];
        if frag.is_none() {
            self.seq_count += 1;
        }
        self.last_update = Instant::now();
        *frag = Some(packet);
    }

    #[inline]
    fn is_old(&self) -> bool {
        self.last_update.elapsed() > BUNDLE_FRAGMENT_MAX_AGE
    }

    #[inline]
    fn is_full(&self) -> bool {
        self.seq_count as usize == self.fragments.len()
    }

    /// Convert this structure to a bundle, **safe to call only if `is_full() == true`**.
    fn into_bundle(self) -> Bundle {
        debug_assert!(self.is_full(), "You must call this only if the ");
        let packets = self.fragments.into_iter()
            .map(|o| o.unwrap())
            .collect();
        Bundle::from_packets(packets)
    }

}
