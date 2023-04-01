//! Definitions for elements contained in bundles (and so in packets).

use std::io::{self, Read, Write};

use crate::util::io::*;


pub mod login;
pub mod reply;
pub mod base;
pub mod client;

pub mod entity;


/// A trait to be implemented on a structure that can be interpreted as
/// bundle's elements. Elements are slices of data in a bundle of packets. 
/// If a bundle contains multiple elements they are written contiguously.
/// 
/// Note that elements doesn't need to specify their length because they
/// could be used for replies to requests, if you want to use the element
/// as a top element (which mean that it provides a way to know its length
/// in the bundle), implement the [`TopElement`] trait and specify its type
/// of length.
/// 
/// You must provide a configuration type that will be given to encode
/// and decode functions.
pub trait Element: Sized {

    /// Type of the element's config that is being encoded and decoded.
    type Config;

    /// Encode the element with the given writer and the given configuration.
    fn encode(&self, write: &mut impl Write, config: &Self::Config) -> io::Result<()>;

    /// Decode the element from the given reader and the given configuration.
    /// 
    /// The total length that is available in the reader is also given. **Note
    /// that** the given length will be equal to zero if the element's length
    /// is set to [`ElementLength::Unknown`] (relevant for top elements).
    fn decode(read: &mut impl Read, len: usize, config: &Self::Config) -> io::Result<Self>;

}

/// A "top element" extends the behavior of a regular [`Element`] by providing
/// a length that describes how to encode and decode the length of this element.
pub trait TopElement: Element {

    /// The type of length that prefixes the element's content and describe
    /// how much space is taken by the element.
    const LEN: ElementLength;

}

/// This trait provides an easier implementation of [`Element`], therefore
/// both traits cannot be implemented at the same time.
pub trait SimpleElement: Sized {

    /// Encode the element with the given writer.
    fn encode(&self, write: &mut impl Write) -> io::Result<()>;

    /// Decode the element from the given reader.
    /// 
    /// The total length that is available in the reader is also given. **Note
    /// that** the given length will be equal to zero if the element's length
    /// is set to [`ElementLength::Unknown`] (relevant for top elements).
    fn decode(read: &mut impl Read, len: usize) -> io::Result<Self>;

}

impl<E: SimpleElement> Element for E {

    type Config = ();

    #[inline]
    fn encode(&self, write: &mut impl Write, _config: &Self::Config) -> io::Result<()> {
        SimpleElement::encode(self, write)
    }

    #[inline]
    fn decode(read: &mut impl Read, len: usize, _config: &Self::Config) -> io::Result<Self> {
        SimpleElement::decode(read, len)
    }

}


/// An alternative trait to both [`Element`] and [`TopElement`] that 
/// automatically implements nothing for encode and provides the default 
/// value on decoding without actually reading. The trait [`TopElement`]
/// is also implemented to specify a fixed length of 0.
pub trait EmptyElement: Default {}

impl<E: EmptyElement> SimpleElement for E {

    fn encode(&self, _write: &mut impl Write) -> io::Result<()> {
        Ok(())
    }

    fn decode(_read: &mut impl Read, _len: usize) -> io::Result<Self> {
        Ok(Self::default())
    }
    
}

impl<E: EmptyElement> TopElement for E {
    const LEN: ElementLength = ElementLength::Fixed(0);
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
    Variable32,
    /// The size of the element is unknown at runtime and will be determined by
    /// the decoder or the encoder by how much the reader or write is consumed.
    /// 
    /// *This is the way to go for lengths of type Callback in BigWorld engine.*
    Unknown,
}

impl ElementLength {

    /// Read the length from a given reader.
    pub fn read<R: Read>(&self, mut reader: R) -> std::io::Result<Option<u32>> {
        match self {
            Self::Fixed(len) => Ok(Some(*len)),
            Self::Variable8 => reader.read_u8().map(|n| Some(n as u32)),
            Self::Variable16 => reader.read_u16().map(|n| Some(n as u32)),
            Self::Variable24 => reader.read_u24().map(|n| Some(n)),
            Self::Variable32 => reader.read_u32().map(|n| Some(n)),
            Self::Unknown => Ok(None),
        }
    }

    /// Write the length to the given writer.
    pub fn write<W: Write>(&self, mut writer: W, len: u32) -> std::io::Result<()> {
        match self {
            Self::Fixed(fixed_len) => { assert_eq!(*fixed_len, len); Ok(()) },
            Self::Variable8 => writer.write_u8(len as u8),
            Self::Variable16 => writer.write_u16(len as u16),
            Self::Variable24 => writer.write_u24(len),
            Self::Variable32 => writer.write_u32(len),
            Self::Unknown => Ok(()),
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
            Self::Unknown => 0,
        }
    }

}


#[derive(Debug)]
pub struct UnknownElement(pub Vec<u8>);

impl SimpleElement for UnknownElement {

    fn encode(&self, write: &mut impl Write) -> io::Result<()> {
        write.write_blob(&self.0)
    }

    fn decode(read: &mut impl Read, _len: usize) -> io::Result<Self> {
        let mut buf = Vec::new();
        read.read_to_end(&mut buf)?;
        Ok(UnknownElement(buf))
    }

}

impl TopElement for UnknownElement {
    const LEN: ElementLength = ElementLength::Unknown;
}
