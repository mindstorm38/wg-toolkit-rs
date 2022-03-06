use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use std::io::{Cursor, Read};

use super::PacketFlags;


/// According to disassembly of WoT, outside of a channel, the max size if always
/// `1500 - 28 = 1472`, this includes the 4-bytes prefix.
pub const PACKET_MAX_LEN: usize = 1472;
/// According to disassembly of WoT's `Packet::freeSpace` function.
pub const PACKET_MAX_FOOTER_LEN: usize = 35;
/// Flags are u16.
pub const PACKET_FLAGS_LEN: usize = 2;
/// The length of the unknown 4-byte prefix.
pub const PACKET_PREFIX_LEN: usize = 4;


pub struct Packet {
    /// Raw data of the packet, header and footer data is not valid until
    /// finalization of the packet.
    pub data: [u8; PACKET_MAX_LEN],
    /// Length of data currently used in the data array, this also includes the
    /// packet's header (flags) and footer (when finalized).
    pub len: usize,
    /// If the data contains a 4-bytes prefix.
    has_prefix: bool,
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
    /// If it is less or equals to `seq_first` then
    seq_last: u32,
    /// Sequence number of the owning packet.
    seq: u32,
    /// Enable or disable checksum.
    has_checksum: bool,
}

impl Packet {

    pub fn new(has_prefix: bool) -> Self {
        let prefix_len = if has_prefix { PACKET_PREFIX_LEN } else { 0 };
        Self {
            data: [0; PACKET_MAX_LEN],
            len: PACKET_FLAGS_LEN + prefix_len, // Size of flags
            has_prefix,
            footer_offset: 0,
            request_first_offset: 0,
            request_previous_link_offset: 0,
            seq_first: 0,
            seq_last: 0,
            seq: 0,
            has_checksum: false,
        }
    }

    // Memory management

    /// Returns the free length available in this packet.
    pub fn available_len(&self) -> usize {
        self.data.len() - self.len - PACKET_MAX_FOOTER_LEN
    }

    /// Internal method used to increment the cursor's offset and return a mutable
    /// slice to the reserved data.
    pub fn reserve_unchecked(&mut self, len: usize) -> &mut [u8] {
        debug_assert!(self.len + len <= self.data.len(), "Reserve overflow.");
        let ptr = &mut self.data[self.len..][..len];
        self.len += len;
        ptr
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
        self.seq_first = 0;
        self.seq_last = 0;
    }

    /// Returns `true` if this packet has sequence number.
    /// This also ensure that at least two fragments are required.
    pub fn has_seq(&self) -> bool {
        self.seq_first < self.seq_last
    }

    /// Set a sequence number for this packet.
    pub fn set_seq(&mut self, seq_first: u32, seq_last: u32, seq: u32) {
        debug_assert!(seq_first < seq_last, "At least to packet must be in the sequence range.");
        debug_assert!(seq >= seq_first && seq <= seq_last, "The given sequence number is not in the sequence range.");
        self.seq_first = seq_first;
        self.seq_last = seq_last;
        self.seq = seq;
    }

    pub fn get_seq(&self) -> (u32, u32, u32) {
        (self.seq_first, self.seq_last, self.seq)
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

        let prefix_len = if self.has_prefix { PACKET_PREFIX_LEN as u64 } else { 0 };

        // Finally, write flags.
        cursor.set_position(prefix_len);
        cursor.write_u16::<LittleEndian>(flags).unwrap();

        // Calculate checksum and write it if enabled.
        // Placed here to take flags into checksum.
        if self.has_checksum {
            cursor.set_position(prefix_len);
            let checksum = Self::calc_checksum(&mut cursor, self.len as u64);
            cursor.write_u32::<LittleEndian>(checksum).unwrap();
            self.len += 4;
        }

    }

    /// Load this the packet from internal of the given length. You can specify if the data
    /// has the 4-byte prefix that should be ignored (for now, until this is understood).
    /// This can be called multiple times, the result is stable.
    ///
    /// If this function returns an error, the integrity of the internal state is not guaranteed.
    pub fn load(&mut self, len: usize, has_prefix: bool) -> Result<(), PacketLoadError> {

        let mut cursor = Cursor::new(&mut self.data[..]);

        let prefix_len = if has_prefix { PACKET_PREFIX_LEN } else { 0 };
        cursor.set_position(prefix_len as u64);

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
        self.has_prefix = has_prefix;

        let footer_len =
            if self.has_checksum { 4 } else { 0 } +
            if has_seq { 12 } else { 0 } +
            if has_requests { 2 } else { 0 };

        if len < footer_len + prefix_len + PACKET_FLAGS_LEN { // +2 is flags size.
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
            cursor.set_position(prefix_len as u64);
            let real_checksum = Self::calc_checksum(&mut cursor, pos);
            if checksum != real_checksum {
                return Err(PacketLoadError::InvalidChecksum);
            }
            cursor.set_position(pos + 4);
        }

        debug_assert_eq!(cursor.position(), len as u64, "Wrong calculated footer size.");
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






/*/// A packet constructed from its raw data, used to extract headers,
/// footers and elements stored in it.
pub struct Packet<'data, 'codec> {
    /// The elements registry used to iterate elements.
    elements: &'codec ElementRegistry,
    /// The full raw data of the packet.
    data: &'data [u8],
    /// A cursor over the raw data.
    cursor: Cursor<&'data [u8]>,
    /// The upper limit in the raw data where the footer begins.
    limit: u64,
    flags: PacketFlags,
    checksum: u32,
    next_request_offset: u64,
}

/// A single element in a packet.
#[derive(Debug)]
pub struct PacketElement<'data, 'codec> {
    id: u8,
    data: &'data [u8],
    element: &'codec ElementDef,
    spec: PacketElementSpec
}

#[derive(Debug)]
pub enum PacketElementSpec {
    Standard,
    Request {
        reply_id: u32
    }
}

impl<'data, 'codec> Packet<'data, 'codec> {

    /// Construct a new decoder for the given raw packet's data, this data must not
    /// contains the 4 bytes prefix.
    pub fn new(data: &'data [u8], elements: &'codec ElementRegistry) -> Self {

        // TODO: Check data minimum length requirements.

        let mut head_cursor = Cursor::new(data);
        let mut foot_cursor = Cursor::new(data);

        foot_cursor.set_position(data.len() as u64);

        let flags = PacketFlags(head_cursor.read_u16::<LittleEndian>().unwrap());

        // Internal util
        #[inline(always)]
        fn strip_foot<T>(cursor: &mut Cursor<&[u8]>) {
            cursor.set_position(cursor.position() - std::mem::size_of::<T>() as u64);
        }

        let mut checksum = 0;
        if flags.has_checksum() {
            strip_foot::<u32>(&mut foot_cursor);
            checksum = foot_cursor.read_u32::<LittleEndian>().unwrap();
            strip_foot::<u32>(&mut foot_cursor);
        }

        let mut next_request_offset = 0;
        if flags.has_requests() {
            strip_foot::<u16>(&mut foot_cursor);
            next_request_offset = foot_cursor.read_u16::<LittleEndian>().unwrap() as u64;
            strip_foot::<u16>(&mut foot_cursor);
        }

        Self {
            elements,
            data,
            cursor: head_cursor,
            limit: foot_cursor.position(),
            flags,
            checksum,
            next_request_offset
        }

    }

}

impl<'data, 'codec> Iterator for Packet<'data, 'codec> {

    type Item = PacketElement<'data, 'codec>;

    fn next(&mut self) -> Option<Self::Item> {

        let offset = self.cursor.position();
        if offset >= self.limit {
            return None;
        }

        let request = self.next_request_offset == offset;

        let id = self.cursor.read_u8().unwrap();
        let element = self.elements.get(id).expect("TODO: remove this .expect");

        let length = element.length.read(&mut self.cursor).unwrap();
        let mut spec = PacketElementSpec::Standard;

        if request {
            let reply_id = self.cursor.read_u32::<LittleEndian>().unwrap();
            self.next_request_offset = self.cursor.read_u16::<LittleEndian>().unwrap() as u64;
            spec = PacketElementSpec::Request { reply_id };
        }

        let data_begin = self.cursor.position() as usize;
        let data_end = data_begin + length as usize;
        let data = &self.data[data_begin..data_end];
        self.cursor.set_position(data_end as u64);

        Some(PacketElement {
            id,
            data,
            element,
            spec
        })

    }

}
*/