use std::io::{self, Read};


pub struct SubCursor<T> {
    inner: T,
    len: usize
}

impl<T> SubCursor<T> {

    pub fn new(inner: T, len: usize) -> Self {
        Self {
            inner,
            len
        }
    }

    #[inline]
    pub fn remaining(&self) -> usize {
        self.len
    }

}

impl<T: Read> Read for SubCursor<T> {

    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {

        if self.len == 0 {
            return io::Result::Err(io::Error::from(io::ErrorKind::UnexpectedEof));
        }

        let buf = if self.len < buf.len() {
            &mut buf[..self.len]
        } else {
            buf
        };

        let len = self.inner.read(buf)?;
        self.len -= len;
        Ok(len)

    }

}
