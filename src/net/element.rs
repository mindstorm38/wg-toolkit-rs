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

    /// Encode options type, use `()` if no particular options are expected.
    type EncodeCfg;
    /// Decode options type, use `()` if no particular options are expected.
    type DecodeCfg;

    /// Encode the element in the given writer.
    /// IO errors should only be returned if operations on the output fails.
    fn encode<W: Write>(&self, output: &mut W, cfg: &Self::EncodeCfg) -> io::Result<()>;

    /// Decode the element from the given reader.
    /// IO errors should only be returned if operations on the input fails.
    fn decode<R: Read + Seek>(&mut self, input: &mut R, cfg: &Self::DecodeCfg) -> io::Result<()>;

}


pub trait ElementEncoder {
    type Element;
    fn encode<W: Write>(&mut self, elt: &Self::Element) -> io::Result<()>;
}

pub trait ElementDecoder {
    type Element;
    fn decode<R: Read + Seek>(&mut self) -> io::Result<Self::Element>;
}


/// A extension trait for `Read` specific to element decoding.
pub trait ElementReadExt: Read {

    /// Read a packed 32-bits integer.
    fn read_packed_u32(&mut self) -> io::Result<u32> {
        match self.read_u8()? {
            255 => self.read_u24::<LittleEndian>(),
            n => Ok(n as u32)
        }
    }

    fn read_rich_blob(&mut self, dst: &mut Vec<u8>) -> io::Result<()> {
        let len = self.read_packed_u32()? as usize;
        let mut buf = vec![0; len];
        self.read_exact(&mut buf[..])?;
        dst.extend_from_slice(&buf[..]);
        Ok(())
    }

    fn read_rich_string(&mut self, dst: &mut String) -> io::Result<()> {
        let len = self.read_packed_u32()? as usize;
        let mut buf = vec![0; len];
        self.read_exact(&mut buf[..])?;
        match std::str::from_utf8(&buf[..]) {
            Ok(s) => {
                dst.push_str(s);
                Ok(())
            },
            Err(_) => Err(io::ErrorKind::InvalidData.into())
        }
    }

}


/// A extension trait for `Write` specific to element encoding.
pub trait ElementWriteExt: Write {

    /// Write a packed 32-bits integer.
    fn write_packed_u32(&mut self, n: u32) -> io::Result<()> {
        if n >= 255 {
            self.write_u8(255)?;
            self.write_u24::<LittleEndian>(n)
        } else {
            self.write_u8(n as u8)
        }
    }

    /// Write a blob of data with its packed length before.
    fn write_rich_blob(&mut self, data: &[u8]) -> io::Result<()> {
        self.write_packed_u32(data.len() as u32)?;
        self.write_all(data)
    }

    /// Write a string with its packed length before.
    fn write_rich_string(&mut self, s: &str) -> io::Result<()> {
        self.write_rich_blob(s.as_bytes())
    }

}

impl<R: Read> ElementReadExt for R {}
impl<W: Write> ElementWriteExt for W {}


/// A reply element.
pub struct ReplyElement<C> {
    inner: C,
    reply_id: u32,
}

impl<C> ReplyElement<C> {
    pub fn new(inner: C, reply_id: u32) -> Self {
        Self {
            inner,
            reply_id
        }
    }
}

impl<C: Default> Default for ReplyElement<C> {
    fn default() -> Self {
        Self::new(C::default(), 0)
    }
}

impl<C: ElementCodec> ElementCodec for ReplyElement<C> {

    const LEN: ElementLength = ElementLength::Variable32;
    type EncodeCfg = C::EncodeCfg;
    type DecodeCfg = C::DecodeCfg;

    fn encode<W: Write>(&self, output: &mut W, cfg: &Self::EncodeCfg) -> io::Result<()> {
        output.write_u32::<LittleEndian>(self.reply_id)?;
        self.inner.encode(output, cfg)
    }

    fn decode<R: Read + Seek>(&mut self, input: &mut R, cfg: &Self::DecodeCfg) -> io::Result<()> {
        self.reply_id = input.read_u32::<LittleEndian>()?;
        self.inner.decode(input, cfg)
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
            type EncodeCfg = ();
            type DecodeCfg = ();

            fn encode<W: Write>(&self, output: &mut W, _cfg: &Self::EncodeCfg) -> io::Result<()> {
                output.write_all(&self.0[..])
            }

            fn decode<R: Read + Seek>(&mut self, input: &mut R, _cfg: &Self::DecodeCfg) -> io::Result<()> {
                self.0.clear();
                input.read_to_end(&mut self.0).map(|_| ())
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
    type EncodeCfg = ();
    type DecodeCfg = ();

    fn encode<W: Write>(&self, output: &mut W, _cfg: &Self::EncodeCfg) -> io::Result<()> {
        output.write_all(&self.0[..])
    }

    fn decode<R: Read + Seek>(&mut self, input: &mut R, _cfg: &Self::DecodeCfg) -> io::Result<()> {
        input.read_exact(&mut self.0[..])
    }

}
