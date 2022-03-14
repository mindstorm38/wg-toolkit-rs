//! Structures for managing bundles of packets.

use std::io::{Write, Cursor, Read, Seek, SeekFrom};
use std::collections::hash_map::Entry;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::hash::Hash;

use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

use super::packet::{Packet, PACKET_MAX_BODY_LEN, PACKET_FLAGS_LEN};
use super::element::ElementCodec;
use crate::util::SubCursor;


pub const BUNDLE_FRAGMENT_MAX_AGE: Duration = Duration::from_secs(10);


/// A elements bundle, used to pack elements and encode them.
pub struct Bundle {
    /// Chain of packets.
    packets: Vec<Box<Packet>>,
    /// Indicate if a new packet must be added before a new message. It's used to avoid
    /// mixing manually-added packets with packets from newly inserted elements. It's
    /// mandatory because we don't know what `request_last_link_offset` should be from
    /// manually-added packets.
    force_new_packet: bool,
    /// Available length on the last packet, used to avoid borrowing issues.
    available_len: usize,
    /// If packets in this bundle has a prefix.
    has_prefix: bool,
    /// Offset of the link of the last request, `0` if not request yet.
    request_last_link_offset: usize,
    /// Offsets to all requests' headers in this bundle, it's used to add replay IDs.
    /// Each tuple in the vec are of the form `(packet_index, request_header_offset)`.
    request_header_offsets: Vec<(usize, usize)>
}

impl Bundle {

    /// Internal common function to create new bundle.
    #[inline]
    fn new(packets: Vec<Box<Packet>>, has_prefix: bool) -> Self {
        Bundle {
            available_len: packets.last().map(|p| p.available_len()).unwrap_or(0),
            packets,
            force_new_packet: true,
            has_prefix,
            request_last_link_offset: 0,
            request_header_offsets: Vec::new()
        }
    }

    /// Construct a new empty bundle, this bundle doesn't
    /// allocate until you add the first element.
    pub fn new_empty(has_prefix: bool) -> Bundle {
        Self::new(Vec::new(), has_prefix)
    }

    /// Create a new bundle with one predefined packet.
    pub fn from_single(packet: Box<Packet>, has_prefix: bool) -> Self {
        Self::new(vec![packet], has_prefix)
    }

    /// Create a new bundle with multiple predefined packets.
    pub fn from_packets(packets: Vec<Box<Packet>>, has_prefix: bool) -> Self {
        Self::new(packets, has_prefix)
    }

    /// Add a new element to this bundle, everything is managed for the caller,
    /// new packets are created if needed and the message can be a request.
    pub fn add_element<E: ElementCodec>(&mut self, id: u8, elt: &E, request: bool) {

        if self.force_new_packet {
            self.add_packet();
            self.force_new_packet = false;
        }

        // Allocate element's header, +1 for element's ID, +6 reply_id and link offset.
        let header_len = E::LEN.header_len() + 1 + if request { 6 } else { 0 };
        let header_slice = self.reserve_exact(header_len);
        header_slice[0] = id;

        if request {
            // Reset all request's header fields.
            header_slice[header_len - 6..][..6].fill(0);
        }

        // Update the current packet's cursor and header length.
        let cur_packet_idx = self.packets.len() - 1;
        let cur_packet = &mut self.packets[cur_packet_idx];
        let cur_packet_elt_offset = cur_packet.len() - header_len;

        if request {

            // Register request.
            self.request_header_offsets.push((cur_packet_idx, cur_packet_elt_offset));

            if self.request_last_link_offset == 0 {
                cur_packet.set_request_first_offset(cur_packet_elt_offset);
            } else {

                let link_slice = &mut cur_packet.data[self.request_last_link_offset..];
                Cursor::new(link_slice)
                    .write_u16::<LittleEndian>(cur_packet_elt_offset as u16)
                    .unwrap();

            }

            self.request_last_link_offset = cur_packet.len() - 2;

        }

        // Write the actual element's content.
        let mut writer = BundleWriter::new(self);
        // For now we just unwrap the encode result, because no IO error should be produced by a BundleWriter.
        elt.encode(&mut writer).unwrap();
        let length = writer.len as u32;

        // Finally write length.
        let cur_packet = &mut self.packets[cur_packet_idx];
        let cur_len_slice = &mut cur_packet.data[cur_packet_elt_offset + 1..];
        // Unwrap because we now there is enough space at the given position.
        E::LEN.write(&mut Cursor::new(cur_len_slice), length).unwrap();

    }

    /// Finalize the bundle by synchronizing all packets in it and setting
    /// their sequence id.
    /// This can be called multiple times, the result is stable.
    pub fn finalize(&mut self, seq_id: &mut u32) {

        // TODO: Setup reply for requests IDs

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
            packet.sync_data();
        }

    }

    #[inline]
    pub fn len(&self) -> usize {
        self.packets.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.packets.is_empty()
    }

    /// Get a slice of all packets of this bundle.
    #[inline]
    pub fn get_packets(&self) -> &[Box<Packet>] {
        &self.packets[..]
    }

    #[inline]
    pub fn get_packet(&self, index: usize) -> Option<&Packet> {
        self.packets.get(index).map(|p| &**p)
    }

    /// See `BundleRawElementsIter`.
    pub fn iter_raw_elements(&self) -> BundleRawElementsIter {
        BundleRawElementsIter::new(self)
    }

    /// Internal method to add a new packet at the end of the chain.
    fn add_packet(&mut self) {
        self.packets.push(Packet::new_boxed(self.has_prefix));
        self.request_last_link_offset = 0;
    }

    /// Reserve exactly the given length in the current packet or a new one if
    /// this such space is not available in the current packet. **An exact
    /// reservation must not exceed maximum packet size.**
    fn reserve_exact(&mut self, len: usize) -> &mut [u8] {
        debug_assert!(len <= PACKET_MAX_BODY_LEN);
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


/// An internal writer implementation used to append data to a bundle,
/// adding packets if needed.
struct BundleWriter<'a> {
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


/// A reader implementation used to read a bundle while ignoring headers
/// and footers. This reader is mainly used by `BundleRawElementsIter`.
/// Headers and footers are ignored while reading and seeking because
/// elements can span over multiple packet's body and headers/footers
/// are only relevant to know the first request
struct BundleReader<'a> {
    /// The bundle we are reading from.
    bundle: &'a Bundle,
    /// The current packet's index in the bundle.
    packets: &'a [Box<Packet>],
    /// The current remaining packet's data from the current packet.
    packet_body: &'a [u8],
    /// The internal position of the reader within the current packet's body.
    packet_body_pos: usize,
    /// The total length available in the bundle.
    len: u64,
    /// The internal position of the reader, used for seeking operations.
    pos: u64,
}

impl<'a> BundleReader<'a> {

    fn new(bundle: &'a Bundle) -> Self {
        let mut ret = Self {
            bundle,
            packets: bundle.get_packets(),
            packet_body: &[],
            len: bundle.get_packets().iter().map(|p| p.get_body_len()).map(|n| n as u64).sum(),
            pos: 0,
            packet_body_pos: 0
        };
        ret.discard_packets_until_non_empty();
        ret
    }

    #[inline]
    fn len(&self) -> u64 {
        self.len
    }

    #[inline]
    fn pos(&self) -> u64 {
        self.pos
    }

    /// Internal function to discord all head packets until there is one
    /// with a non-empty body.
    fn discard_packets_until_non_empty(&mut self) {
        let mut packets = self.packets;
        while !packets.is_empty() && packets[0].get_body_len() == 0 {
            packets = &packets[1..];
        }
        self.packets = packets;
        self.packet_body = packets.first().map(|p| p.get_body_data()).unwrap_or(&[]);
        self.packet_body_pos = 0;
    }

    /// Get the packet that will be read on the next call. `None` if
    /// no more data is available. This packet always has a non-empty
    /// body and the read operation will read at least 1 byte.
    fn get_packet(&self) -> Option<&'a Packet> {
        self.packets.first().map(|p| &**p)
    }

    /// Get a slice to the remaining data in the current packet.
    /// Empty if no more packet to read from.
    #[inline]
    fn get_packet_remaining_data(&self) -> &[u8] {
        self.packet_body
    }

    /// Get the real position of the cursor within the current packet's body.
    #[inline]
    fn get_packet_body_pos(&self) -> usize {
        self.packet_body_pos
    }

    /// Optimized absolute position seek for bundle structure.
    fn seek_absolute(&mut self, abs_pos: u64) -> u64 {
        if abs_pos >= self.len {
            self.packets = &[];
            self.packet_body = &[];
            self.packet_body_pos = 0;
            self.pos = self.len;
        } else {
            let rel_pos = abs_pos as i64 - self.pos as i64;
            if rel_pos > 0 {
                if (rel_pos as usize) < self.packet_body.len() {
                    // Here we are in the same packet.
                    self.packet_body = &self.packet_body[rel_pos as usize..];
                    self.packet_body_pos += rel_pos as usize;
                } else {
                    // We are after the current packet.
                    self.packets = &self.packets[1..];
                    self.seek_relative_unchecked(rel_pos as u64);
                }
            } else if rel_pos < 0 {
                if rel_pos >= -(self.packet_body_pos as i64) {
                    // We are in the same packet but before data pointer.
                    self.packet_body_pos = (self.packet_body_pos as i64 + rel_pos) as usize;
                    self.packet_body = &self.packets[0].get_body_data()[self.packet_body_pos..];
                } else {
                    // We are in a packet before.
                    self.packets = self.bundle.get_packets();
                    self.seek_relative_unchecked(abs_pos);
                }
            }
            self.pos = abs_pos;
        }
        self.pos
    }

    /// An unchecked relative incremented seek, to use only with `seek`.
    /// It is unchecked because the internal `pos` is not updated and
    /// this shouldn't go above or equal to length.
    fn seek_relative_unchecked(&mut self, mut offset: u64) {
        while offset != 0 {
            let packet = &self.packets[0];
            let packet_len = packet.get_body_len() as u64;
            if offset >= packet_len {
                offset -= packet_len;
                self.packets = &self.packets[1..];
            } else {
                self.packet_body_pos = offset as usize;
                self.packet_body = &packet.get_body_data()[self.packet_body_pos..];
                return;
            }
        }
        self.discard_packets_until_non_empty();
    }

}

impl<'a> Read for BundleReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.packets.is_empty() {
            Err(std::io::ErrorKind::UnexpectedEof.into())
        } else {
            let len = buf.len().min(self.packet_body.len());
            buf[..len].copy_from_slice(&self.packet_body[..len]);
            self.packet_body = &self.packet_body[len..];
            self.pos += len as u64;
            if self.packet_body.is_empty() {
                self.packets = &self.packets[1..];
                self.discard_packets_until_non_empty();
            }
            Ok(len)
        }
    }
}

impl<'a> Seek for BundleReader<'a> {

    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        Ok(self.seek_absolute(match pos {
            SeekFrom::Start(pos) => pos,
            SeekFrom::Current(pos) => (self.pos as i64 + pos) as u64,
            SeekFrom::End(pos) => (self.pos as i64 + pos) as u64
        }))
    }

    fn stream_position(&mut self) -> std::io::Result<u64> {
        Ok(self.pos)
    }

}


/// A special iterator designed to fetch each message on the bundle.
/// This is not a real `Iterator` implementor because the `ElementCodec`
/// is needed. And you can get the right codec from the next element id.
pub struct BundleRawElementsIter<'a> {
    bundle_reader: BundleReader<'a>,
    next_request_offset: usize
}

impl<'a> BundleRawElementsIter<'a> {

    fn new(bundle: &'a Bundle) -> Self {
        let bundle_reader = BundleReader::new(bundle);
        Self {
            next_request_offset: bundle_reader.get_packet()
                .map(Packet::get_request_first_offset)
                .unwrap_or(0),
            bundle_reader
        }
    }

    /// Return the next element's identifier, to actually decode it you
    /// need to call `next_element`.
    #[inline]
    pub fn next_id(&self) -> Option<u8> {
        self.bundle_reader.get_packet_remaining_data().get(0).copied()
    }

    /// Return `true` if the next element is a request.
    pub fn next_is_request(&self) -> bool {
        // In this packet_pos we just ignore the optional prefix, this is why we add flags len.
        let packet_pos = PACKET_FLAGS_LEN + self.bundle_reader.get_packet_body_pos();
        self.next_request_offset != 0 && packet_pos == self.next_request_offset
    }

    /// Decode the next element using the given codec, if the decode
    /// fails (and return `None`), the internal state is kept as before
    /// the call.
    pub fn next<E: ElementCodec>(&mut self) -> Option<RawElement<E>> {

        let request = self.next_is_request();
        let header_len = E::LEN.header_len() + 1 + if request { 6 } else { 0 };

        if self.bundle_reader.get_packet_remaining_data().len() < header_len {
            return None;
        }

        let elt_pos = self.bundle_reader.pos();

        match self.next_internal::<E>(request) {
            Ok(elt) => Some(elt),
            Err(_) => {
                // If any error happens, we cancel the operation.
                self.bundle_reader.seek_absolute(elt_pos);
                None
            }
        }

    }

    /// Internal only. Used by `next` to wrap all IO errors and reset if an error happens.
    #[inline(always)]
    fn next_internal<E: ElementCodec>(&mut self, request: bool) -> std::io::Result<RawElement<E>> {

        let start_packet = self.bundle_reader.get_packet().unwrap();

        let elt_id = self.bundle_reader.read_u8()?;
        let elt_len = E::LEN.read(&mut self.bundle_reader)? as u64;

        let reply_id = if request {
            let reply_id = self.bundle_reader.read_u32::<LittleEndian>()?;
            self.next_request_offset = self.bundle_reader.read_u16::<LittleEndian>()? as usize;
            Some(reply_id)
        } else {
            None
        };

        let elt_data_begin = self.bundle_reader.pos();
        let elt_data_end = elt_data_begin + elt_len;

        let mut elt_data_reader = SubCursor::new(
            &mut self.bundle_reader,
            elt_data_begin,
            elt_data_end
        );

        let elt = E::decode(&mut elt_data_reader)?;

        self.bundle_reader.seek_absolute(elt_data_end);

        match self.bundle_reader.get_packet() {
            Some(end_packet) => {
                if !std::ptr::eq(start_packet, end_packet) {
                    self.next_request_offset = end_packet.get_request_first_offset();
                }
                // Else, we are still in the same packet so we don't need to change this.
            }
            None => self.next_request_offset = 0
        }

        Ok(RawElement {
            elt,
            id: elt_id,
            reply_id
        })

    }

}


pub struct RawElement<E: ElementCodec> {
    pub elt: E,
    pub id: u8,
    pub reply_id: Option<u32>
}


/// A structure that reassemble received bundles' fragments. You can provide an
/// additional key type `O` to be used to identify fragments' origin. For example
/// it can be a client address.
pub struct BundleAssembler<O = ()> {
    /// Fragments tracker.
    fragments: HashMap<(O, u32), BundleFragments>,
    /// If packets in this bundle has a prefix.
    has_prefix: bool
}

impl<O> BundleAssembler<O>
where
    O: Hash + Eq
{

    pub fn new(has_prefix: bool) -> Self {
        Self {
            fragments: HashMap::new(),
            has_prefix
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
                        Some(o.remove().into_bundle(self.has_prefix))
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
            Some(Bundle::from_single(packet, self.has_prefix))
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
    fn into_bundle(self, has_prefix: bool) -> Bundle {
        debug_assert!(self.is_full());
        let packets = self.fragments.into_iter()
            .map(|o| o.unwrap())
            .collect();
        Bundle::from_packets(packets, has_prefix)
    }

}
