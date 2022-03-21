//! Definitions for elements contained in bundles (and so in packets).

use std::io::{self, Read, Seek, Write};

use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

pub mod login;


/*/// A codec implemented on a particular element, this is used when writing
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

}*/

// Following traits/structs are for the new element API //

/*/// Encoder for an element, such types are moved when encoding is required,
/// if you want to avoid moving a huge type, implement this trait on
/// (mutable) references.
pub trait ElementEncoder {
    /// Length codec used for this element.
    const LEN: ElementLength;
    /// The input type to encode.
    type Input;
    /// Encode an element.
    fn encode<W: Write>(&self, write: W, input: Self::Input) -> io::Result<()>;
}

/// Decoder for an element, such types are moved when decoding is required,
/// if you want to avoid moving a huge type, implement this trait on
/// (mutable) references.
pub trait ElementDecoder {
    /// Length codec used for this element.
    const LEN: ElementLength;
    /// The output type for this decoder.
    type Output;
    /// Decode an element, the given reader is seek-able and its length is given separately.
    fn decode<R: Read + Seek>(&self, read: R, len: u64) -> io::Result<Self::Output>;
}*/

pub trait ElementCodec {

    /// Length codec used for this element.
    const LEN: ElementLength;
    /// Type of the element.
    type Element;

    /// Encode an element.
    fn encode<W: Write>(&self, write: W, input: Self::Element) -> io::Result<()>;

    /// Decode an element, the given reader is seek-able and its length is given separately.
    fn decode<R: Read + Seek>(&self, read: R, len: u64) -> io::Result<Self::Element>;

}


/// Type of length used by a specific message codec.
/// This describes how the length of an element should be encoded in the packet.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ElementLength {
    /// The size of the element is fixed, and every writen element must be of this size.
    Fixed(u32),
    /// The size of the element is variable, and is encoded on 8 bits.
    Variable8,
    /// The size of the element is variable, and is encoded on 16 bits.
    Variable16,
    /// The size of the element is variable, and is encoded on 24 bits.
    Variable24,
    /// The size of the element is variable, and is encoded on 32 bits.
    Variable32
}

impl ElementLength {

    /// Read the length from a given reader.
    pub fn read<R: Read>(&self, mut reader: R) -> std::io::Result<u32> {
        match self {
            Self::Fixed(len) => Ok(*len),
            Self::Variable8 => reader.read_u8().map(|n| n as u32),
            Self::Variable16 => reader.read_u16::<LittleEndian>().map(|n| n as u32),
            Self::Variable24 => reader.read_u24::<LittleEndian>(),
            Self::Variable32 => reader.read_u32::<LittleEndian>(),
        }
    }

    /// Write the length to the given writer.
    pub fn write<W: Write>(&self, mut writer: W, len: u32) -> std::io::Result<()> {
        match self {
            Self::Fixed(fixed_len) => { assert_eq!(*fixed_len, len); Ok(()) },
            Self::Variable8 => writer.write_u8(len as u8),
            Self::Variable16 => writer.write_u16::<LittleEndian>(len as u16),
            Self::Variable24 => writer.write_u24::<LittleEndian>(len),
            Self::Variable32 => writer.write_u32::<LittleEndian>(len),
        }
    }

    /// Return the size in bytes of this type of length.
    pub fn len(&self) -> usize {
        match self {
            Self::Fixed(_) => 0,
            Self::Variable8 => 1,
            Self::Variable16 => 2,
            Self::Variable24 => 3,
            Self::Variable32 => 4,
        }
    }

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

    fn read_rich_blob(&mut self) -> io::Result<Vec<u8>> {
        let len = self.read_packed_u32()? as usize;
        let mut buf = vec![0; len];
        self.read_exact(&mut buf[..])?;
        Ok(buf)
    }

    fn read_rich_string(&mut self) -> io::Result<String> {
        let blob = self.read_rich_blob()?;
        match String::from_utf8(blob) {
            Ok(s) => Ok(s),
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


// Raw elements to use for debugging purposes

pub struct RawElementCodec<I: RawElementCodecLen>(I);

impl<I: RawElementCodecLen> ElementCodec for RawElementCodec<I> {

    const LEN: ElementLength = I::LEN;
    type Element = Vec<u8>;

    fn encode<W: Write>(&self, mut write: W, input: Self::Element) -> io::Result<()> {
        write.write_all(&input[..])
    }

    fn decode<R: Read + Seek>(&self, mut read: R, len: u64) -> io::Result<Self::Element> {
        let mut buf = Vec::with_capacity(len as usize);
        read.read_to_end(&mut buf)?;
        Ok(buf)
    }

}

impl<I: RawElementCodecLen + Default> RawElementCodec<I> {
    pub fn new() -> Self {
        Self(I::default())
    }
}

pub trait RawElementCodecLen {
    const LEN: ElementLength;
}

#[derive(Default)] pub struct RawElementCodecLenVar8;
#[derive(Default)] pub struct RawElementCodecLenVar16;
#[derive(Default)] pub struct RawElementCodecLenVar24;
#[derive(Default)] pub struct RawElementCodecLenVar32;
pub struct RawElementCodecLenFixed<const LEN: usize>([(); LEN]);

impl RawElementCodecLen for RawElementCodecLenVar8 {
    const LEN: ElementLength = ElementLength::Variable8;
}
impl RawElementCodecLen for RawElementCodecLenVar16 {
    const LEN: ElementLength = ElementLength::Variable16;
}
impl RawElementCodecLen for RawElementCodecLenVar24 {
    const LEN: ElementLength = ElementLength::Variable24;
}
impl RawElementCodecLen for RawElementCodecLenVar32 {
    const LEN: ElementLength = ElementLength::Variable32;
}
impl<const LEN: usize> RawElementCodecLen for RawElementCodecLenFixed<LEN> {
    const LEN: ElementLength = ElementLength::Fixed(LEN as u32);
}
impl<const LEN: usize> Default for RawElementCodecLenFixed<LEN> {
    fn default() -> Self {
        Self([(); LEN])
    }
}

pub type RawElementCodecVar8 = RawElementCodec<RawElementCodecLenVar8>;
pub type RawElementCodecVar16 = RawElementCodec<RawElementCodecLenVar16>;
pub type RawElementCodecVar24 = RawElementCodec<RawElementCodecLenVar24>;
pub type RawElementCodecVar32 = RawElementCodec<RawElementCodecLenVar32>;
pub type RawElementCodecFixed<const LEN: usize> = RawElementCodec<RawElementCodecLenFixed<LEN>>;

/*/// A reply element.
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

}*/
