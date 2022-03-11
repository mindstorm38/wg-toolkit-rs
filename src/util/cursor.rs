use std::io::{Read, Seek, SeekFrom, Write};


/// A cursor of a slice of another stream (`Read` and/or `Write` implementor).
pub struct SubCursor<T> {
    /// The inner stream.
    inner: T,
    /// Begin of the slice, in position from the start of inner stream.
    begin: u64,
    /// End of the slice, in position from the start of inner stream.
    end: u64,
    /// The position from the start of inner stream, should never be less
    /// than `begin` or greater than `end`.
    pos: u64
}

impl<T> SubCursor<T> {

    pub fn new(inner: T, begin: u64, end: u64) -> Self {

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

    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let remaining = self.remaining();
        if remaining == 0 {
            Err(std::io::ErrorKind::UnexpectedEof.into())
        } else {
            let exp_len = buf.len().min(sat_u64_to_usize(remaining));
            let len = self.inner.read(&mut buf[..exp_len])?;
            self.pos += len as u64;
            Ok(len)
        }
    }

}

impl<T: Write> Write for SubCursor<T> {

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
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
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }

}

impl<T: Seek> Seek for SubCursor<T> {

    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let abs_pos = match pos {
            SeekFrom::Start(pos) => (self.begin + pos).min(self.end),
            SeekFrom::End(pos) => {
                if pos > 0 {
                    self.end
                } else {
                    let abs_pos = self.end as i64 - pos;
                    if abs_pos < self.begin as i64 {
                        return Err(std::io::ErrorKind::InvalidInput.into());
                    }
                    abs_pos as u64
                }
            },
            SeekFrom::Current(pos) => {
                let abs_pos = self.pos as i64 + pos;
                if abs_pos < self.begin as i64 {
                    return Err(std::io::ErrorKind::InvalidInput.into());
                }
                (abs_pos as u64).min(self.end)
            }
        };
        self.pos = self.inner.seek(SeekFrom::Start(abs_pos))?;
        Ok(self.pos())
    }

    fn rewind(&mut self) -> std::io::Result<()> {
        self.pos = self.begin;
        self.inner.seek(SeekFrom::Start(self.pos))?;
        Ok(())
    }

    #[inline]
    fn stream_position(&mut self) -> std::io::Result<u64> {
        Ok(self.pos())
    }

}


/// A saturated cast from `u64` to `usize`, used when on 32bits (or less)
/// systems to avoid overflowing casts.
#[inline]
fn sat_u64_to_usize(n: u64) -> usize {
    n.min(usize::MAX as u64) as usize
}
