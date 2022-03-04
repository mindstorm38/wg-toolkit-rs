use std::io::{Write, Seek, SeekFrom, Cursor};
use byteorder::{WriteBytesExt, LittleEndian};

use super::element::ElementCodec;
use super::PacketFlags;


const PACKET_MAX_SIZE: usize = 1472;


/// A elements bundle, used to pack elements and send them on an interface.
pub struct Bundle {
    /// Chain of packets.
    packets: Vec<BundlePacket>,
    /// Available length on the tail packet.
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

        // Allocate element's header, +1 for element's ID.
        let header_len = E::LEN.header_len() + 1 + if request { 6 } else { 0 };
        self.reserve_exact(header_len)[0] = E::ID;

        // Update the current packet's cursor and header length.
        let first_packet_idx = self.packets.len() - 1;
        let first_packet = &mut self.packets[first_packet_idx];
        if request {
            let cursor = first_packet.elt_cursor;
            // -2 because link offset is encoded on two bytes (u16).
            first_packet.add_request(cursor, cursor + header_len - 2);
        }
        first_packet.elt_cursor += header_len;
        first_packet.elt_header_len = header_len;

        // Write the actual element's content.
        let mut writer = BundleWriter::new(self);
        elt.encode(&mut writer);
        let length = writer.len as u32;

        // Finally write length.
        let first_packet = &mut self.packets[first_packet_idx];
        let first_len_slice = &mut first_packet.data[first_packet.elt_offset + 1..][..header_len];
        E::LEN.write(&mut Cursor::new(first_len_slice), length);

        // Finally, we update the last packet's next element offset.
        let last_packet = self.packets.last_mut().unwrap();
        last_packet.elt_offset = last_packet.elt_cursor;
        last_packet.elt_header_len = 0;

    }

    /// Finalize the bundle. This method can be called many times.
    pub fn finalize(&mut self, seq_id: &mut u32) {

        let multi_packet = self.packets.len() > 1;

        let first_seq;
        let last_seq;
        let mut current_seq = 0;

        if multi_packet {
            first_seq = *seq_id;
            current_seq = first_seq;
            *seq_id += self.packets.len() as usize;
            last_seq = *seq_id - 1;
        } else {
            first_seq = 0;
            last_seq = 0;
        }

        for packet in &mut self.packets {

            if packet.elt_footer_offset != 0 {
                // If the footer has already been written, reset the cursor to the footer offset.
                packet.elt_cursor = packet.elt_footer_offset;
            } else {
                // Else, set the footer offset to the cursor.
                packet.elt_footer_offset = packet.elt_offset;
            }

            let mut footer = Cursor::new(&mut packet.data[packet.elt_cursor..]);

            if multi_packet {
                packet.flags.enable(PacketFlags::IS_FRAGMENT);
                packet.flags.enable(PacketFlags::HAS_SEQUENCE_NUMBER);
                footer.write_u32::<LittleEndian>(first_seq).unwrap();
                footer.write_u32::<LittleEndian>(last_seq).unwrap();
            }

            if !packet.requests.is_empty() {
                packet.flags.enable(PacketFlags::HAS_REQUESTS);
                let mut last_link_offset = 0;
                for &(offset, link_offset) in &packet.requests {
                    if last_link_offset != 0 {
                        Cursor::new(&mut packet.data[last_link_offset..])
                            .write_u16::<LittleEndian>(offset as u16).unwrap();
                    }
                    last_link_offset = link_offset;
                }
                footer.write_u16::<LittleEndian>(packet.requests[0].0 as u16).unwrap();
            }

            if multi_packet {
                footer.write_u32::<LittleEndian>(current_seq).unwrap();
                current_seq += 1;
            }

            todo!()

        }

    }

    /// Internal method to add a new packet at the end of the chain.
    fn add_packet(&mut self) {
        self.packets.push(BundlePacket::new());
    }

    /// Reserve exactly the given length in the current packet or a new one if
    /// this such space is not available in the current packet.
    fn reserve_exact(&mut self, len: usize) -> &mut [u8] {
        let new_packet = (self.available_len < len);
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
        let new_packet = (self.available_len == 0);
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

pub struct BundlePacket {
    /// Flags of the packet, only in-sync with the raw data after finalization.
    flags: PacketFlags,
    /// Raw data of the packet, header and footer data is not valid until
    /// finalization of the packet.
    data: [u8; PACKET_MAX_SIZE],
    /// All requests in this bundle (offset, link_offset).
    requests: Vec<(usize, usize)>,
    /// The current element's offset.
    elt_offset: usize,
    /// The current element's cursor used for writing.
    /// Like the offset, this cursor should never get higher than reserved footer
    /// while writing message data, however footer data can grow the cursor.
    elt_cursor: usize,
    /// Length of the element's header (element id + length).
    elt_header_len: usize,
    /// Offset of the footer when the packet is finalized (0 if not).
    elt_footer_offset: usize
    /*/// Offset of the first "request element" in this packet.
    /// Special value 0 means no request is currently in the packet.
    /// This value is written in the packet's footer on finalization.
    first_request_elt_offset: usize,
    /// Offset of the last request packet, it's used when adding a new request
    /// to update the "next request offset" field in the last request's element.
    last_request_elt_link_offset: usize*/
}

impl BundlePacket {

    pub fn new() -> Self {
        Self {
            flags: PacketFlags(0),
            data: [0; PACKET_MAX_SIZE],
            requests: Vec::new(),
            elt_offset: 2, // Size of flags
            elt_cursor: 2,
            elt_header_len: 0,
            elt_footer_offset: 0
        }
    }

    /// Returns the free length available in this packet after the current cursor.
    pub fn available_len(&self) -> usize {
        self.data.len() - self.elt_cursor - 50  // TODO: Change -50 by a more accurate maximum footer size.
    }

    /// Returns the current length used by the current element.
    pub fn element_len(&self) -> usize {
        self.elt_cursor - (self.elt_offset + self.elt_header_len)
    }

    /// Enable requests flag on this packet and add the request.
    pub fn add_request(&mut self, offset: usize, link_offset: usize) {
        self.requests.push((offset, link_offset));
    }

    /// Internal method used to increment the cursor's offset and return a mutable
    /// slice to the reserved data.
    fn reserve_unchecked(&mut self, len: usize) -> &mut [u8] {
        let ptr = &mut self.data[self.elt_cursor..][..len];
        self.elt_cursor += 1;
        ptr
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
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }

}
