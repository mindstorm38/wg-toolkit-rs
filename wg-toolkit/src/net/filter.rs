//! Multiple IO filters (RSA, Blowfish, plain) that may be used in element
//! codecs.

use std::io::{self, Read, Write};

pub mod rsa;
pub mod blowfish;


/// A trait to implement on a filter that support block reads.
pub trait BlockReadFilter {

    /// Return the input block' size that triggers call to `filter_read` 
    /// method after the internal buffer is filled with data to filter.
    fn block_size(&self) -> usize;
    
    /// Filter the given block and produces new data derived from it
    /// when read data size reaches the block size returned from 
    /// `block_size` method.
    fn filter_read(&mut self, input: &[u8], output: &mut Vec<u8>);

}

/// A trait to implement on a filter that support block writes.
pub trait BlockWriteFilter {

    /// Return the input block' size that triggers call to `filter_write` 
    /// method after the internal buffer is filled with data to filter.
    fn block_size(&self) -> usize;

    /// Filter the given block and produces new data derived from it
    /// when written data size reaches the block size returned from 
    /// `block_size` method **or** if flushed.
    /// 
    /// *Note that* the given block might be smaller than given block
    /// size because of an early flush. However the block will never
    /// be larger than the block size.
    fn filter_write(&mut self, input: &[u8], output: &mut Vec<u8>);

}

impl<'a, F: BlockReadFilter> BlockReadFilter for &'a mut F {

    #[inline]
    fn block_size(&self) -> usize {
        BlockReadFilter::block_size(*self)
    }

    #[inline]
    fn filter_read(&mut self, input: &[u8], output: &mut Vec<u8>) {
        BlockReadFilter::filter_read(*self, input, output)
    }

}

impl<'a, F: BlockWriteFilter> BlockWriteFilter for &'a mut F {
    
    #[inline]
    fn block_size(&self) -> usize {
        BlockWriteFilter::block_size(*self)
    }

    #[inline]
    fn filter_write(&mut self, input: &[u8], output: &mut Vec<u8>) {
        BlockWriteFilter::filter_write(*self, input, output)
    }

}

/// A block reader filter.
/// 
/// When reading this block reader, the inner reader is read
/// by a whole [`BlockReadFilter::block_size`], then its data
/// is processed through the [`BlockReadFilter`], the returned
/// block is then used for actual reading. When no more processed
/// data is available, this process repeats.
pub struct BlockReader<R: Read, F: BlockReadFilter> {
    inner: R,
    filter: F,
    cipher_block: Vec<u8>,
    clear_block: Vec<u8>,
    pos: usize
}

impl<R: Read, F: BlockReadFilter> BlockReader<R, F> {

    pub fn new(inner: R, filter: F) -> Self {
        Self {
            cipher_block: vec![0; filter.block_size()],
            clear_block: Vec::new(),
            pos: 0,
            inner,
            filter,
        }
    }

}

impl<R: Read, F: BlockReadFilter> Read for BlockReader<R, F> {

    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {

        if self.pos >= self.clear_block.len() {
            // If the current pos is not a valid index for clear block,
            // read next cipher block and decrypt it.
            // Note: we need to read exactly the block's length.
            match self.inner.read_exact(&mut self.cipher_block[..]) {
                Ok(()) => {
                    self.clear_block.clear();
                    self.filter.filter_read(&self.cipher_block, &mut self.clear_block);
                    self.pos = 0;
                }
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                    // If we can't read a cipher block for now, return 0 and hope it will
                    // be possible later if the user need it.
                    return Ok(0);
                }
                Err(e) => return Err(e)
            }
        }

        // Here we know that cipher_block is ready.
        let remaining = self.clear_block.len() - self.pos;
        let len = buf.len().min(remaining);
        buf[..len].copy_from_slice(&self.clear_block[self.pos..][..len]);
        self.pos += len;
        Ok(len)

    }

}

/// A block writer filter.
/// 
/// This type of writer write incoming bytes to an internal buffer,
/// when this buffer reaches the [`BlockWriteFilter::block_size`] or
/// if the block writer is flushed, the buffer is processed through
/// the [`BlockWriteFilter`] and the returned data is then written
/// to the inner writer.
pub struct BlockWriter<W: Write, F: BlockWriteFilter> {
    inner: W,
    filter: F,
    clear_block: Vec<u8>,
    clear_block_cap: usize,
    cipher_buf: Vec<u8>,
}

impl<W: Write, F: BlockWriteFilter> BlockWriter<W, F> {

    pub fn new(inner: W, filter: F) -> Self {
        Self {
            clear_block: Vec::new(),
            clear_block_cap: filter.block_size(),
            cipher_buf: Vec::new(),
            inner,
            filter,
        }
    }

}

impl<W: Write, F: BlockWriteFilter> Write for BlockWriter<W, F> {
    
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let remaining = self.clear_block_cap - self.clear_block.len();
        let len = buf.len().min(remaining);
        self.clear_block.extend_from_slice(&buf[..len]);
        if remaining == len {
            // If the length ultimately written is less than expected,
            // flush to cleanup 'clear_block'.
            self.flush()?;
        }
        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        if !self.clear_block.is_empty() {
            self.cipher_buf.clear();
            self.filter.filter_write(&self.clear_block[..], &mut self.cipher_buf);
            self.inner.write_all(&self.cipher_buf[..])?;
            self.clear_block.clear();
        }
        Ok(())
    }

}

impl<W: Write, F: BlockWriteFilter> Drop for BlockWriter<W, F> {
    fn drop(&mut self) {
        let _ = Write::flush(self);
    }
}
