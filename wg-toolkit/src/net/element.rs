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
    /// A fixed length element, the length is not written in the header.
    Fixed(u32),
    /// The length is encoded on 8 bits in the element's header.
    Variable8,
    /// The length is encoded on 16 bits in the element's header.
    Variable16,
    /// The length is encoded on 24 bits in the element's header.
    Variable24,
    /// The length is encoded on 32 bits in the element's header.
    Variable32,
    /// The real length of the element is queried dynamically with a callback,
    /// given the element's identifier.
    Callback(fn(id: u8) -> ElementLength),
    // /// The size of the element is unknown at runtime and will be determined by
    // /// the decoder or the encoder by how much the reader or write is consumed.
    // /// 
    // /// *This is the way to go for lengths of type Callback in BigWorld engine.*
    // Unknown,
}

impl ElementLength {

    /// Read the length from a given reader.
    pub fn read(mut self, mut reader: impl Read, id: u8) -> std::io::Result<u32> {

        // If the length is a callback, get the real length from the message id.
        if let Self::Callback(cb) = self {
            self = cb(id);
        }

        match self {
            Self::Fixed(len) => Ok(len),
            Self::Variable8 => reader.read_u8().map(|n| n as u32),
            Self::Variable16 => reader.read_u16().map(|n| n as u32),
            Self::Variable24 => reader.read_u24().map(|n| n),
            Self::Variable32 => reader.read_u32().map(|n| n),
            // Self::Unknown => Ok(None),
            Self::Callback(_) => unreachable!("a callback returned a callback length")
        }

    }

    /// Write the length to the given writer.
    pub fn write(self, mut writer: impl Write, len: u32) -> std::io::Result<()> {

        match self {
            Self::Fixed(expected_len) => { 
                assert_eq!(expected_len, len); 
                Ok(()) 
            }
            Self::Variable8 => writer.write_u8(len as u8),
            Self::Variable16 => writer.write_u16(len as u16),
            Self::Variable24 => writer.write_u24(len),
            Self::Variable32 => writer.write_u32(len),
            Self::Callback(_) => Ok(())
            // Self::Unknown => Ok(()),
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
            Self::Callback(_) => 0,
            // Self::Unknown => 0,
        }
    }

}


/// An utility structure for storing ranges of element's ids. It provides way
/// of converting between **element id** (with optional **sub-id**) and 
/// **exposed id**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ElementIdRange {
    pub first: u8,
    pub last: u8,
}

impl ElementIdRange {

    pub const fn new(first: u8, last: u8) -> Self {
        Self { first, last }
    }

    /// Returns the number of slots in this range.
    #[inline]
    pub const fn slots_count(self) -> u8 {
        self.last - self.first + 1
    }

    /// Returns the number of slots that requires a sub-id. These slots are 
    /// starting from the end of the range. For example, if this function
    /// returns 1, this means that the last slot (`.last`), if used, will be
    /// followed by a sub-id.
    /// 
    /// You must given the total number of exposed ids, because the presence
    /// of sub-id depends on how exposed ids can fit in the id range.
    #[inline]
    pub const fn sub_slots_count(self, exposed_count: u16) -> u8 {
        // Calculate the number of excess exposed ids, compared to slots count.
        let excess_count = exposed_count as i32 - self.slots_count() as i32;
        // If the are excess slots, calculate how much additional bytes are 
        // required to represent such number.
        if excess_count > 0 {
            (excess_count / 255 + 1) as u8
        } else {
            0
        }
    }
    
    /// Returns the number of full slots that don't require a sub-id. This
    /// is the opposite of `sub_slots_count`, read its documentation.
    #[inline]
    pub const fn full_slots_count(self, exposed_count: u16) -> u8 {
        self.slots_count() - self.sub_slots_count(exposed_count)
    }

    /// Get the element's id and optional sub-id from the given exposed id
    /// and total count of exposed ids.
    pub fn from_exposed_id(self, exposed_count: u16, exposed_id: u16) -> (u8, Option<u8>) {

        let full_slots = self.full_slots_count(exposed_count);

        if exposed_id < full_slots as u16 {
            // If the exposed id fits in the full slots.
            (self.first + exposed_id as u8, None)
        } else {
            // If the given exposed id require to be put in a sub-slot.
            // First we get how much offset the given exposed id is from the first
            // sub slot (full_slots represent the first sub slot).
            let overflow = exposed_id - full_slots as u16;
            let first_sub_slot = self.first + full_slots;
            // Casts are safe.
            ((first_sub_slot as u16 + overflow / 256) as u8, Some((overflow % 256) as u8))
        }

    }

    /// Get the exposed id from an element, but only return some exposed id if
    /// it fits into 
    pub fn to_exposed_id_checked(self, exposed_count: u16, element_id: u8) -> Option<u16> {
        let raw_exposed_id = element_id - self.first;
        (raw_exposed_id < self.full_slots_count(exposed_count)).then_some(raw_exposed_id as u16)
    }

    /// Get the exposed id from an element id and optionally a sub-id, which 
    /// should be lazily provided with a closure.
    pub fn to_exposed_id(self, exposed_count: u16, element_id: u8, sub_id_getter: impl FnOnce() -> u8) -> u16 {
        
        // This is the raw exposed id, it will be used, with full_slots to determine
        // if a sub-id is needed.
        let exposed_id = element_id - self.first;
        let full_slots = self.full_slots_count(exposed_count);
        
        if exposed_id < full_slots {
            exposed_id as u16
        } else {
            // Calculate of the sub-slot offset within sub-slots.
            let offset_id = exposed_id - full_slots;
            let sub_id = sub_id_getter();
            // Calculate the final exposed id from the sub-id and offset.
            full_slots as u16 + 256 * offset_id as u16 + sub_id as u16
        }
        
    }

}