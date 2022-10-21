//! Read and write extensions specific to WG.

use std::io::{self, Read, Write, Cursor, Seek, SeekFrom};

use byteorder::{ReadBytesExt, WriteBytesExt, LE};


/// An extension to the `Read` trait specifically used to decode WG formats.
pub trait WgReadExt: Read {

    #[inline]
    fn read_u8(&mut self) -> io::Result<u8> {
        ReadBytesExt::read_u8(self)
    }

    #[inline]
    fn read_i8(&mut self) -> io::Result<i8> {
        ReadBytesExt::read_i8(self)
    }

    #[inline]
    fn skip<const N: usize>(&mut self) -> io::Result<()> {
        let mut buf = [0; N];
        self.read_exact(&mut buf)?;
        Ok(())
    }

    #[inline]
    fn read_u16(&mut self) -> io::Result<u16> {
        ReadBytesExt::read_u16::<LE>(self)
    }

    #[inline]
    fn read_i16(&mut self) -> io::Result<i16> {
        ReadBytesExt::read_i16::<LE>(self)
    }

    #[inline]
    fn read_u32(&mut self) -> io::Result<u32> {
        ReadBytesExt::read_u32::<LE>(self)
    }

    #[inline]
    fn read_i32(&mut self) -> io::Result<i32> {
        ReadBytesExt::read_i32::<LE>(self)
    }

    #[inline]
    fn read_u64(&mut self) -> io::Result<u64> {
        ReadBytesExt::read_u64::<LE>(self)
    }

    #[inline]
    fn read_i64(&mut self) -> io::Result<i64> {
        ReadBytesExt::read_i64::<LE>(self)
    }

    #[inline]
    fn read_f32(&mut self) -> io::Result<f32> {
        ReadBytesExt::read_f32::<LE>(self)
    }

    /// Check that the next `N` bytes are the exact same as the on given.
    #[inline]
    fn check_exact<const N: usize>(&mut self, bytes: &[u8; N]) -> io::Result<bool> {
        let mut buf = [0; N];
        self.read_exact(&mut buf[..])?;
        Ok(&buf == bytes)
    }

    /// Directly read a raw buffer of the given length.
    #[inline]
    fn read_vec(&mut self, len: usize) -> io::Result<Vec<u8>> {
        // TODO: Maybe use a better uninit approach in the future.
        let mut buf = vec![0; len];
        self.read_exact(&mut buf[..])?;
        Ok(buf)
    }

    #[inline]
    fn read_string(&mut self, len: usize) -> io::Result<String> {
        String::from_utf8(self.read_vec(len)?)
            .map_err(|_| io::ErrorKind::InvalidData.into())
    }

    /// Read a null-terminated string of a fixed length, trailing zeros
    /// are ignored and if no zero is encountered, an invalid data error
    /// is returned.
    fn read_cstring_fixed(&mut self, len: usize) -> io::Result<String> {
        let mut buf = self.read_vec(len)?;
        let pos = buf.iter().position(|&o| o == 0)
            .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?;
        buf.truncate(pos); // Truncate trailing zeros.
        String::from_utf8(buf).map_err(|_| io::ErrorKind::InvalidData.into())
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

pub trait WgReadSeekExt: Read + Seek {

    /// Special null-terminated string reading function that 
    /// uses seekability of the underlying stream.
    fn read_cstring_fast(&mut self) -> io::Result<String> {

        let mut cursor = self.stream_position()?;
        let mut buf = [0; 32];
        let mut string = Vec::new();

        'e: loop {

            let mut len = match self.read(&mut buf) {
                Ok(len) => len,
                Err(e) if e.kind() != io::ErrorKind::Interrupted => return Err(e.into()),
                _ => continue
            };

            for &c in &buf[..len] {
                cursor += 1;
                len -= 1;
                if c == 0 {
                    if len != 0 { // Only seek if bytes remains.
                        self.seek(SeekFrom::Start(cursor))?;
                    }
                    break 'e;
                }
                string.push(c);
            }

        }

        String::from_utf8(string).map_err(|_| io::ErrorKind::InvalidData.into())

    }

}

pub trait WgWriteExt: Write {

    #[inline]
    fn write_u8(&mut self, n: u8) -> io::Result<()> {
        WriteBytesExt::write_u8(self, n)
    }

    #[inline]
    fn write_i8(&mut self, n: i8) -> io::Result<()> {
        WriteBytesExt::write_i8(self, n)
    }

    #[inline]
    fn write_u16(&mut self, n: u16) -> io::Result<()> {
        WriteBytesExt::write_u16::<LE>(self, n)
    }

    #[inline]
    fn write_i16(&mut self, n: i16) -> io::Result<()> {
        WriteBytesExt::write_i16::<LE>(self, n)
    }

    #[inline]
    fn write_u32(&mut self, n: u32) -> io::Result<()> {
        WriteBytesExt::write_u32::<LE>(self, n)
    }

    #[inline]
    fn write_i32(&mut self, n: i32) -> io::Result<()> {
        WriteBytesExt::write_i32::<LE>(self, n)
    }

    #[inline]
    fn write_u64(&mut self, n: u64) -> io::Result<()> {
        WriteBytesExt::write_u64::<LE>(self, n)
    }

    #[inline]
    fn write_i64(&mut self, n: i64) -> io::Result<()> {
        WriteBytesExt::write_i64::<LE>(self, n)
    }

    #[inline]
    fn write_f32(&mut self, n: f32) -> io::Result<()> {
        WriteBytesExt::write_f32::<LE>(self, n)
    }

    #[inline]
    fn write_string<S: AsRef<str>>(&mut self, s: S) -> io::Result<()> {
        self.write_all(s.as_ref().as_bytes())
    }

    #[inline]
    fn write_cstring<S: AsRef<str>>(&mut self, s: S) -> io::Result<()> {
        self.write_string(s)?;
        self.write_u8(0)
    }

    /// Write the size header for a single structure. To write the header of
    /// a vector, see `write_vector_head`.
    fn write_single_head(&mut self, n: usize) -> io::Result<()> {
        self.write_u32(n as u32)
    }

    /// Write header for vector of structure.
    fn write_vector_head(&mut self, size: usize, count: usize) -> io::Result<()> {
        self.write_u32(size as u32)?;
        self.write_u32(count as u32)
    }

    /// Write a vector of structure. Items give in the iterator are converted 
    /// through the given function. This function take the 
    fn write_vector<T, I, F>(&mut self, vec: &[T], size: usize, mut func: F) -> io::Result<()>
    where
        F: FnMut(&T, &mut Self),
    {
        self.write_vector_head(size, vec.len())?;
        for elt in vec {
            (func)(elt, self);
        }
        Ok(())
    }

}

impl<R: Read> WgReadExt for R {}
impl<R: Read + Seek> WgReadSeekExt for R {}
impl<W: Write> WgWriteExt for W {}
