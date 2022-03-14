//! Definitions for elements contained in bundles (and so in packets).

use std::io::{self, Read, Seek, Write};

use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

pub mod login;



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

    /// Encode the element in the given writer.
    /// IO errors should only be returned if operations on the output fails.
    fn encode<W: Write>(&self, out: &mut W) -> io::Result<()>;

    /// Decode the element from the given reader.
    /// IO errors should only be returned if operations on the input fails.
    fn decode<R: Read + Seek>(input: &mut R) -> io::Result<Self>;

}


/// A reply element.
pub struct ReplyElement<C> {
    reply_id: u32,
    inner: C
}

impl<C> ElementCodec for ReplyElement<C>
where
    C: ElementCodec
{

    const LEN: ElementLength = ElementLength::Variable32;

    fn encode<W: Write>(&self, out: &mut W) -> io::Result<()> {
        out.write_u32::<LittleEndian>(self.reply_id)?;
        self.inner.encode(out)
    }

    fn decode<R: Read + Seek>(input: &mut R) -> io::Result<Self> {
        let reply_id = input.read_u32::<LittleEndian>()?;
        let inner = C::decode(input)?;
        Ok(Self { reply_id, inner })
    }

}


macro_rules! def_raw_element_struct {
    ($name:ident, $len: ident) => {

        #[derive(Debug)]
        pub struct $name(pub Vec<u8>);

        impl $name {
            pub fn new() -> Self {
                Self(Vec::new())
            }
        }

        impl ElementCodec for $name {

            const LEN: ElementLength = ElementLength::$len;

            fn encode<W: Write>(&self, out: &mut W) -> io::Result<()> {
                out.write_all(&self.0[..])
            }

            fn decode<R: Read + Seek>(input: &mut R) -> io::Result<Self> {
                let mut data = Vec::new();
                input.read_to_end(&mut data)?;
                Ok(Self(data))
            }

        }

    };
}

def_raw_element_struct!(RawElementVariable8, Variable8);
def_raw_element_struct!(RawElementVariable16, Variable16);
def_raw_element_struct!(RawElementVariable24, Variable24);
def_raw_element_struct!(RawElementVariable32, Variable32);


#[derive(Debug)]
pub struct RawElementFixed<const LEN: usize>(pub [u8; LEN]);

impl<const LEN: usize> ElementCodec for RawElementFixed<LEN> {

    const LEN: ElementLength = ElementLength::Fixed(LEN as u32);

    fn encode<W: Write>(&self, out: &mut W) -> io::Result<()> {
        out.write_all(&self.0[..])
    }

    fn decode<R: Read + Seek>(input: &mut R) -> io::Result<Self> {
        let mut ret = Self([0; LEN]);
        input.read_exact(&mut ret.0[..])?;
        Ok(ret)
    }

}
