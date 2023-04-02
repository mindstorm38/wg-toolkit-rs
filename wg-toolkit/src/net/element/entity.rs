//! Base traits and functionalities for entity codecs.

use std::io::{self, Write, Read};

use super::{ElementLength, ElementIdRange};


/// An entity method definition for a specific entity structure.
pub trait ExposedMethod: Sized {

    /// Return the index of the method.
    fn index(&self) -> u16;

    /// Return the length for a given method index.
    fn len(index: u16) -> ElementLength;

    /// Encode the method with the given writer.
    fn encode(&self, write: &mut impl Write) -> io::Result<()>;

    /// Decode the method with the given reader, length and for a specific index.
    fn decode(read: &mut impl Read, len: usize, index: u16) -> io::Result<Self>;

}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MethodIdRange {
    pub first: u8,
    pub last: u8,
}

impl MethodIdRange {

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

    /// Returns true if the given exposed id requires to be encoded in a 
    /// sub slot regarding the total count of exposed ids (this means that
    /// `exposed_id < exposed_count`).
    pub fn needs_sub_id(self, exposed_count: u16, exposed_id: u16) -> bool {
        debug_assert!(exposed_id < exposed_count);
        exposed_id >= self.full_slots_count(exposed_count) as u16
    }

    /// Get the element's id and optional sub-id from the given exposed id
    /// and total count of exposed ids.
    pub fn from_exposed_id(self, exposed_count: u16, exposed_id: u16) -> (u8, Option<u8>) {

        let full_slots = self.full_slots_count(exposed_count);

        if exposed_id < full_slots as u16 {
            // If the exposed id fits in the full slots.
            (self.first + exposed_id as u8, None)
        } else {
            // If the given exposed id requires the usage of sub-slots.
            // First we get how much offset the given exposed id is from the first
            // sub slot (full_slots represent the first sub slot).
            let overflow = exposed_id - full_slots as u16;
            let first_sub_slot = self.first + full_slots;
            // Casts are safe.
            ((first_sub_slot as u16 + overflow / 256) as u8, Some((overflow % 256) as u8))
        }

    }

    pub fn to_exposed_id(self, exposed_count: u16, elt_id: u8, sub_id: impl FnOnce() -> u16) -> u16 {
        
        let full_slots = self.full_slots_count(exposed_count);
        todo!()
        
    }

}
