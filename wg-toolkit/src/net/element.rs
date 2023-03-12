//! Definitions for elements contained in bundles (and so in packets).

use std::io::{self, Read, Write};

use crate::util::io::{WgReadExt, WgWriteExt};


pub mod ping;
pub mod login;
pub mod reply;
pub mod base;
pub mod client;


/// A trait to be implemented on an element structure. Elements are slices 
/// of data in a bundle of packets. If a bundle contains multiple elements
/// they are contiguously written.
/// 
/// Note that elements doesn't need to specify their length because they
/// could be used for replies to requests, if you want to use the element
/// as a top element (which mean that it provides a way to know its length
/// in the bundle), implement the [`TopElement`] trait and specify of the 
/// length of the element must be decoded.
/// 
/// You must provide a configuration type that will be given to encode
/// and decode functions.
pub trait Element: Sized {

    /// Type of the element that is being encoded and decoded.
    type Config;

    /// Encode the element with the given writer and the given configuration.
    fn encode<W: Write>(&self, write: W, config: &Self::Config) -> io::Result<()>;

    /// Decode the element from the given reader and the given configuration.
    /// The total length that is available in the reader is given.
    fn decode<R: Read>(read: R, len: usize, config: &Self::Config) -> io::Result<Self>;

}

/// An extension trait to implement for [`Element`] or [`SimpleElement`]
/// that can be used as top elements in a bundle by providing a way to
/// decode their length.
pub trait TopElement: Element {
    
    /// The type of length that prefixes the element's content and describe
    /// how much space is taken by the element.
    const LEN: ElementLength;

}

/// An alternative trait to [`Element`] (and incompatible) but without
/// provided configuration. Read the documentation of [`Element`] for
/// more information.
pub trait SimpleElement: Sized {

    /// Encode an element.
    fn encode<W: Write>(&self, write: W) -> io::Result<()>;

    /// Decode the element from the given stream
    fn decode<R: Read>(read: R, len: usize) -> io::Result<Self>;

}

impl<E: SimpleElement> Element for E {

    type Config = ();

    #[inline]
    fn encode<W: Write>(&self, write: W, _config: &Self::Config) -> io::Result<()> {
        SimpleElement::encode(self, write)
    }

    #[inline]
    fn decode<R: Read>(read: R, len: usize, _config: &Self::Config) -> io::Result<Self> {
        SimpleElement::decode(read, len)
    }

}


// /// A trait to be implemented on structures that acts as codec 
// /// for a given element type.
// #[deprecated]
// pub trait ElementCodec {

//     /// Type of the element that is being encoded and decoded.
//     type Element;

//     /// Encode an element.
//     fn encode<W: Write>(&self, write: W, input: Self::Element) -> io::Result<()>;

//     /// Decode an element, its length is given separately.
//     fn decode<R: Read>(&self, read: R, len: usize) -> io::Result<Self::Element>;

// }

// /// An extension trait for implementor of [`ElementCodec`] that
// /// can be decoded as top elements. 
// /// 
// /// For example, you don't need such top element for decoding 
// /// or encoding a reply, because a reply is always of varying
// /// 32 bit length.
// #[deprecated]
// pub trait TopElementCodec: ElementCodec {

//     /// If this element is being decoded as top element, this
//     /// length describe how to decode it.
//     const LEN: ElementLength;

// }


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
            Self::Variable16 => reader.read_u16().map(|n| n as u32),
            Self::Variable24 => reader.read_u24(),
            Self::Variable32 => reader.read_u32(),
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


// // Raw elements to use for debugging purposes

// pub struct RawElementCodec<I: RawElementCodecLen>(I);

// impl<I: RawElementCodecLen> ElementCodec for RawElementCodec<I> {

//     type Element = Vec<u8>;

//     fn encode<W: Write>(&self, mut write: W, input: Self::Element) -> io::Result<()> {
//         write.write_all(&input[..])
//     }

//     fn decode<R: Read>(&self, mut read: R, len: usize) -> io::Result<Self::Element> {
//         let mut buf = Vec::with_capacity(len);
//         read.read_to_end(&mut buf)?;
//         Ok(buf)
//     }

// }

// impl<I: RawElementCodecLen> TopElementCodec for RawElementCodec<I> {
//     const LEN: ElementLength = I::LEN;
// }

// impl<I: RawElementCodecLen + Default> RawElementCodec<I> {
//     pub fn new() -> Self {
//         Self(I::default())
//     }
// }

// pub trait RawElementCodecLen {
//     const LEN: ElementLength;
// }

// #[derive(Default)] pub struct RawElementCodecLenVar8;
// #[derive(Default)] pub struct RawElementCodecLenVar16;
// #[derive(Default)] pub struct RawElementCodecLenVar24;
// #[derive(Default)] pub struct RawElementCodecLenVar32;
// pub struct RawElementCodecLenFixed<const LEN: usize>([(); LEN]);

// impl RawElementCodecLen for RawElementCodecLenVar8 {
//     const LEN: ElementLength = ElementLength::Variable8;
// }
// impl RawElementCodecLen for RawElementCodecLenVar16 {
//     const LEN: ElementLength = ElementLength::Variable16;
// }
// impl RawElementCodecLen for RawElementCodecLenVar24 {
//     const LEN: ElementLength = ElementLength::Variable24;
// }
// impl RawElementCodecLen for RawElementCodecLenVar32 {
//     const LEN: ElementLength = ElementLength::Variable32;
// }
// impl<const LEN: usize> RawElementCodecLen for RawElementCodecLenFixed<LEN> {
//     const LEN: ElementLength = ElementLength::Fixed(LEN as u32);
// }
// impl<const LEN: usize> Default for RawElementCodecLenFixed<LEN> {
//     fn default() -> Self {
//         Self([(); LEN])
//     }
// }

// pub type Var8ElementCodec = RawElementCodec<RawElementCodecLenVar8>;
// pub type Var16ElementCodec = RawElementCodec<RawElementCodecLenVar16>;
// pub type Var24ElementCodec = RawElementCodec<RawElementCodecLenVar24>;
// pub type Var32ElementCodec = RawElementCodec<RawElementCodecLenVar32>;
// pub type FixedElementCodec<const LEN: usize> = RawElementCodec<RawElementCodecLenFixed<LEN>>;


// /// Use this macro to easily define symmetric element codecs.
// macro_rules! symmetric_codec {
//     (
//         $codec_ident:ident -> $element_ident:ident {
//             $( $field_ident:ident: $field_ty:ident ),*
//             $(,)?
//         }
//         $($more:tt)*
//     ) => {
        
//         pub struct $codec_ident;

//         impl $crate::net::element::ElementCodec for $codec_ident {

//             type Element = $element_ident;
        
//             fn encode<W: std::io::Write>(&self, mut write: W, input: Self::Element) -> std::io::Result<()> {
//                 use crate::util::io::WgWriteExt;
//                 $( $crate::net::element::symmetric_codec_write_field!(write, input.$field_ident, $field_ty)?; )*
//                 Ok(())
//             }
        
//             fn decode<R: std::io::Read>(&self, mut read: R, _len: usize) -> std::io::Result<Self::Element> {
//                 use crate::util::io::WgReadExt;
//                 Ok($element_ident {
//                     $( $field_ident: $crate::net::element::symmetric_codec_read_field!(read, $field_ty)? ),*
//                 })
//             }
        
//         }

//     };
// }

// macro_rules! symmetric_codec_write_field {
//     ($writer:ident, $val:expr, u8) => { $writer.write_u8($val) };
//     ($writer:ident, $val:expr, i8) => { $writer.write_i8($val) };
//     ($writer:ident, $val:expr, u32) => { $writer.write_u32($val) };
//     ($writer:ident, $val:expr, i32) => { $writer.write_i32($val) };
// }

// macro_rules! symmetric_codec_read_field {
//     ($reader:ident, u8) => { $reader.read_u8() };
//     ($reader:ident, i8) => { $reader.read_i8() };
//     ($reader:ident, u32) => { $reader.read_u32() };
//     ($reader:ident, i32) => { $reader.read_i32() };
// }

// pub(crate) use {symmetric_codec, symmetric_codec_write_field, symmetric_codec_read_field};
