//! Compiled space sections structures definitions.

use byteorder::{ReadBytesExt, LittleEndian};
use std::io::{self, Cursor, Read, Seek};


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


/// An extension to the `Read` trait specifically used to decode compiled space's sections.
pub(self) trait ReadSectionExt: Read {

    fn skip<const N: usize>(&mut self) -> io::Result<()> {
        let mut buf = [0; N];
        self.read_exact(&mut buf)?;
        Ok(())
    }

    fn read_u16(&mut self) -> io::Result<u16> {
        ReadBytesExt::read_u16::<LittleEndian>(self)
    }

    fn read_i16(&mut self) -> io::Result<i16> {
        ReadBytesExt::read_i16::<LittleEndian>(self)
    }

    fn read_u32(&mut self) -> io::Result<u32> {
        ReadBytesExt::read_u32::<LittleEndian>(self)
    }

    fn read_i32(&mut self) -> io::Result<i32> {
        ReadBytesExt::read_i32::<LittleEndian>(self)
    }

    fn read_u64(&mut self) -> io::Result<u64> {
        ReadBytesExt::read_u64::<LittleEndian>(self)
    }

    fn read_i64(&mut self) -> io::Result<i64> {
        ReadBytesExt::read_i64::<LittleEndian>(self)
    }

    fn read_f32(&mut self) -> io::Result<f32> {
        ReadBytesExt::read_f32::<LittleEndian>(self)
    }

    /// Read the size header for a single structure. To read the header of
    /// a vector, see `read_vector_head`.
    fn read_single_head(&mut self) -> io::Result<usize> {
        Ok(self.read_u32()? as usize)
    }

    /// Read header for vector of structure, returns `(size, count)` with the
    /// number of structure of the given size, total size is `size * count`.
    fn read_vector_head(&mut self) -> io::Result<(usize, usize)> {
        let sec_size = self.read_u32()? as usize;
        let sec_count = self.read_u32()? as usize;
        Ok((sec_size, sec_count))
    }

    /// Read a full vector of structure, use a function to convert each structures'
    /// bytes to an object, returns a vector with all vector's objects.
    fn read_vector<F, T>(&mut self, mut func: F) -> io::Result<Vec<T>>
    where
        F: FnMut(&mut Cursor<&Vec<u8>>) -> io::Result<T>
    {

        let (sec_size, sec_count) = self.read_vector_head()?;

        let mut buf = Vec::with_capacity(sec_size);
        buf.resize(sec_size, 0);

        let mut data = Vec::with_capacity(sec_count);
        for _ in 0..sec_count {
            self.read_exact(&mut buf[..])?;
            data.push((func)(&mut Cursor::new(&buf))?);
        }

        Ok(data)

    }

}

impl<R: Read> ReadSectionExt for R {}
