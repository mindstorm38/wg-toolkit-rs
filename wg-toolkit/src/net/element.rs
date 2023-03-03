//! Definitions for elements contained in bundles (and so in packets).

use std::io::{self, Read, Write};
use std::net::{SocketAddrV4, Ipv4Addr};

use byteorder::{ReadBytesExt, WriteBytesExt, LE, BE};


pub mod ping;
pub mod login;
pub mod reply;
pub mod base;


/// A trait to be implemented on structures that acts as codec 
/// for a given element type.
pub trait ElementCodec {

    /// Type of the element that is being encoded and decoded.
    type Element;

    /// Encode an element.
    fn encode<W: Write>(&self, write: W, input: Self::Element) -> io::Result<()>;

    /// Decode an element, its length is given separately.
    fn decode<R: Read>(&self, read: R, len: usize) -> io::Result<Self::Element>;

}

/// An extension trait for implementor of [`ElementCodec`] that
/// can be decoded as top elements. 
/// 
/// For example, you don't need such top element for decoding 
/// or encoding a reply, because a reply is always of varying
/// 32 bit length.
pub trait TopElementCodec: ElementCodec {

    /// If this element is being decoded as top element, this
    /// length describe how to decode it.
    const LEN: ElementLength;

}


/// Type of length used by a specific message codec.
/// This describes how the length of an element should be encoded in the packet.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ElementLength {
    /// The size of the element is fixed, and every written element must be of this size.
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
            Self::Variable16 => reader.read_u16::<LE>().map(|n| n as u32),
            Self::Variable24 => reader.read_u24::<LE>(),
            Self::Variable32 => reader.read_u32::<LE>(),
        }
    }

    /// Write the length to the given writer.
    pub fn write<W: Write>(&self, mut writer: W, len: u32) -> std::io::Result<()> {
        match self {
            Self::Fixed(fixed_len) => { assert_eq!(*fixed_len, len); Ok(()) },
            Self::Variable8 => writer.write_u8(len as u8),
            Self::Variable16 => writer.write_u16::<LE>(len as u16),
            Self::Variable24 => writer.write_u24::<LE>(len),
            Self::Variable32 => writer.write_u32::<LE>(len),
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
            255 => self.read_u24::<LE>(),
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
            Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "invalid utf8 string"))
        }
    }

    fn read_sock_addr_v4(&mut self) -> io::Result<SocketAddrV4> {
        let mut ip_raw = [0; 4];
        self.read_exact(&mut ip_raw[..])?;
        let port = self.read_u16::<BE>()?;
        let _salt = self.read_u16::<LE>()?;
        Ok(SocketAddrV4::new(Ipv4Addr::from(ip_raw), port))
    }

}


/// A extension trait for `Write` specific to element encoding.
pub trait ElementWriteExt: Write {

    /// Write a packed 32-bits integer.
    fn write_packed_u32(&mut self, n: u32) -> io::Result<()> {
        if n >= 255 {
            self.write_u8(255)?;
            self.write_u24::<LE>(n)
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
    fn write_sock_addr_v4(&mut self, addr: SocketAddrV4) -> io::Result<()> {
        self.write_all(&addr.ip().octets()[..])?;
        self.write_u16::<BE>(addr.port())?;
        self.write_u16::<LE>(0)?; // Salt
        Ok(())
    }

}

impl<R: Read> ElementReadExt for R {}
impl<W: Write> ElementWriteExt for W {}


// Raw elements to use for debugging purposes

pub struct RawElementCodec<I: RawElementCodecLen>(I);

impl<I: RawElementCodecLen> ElementCodec for RawElementCodec<I> {

    type Element = Vec<u8>;

    fn encode<W: Write>(&self, mut write: W, input: Self::Element) -> io::Result<()> {
        write.write_all(&input[..])
    }

    fn decode<R: Read>(&self, mut read: R, len: usize) -> io::Result<Self::Element> {
        let mut buf = Vec::with_capacity(len);
        read.read_to_end(&mut buf)?;
        Ok(buf)
    }

}

impl<I: RawElementCodecLen> TopElementCodec for RawElementCodec<I> {
    const LEN: ElementLength = I::LEN;
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

pub type Var8ElementCodec = RawElementCodec<RawElementCodecLenVar8>;
pub type Var16ElementCodec = RawElementCodec<RawElementCodecLenVar16>;
pub type Var24ElementCodec = RawElementCodec<RawElementCodecLenVar24>;
pub type Var32ElementCodec = RawElementCodec<RawElementCodecLenVar32>;
pub type FixedElementCodec<const LEN: usize> = RawElementCodec<RawElementCodecLenFixed<LEN>>;
