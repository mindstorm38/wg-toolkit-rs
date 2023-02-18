//! Structures for managing bundles of packets.

use std::io::{self, Write, Cursor, Read, Seek, SeekFrom};
use std::collections::hash_map::Entry;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::hash::Hash;

use byteorder::{ReadBytesExt, WriteBytesExt, LE};

use super::packet::{Packet, PACKET_MAX_BODY_LEN, PACKET_FLAGS_LEN};
use super::element::reply::{ReplyHeaderCodec, ReplyCodec, Reply, REPLY_ID};
use super::element::ElementCodec;

use crate::util::cursor::SubCursor;


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
    last_request_header_offset: usize,
    // /// Offsets to all requests' headers in this bundle, it's used to add replay IDs.
    // /// Each tuple in the vec are of the form `(packet_index, request_header_offset)`.
    // request_header_offsets: Vec<(usize, usize)>
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
            last_request_header_offset: 0,
            // request_header_offsets: Vec::new()
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

    /// Add a basic element to this bundle.
    #[inline]
    pub fn add_element<E: ElementCodec>(&mut self, id: u8, codec: &E, elt: E::Element) {
        self.add_element_raw(id, codec, elt, None);
    }

    /// Add a request element to this bundle, with a given request ID.
    #[inline]
    pub fn add_request<E: ElementCodec>(&mut self, id: u8, codec: &E, elt: E::Element, request_id: u32) {
        self.add_element_raw(id, codec, elt, Some(request_id));
    }

    /// Add a reply element to this bundle, for a given request ID.
    /// Such elements are special and don't require an ID, such elements are always of
    /// a 32-bit variable length and prefixed with the request ID. The length codec from
    /// the given codec is not used.
    #[inline]
    pub fn add_reply<E: ElementCodec>(&mut self, codec: &E, elt: E::Element, request_id: u32) {
        self.add_element(REPLY_ID, &ReplyCodec::new(codec), Reply::new(request_id, elt))
    }

    pub fn add_element_raw<E>(&mut self, id: u8, codec: &E, elt: E::Element, request: Option<u32>)
    where
        E: ElementCodec
    {

        if self.force_new_packet {
            self.add_packet();
            self.force_new_packet = false;
        }

        // Allocate element's header, +1 for element's ID, +6 reply_id and link offset.
        let header_len = E::LEN.len() + 1 + if request.is_some() { 6 } else { 0 };
        let header_slice = self.reserve_exact(header_len);
        header_slice[0] = id;

        if let Some(request_id) = request {
            let mut request_header_cursor = Cursor::new(&mut header_slice[header_len - 6..]);
            request_header_cursor.write_u32::<LE>(request_id).unwrap();
            request_header_cursor.write_u16::<LE>(0).unwrap(); // Next request offset set to null.
        }

        // Update the current packet's cursor and header length.
        let cur_packet_idx = self.packets.len() - 1;
        let cur_packet = &mut self.packets[cur_packet_idx];
        let cur_packet_end = cur_packet.len();
        let cur_packet_elt_offset = cur_packet_end - header_len;

        if request.is_some() {
            let cur_request_header_offset = cur_packet_end - 6;
            if self.last_request_header_offset == 0 {
                cur_packet.set_request_first_offset(cur_packet_elt_offset);
            } else {
                let mut next_request_offset_cursor = Cursor::new(
                    &mut cur_packet.get_data_mut()[self.last_request_header_offset + 4..]);
                next_request_offset_cursor.write_u16::<LE>(cur_packet_elt_offset as u16).unwrap();
            }
            self.last_request_header_offset = cur_request_header_offset;
        }

        // Write the actual element's content.
        let mut writer = BundleWriter::new(self);
        // For now we just unwrap the encode result, because no IO error should be produced by a BundleWriter.
        codec.encode(&mut writer, elt).unwrap();
        // encoder.encode(&mut writer).unwrap();
        let length = writer.len as u32;

        // Finally write length.
        let cur_packet = &mut self.packets[cur_packet_idx];
        let cur_len_slice = &mut cur_packet.get_data_mut()[cur_packet_elt_offset + 1..];
        // Unwrap because we now there is enough space at the given position.
        E::LEN.write(Cursor::new(cur_len_slice), length).unwrap();

    }

    /// Finalize the bundle by synchronizing all packets in it and setting
    /// their sequence id.
    /// This can be called multiple times, the result is stable.
    pub fn finalize(&mut self, seq_id: &mut u32) {

        // Sequence IDs
        let multi_packet = self.packets.len() > 1;
        let seq_first = *seq_id;
        let seq_last = seq_first + self.packets.len() as u32 - 1;

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
    pub fn get_packets_mut(&mut self) -> &mut [Box<Packet>] {
        &mut self.packets[..]
    }

    /// See `BundleElementReader`.
    pub fn get_element_reader(&self) -> BundleElementReader<'_> {
        BundleElementReader::new(self)
    }

    /// Internal method to add a new packet at the end of the chain.
    fn add_packet(&mut self) {
        let packet = Packet::new_boxed(self.has_prefix);
        self.available_len = packet.available_len();
        self.packets.push(packet);
        self.last_request_header_offset = 0;
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

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let slice = self.bundle.reserve(buf.len());
        slice.copy_from_slice(&buf[..slice.len()]);
        self.len += slice.len();
        Ok(slice.len())
    }

    fn flush(&mut self) -> io::Result<()> {
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
            len: bundle.get_packets().iter().map(|p| p.body_len()).map(|n| n as u64).sum(),
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

    /// Internal function to discard all head packets until there is one
    /// with a non-empty body.
    fn discard_packets_until_non_empty(&mut self) {
        let mut packets = self.packets;
        while !packets.is_empty() && packets[0].body_len() == 0 {
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
    fn get_packet_body_pos(&self) -> usize {
        self.packet_body_pos
    }

    /// Get the real position of the cursor within the current packet's data.
    fn get_packet_data_pos(&self) -> usize {
        self.get_packet_body_pos() + PACKET_FLAGS_LEN
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
            let packet_len = packet.body_len() as u64;
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
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.packets.is_empty() {
            Ok(0)
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

    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        Ok(self.seek_absolute(match pos {
            SeekFrom::Start(pos) => pos,
            SeekFrom::Current(pos) => (self.pos as i64 + pos) as u64,
            SeekFrom::End(pos) => (self.pos as i64 + pos) as u64
        }))
    }

    fn stream_position(&mut self) -> io::Result<u64> {
        Ok(self.pos)
    }

}


/// A special iterator designed to fetch each element on the bundle.
pub struct BundleElementReader<'bundle> {
    bundle_reader: BundleReader<'bundle>,
    next_request_offset: usize
}

impl<'bundle> BundleElementReader<'bundle> {

    fn new(bundle: &'bundle Bundle) -> Self {
        let bundle_reader = BundleReader::new(bundle);
        Self {
            next_request_offset: bundle_reader.get_packet()
                .map(Packet::get_request_first_offset)
                .unwrap_or(0),
            bundle_reader
        }
    }

    /// Read the current element's identifier. This call return the same result until
    /// you explicitly choose to go to the next element while reading the element
    pub fn read_id(&self) -> Option<u8> {
        self.bundle_reader.get_packet_remaining_data().get(0).copied()
    }

    /// Return `true` if the current element is a request, this is just dependent of
    /// the current position within the current packet.
    pub fn is_request(&self) -> bool {
        let data_pos = self.bundle_reader.get_packet_data_pos();
        self.next_request_offset != 0 && data_pos == self.next_request_offset
    }

    /// Read the current element, return a guard that you should use a codec to decode
    /// the element depending on its type with. *This is a simpler version to use over
    /// standard `read_element` method because it handle reply elements for you.*
    pub fn next_element(&mut self) -> Option<BundleElement<'_, 'bundle>> {
        match self.read_id() {
            Some(REPLY_ID) => {
                match self.read_element(&ReplyHeaderCodec, false) {
                    Ok(elt) => {
                        debug_assert!(elt.request_id.is_none(), "Replies should not be request at the same time.");
                        Some(BundleElement::Reply(elt.element, ReplyElementReader(self)))
                    }
                    Err(_) => {
                        None
                    }
                }
            }
            Some(id) => {
                Some(BundleElement::Simple(id, SimpleElementReader(self)))
            }
            None => None
        }
    }

    /// Try to decode the current element using a given codec. You can choose to go
    /// to the next element using the `next` argument.
    pub fn read_element<E>(&mut self, codec: &E, next: bool) -> Result<Element<E::Element>, ReadElementError>
    where
        E: ElementCodec
    {

        let request = self.is_request();
        let header_len = E::LEN.len() + 1 + if request { 6 } else { 0 };

        if self.bundle_reader.get_packet_remaining_data().len() < header_len {
            return Err(ReadElementError::TooShortPacket);
        }

        // We store the starting position of the element, it will be used if we need to rollback.
        let elt_pos = self.bundle_reader.pos();

        match self.read_element_internal(codec, next, request) {
            Ok(elt) if next => Ok(elt),
            Ok(elt) => {
                // If no error but we don't want to go next.
                self.bundle_reader.seek_absolute(elt_pos);
                Ok(elt)
            }
            Err(e) => {
                // If any error happens, we cancel the operation.
                self.bundle_reader.seek_absolute(elt_pos);
                Err(ReadElementError::Io(e))
            }
        }

    }

    /// Internal only. Used by `next` to wrap all IO errors and reset seek if an error happens.
    #[inline(always)]
    fn read_element_internal<E>(&mut self, codec: &E, next: bool, request: bool) -> io::Result<Element<E::Element>>
    where
        E: ElementCodec
    {

        let start_packet = self.bundle_reader.get_packet().unwrap();

        let _elt_id = self.bundle_reader.read_u8()?;
        let elt_len = E::LEN.read(&mut self.bundle_reader)? as u64;

        let reply_id = if request {
            let reply_id = self.bundle_reader.read_u32::<LE>()?;
            self.next_request_offset = self.bundle_reader.read_u16::<LE>()? as usize;
            Some(reply_id)
        } else {
            None
        };

        let elt_data_begin = self.bundle_reader.pos();
        let elt_data_end = elt_data_begin + elt_len;

        // We can use unchecked because we know that the given 'inner' reader
        // is placed at the same position as given 'begin'.
        let elt_data_reader = SubCursor::new_unchecked(
            &mut self.bundle_reader,
            elt_data_begin,
            elt_data_end
        );

        let element = codec.decode(elt_data_reader, elt_len)?;

        // We seek to the end only if we want to go next.
        if next {

            // If decoding is successful, go to the next packet.
            self.bundle_reader.seek_absolute(elt_data_end);

            // Here we check if we have changed packets during decoding of the element.
            // If changed, we change the next request offset.
            match self.bundle_reader.get_packet() {
                Some(end_packet) => {
                    if !std::ptr::eq(start_packet, end_packet) {
                        self.next_request_offset = end_packet.get_request_first_offset();
                    }
                    // Else, we are still in the same packet so we don't need to change this.
                }
                None => self.next_request_offset = 0
            }

        }

        Ok(Element {
            element,
            // id: elt_id,
            request_id: reply_id
        })

    }

}


/// An element read from `BundleElementReader` and `BundleElement` variants,
/// also containing the element's ID and an optional request ID.
pub struct Element<E> {
    /// The actual element.
    pub element: E,
    /// The request ID if the element is a request. Not to be confused with
    /// the reply ID if the element is a `Reply`.
    pub request_id: Option<u32>
}

impl<E> Into<Element<E>> for Element<Reply<E>> {
    fn into(self) -> Element<E> {
        Element {
            element: self.element.element,
            request_id: self.request_id
        }
    }
}


/// Error variants when polling next element from a bundle reader.
#[derive(Debug)]
pub enum ReadElementError {
    /// The current packet isn't enough large for element's header,
    /// which need to be on a single packet.
    TooShortPacket,
    /// An unexpected or unhandled IO error happened.
    Io(io::Error)
}


/// Bundle element variant iterated from `BundleElementIter`.
/// This enum provides a better way to read replies using sub codecs.
pub enum BundleElement<'reader, 'bundle> {
    /// A simple element with an ID and a reader.
    Simple(u8, SimpleElementReader<'reader, 'bundle>),
    /// A reply element with request ID and a reader.
    Reply(u32, ReplyElementReader<'reader, 'bundle>)
}

impl BundleElement<'_, '_> {

    /// Return `true` if this element is a simple one.
    pub fn is_simple(&self) -> bool {
        matches!(self, BundleElement::Simple(_, _))
    }

    /// Return `true` if this element is a reply.
    pub fn is_reply(&self) -> bool {
        matches!(self, BundleElement::Reply(_, _))
    }

}

/// The simple variant of element, provides direct decoding using a codec.
pub struct SimpleElementReader<'reader, 'bundle>(&'reader mut BundleElementReader<'bundle>);

impl SimpleElementReader<'_, '_> {

    /// Same as `read` but never go to the next element *(this is why this method doesn't take
    /// self by value)*.
    pub fn read_stable<E: ElementCodec>(&mut self, codec: &E) -> Result<Element<E::Element>, ReadElementError> {
        self.0.read_element(codec, false)
    }

    /// Read the element using the given codec. This method take self by value and automatically
    /// go the next element if read is successful, if not successful you will need to call
    /// `Bundle::next_element` again.
    pub fn read<E: ElementCodec>(self, codec: &E) -> Result<Element<E::Element>, ReadElementError> {
        self.0.read_element(codec, true)
    }

}

/// The reply variant of element, provides a way to read replies and get `Reply` elements
/// containing the final element.
pub struct ReplyElementReader<'reader, 'bundle>(&'reader mut BundleElementReader<'bundle>);

impl<'reader, 'bundle> ReplyElementReader<'reader, 'bundle> {

    /// Same as `read` but never go to the next element *(this is why this method doesn't take
    /// self by value)*.
    ///
    /// This method doesn't returns the reply element but the final element.
    pub fn read_stable<E: ElementCodec>(&mut self, codec: &E) -> Result<Element<E::Element>, ReadElementError> {
        self.0.read_element(&ReplyCodec::new(codec), false).map(Into::into)
    }

    /// Read the reply element using the given codec. This method take self by value and
    /// automatically go the next element if read is successful, if not successful you
    /// will need to call `Bundle::next_element` again.
    ///
    /// This method doesn't returns the reply element but the final element.
    pub fn read<E: ElementCodec>(self, codec: &E) -> Result<Element<E::Element>, ReadElementError> {
        self.0.read_element(&ReplyCodec::new(codec), true).map(Into::into)
    }

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
