//! Structures for managing bundles of packets.

use std::io::{self, Write, Cursor, Read};
use std::collections::hash_map::Entry;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt;

use super::packet::{Packet, PacketConfig, PACKET_FLAGS_LEN, PACKET_MAX_BODY_LEN};
use super::element::reply::{Reply, ReplyHeader, REPLY_ID};
use super::element::{Element, TopElement};

use crate::util::io::*;
use crate::util::BytesFmt;


/// The default timeout duration for bundle fragments before being forgotten.
pub const BUNDLE_FRAGMENT_TIMEOUT: Duration = Duration::from_secs(10);


/// A bundle is a sequence of packets that are used to store elements. Elements of
/// various types, like regular elements, requests or replies can be simply added
/// and the number of packets contained in this bundle is automatically adjusted if
/// no more space is available.
/// 
/// Functions that are used to add elements provide a builder-like structure by
/// returning a mutable reference to itself.
#[derive(Debug)]
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
    /// Offset of the link of the last request, `0` if not request yet.
    last_request_header_offset: usize,
}

impl Bundle {

    /// Construct a new empty bundle, this bundle doesn't
    /// allocate until you add the first element.
    pub fn new() -> Bundle {
        Self::with_multiple(vec![])
    }

    /// Create a new bundle with one predefined packet.
    pub fn with_single(packet: Box<Packet>) -> Self {
        Self::with_multiple(vec![packet])
    }

    /// Create a new bundle with multiple predefined packets.
    pub fn with_multiple(packets: Vec<Box<Packet>>) -> Self {
        Self {
            available_len: packets.last().map(|p| p.content_available_len()).unwrap_or(0),
            packets,
            force_new_packet: true,
            last_request_header_offset: 0,
        }
    }

    /// Add an element to this bundle.
    #[inline]
    pub fn add_element<E: TopElement>(&mut self, elt: E, config: &E::Config) -> &mut Self {
        self.add_element_raw(elt, config, None)
    }

    /// Add a simple element to this bundle. Such elements have no config.
    #[inline]
    pub fn add_simple_element<E: TopElement<Config = ()>>(&mut self, elt: E) -> &mut Self {
        self.add_element(elt, &())
    }

    /// Add a request element to this bundle, with a given request ID.
    #[inline]
    pub fn add_request<E: TopElement>(&mut self, elt: E, config: &E::Config, request_id: u32) -> &mut Self {
        self.add_element_raw(elt, config, Some(request_id))
    }

    /// Add a request element to this bundle, with a given request ID. 
    /// Such elements have no config.
    #[inline]
    pub fn add_simple_request<E: TopElement<Config = ()>>(&mut self, elt: E, request_id: u32) -> &mut Self {
        self.add_request(elt, &(), request_id)
    }

    /// Add a reply element to this bundle, for a given request ID.
    /// 
    /// Such elements are special and don't require an ID, because they are always of
    /// a 32-bit variable length and prefixed with the request ID.
    #[inline]
    pub fn add_reply<E: Element>(&mut self, elt: E, config: &E::Config, request_id: u32) -> &mut Self {
        self.add_element(Reply::new(request_id, elt), config)
    }

    /// Add a reply element to this bundle, for a given request ID.
    /// Such elements have no config.
    #[inline]
    pub fn add_simple_reply<E: Element<Config = ()>>(&mut self, elt: E, request_id: u32) -> &mut Self {
        self.add_reply(elt, &(), request_id)
    }

    /// Raw method to add an element to this bundle, given an ID, the element and its 
    /// config. With an optional request ID.
    pub fn add_element_raw<E: TopElement>(&mut self, elt: E, config: &E::Config, request: Option<u32>) -> &mut Self {

        if self.force_new_packet {
            self.add_packet();
            self.force_new_packet = false;
        }

        const REQUEST_HEADER_LEN: usize = 6;

        // Allocate element's header, +1 for element's ID, +6 reply_id and link offset.
        let header_len = 1 + E::LEN.len() + if request.is_some() { REQUEST_HEADER_LEN } else { 0 };
        let header_slice = self.reserve_exact(header_len);

        if let Some(request_id) = request {
            let mut request_header_cursor = Cursor::new(&mut header_slice[header_len - 6..]);
            request_header_cursor.write_u32(request_id).unwrap();
            request_header_cursor.write_u16(0).unwrap(); // Next request offset set to null.
        }

        // Keep the packet index to rewrite the packet's length after writing it.
        let cur_packet_idx = self.packets.len() - 1;

        // IMPORTANT: All offsets are in the content, not the raw body or raw data.
        let cur_packet = &mut self.packets[cur_packet_idx];
        let cur_packet_len = cur_packet.content_len();
        let cur_packet_elt_offset = cur_packet_len - header_len;

        // NOTE: We add flags length to element offset because offset contains flags.
        if request.is_some() {
        
            if self.last_request_header_offset == 0 {
                // If there is no previous request, we set the first request offset.
                cur_packet.set_first_request_offset(PACKET_FLAGS_LEN + cur_packet_elt_offset);
            } else {
                // Add 4 because first 4 bytes is the request id.
                Cursor::new(&mut cur_packet.content_mut()[self.last_request_header_offset + 4..])
                    .write_u16((PACKET_FLAGS_LEN + cur_packet_elt_offset) as u16).unwrap();
            }

            // We keep the offset of the request header, it will be used if a request
            // element is added after this one so we can write the link to the next.
            self.last_request_header_offset = cur_packet_len - REQUEST_HEADER_LEN;
            
        }

        // Write the actual element's content.
        let mut writer = BundleWriter::new(self);
        // For now we just unwrap the encode result, because no IO error should be produced by a BundleWriter.
        let elt_id = elt.encode(&mut writer, config).unwrap();
        let length = writer.len as u32;

        // Finally write id and length, we can unwrap because we know that enough length is available.
        let cur_head_slice = &mut self.packets[cur_packet_idx].content_mut()[cur_packet_elt_offset..];
        cur_head_slice[0] = elt_id;
        E::LEN.write(Cursor::new(&mut cur_head_slice[1..]), length).unwrap();

        self

    }

    /// Return the number of packets in this bundle.
    #[inline]
    pub fn len(&self) -> usize {
        self.packets.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.packets.is_empty()
    }

    /// Clear the bundle by removing all packets.
    pub fn clear(&mut self) {
        self.packets.clear();
        self.force_new_packet = true;
        self.available_len = 0;
        self.last_request_header_offset = 0;
    }

    /// Get a slice of all packets of this bundle.
    #[inline]
    pub fn packets(&self) -> &[Box<Packet>] {
        &self.packets[..]
    }

    #[inline]
    pub fn packets_mut(&mut self) -> &mut [Box<Packet>] {
        &mut self.packets[..]
    }

    /// See `BundleElementReader`.
    pub fn get_element_reader(&self) -> BundleElementReader<'_> {
        BundleElementReader::new(self)
    }

    /// Internal method to add a new packet at the end of the chain.
    fn add_packet(&mut self) {
        let packet = Packet::new_boxed();
        self.available_len = packet.content_available_len();
        self.packets.push(packet);
        self.last_request_header_offset = 0;
    }

    /// Reserve exactly the given length in the current packet or a new one if
    /// such space is not available in the current packet. **Given length must 
    /// not exceed maximum packet size.**
    /// 
    /// This function is currently only used for writing the element's header.
    fn reserve_exact(&mut self, len: usize) -> &mut [u8] {
        debug_assert!(len <= PACKET_MAX_BODY_LEN);
        let new_packet = self.available_len < len;
        if new_packet {
            self.add_packet();
        }
        let packet = self.packets.last_mut().unwrap();
        self.available_len -= len;
        packet.grow(len)
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
        packet.grow(len)
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


/// A simple reader for bundle that join all packet's bodies into
/// a single stream. This is internally used by [`BundleElementReader`] 
/// for reading elements and replies.
/// 
/// *Note that it implements clone in order to save advancement of
/// the reader and allowing rollbacks.*
#[derive(Clone)]
struct BundleReader<'a> {
    /// The current packet with the remaining ones.
    packets: &'a [Box<Packet>],
    /// The remaining body data in the current packet.
    body: &'a [u8],
    /// The current position of the reader, used for requests.
    pos: usize,
}

impl<'a> BundleReader<'a> {

    fn new(bundle: &'a Bundle) -> Self {
        let packets = bundle.packets();
        Self {
            packets,
            body: packets.get(0)
                .map(|p| p.content())
                .unwrap_or(&[]),
            pos: 0,
        }
    }

    /// Internal function to get a reference to the current packet.
    fn packet(&self) -> Option<&'a Packet> {
        self.packets.get(0).map(|b| &**b)
    }

    /// Internal function that ensures that the body is not empty.
    /// If empty, it search for the next non-empty packet and return.
    /// 
    /// It returns true if the operation was successful, false otherwise.
    fn ensure(&mut self) -> bool {
        while self.body.is_empty() {
            if self.packets.is_empty() {
                return false; // No more data.
            } else {
                // Discard the current packet from the slice.
                self.packets = &self.packets[1..];
                // And if there is one packet, set the body from this packet.
                if let Some(p) = self.packets.get(0) {
                    self.body = p.content();
                }
            }
        }
        true
    }

    /// Internal function to goto a given position in the bundle.
    /// 
    /// *The given position is checked to be past the current one.*
    fn goto(&mut self, pos: usize) {
        assert!(pos >= self.pos, "given pos is lower than current pos");
        let mut remaining = pos - self.pos;
        while remaining != 0 && self.ensure() {
            let len = self.body.len().min(remaining);
            self.pos += len;
            remaining -= len;
        }
    }

}

impl<'a> Read for BundleReader<'a> {

    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {

        if !self.ensure() {
            return Ok(0);
        }

        let len = buf.len().min(self.body.len());
        buf[..len].copy_from_slice(&self.body[..len]);

        self.body = &self.body[len..];
        self.pos += len;

        Ok(len)

    }

}

impl fmt::Debug for BundleReader<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BundleReader")
            .field("packets", &self.packets)
            .field("body", &format_args!("{:X}", BytesFmt(self.body)))
            .field("pos", &self.pos)
            .finish()
    }
}


/// A special iterator designed to fetch each element on the bundle.
pub struct BundleElementReader<'a> {
    bundle_reader: BundleReader<'a>,
    next_request_offset: usize
}

impl<'a> BundleElementReader<'a> {

    fn new(bundle: &'a Bundle) -> Self {
        let bundle_reader = BundleReader::new(bundle);
        Self {
            next_request_offset: bundle_reader.packet()
                .map(|p| p.first_request_offset().unwrap_or(0))
                .unwrap_or(0),
            bundle_reader
        }
    }

    /// Read the current element's identifier. This call return the same result until
    /// you explicitly choose to go to the next element while reading the element
    pub fn read_id(&self) -> Option<u8> {
        self.bundle_reader.body.get(0).copied()
    }

    /// Return `true` if the current element is a request, this is just dependent of
    /// the current position within the current packet.
    pub fn is_request(&self) -> bool {
        // Get the real data pos (instead of the body pos).
        let data_pos = self.bundle_reader.pos + PACKET_FLAGS_LEN;
        self.next_request_offset != 0 && data_pos == self.next_request_offset
    }

    /// Read the current element, return a guard that you should use a codec to decode
    /// the element depending on its type with. *This is a simpler version to use over
    /// standard `read_element` method because it handle reply elements for you.*
    pub fn next_element(&mut self) -> Option<BundleElement<'_, 'a>> {
        match self.read_id() {
            Some(REPLY_ID) => {
                match self.read_element::<ReplyHeader>(&(), false) {
                    Ok(elt) => {
                        debug_assert!(elt.request_id.is_none(), "Replies should not be request at the same time.");
                        Some(BundleElement::Reply(elt.element.request_id, ReplyElementReader(self)))
                    }
                    Err(_) => None
                }
            }
            Some(id) => {
                Some(BundleElement::Top(id, TopElementReader(self)))
            }
            None => None
        }
    }

    /// Try to decode the current element using a given codec. You can choose to go
    /// to the next element using the `next` argument.
    pub fn read_element<E>(&mut self, config: &E::Config, next: bool) -> Result<ReadElement<E>, ReadElementError>
    where
        E: TopElement
    {

        let request = self.is_request();
        let header_len = E::LEN.len() + 1 + if request { 6 } else { 0 };

        if self.bundle_reader.body.len() < header_len {
            return Err(ReadElementError::TooShortPacket);
        }

        // We store a screenshot of the reader in order to be able to rollback in case of error.
        let reader_save = self.bundle_reader.clone();

        match self.read_element_internal::<E>(config, next, request) {
            Ok(elt) if next => Ok(elt),
            Ok(elt) => {
                // If no error but we don't want to go next.
                self.bundle_reader.clone_from(&reader_save);
                Ok(elt)
            }
            Err(e) => {
                // If any error happens, we cancel the operation.
                self.bundle_reader.clone_from(&reader_save);
                Err(ReadElementError::Io(e))
            }
        }

    }

    /// Internal only. Used by `next` to wrap all IO errors and reset seek if an error happens.
    #[inline(always)]
    fn read_element_internal<E>(&mut self, config: &E::Config, next: bool, request: bool) -> io::Result<ReadElement<E>>
    where
        E: TopElement
    {

        let start_packet = self.bundle_reader.packet().unwrap();

        let elt_id = self.bundle_reader.read_u8()?;
        let elt_len = E::LEN.read(&mut self.bundle_reader)?;

        let reply_id = if request {
            let reply_id = self.bundle_reader.read_u32()?;
            self.next_request_offset = self.bundle_reader.read_u16()? as usize;
            Some(reply_id)
        } else {
            None
        };

        let elt_data_begin = self.bundle_reader.pos;

        let element = 
            if let Some(len) = elt_len {
                let mut limited = Read::take(&mut self.bundle_reader, len as u64);
                E::decode(&mut limited, len as usize, elt_id, config)
            } else {
                E::decode(&mut self.bundle_reader, 0, elt_id, config)
            }?;

        // We seek to the end only if we want to go next.
        if next {

            // If decoding is successful (and element has determined length), 
            // jump to the next packet, this happen if not all the element
            // has been read.
            if let Some(len) = elt_len {
                self.bundle_reader.goto(elt_data_begin + len as usize);
            }

            // Here we check if we have changed packets during decoding of the element.
            // If changed, we change the next request offset.
            match self.bundle_reader.packet() {
                Some(end_packet) => {
                    if !std::ptr::eq(start_packet, end_packet) {
                        self.next_request_offset = end_packet.first_request_offset().unwrap_or(0);
                    }
                    // Else, we are still in the same packet so we don't need to change this.
                }
                None => self.next_request_offset = 0
            }

        }

        Ok(ReadElement {
            id: elt_id,
            element,
            request_id: reply_id
        })

    }

}

impl fmt::Debug for BundleElementReader<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BundleElementReader")
            .field("bundle_reader", &self.bundle_reader)
            .field("next_request_offset", &self.next_request_offset)
            .field("read_id()", &self.read_id())
            .field("is_request()", &self.is_request())
            .finish()
    }
}


/// An element read from `BundleElementReader` and `BundleElement` variants,
/// also containing the element's ID and an optional request ID.
#[derive(Debug)]
pub struct ReadElement<E: Element> {
    /// Numeric identifier of the element.
    pub id: u8,
    /// The actual element.
    pub element: E,
    /// The request ID if the element is a request. Not to be confused with
    /// the reply ID if the element is a `Reply`.
    pub request_id: Option<u32>
}

impl<E: Element> Into<ReadElement<E>> for ReadElement<Reply<E>> {
    fn into(self) -> ReadElement<E> {
        ReadElement {
            id: REPLY_ID,
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
#[derive(Debug)]
pub enum BundleElement<'reader, 'bundle> {
    /// A top element with a proper ID and a reader.
    Top(u8, TopElementReader<'reader, 'bundle>),
    /// A reply element with request ID and a reader.
    Reply(u32, ReplyElementReader<'reader, 'bundle>)
}

impl BundleElement<'_, '_> {

    /// Return `true` if this element is a simple one.
    pub fn is_simple(&self) -> bool {
        matches!(self, BundleElement::Top(_, _))
    }

    /// Return `true` if this element is a reply.
    pub fn is_reply(&self) -> bool {
        matches!(self, BundleElement::Reply(_, _))
    }

}

/// The simple variant of element, provides direct decoding using a codec.
#[derive(Debug)]
pub struct TopElementReader<'reader, 'bundle>(&'reader mut BundleElementReader<'bundle>);

impl TopElementReader<'_, '_> {

    /// Same as `read` but never go to the next element *(this is why this method doesn't take
    /// self by value)*.
    pub fn read_stable<E: TopElement>(&mut self, config: &E::Config) -> Result<ReadElement<E>, ReadElementError> {
        self.0.read_element(config, false)
    }

    #[inline]
    pub fn read_simple_stable<E: TopElement<Config = ()>>(&mut self) -> Result<ReadElement<E>, ReadElementError> {
        self.read_stable::<E>(&())
    }

    /// Read the element using the given codec. This method take self by value and automatically
    /// go the next element if read is successful, if not successful you will need to call
    /// `Bundle::next_element` again.
    pub fn read<E: TopElement>(self, config: &E::Config) -> Result<ReadElement<E>, ReadElementError> {
        self.0.read_element(config, true)
    }

    #[inline]
    pub fn read_simple<E: TopElement<Config = ()>>(self) -> Result<ReadElement<E>, ReadElementError> {
        self.read::<E>(&())
    }

}

/// The reply variant of element, provides a way to read replies and get `Reply` elements
/// containing the final element.
#[derive(Debug)]
pub struct ReplyElementReader<'reader, 'bundle>(&'reader mut BundleElementReader<'bundle>);

impl<'reader, 'bundle> ReplyElementReader<'reader, 'bundle> {

    /// Same as `read` but never go to the next element *(this is why this method doesn't take
    /// self by value)*.
    ///
    /// This method doesn't returns the reply element but the final element.
    pub fn read_stable<E: Element>(&mut self, config: &E::Config) -> Result<ReadElement<E>, ReadElementError> {
        self.0.read_element::<Reply<E>>(config, false).map(Into::into)
    }

    #[inline]
    pub fn read_simple_stable<E: Element<Config = ()>>(&mut self) -> Result<ReadElement<E>, ReadElementError> {
        self.read_stable::<E>(&())
    }

    /// Read the reply element using the given codec. This method take self by value and
    /// automatically go the next element if read is successful, if not successful you
    /// will need to call `Bundle::next_element` again.
    ///
    /// This method doesn't returns the reply element but the final element.
    pub fn read<E: Element>(self, config: &E::Config) -> Result<ReadElement<E>, ReadElementError> {
        self.0.read_element::<Reply<E>>(config, true).map(Into::into)
    }

    #[inline]
    pub fn read_simple<E: Element<Config = ()>>(self) -> Result<ReadElement<E>, ReadElementError> {
        self.read::<E>(&())
    }

}



/// A structure that reassemble received bundles' fragments. You can provide an
/// additional key type `O` to be used to identify fragments' origin. For example
/// it can be a client address.
pub struct BundleAssembler<O = ()> {
    /// Fragments tracker.
    fragments: HashMap<(O, u32), BundleFragments>,
}

impl<O> BundleAssembler<O> {

    pub fn new() -> Self {
        Self {
            fragments: HashMap::new(),
        }
    }

}

// Requires copy to ensure that `from` is small and can be copied
// for each packet when draining old bundles.
impl<O: Hash + Eq + Copy> BundleAssembler<O> {

    /// Add the given packet to internal fragments and try to make a bundle if all fragments
    /// were received. *Special case for packet with no sequence number, in such case a bundle
    /// with this single packet is returned.*
    pub fn try_assemble(&mut self, from: O, packet: Box<Packet>, packet_config: &PacketConfig) -> Option<Bundle> {
        if let Some((seq_first, seq_last)) = packet_config.sequence_range() {
            let seq = packet_config.sequence_num();
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
            Some(Bundle::with_single(packet))
        }
    }

    /// Drain all timed out bundles, a bundle is timed out if it was not updated
    /// (a packed being received) in the past [`BUNDLE_FRAGMENT_TIMEOUT`] duration.
    /// 
    /// The discarded packets are returned.
    pub fn drain_old(&mut self) -> Vec<(O, Box<Packet>)> {
        let mut packets = Vec::new();
        self.fragments.retain(|(o, _), v| {
            if v.is_old() {
                packets.extend(v.fragments.drain(..)
                    .filter_map(|p| p)
                    .map(|p| (*o, p)));
                false
            } else {
                true
            }
        });
        packets
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
        self.last_update.elapsed() > BUNDLE_FRAGMENT_TIMEOUT
    }

    #[inline]
    fn is_full(&self) -> bool {
        self.seq_count as usize == self.fragments.len()
    }

    /// Convert this structure to a bundle, **safe to call only if `is_full() == true`**.
    fn into_bundle(self) -> Bundle {
        debug_assert!(self.is_full());
        let packets = self.fragments.into_iter()
            .map(|o| o.unwrap())
            .collect();
        Bundle::with_multiple(packets)
    }

}
