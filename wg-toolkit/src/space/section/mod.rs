//! Compiled space sections structures definitions.

use std::io::{self, Read, Seek};


mod bwtb;
mod bwst;
mod bwal;
mod bwcs;
mod bwsg;
mod bwt2;

pub use bwtb::*;
pub use bwst::*;
pub use bwal::*;
pub use bwcs::*;
pub use bwsg::*;
pub use bwt2::*;


/// Alias for 4-bytes array, which is used to identify sections in a compiled space.
pub type SectionId = [u8; 4];


/// Common trait for section in compiled space binaries.
pub trait Section: Sized {

    const ID: &'static SectionId;

    fn decode<R: Read + Seek>(read: &mut R) -> io::Result<Self>;

}
