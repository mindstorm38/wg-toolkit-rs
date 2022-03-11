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
/// The theoretical maximum length for the body, if maximum length is used by header + footer.
pub const PACKET_MAX_BODY_LEN: usize =
    PACKET_MAX_LEN -
    PACKET_MAX_FOOTER_LEN -
    PACKET_FLAGS_LEN -
    PACKET_PREFIX_LEN;


pub struct Packet {
    /// Raw data of the packet, header and footer data is not valid until
    /// finalization of the packet.
    pub data: [u8; PACKET_MAX_LEN],
    /// Length of data currently used in the data array, this also includes the
    /// packet's header (flags) and footer (when finalized).
    len: usize,
    /// If the data contains a 4-bytes prefix.
    has_prefix: bool,
    /// Offset of the footer when the packet is finalized or loaded.
    footer_offset: usize,
    /// The first request's offset in the packet.
    request_first_offset: usize,
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
        let len = PACKET_FLAGS_LEN + if has_prefix { PACKET_PREFIX_LEN } else { 0 };
        Self {
            data: [0; PACKET_MAX_LEN],
            len,
            has_prefix,
            footer_offset: len,
            request_first_offset: 0,
            seq_first: 0,
            seq_last: 0,
            seq: 0,
            has_checksum: false,
        }
    }

    pub fn new_boxed(has_prefix: bool) -> Box<Self> {
        Box::new(Self::new(has_prefix))
    }

    // Memory management

    /// Return the total used length of data.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the free length available in this packet.
    #[inline]
    pub fn available_len(&self) -> usize {
        self.data.len() - self.len - PACKET_MAX_FOOTER_LEN
    }

    /// Returns the offset in data to the flags, varying between 0
    /// and `PACKET_PREFIX_LEN` if the packet has prefix.
    #[inline]
    pub fn get_flags_offset(&self) -> usize {
        if self.has_prefix { PACKET_PREFIX_LEN } else { 0 }
    }

    /// Returns the offset in data to the body (just after flags).
    #[inline]
    pub fn get_body_offset(&self) -> usize {
        self.get_flags_offset() + PACKET_FLAGS_LEN
    }

    /// Returns the offset in data to the footer, 0 if not yet defined.
    #[inline]
    pub fn get_footer_offset(&self) -> usize {
        self.footer_offset
    }

    /// Return the size of the body.
    #[inline]
    pub fn get_body_len(&self) -> usize {
        self.get_footer_offset() - self.get_body_offset()
    }

    /// Return a slice to the body part of the internal data.
    #[inline]
    pub fn get_body_data(&self) -> &[u8] {
        &self.data[self.get_body_offset()..self.get_footer_offset()]
    }

    /// Return a slice to the valid portion of data, equivalent to `&data[..len]`.
    #[inline]
    pub fn get_valid_data(&self) -> &[u8] {
        &self.data[..self.len]
    }

    /// Internal method used to increment the cursor's offset and return a mutable
    /// slice to the reserved data.
    pub fn reserve_unchecked(&mut self, len: usize) -> &mut [u8] {
        debug_assert!(self.len + len <= self.data.len(), "Reserve overflow.");
        let ptr = &mut self.data[self.len..][..len];
        self.len += len;
        self.footer_offset = self.len; // As we may overwrite the footer.
        ptr
    }

    /// Clear all this packet and restart from after the flags.
    pub fn clear(&mut self) {
        self.len = self.get_body_offset();
        self.footer_offset = self.len;
        self.clear_seq();
        self.clear_requests();
    }

    // Requests

    /// Clear requests.
    #[inline]
    pub fn clear_requests(&mut self) {
        self.request_first_offset = 0;
    }

    /// Returns `true` if this packet contains any request.
    #[inline]
    pub fn has_requests(&self) -> bool {
        self.request_first_offset != 0
    }

    #[inline]
    pub fn set_request_first_offset(&mut self, offset: usize) {
        debug_assert!(offset <= self.len - 7); // -1 (flag) -6 (request header) = -7
        self.request_first_offset = offset;
    }

    /// Return the offset where the next request message is in this packet.
    /// Return `0` if there is no request in this packet.
    #[inline]
    pub fn get_request_first_offset(&self) -> usize {
        self.request_first_offset
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
        debug_assert!(seq_first < seq_last, "At least two packet must be in the sequence range.");
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

    /// Generic function to calculate the checksum from a reader and
    /// a given number of bytes available.
    fn calc_checksum<R: Read>(reader: &mut R, mut len: u64) -> u32 {
        let mut checksum = 0;
        while len >= 4 {
            checksum ^= reader.read_u32::<LittleEndian>().unwrap();
            len -= 4;
        }
        checksum
    }

    // Data and state synchronization

    /// Synchronize internal packet's data from its state.
    /// This can be called multiple times, the result is stable.
    pub fn sync_data(&mut self) {

        // If footer offset is less than length, then we know that a
        // footer is already existing so we want to overwrite it.
        if self.footer_offset < self.len {
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

    /// Synchronize internal packet's state from its data.
    /// This can be called multiple times, the result is stable.
    ///
    /// *If this function returns an error, the integrity of the internal state is not guaranteed.*
    pub fn sync_state(&mut self, len: usize, has_prefix: bool) -> Result<(), PacketSyncError> {

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
            return Err(PacketSyncError::UnknownFlags(flags & !KNOWN_FLAGS));
        }

        self.has_checksum = flags & PacketFlags::HAS_CHECKSUM != 0;
        let has_seq = flags & PacketFlags::HAS_SEQUENCE_NUMBER != 0;
        let has_requests = flags & PacketFlags::HAS_REQUESTS != 0;

        if has_seq && flags & PacketFlags::IS_FRAGMENT == 0 {
            return Err(PacketSyncError::MissingFragmentFlag);
        }

        let footer_len =
            if self.has_checksum { 4 } else { 0 } +
            if has_seq { 12 } else { 0 } +
            if has_requests { 2 } else { 0 };

        if len < footer_len + prefix_len + PACKET_FLAGS_LEN { // +2 is flags size.
            return Err(PacketSyncError::TooShort);
        }

        self.len = len;
        self.has_prefix = has_prefix;
        self.footer_offset = len - footer_len;

        cursor.set_position(self.footer_offset as u64);

        if has_seq {
            self.seq_first = cursor.read_u32::<LittleEndian>().unwrap();
            self.seq_last = cursor.read_u32::<LittleEndian>().unwrap();
        } else {
            self.seq_last = 0;  // Clear sequence number.
        }

        // self.request_previous_link_offset = 0;
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
                return Err(PacketSyncError::InvalidChecksum);
            }
            cursor.set_position(pos + 4);
        }

        debug_assert_eq!(cursor.position(), len as u64, "Wrong calculated footer size.");
        Ok(())

    }

}


/// Packet synchronization error.
#[derive(Debug)]
pub enum PacketSyncError {
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
