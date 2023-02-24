//! Blowfish symmetric encryption filter.

use std::io::{self, Read, Write};
use std::ops::{BitXorAssign, BitXor};

use blowfish::cipher::{BlockEncrypt, BlockDecrypt, Block};
use blowfish::Blowfish;


/// A reader that filters the underlying reader through a blowfish 
/// symmetric encryption.
pub struct BlowfishReader<'a, R: Read> {
    /// Underlying reader.
    inner: R,
    /// Blowfish key.
    blowfish: &'a Blowfish,
    /// The block with clear data that is being read.
    cur_block: BlowfishBlock,
    /// The cursor within the clear block being read.
    cur_pos: usize,
    /// The last block saved, it's used for XOR, initially zero.
    last_block: BlowfishBlock,
}

impl<'a, R: Read> BlowfishReader<'a, R> {

    #[inline]
    pub fn new(inner: R, blowfish: &'a Blowfish) -> Self {
        Self {
            inner,
            blowfish,
            cur_block: BlowfishBlock::new(),
            cur_pos: BlowfishBlock::SIZE,
            last_block: BlowfishBlock::new(),
        }
    }

}

impl<'a, R: Read> Read for BlowfishReader<'a, R> {

    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        
        if buf.is_empty() {
            return Ok(0)
        }

        debug_assert!(self.cur_pos <= BlowfishBlock::SIZE);

        // Position 'BLOCK_SIZE' inform us that we should read and decode a new block.
        if self.cur_pos == BlowfishBlock::SIZE {

            // The a whole block before decrypting it.
            match self.inner.read_exact(self.cur_block.slice_mut()) {
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(0),
                Err(e) => return Err(e),
                _ => ()
            }

            self.blowfish.decrypt_block(Block::<Blowfish>::from_mut_slice(self.cur_block.slice_mut()));
            self.cur_pos = 0;

            // XOR the current block with the last block.
            self.cur_block ^= self.last_block;
            self.last_block = self.cur_block;

        }

        // Actual length we can read.
        let len = buf.len().min(BlowfishBlock::SIZE - self.cur_pos);
        buf[..len].copy_from_slice(&self.cur_block.slice()[self.cur_pos..]);
        self.cur_pos += len;

        Ok(len)

    }

}


/// A writer that filters incoming writes through a symmetric blowfish
/// encryption, when a whole block has been written the block is encrypted
/// and written to the underlying writer.
pub struct BlowfishWriter<'a, W: Write> {
    /// Underlying writer.
    inner: W,
    /// Blowfish key.
    blowfish: &'a Blowfish,
    /// The temporary block buffer. The block is reset to 0 before any
    /// new write, this allows us to optimize out the padding as it 
    /// already is.
    tmp_block: BlowfishBlock,
    /// Cursor position in the temporary block.
    tmp_pos: usize,
    /// This block is the one actually written to the underlying after
    /// XOR-ing the temporary block into it.
    xor_block: BlowfishBlock,
}

impl<'a, W: Write> BlowfishWriter<'a, W> {

    #[inline]
    pub fn new(inner: W, blowfish: &'a Blowfish) -> Self {
        Self { 
            inner, 
            blowfish,
            tmp_block: BlowfishBlock::new(),
            tmp_pos: 0,
            xor_block: BlowfishBlock::new(),
        }
    }

}

impl<'a, W: Write> Write for BlowfishWriter<'a, W> {

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {

        let len = buf.len().min(BlowfishBlock::SIZE - self.tmp_pos);
        self.tmp_block.slice_mut()[self.tmp_pos..][..len].copy_from_slice(&buf[..len]);
        self.tmp_pos += len;

        debug_assert!(self.tmp_pos <= BlowfishBlock::SIZE);

        // Flush when the block is filled.
        if self.tmp_pos == BlowfishBlock::SIZE {
            self.flush()?;
        }

        Ok(len)

    }

    fn flush(&mut self) -> io::Result<()> {

        if self.tmp_pos == 0 {
            return Ok(())
        }

        // At this point the xor block contains the last clear block that was flushed.
        self.xor_block ^= self.tmp_block;

        // Save the temporary block that contains the current clear block.
        let saved = self.tmp_block;

        // Then we encrypt from the XOR block to the temporary one.
        let src = Block::<Blowfish>::from_slice(self.xor_block.slice());
        let dst = Block::<Blowfish>::from_mut_slice(self.tmp_block.slice_mut());
        self.blowfish.encrypt_block_b2b(src, dst);

        // Write the temporary block that has been encrypted.
        self.inner.write_all(self.tmp_block.slice())?;

        // We write back the current clear block to the XOR for future usage.
        self.xor_block = saved;

        // Zero-out the temporary block and position.
        self.tmp_block.clear();
        self.tmp_pos = 0;

        Ok(())

    }

}

impl<'a, W: Write> Drop for BlowfishWriter<'a, W> {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}


/// Internal union type for a single blowfish block.
/// 
/// This ensures alignment and optimization of moves/xors
/// on all bytes at once without misalignment penalties.
/// 
/// The union is globally safe because all bit patterns
/// are valid, so unsafe can be abstracted from public.
#[repr(C)]
#[derive(Clone, Copy)]
union BlowfishBlock {
    full: u64,
    slice: [u8; 8],
}

impl BlowfishBlock {

    const SIZE: usize = 8;

    #[inline]
    fn new() -> Self {
        Self { full: 0 }
    }

    #[inline]
    fn clear(&mut self) {
        self.full = 0;
    }

    #[inline]
    fn full(&self) -> &u64 {
        unsafe { &self.full }
    }
    
    #[inline]
    fn full_mut(&mut self) -> &mut u64 {
       unsafe { &mut self.full }
    }

    #[inline]
    fn slice(&self) -> &[u8; 8] {
        unsafe { &self.slice }
    }

    #[inline]
    fn slice_mut(&mut self) -> &mut [u8; 8] {
        unsafe { &mut self.slice }
    }

}

impl BitXorAssign for BlowfishBlock {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self.full_mut() ^= *rhs.full();
    }
}

impl BitXor for BlowfishBlock {
    type Output = BlowfishBlock;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}




// /// A read/write block filter.
// pub struct BlowfishFilter<'a>(&'a Blowfish);

// impl<'a> BlowfishFilter<'a> {

//     /// Create a new blowfish filter for the given  
//     #[inline]
//     pub fn new(bf: &'a Blowfish) -> Self {
//         Self(bf)
//     }

// }

// impl BlockReadFilter for BlowfishFilter<'_> {

//     #[inline]
//     fn block_size(&self) -> usize {
//         Blowfish::<BE>::block_size()
//     }

//     fn filter_read(&mut self, input: &[u8], output: &mut Vec<u8>) {
//         let block_size = BlockReadFilter::block_size(self);
//         output.extend(std::iter::repeat(0).take(block_size));
//         let in_block = Block::<Blowfish>::from_slice(&input[..block_size]);
//         let out_block = Block::<Blowfish>::from_mut_slice(&mut output[..block_size]);
//         self.0.decrypt_block_b2b(in_block, out_block);
//     }

// }

// impl BlockWriteFilter for BlowfishFilter<'_> {

//     #[inline]
//     fn block_size(&self) -> usize {
//         Blowfish::<BE>::block_size()
//     }

//     #[inline]
//     fn block_padding(&self) -> Option<u8> {
//         Some(0)
//     }

//     fn filter_write(&mut self, input: &[u8], output: &mut Vec<u8>) {

//         println!("¤¤¤¤ {}", crate::util::get_hex_str_from(input, 100));

//         let block_size = BlockReadFilter::block_size(self);
//         output.extend(std::iter::repeat(0).take(block_size));
//         let in_block = Block::<Blowfish>::from_slice(input);
//         let out_block = Block::<Blowfish>::from_mut_slice(&mut output[..block_size]);
//         self.0.encrypt_block_b2b(in_block, out_block);
        
//         println!("==== {}", crate::util::get_hex_str_from(&output, 100));

//     }

// }
