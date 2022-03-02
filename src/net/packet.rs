use std::io::{Cursor, Read, Seek, SeekFrom};
use std::collections::HashMap;

use byteorder::{ReadBytesExt, BigEndian, LittleEndian};
use crate::space::section::ReadSectionExt;


/*pub struct Packet {
    flags: Flags,
    data: Vec<u8>
}

impl Packet {

    pub fn from_read<R: Read>(read: &mut R, codecs: &MessageCodecs) -> std::io::Result<Self> {

        let _prefix = read.read_u32::<LittleEndian>()?;
        let flags = Flags(read.read_u16::<LittleEndian>()?);

        let message_id = read.read_u8()?;
        let codec = codecs.get(message_id).expect("TODO: remove this");
        let message_len = codec.length_type.read(read)?;

        if flags.has_requests() {
            let replyId = read.read_u32::<LittleEndian>()?;
            let offset = read.read_u16::<LittleEndian>()?;
        }

        todo!()

    }

}*/


/// Packet's flags.
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Flags(pub u16);

impl Flags {

    #[inline]
    pub fn has_requests(self) -> bool {
        (self.0 & 0x1) != 0
    }

    #[inline]
    pub fn has_piggybacks(self) -> bool {
        (self.0 & 0x2) != 0
    }

    #[inline]
    pub fn has_checksum(self) -> bool {
        (self.0 & 0x100) != 0
    }

}


/// A decoder applied to specific packet's data. It's also an iterator of `PacketMessage`.
pub struct PacketDecoder<'data, 'codec> {
    codecs: &'codec MessageCodecs,
    data: &'data [u8],
    head_cursor: Cursor<&'data [u8]>,
    foot_cursor: Cursor<&'data [u8]>,
    flags: Flags,
    next_request_offset: u64
}

impl<'data, 'codec> PacketDecoder<'data, 'codec> {

    /// Construct a new decoder for the given raw packet's data, this data must not
    /// contains the 4 bytes prefix.
    pub fn new(data: &'data [u8], codecs: &'codec MessageCodecs) -> Self {

        // TODO: Check data minimum length requirements.

        let mut head_cursor = Cursor::new(data);
        let mut foot_cursor = Cursor::new(data);

        let flags = Flags(head_cursor.read_u16::<LittleEndian>().unwrap());

        let next_request_offset;
        if flags.has_requests() {
            foot_cursor.seek(SeekFrom::End(-4)).unwrap();
            next_request_offset = foot_cursor.read_u16::<LittleEndian>().unwrap() as u64;
            foot_cursor.seek(SeekFrom::End(-4)).unwrap();
        } else {
            foot_cursor.seek(SeekFrom::End(0)).unwrap();
            next_request_offset = 0;
        }

        Self {
            codecs,
            data,
            head_cursor,
            foot_cursor,
            flags,
            next_request_offset
        }

    }

}

impl<'data, 'codec> Iterator for PacketDecoder<'data, 'codec> {

    type Item = PacketMessage<'data, 'codec>;

    fn next(&mut self) -> Option<Self::Item> {

        let offset = self.head_cursor.position();
        let request = self.next_request_offset == offset;

        let id = self.head_cursor.read_u8().unwrap();
        let codec = self.codecs.get(id).expect("TODO: remove this .expect");

        let length = codec.length_type.read(&mut self.head_cursor).unwrap();

        if request {
            let reply_id = self.head_cursor.read_u32::<LittleEndian>().unwrap();
            let next_request_offset = self.head_cursor.read_u16::<LittleEndian>().unwrap();
            todo!();
        }

        let data_begin = self.head_cursor.position() as usize;
        let data_end = data_begin + length as usize;
        let data = &self.data[data_begin..data_end];
        self.head_cursor.set_position(data_end as u64);

        Some(PacketMessage {
            id,
            data,
            codec,
        })

    }

}


/// A single message in a packet.
pub struct PacketMessage<'data, 'codec> {
    id: u8,
    data: &'data [u8],
    codec: &'codec MessageCodec,
}


/// A registry for all know message codecs. It's used to determine how message's length
/// is encoded into the packet's data.
pub struct MessageCodecs {
    codecs: HashMap<u8, MessageCodec>
}

impl MessageCodecs {

    pub fn new() -> Self {
        Self {
            codecs: HashMap::new()
        }
    }

    pub fn register(&mut self, id: u8, codec: MessageCodec) {
        self.codecs.insert(id, codec);
    }

    pub fn get(&self, id: u8) -> Option<&MessageCodec> {
        self.codecs.get(&id)
    }

}


/// A particular message codec, used to know how length is encoded/decoded in the raw packet.
pub struct MessageCodec {
    length_type: LengthType,
}


#[derive(Clone, Eq, PartialEq, Debug)]
pub enum LengthType {
    Fixed(u32),
    Variable8,
    Variable16,
    Variable24,
    Variable32
}

impl LengthType {

    pub fn read<R: Read>(&self, read: &mut R) -> std::io::Result<u32> {
        match self {
            Self::Fixed(len) => Ok(*len),
            Self::Variable8 => read.read_u8().map(|n| n as u32),
            Self::Variable16 => read.read_u16::<LittleEndian>().map(|n| n as u32),
            Self::Variable24 => read.read_u24::<LittleEndian>(),
            Self::Variable32 => read.read_u32::<LittleEndian>(),
            _ => Ok(0)  // TODO: Should be an error in the future.
        }
    }

}