use byteorder::{ReadBytesExt, LittleEndian};
use std::io::{Cursor};

use super::element::{ElementRegistry, ElementDef};
use super::PacketFlags;


/// A packet constructed from its raw data, used to extract headers,
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
