//! Base traits and functionalities for entity codecs.

use std::io::{self, Write, Read};

use super::ElementLength;


/// An entity method definition for a specific entity structure.
pub trait ExposedMethod: Sized {

    /// Return the total number of exposed methods for this type of methods.
    fn count() -> u16;

    /// Return the index of the method.
    fn index(&self) -> u16;

    /// Return the length for a given method index.
    fn len(index: u16) -> ElementLength;

    /// Encode the method with the given writer.
    fn encode(&self, write: &mut impl Write) -> io::Result<()>;

    /// Decode the method with the given reader, length and for a specific index.
    fn decode(read: &mut impl Read, len: usize, index: u16) -> io::Result<Self>;

}
