use std::io::{self, Read, Seek, SeekFrom, Write};


/// A cursor of a slice of another stream (`Read` and/or `Write` implementor).
/// The only constraint for `T` is to be seekable.
pub struct SubCursor<T> {
    /// The inner stream.
    inner: T,
    /// Begin of the slice, in position from the start of inner stream, inclusive.
    begin: u64,
    /// End of the slice, in position from the start of inner stream, exclusive.
    end: u64,
    /// The position from the start of inner stream, should never be less
    /// than `begin` or greater than `end`.
    pos: u64
}

impl<T: Seek> SubCursor<T> {
    pub fn new(mut inner: T, begin: u64, end: u64) -> io::Result<Self> {
        inner.seek(SeekFrom::Start(begin))?;
        Ok(Self::new_unchecked(inner, begin, end))
    }
}

impl<T> SubCursor<T> {

    pub fn new_unchecked(inner: T, begin: u64, end: u64) -> Self {
        Self {
            inner,
            begin,
            end,
            pos: begin
        }
    }

    /// Return the length of the cursor's slice.
    #[inline]
    pub fn len(&self) -> u64 {
        self.end - self.begin
    }

    /// Return the position of the cursor **within the slice**.
    #[inline]
    pub fn pos(&self) -> u64 {
        self.pos - self.begin
    }

    /// Return the external position of the cursor **within the inner reader**.
    #[inline]
    pub fn extern_pos(&self) -> u64 {
        self.pos
    }

    /// Return the remaining length in the cursor.
    #[inline]
    pub fn remaining(&self) -> u64 {
        self.end - self.pos
    }

}

impl<T: Read> Read for SubCursor<T> {

    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let remaining = self.remaining();
        if remaining == 0 {
            Err(io::ErrorKind::UnexpectedEof.into())
        } else {
            let exp_len = buf.len().min(sat_u64_to_usize(remaining));
            let len = self.inner.read(&mut buf[..exp_len])?;
            self.pos += len as u64;
            Ok(len)
        }
    }

}

impl<T: Write> Write for SubCursor<T> {

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let remaining = self.remaining();
        if remaining == 0 {
            Ok(0)
        } else {
            let len = self.inner.write(&buf[..sat_u64_to_usize(remaining)])?;
            self.pos += len as u64;
            Ok(len)
        }
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }

}

impl<T: Seek> Seek for SubCursor<T> {

    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let abs_pos = match pos {
            SeekFrom::Start(pos) => (self.begin + pos).min(self.end),
            SeekFrom::End(pos) => {
                if pos > 0 {
                    self.end
                } else {
                    let abs_pos = self.end as i64 - pos;
                    if abs_pos < self.begin as i64 {
                        return Err(io::ErrorKind::InvalidInput.into());
                    }
                    abs_pos as u64
                }
            },
            SeekFrom::Current(pos) => {
                let abs_pos = self.pos as i64 + pos;
                if abs_pos < self.begin as i64 {
                    return Err(io::ErrorKind::InvalidInput.into());
                }
                (abs_pos as u64).min(self.end)
            }
        };
        self.pos = self.inner.seek(SeekFrom::Start(abs_pos))?;
        Ok(self.pos())
    }

    fn rewind(&mut self) -> io::Result<()> {
        self.pos = self.begin;
        self.inner.seek(SeekFrom::Start(self.pos))?;
        Ok(())
    }

    #[inline]
    fn stream_position(&mut self) -> io::Result<u64> {
        Ok(self.pos())
    }

}


/// A saturated cast from `u64` to `usize`, used when on 32bits (or less)
/// systems to avoid overflowing casts.
#[inline]
fn sat_u64_to_usize(n: u64) -> usize {
    n.min(usize::MAX as u64) as usize
}


#[cfg(test)]
mod tests {

    use super::*;

    use std::io::Cursor;
    use byteorder::ReadBytesExt;

    #[test]
    fn cursor_seek() {

        let data = [0u8, 0, 0, 0, 0, 0];
        let mut cursor = Cursor::new(&data[..]);

        assert_eq!(cursor.seek(SeekFrom::End(0)).unwrap(), 6);
        assert_eq!(cursor.seek(SeekFrom::End(-1)).unwrap(), 5);
        assert_eq!(cursor.seek(SeekFrom::Current(-1)).unwrap(), 4);
        assert_eq!(cursor.seek(SeekFrom::Start(0)).unwrap(), 0);

        let mut cursor = SubCursor::new(cursor, 1, 4).unwrap();

        assert_eq!(cursor.seek(SeekFrom::Start(0)).unwrap(), 0);
        assert_eq!(cursor.seek(SeekFrom::End(0)).unwrap(), 3);
        assert_eq!(cursor.seek(SeekFrom::Current(-1)).unwrap(), 2);
        assert_eq!(cursor.seek(SeekFrom::Current(-1)).unwrap(), 1);
        assert_eq!(cursor.seek(SeekFrom::Current(-1)).unwrap(), 0);

        cursor.seek(SeekFrom::Start(0)).unwrap();
        cursor.seek(SeekFrom::Current(1)).unwrap();
        cursor.seek(SeekFrom::Current(-1)).unwrap();
        cursor.seek(SeekFrom::Current(-1)).unwrap_err();
        let plus_0 = cursor.seek(SeekFrom::End(0)).unwrap();
        let plus_1 = cursor.seek(SeekFrom::End(1)).unwrap();
        assert_eq!(plus_0, plus_1); // Overflow is cancelled

    }

    #[test]
    fn cursor_read() {

        let data = [2u8, 3, 5, 7, 11, 13];
        let cursor = Cursor::new(&data[..]);
        let mut cursor = SubCursor::new(cursor, 1, 4).unwrap();

        assert_eq!(cursor.read_u8().unwrap(), 3);
        assert_eq!(cursor.read_u8().unwrap(), 5);
        assert_eq!(cursor.read_u8().unwrap(), 7);
        cursor.read_u8().unwrap_err();

        cursor.seek(SeekFrom::Start(1)).unwrap();
        assert_eq!(cursor.read_u8().unwrap(), 5);

    }

}
