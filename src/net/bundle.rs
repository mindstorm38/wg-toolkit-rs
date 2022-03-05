use std::io::{Write, Cursor, Read};

use byteorder::{WriteBytesExt, LittleEndian, ReadBytesExt};

use super::element::ElementCodec;
use super::PacketFlags;


const PACKET_MAX_SIZE: usize = 1472;


/// A elements bundle, used to pack elements and encode them.
pub struct Bundle {
    /// Chain of packets.
    packets: Vec<Packet>,
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
        self.packets.push(Packet::new());
    }

    /// Reserve exactly the given length in the current packet or a new one if
    /// this such space is not available in the current packet. An exact
    /// reservation must not exceed maximum packet size.
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


pub struct Packet {
    /// Raw data of the packet, header and footer data is not valid until
    /// finalization of the packet.
    data: [u8; PACKET_MAX_SIZE],
    /// Length of data currently used in the data array, this also includes the
    /// packet's header (flags) and footer (when finalized).
    len: usize,
    /// Offset of the footer when the packet is finalized or loaded.
    footer_offset: usize,
    /// The first request's offset in the packet.
    request_first_offset: usize,
    /// The previous request's offset to the "next link" field.
    /// Only used and valid when writing elements to a packet.
    request_previous_link_offset: usize,
    /// Sequence number of the first packet of the chain where the owning packet is.
    seq_first: u32,
    /// Sequence number of the last packet of the chain where the owning packet is.
    ///
    /// If it is equal to 0, this means that the owning packet has no sequence ID.
    /// This is the only determinant field for this because at least 2 packets must
    /// be in a sequence, and if `seq_first == 0` (which is the smallest sequence
    /// number), then `seq_last` must be at least `== 1`.
    seq_last: u32,
    /// Sequence number of the owning packet.
    seq: u32,
    /// Enable or disable checksum.
    has_checksum: bool,
    /*/// The current element's offset. TODO: Remove this and use local function's variable in add_element.
    elt_offset: usize,
    /// Length of the element's header (element id + length). TODO: Remove this and use local function's variable in add_element.
    elt_header_len: usize,*/
}

impl Packet {

    pub fn new() -> Self {
        Self {
            data: [0; PACKET_MAX_SIZE],
            len: 2, // Size of flags
            footer_offset: 0,
            request_first_offset: 0,
            request_previous_link_offset: 0,
            seq_first: 0,
            seq_last: 0,
            seq: 0,
            has_checksum: false,
            // elt_offset: 2,
            // elt_header_len: 0,
        }
    }

    // Memory management

    /// Returns the free length available in this packet.
    pub fn available_len(&self) -> usize {
        self.data.len() - self.len - 50  // TODO: Change -50 by a more accurate maximum footer size.
    }

    /// Internal method used to increment the cursor's offset and return a mutable
    /// slice to the reserved data.
    pub fn reserve_unchecked(&mut self, len: usize) -> &mut [u8] {
        debug_assert!(self.len + len <= self.data.len(), "Reserve overflow.");
        let ptr = &mut self.data[self.len..][..len];
        self.len += len;
        ptr
    }

    /// Get the internal data.
    pub fn as_data(&self) -> &[u8] {
        &self.data
    }

    /// Get the internal data mutably.
    pub fn as_data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    // Requests

    /// Add the request.
    pub fn add_request(&mut self, offset: usize, link_offset: usize) {
        if self.request_first_offset == 0 {
            self.request_first_offset = offset;
        } else {
            assert_ne!(self.request_previous_link_offset, 0, "No previous link offset.");
            Cursor::new(&mut self.data[self.request_previous_link_offset..])
                .write_u16::<LittleEndian>(offset as u16).unwrap();
        }
        self.request_previous_link_offset = link_offset;
    }

    /// Clear requests.
    pub fn clear_requests(&mut self) {
        self.request_first_offset = 0;
    }

    pub fn has_requests(&self) -> bool {
        self.request_first_offset != 0
    }

    // Sequences

    /// Clear sequence number for this packet.
    pub fn clear_seq(&mut self) {
        self.seq_last = 0;
    }

    /// Returns `true` if this packet has sequence number.
    pub fn has_seq(&self) -> bool {
        self.seq_last != 0
    }

    /// Set a sequence number for this packet.
    pub fn set_seq(&mut self, seq_first: u32, seq_last: u32, seq: u32) {
        debug_assert!(seq_first < seq_last, "At least to packet must be in the sequence range.");
        debug_assert!(seq >= seq_first && seq <= seq_last, "The given sequence number is not in the sequence range.");
        self.seq_first = seq_first;
        self.seq_last = seq_last;
        self.seq = seq;
    }

    // Checksum

    pub fn has_checksum(&self) -> bool {
        self.has_checksum
    }

    pub fn set_checksum(&mut self, enabled: bool) {
        self.has_checksum = enabled;
    }

    /// Generic function to calculate the checksum from a reader and for the given number of
    fn calc_checksum<R: Read>(reader: &mut R, mut len: u64) -> u32 {
        let mut checksum = 0;
        while len >= 4 {
            checksum ^= reader.read_u32::<LittleEndian>().unwrap();
            len -= 4;
        }
        checksum
    }

    // Save and loading

    /// Finalize this packet, write footer.
    /// This can be called multiple times, the result is stable.
    pub fn finalize(&mut self) {

        if self.footer_offset == 0 {
            // If the footer is not already set, set offset to current length.
            self.footer_offset = self.len;
        } else {
            // If the footer is already set, reset the length to it.
            self.len = self.footer_offset;
        }

        let has_seq = self.has_seq();

        let mut cursor = Cursor::new(&mut self.data[..]);
        cursor.set_position(self.len as u64);

        let mut flags = 0u16;

        if has_seq {
            flags |= PacketFlags::IS_FRAGMENT;
            flags |= PacketFlags::HAS_SEQUENCE_NUMBER;
            cursor.write_u32::<LittleEndian>(self.seq_first).unwrap();
            cursor.write_u32::<LittleEndian>(self.seq_last).unwrap();
        }

        if self.request_first_offset != 0 {
            flags |= PacketFlags::HAS_REQUESTS;
            cursor.write_u16::<LittleEndian>(self.request_first_offset as u16).unwrap();
        }

        if has_seq {
            cursor.write_u32::<LittleEndian>(self.seq).unwrap();
        }

        // TODO: Acks

        // Set the length, just before the checksum if enabled.
        self.len = cursor.position() as usize;

        if self.has_checksum {
            flags |= PacketFlags::HAS_CHECKSUM;
        }

        // Finally, write flags.
        cursor.set_position(0);
        cursor.write_u16::<LittleEndian>(flags).unwrap();

        // Calculate checksum and write it if enabled.
        // Placed here to take flags into checksum.
        if self.has_checksum {
            cursor.set_position(0);
            let checksum = Self::calc_checksum(&mut cursor, self.len as u64);
            cursor.write_u32::<LittleEndian>(checksum).unwrap();
            self.len += 4;
        }

    }

    /// Finalize this packet, write footer.
    /// This can be called multiple times, the result is stable.
    ///
    /// If this function returns an error, the integrity of the internal state is not guaranteed.
    pub fn load(&mut self, len: usize) -> Result<(), PacketLoadError> {

        let mut cursor = Cursor::new(&mut self.data[..]);
        let flags = cursor.read_u16::<LittleEndian>().unwrap();

        const KNOWN_FLAGS: u16 =
            PacketFlags::HAS_CHECKSUM |
            PacketFlags::HAS_SEQUENCE_NUMBER |
            PacketFlags::HAS_REQUESTS |
            PacketFlags::IS_FRAGMENT;

        if flags & !KNOWN_FLAGS != 0 {
            return Err(PacketLoadError::UnknownFlags(flags & !KNOWN_FLAGS));
        }

        self.has_checksum = flags & PacketFlags::HAS_CHECKSUM != 0;
        let has_seq = flags & PacketFlags::HAS_SEQUENCE_NUMBER != 0;
        let has_requests = flags & PacketFlags::HAS_REQUESTS != 0;

        if has_seq && flags & PacketFlags::IS_FRAGMENT == 0 {
            return Err(PacketLoadError::MissingFragmentFlag);
        }

        // Reset this, because we have no clue of where elements are in this packet
        // when loading.
        // self.elt_header_len = 0;
        // self.elt_offset = 2;  // Default size of flags.
        self.len = len;

        let footer_len =
            if self.has_checksum { 4 } else { 0 } +
            if has_seq { 12 } else { 0 } +
            if has_requests { 2 } else { 0 };

        if len < footer_len + 2 { // +2 is flags size.
            return Err(PacketLoadError::TooShort);
        }

        self.footer_offset = len - footer_len;
        cursor.set_position(self.footer_offset as u64);

        if has_seq {
            self.seq_first = cursor.read_u32::<LittleEndian>().unwrap();
            self.seq_last = cursor.read_u32::<LittleEndian>().unwrap();
        } else {
            self.seq_last = 0;  // Clear sequence number.
        }

        self.request_previous_link_offset = 0;
        if has_requests {
            self.request_first_offset = cursor.read_u16::<LittleEndian>().unwrap() as usize;
        } else {
            self.request_first_offset = 0;  // Clear requests.
        }

        if has_seq {
            self.seq = cursor.read_u32::<LittleEndian>().unwrap();
        }

        // TODO: Acks

        if self.has_checksum {
            let pos = cursor.position();
            let checksum = cursor.read_u32::<LittleEndian>().unwrap();
            cursor.set_position(0);
            let real_checksum = Self::calc_checksum(&mut cursor, pos);
            if checksum != real_checksum {
                return Err(PacketLoadError::InvalidChecksum);
            }
            cursor.set_position(pos);
        }

        debug_assert_eq!(cursor.position(), len as u64, "Wrong calculated footer size.");
        Ok(())

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


/// Packet loading error.
#[derive(Debug)]
pub enum PacketLoadError {
    /// Unknown flags are used, the packet can't be decoded because this usually
    /// increase length of the footer.
    UnknownFlags(u16),
    /// The packet has sequence number but is not is missing fragment flag.
    MissingFragmentFlag,
    /// Not enough length available to decode this packet's footers correctly.
    TooShort,
    /// The packet has checksum and the calculated checksum doesn't correspond.
    InvalidChecksum
}
