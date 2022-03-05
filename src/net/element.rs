use std::collections::HashMap;
use std::io::{self, Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};


/// Type of length used by a specific message codec.
/// This describes how the length of an element should be encoded in the packet.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ElementLength {
    Fixed(u32),
    Variable8,
    Variable16,
    Variable24,
    Variable32
}

impl ElementLength {

    pub fn read<R: Read>(&self, reader: &mut R) -> std::io::Result<u32> {
        match self {
            Self::Fixed(len) => Ok(*len),
            Self::Variable8 => reader.read_u8().map(|n| n as u32),
            Self::Variable16 => reader.read_u16::<LittleEndian>().map(|n| n as u32),
            Self::Variable24 => reader.read_u24::<LittleEndian>(),
            Self::Variable32 => reader.read_u32::<LittleEndian>(),
        }
    }

    pub fn write<W: Write>(&self, writer: &mut W, len: u32) -> std::io::Result<()> {
        match self {
            Self::Fixed(fixed_len) => { assert_eq!(*fixed_len, len); Ok(()) },
            Self::Variable8 => writer.write_u8(len as u8),
            Self::Variable16 => writer.write_u16::<LittleEndian>(len as u16),
            Self::Variable24 => writer.write_u24::<LittleEndian>(len),
            Self::Variable32 => writer.write_u32::<LittleEndian>(len),
        }
    }

    pub fn header_len(&self) -> usize {
        match self {
            Self::Fixed(_) => 0,
            Self::Variable8 => 1,
            Self::Variable16 => 2,
            Self::Variable24 => 3,
            Self::Variable32 => 4,
        }
    }

}


/// A codec implemented on a particular element, this is used when writing
/// an element on a bundle or when adding receive handlers for particular
/// elements.
pub trait ElementCodec: Sized {

    /// Type of length used by this element.
    const LEN: ElementLength;

    /// The element's identifier.
    const ID: u8;

    /// Encode the element in the given writer.
    /// IO errors should only be returned if operations on the output fails.
    fn encode<W: Write>(&self, out: &mut W) -> io::Result<()>;

    /// Decode the element from the given reader.
    /// IO errors should only be returned if operations on the input fails.
    fn decode<R: Read>(input: &mut R) -> io::Result<Self>;

}


/// An element definition.
#[derive(Debug)]
pub struct ElementDef {
    pub name: &'static str,
    pub length: ElementLength,
}

impl ElementDef {
    pub const fn new(name: &'static str, length: ElementLength) -> Self {
        Self { name, length }
    }
}


/// A registry for all know elements. It's used to determine how
/// element's length is encoded into the packet's data.
pub struct ElementRegistry {
    codecs: HashMap<u8, ElementDef>
}

impl ElementRegistry {

    pub fn new() -> Self {
        Self {
            codecs: HashMap::new()
        }
    }

    pub fn register(&mut self, id: u8, codec: ElementDef) {
        self.codecs.insert(id, codec);
    }

    pub fn get(&self, id: u8) -> Option<&ElementDef> {
        self.codecs.get(&id)
    }

}
