//! Packet structure definition with synchronization methods.

use std::fmt::{Debug, Formatter, Write};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian, BigEndian};
use std::io::{Cursor, Read};

use rand::RngCore;
use rand::rngs::OsRng;

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
    /// finalization of the packet. This first 4 bytes are always reserved for
    /// prefix, but are used only if `has_prefix` is set to true.
    data: [u8; PACKET_MAX_LEN],
    /// Length of data currently used in the data array, this also includes the
    /// packet's header (flags) and footer (when finalized), but not the length
    /// of of the prefix.
    len: usize,
    /// Some optional prefix in the first 4 bytes in `data`, if none the first 4
    /// bytes are unused.
    prefix: Option<u32>,
    // /// If the data contains a 4-bytes prefix.
    // has_prefix: bool,
    /// Offset of the footer when the packet is finalized or loaded.
    footer_offset: usize,
    /// The first request's offset in the packet. Zero if no request in the packet.
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
        Self {
            data: [0; PACKET_MAX_LEN],
            len: PACKET_FLAGS_LEN,
            prefix: if has_prefix { Some(0) } else { None },
            footer_offset: PACKET_FLAGS_LEN,
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

    // Prefix

    /// Returns true if the first 4 bytes are used.
    #[inline]
    pub fn has_prefix(&self) -> bool {
        self.prefix.is_some()
    }

    #[inline]
    pub fn get_prefix(&self) -> Option<u32> {
        self.prefix
    }

    #[inline]
    pub fn set_prefix(&mut self, prefix: Option<u32>) {
        self.prefix = prefix;
    }

    // Various lengths

    /// Return the length of this packet.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Return the raw length of this packet, including reserved first 4 bytes.
    #[inline]
    pub fn raw_len(&self) -> usize {
        self.len + PACKET_PREFIX_LEN
    }

    /// Return the free length available in this packet.
    #[inline]
    pub fn available_len(&self) -> usize {
        self.data.len() - self.len - PACKET_MAX_FOOTER_LEN - PACKET_PREFIX_LEN
    }

    /// Return the size of the body.
    #[inline]
    pub fn body_len(&self) -> usize {
        self.get_footer_offset() - PACKET_FLAGS_LEN
    }

    /// Return the offset of the first raw data in the internal data array.
    /// If this packet has a prefix, the raw offset is 0, if not it's 4.
    #[inline]
    pub fn get_raw_offset(&self) -> usize {
        // If we have a prefix, the raw data starts immediately (offset 0).
        if self.has_prefix() { 0 } else { PACKET_PREFIX_LEN }
    }

    /// Returns the offset in data to the footer, 0 if not yet defined.
    #[inline]
    pub fn get_footer_offset(&self) -> usize {
        self.footer_offset
    }

    /*/// Returns the offset in data to the flags, varying between 0
    /// and `PACKET_PREFIX_LEN` if the packet has prefix.
    #[inline]
    pub fn get_flags_offset(&self) -> usize {
        // if self.has_prefix { PACKET_PREFIX_LEN } else { 0 }
        PACKET_PREFIX_LEN
    }

    /// Returns the offset in data to the body (just after flags).
    #[inline]
    pub fn get_body_offset(&self) -> usize {
        self.get_flags_offset() + PACKET_FLAGS_LEN
    }*/

    // Raw data

    /// Return a slice to the raw data, optionally including the prefix.
    /// This slice has no upper bound, to get the raw length, call `raw_len`.
    #[inline]
    pub fn get_raw_data(&self) -> &[u8] {
        let off = self.get_raw_offset();
        &self.data[off..]
    }

    /// Return a slice to the raw data, optionally including the prefix.
    /// This slice has no upper bound, to get the raw length, call `raw_len`.
    ///
    /// You can use this to received datagram's data on, and then
    /// call `sync_state` with the received length.
    #[inline]
    pub fn get_raw_data_mut(&mut self) -> &mut [u8] {
        let off = self.get_raw_offset();
        &mut self.data[off..]
    }

    // Data

    /// Return a slice to the data, this doesn't contains the prefix.
    #[inline]
    pub fn get_data(&self) -> &[u8] {
        &self.data[PACKET_PREFIX_LEN..][..self.len]
    }

    /// Return a mutable slice to the data, this doesn't contains the prefix
    /// and has the length as the upper bound. This is used for example by
    /// bundles to write elements.
    #[inline]
    pub fn get_data_mut(&mut self) -> &mut [u8] {
        &mut self.data[PACKET_PREFIX_LEN..][..self.len]
    }

    /// Return a slice to the body part of the internal data, starting after
    /// the flags header and ending before existing footers.
    #[inline]
    pub fn get_body_data(&self) -> &[u8] {
        &self.get_data()[PACKET_FLAGS_LEN..self.get_footer_offset()]
    }

    // Data reservation

    /// Internal method used to increment the length and return a mutable
    /// slice to the reserved data.
    pub fn reserve_unchecked(&mut self, len: usize) -> &mut [u8] {
        debug_assert!(self.len + len <= self.data.len() - PACKET_PREFIX_LEN, "Reserve overflow.");
        let ptr = &mut self.data[PACKET_PREFIX_LEN + self.len..][..len];
        self.len += len;
        self.footer_offset = self.len; // As we may overwrite the footer.
        ptr
    }

    /// Clear all this packet and restart from after the flags.
    pub fn clear(&mut self) {
        self.len = PACKET_FLAGS_LEN;
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

        // We need to get seq here to avoid &mut self/&self interference.
        let has_seq = self.has_seq();

        let mut cursor = Cursor::new(&mut self.data[..]);

        // Immediately write the prefix if needed.
        if let Some(prefix) = self.prefix {
            cursor.write_u32::<LittleEndian>(prefix).unwrap();
        }

        // Go to the end of the packet.
        cursor.set_position((PACKET_PREFIX_LEN + self.len) as u64);

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
        self.len = cursor.position() as usize - PACKET_PREFIX_LEN;

        if self.has_checksum {
            flags |= PacketFlags::HAS_CHECKSUM;
        }

        // let prefix_len = if self.has_prefix { PACKET_PREFIX_LEN as u64 } else { 0 };

        // Finally, write flags.
        cursor.set_position(PACKET_PREFIX_LEN as u64);
        cursor.write_u16::<LittleEndian>(flags).unwrap();

        // Calculate checksum and write it if enabled.
        // Placed here to take flags into checksum.
        if self.has_checksum {
            cursor.set_position(PACKET_PREFIX_LEN as u64);
            let checksum = Self::calc_checksum(&mut cursor, self.len as u64);
            cursor.write_u32::<LittleEndian>(checksum).unwrap();
            self.len += 4;
        }

    }

    /// Synchronize internal packet's state from its data.
    /// This can be called multiple times, the result is stable.
    /// If this packet has prefix, the length given to this method must count additional 4.
    ///
    /// *If this function returns an error, the integrity of the internal state is not guaranteed.*
    pub fn sync_state(&mut self, len: usize/*, has_prefix: bool*/) -> Result<(), PacketSyncError> {

        // Fix length if it contains a 4-bytes prefix.
        let real_len = len - if self.has_prefix() { PACKET_PREFIX_LEN } else { 0 };

        let mut cursor = Cursor::new(&mut self.data[..]);

        // If we have a prefix, read it, if not just seek after it.
        if let Some(ref mut prefix) = self.prefix {
            *prefix = cursor.read_u32::<LittleEndian>().unwrap();
        } else {
            cursor.set_position(PACKET_PREFIX_LEN as u64);
        }

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

        if real_len < footer_len + PACKET_FLAGS_LEN {
            return Err(PacketSyncError::TooShort);
        }

        self.len = real_len;
        // self.has_prefix = has_prefix;
        self.footer_offset = real_len - footer_len;

        cursor.set_position((PACKET_PREFIX_LEN + self.footer_offset) as u64);

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
            cursor.set_position(PACKET_PREFIX_LEN as u64);
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

impl Debug for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        let mut s = f.debug_struct("Packet");

        s.field("len", &self.len());
        s.field("raw_len", &self.raw_len());
        s.field("body_len", &self.body_len());

        s.field("body", &crate::util::get_hex_str_from(self.get_body_data(), 24));

        if let Some(prefix) = self.prefix {
            s.field("prefix", &format!("{:08X}", prefix));
        }

        if self.footer_offset < self.len {
            s.field("footer_offset", &self.footer_offset);
            s.field("footer_len", &(self.len - self.footer_offset));
        }

        if self.has_requests() {
            s.field("request_offset", &self.request_first_offset);
        }

        if self.has_seq() {
            s.field("seq_first", &self.seq_first);
            s.field("seq", &self.seq);
            s.field("seq_last", &self.seq_last);
        }

        s.finish()

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
