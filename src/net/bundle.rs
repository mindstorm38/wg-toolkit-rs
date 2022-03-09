use std::io::{Write, Cursor, Read, Seek, SeekFrom};
use std::collections::hash_map::Entry;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::hash::Hash;

use byteorder::ReadBytesExt;

use super::packet::{Packet, PACKET_MAX_BODY_LEN};
use super::element::ElementCodec;


pub const BUNDLE_FRAGMENT_MAX_AGE: Duration = Duration::from_secs(10);


/// A elements bundle, used to pack elements and encode them.
pub struct Bundle {
    /// Chain of packets.
    packets: Vec<Box<Packet>>,
    /// Available length on the last packet.
    available_len: usize,
    /// If packets in this bundle has a prefix.
    has_prefix: bool
}

impl Bundle {

    /// Construct a new empty bundle, this bundle doesn't
    /// allocate until you add the first element.
    pub fn new(has_prefix: bool) -> Bundle {
        Bundle {
            packets: Vec::new(),
            available_len: 0,
            has_prefix
        }
    }

    /// Create a new bundle with one packet.
    pub fn from_single(packet: Box<Packet>, has_prefix: bool) -> Self {
        Bundle {
            available_len: packet.available_len(),
            packets: vec![packet],
            has_prefix
        }
    }

    pub fn from_packets(packets: Vec<Box<Packet>>, has_prefix: bool) -> Self {
        debug_assert!(!packets.is_empty());
        Bundle {
            available_len: packets.last().unwrap().available_len(),
            packets,
            has_prefix
        }
    }

    /// Add a new element to this bundle, everything is manager for the caller,
    /// new packets are created if needed and the message can be a request.
    pub fn add_element<E: ElementCodec>(&mut self, elt: &E, request: bool) {

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
            let cursor = first_packet.len();
            // -2 because link offset is encoded on two bytes (u16).
            first_packet.add_request(cursor, cursor + header_len - 2);
        }
        let first_packet_elt_offset = first_packet.len();

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

    /// Finalize the bundle by synchronizing all packets in it and setting
    /// their sequence id.
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
            packet.sync_data();
        }

    }

    pub fn len(&self) -> usize {
        self.packets.len()
    }

    /// Get a slice of all packets of this bundle.
    #[inline]
    pub fn get_packets(&self) -> &[Box<Packet>] {
        &self.packets[..]
    }

    /// See `BundleRawElementsIter`.
    pub fn iter_raw_elements(&self) -> BundleRawElementsIter {
        BundleRawElementsIter::new(self)
    }

    pub fn iter_elements(&self) -> BundleElementIter {
        todo!()
    }

    /// Internal method to add a new packet at the end of the chain.
    fn add_packet(&mut self) {
        self.packets.push(Packet::new_boxed(self.has_prefix));
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
struct BundleReader<'a> {
    /// The bundle we are reading from.
    bundle: &'a Bundle,
    /// The current packet's index in the bundle.
    packets: &'a [Box<Packet>],
    /// The current remaining packet's data from the current packet.
    data: &'a [u8],
    /// The total length available in the bundle.
    len: usize,
    /// The internal position of the reader, used for seeking operations.
    pos: usize,
}

impl<'a> BundleReader<'a> {

    fn new(bundle: &'a Bundle) -> Self {
        let mut ret = Self {
            bundle,
            packets: bundle.get_packets(),
            data: &[],
            len: bundle.get_packets().iter().map(|p| p.get_body_len()).sum(),
            pos: 0
        };
        ret.discard_packets_until_non_empty();
        ret
    }

    /// Internal function to discord all head packets until there is one
    /// with a non-empty body.
    fn discard_packets_until_non_empty(&mut self) {
        let mut packets = self.packets;
        while !packets.is_empty() && packets[0].get_body_len() == 0 {
            packets = &packets[1..];
        }
        self.packets = packets;
        self.data = packets.first().map(|p| p.get_body_data()).unwrap_or(&[]);
    }

    /// Get the packet that will be read on the next call. `None` if
    /// no more data is available. This packet always has a non-empty
    /// body and the read operation will read at least 1 byte.
    fn get_packet(&self) -> Option<&'a Packet> {
        self.packets.first().map(|p| &**p)
    }

    /// Get a slice to the remaining data in the current packet.
    /// Empty if no more packet to read from.
    fn get_packet_remaining_data(&self) -> &[u8] {
        self.data
    }

}

impl<'a> Read for BundleReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.packets.is_empty() {
            Err(std::io::ErrorKind::UnexpectedEof.into())
        } else {
            let len = buf.len().min(self.data.len());
            buf[..len].copy_from_slice(&self.data[..len]);
            self.data = &self.data[len..];
            if self.data.is_empty() {
                self.packets = &self.packets[1..];
                self.discard_packets_until_non_empty();
            }
            Ok(len)
        }
    }
}

impl<'a> Seek for BundleReader<'a> {

    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {

        let abs_pos = match pos {
            SeekFrom::Start(pos) => pos,
            SeekFrom::Current(pos) => (self.pos as i64 + pos) as u64,
            SeekFrom::End(pos) => (self.pos as i64 + pos) as u64
        };

        let mut remaining = abs_pos as usize;
        self.packets = self.bundle.get_packets();

        while !self.packets.is_empty() && remaining != 0 {
            let packet = &self.packets[0];
            let packet_body_len = packet.get_body_len();
            if remaining >= packet_body_len {
                remaining -= packet_body_len;
                self.packets = &self.packets[1..];
            } else {
                self.data = &packet.get_body_data()[remaining..];
                return Ok(abs_pos);
            }
        }

        self.data = self.packets.get(0)
            .map(|p| p.get_body_data())
            .unwrap_or(&[]);

        Ok(abs_pos)

    }

    fn stream_position(&mut self) -> std::io::Result<u64> {
        Ok(self.pos as u64)
    }

}


/// A special iterator designed to fetch each message on the bundle.
/// This is not a real `Iterator` implementor because the `ElementCodec`
/// is needed. And you can get the right codec from the next element id.
pub struct BundleRawElementsIter<'a> {
    bundle_reader: BundleReader<'a>
}

impl<'a> BundleRawElementsIter<'a> {

    fn new(bundle: &'a Bundle) -> Self {
        Self {
            bundle_reader: BundleReader::new(bundle)
        }
    }

    #[inline]
    pub fn next_element_id(&self) -> u8 {
        self.bundle_reader.get_packet_remaining_data()[0]
    }

    pub fn next_element<E: ElementCodec>(&mut self) -> Option<E> {

        let header_len = E::LEN.header_len() + 1;
        self.bundle_reader.read_u8().ok()?;
        let len = E::LEN.read(&mut self.bundle_reader).ok()?;

        E::decode(&mut self.bundle_reader).ok()

        /*if let Some(ref mut packet) = self.packet {
            let header_len = E::LEN.header_len() + 1;
            if self.element_data.len() >= header_len { // +1 for element ID

                let mut cursor = Cursor::new(&self.element_data[1..]);
                let len = E::LEN.read(&mut cursor).ok()? as usize;

                // Strip element's header.
                self.element_data = &self.element_data[header_len..];

                let mut slices = [&[][..]; 32];
                let mut slices_count = 1;
                slices[0] = self.element_data;

                if self.element_data.len() < len {
                    let mut remaining_len = len - self.element_data.len();
                    let mut packet_index = self.packet_index;
                    while remaining_len != 0 {
                        packet_index += 1;
                        match self.bundle.packets.get(packet_index) {
                            Some(packet) => self.element_data = packet.get_body_data(),
                            None => todo!("not enough length for this packet")
                        }
                    }
                    let slice_len = self.element_data.len().min(remaining_len);
                    remaining_len -= slice_len;
                    slices[slices_count] = &self.element_data[..slice_len];
                    slices_count += 1;
                }

                /*if self.element_data.len() >= len {
                    // In this case, we use a simple cursor.
                    let mut cursor = Cursor::new(&self.element_data[..len]);
                    E::decode(&mut cursor).ok()
                } else {

                }*/

                todo!()

            } else {
                None
            }
        } else {
            None
        }*/

    }

}

/*struct MultipartCursor<'a, 'b> {
    slices: &'a [&'b [u8]],
    slice_index: usize,
    slice: &'b [u8]
}

impl<'a, 'b> MultipartCursor<'a, 'b> {

    fn new(slices: &'a [&'b [u8]]) -> Self {
        Self {
            slices,
            slice_index: 0,
            slice: &slices[0][..]
        }
    }

}

impl<'a, 'b> Read for MultipartCursor<'a, 'b> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {

    }
}*/



pub struct BundleElementIter<'a> {
    raw: BundleRawElementsIter<'a>
}

impl<'a> Iterator for BundleElementIter<'a> {

    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
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
        debug_assert!(self.is_full(), "You must call this only if the ");
        let packets = self.fragments.into_iter()
            .map(|o| o.unwrap())
            .collect();
        Bundle::from_packets(packets, has_prefix)
    }

}
