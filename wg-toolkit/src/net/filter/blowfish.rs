//! Blowfish symmetric encryption filter.

use blowfish::cipher::{BlockEncrypt, BlockDecrypt, BlockSizeUser, Block};
use blowfish::Blowfish;

use byteorder::BE;

use super::{BlockReadFilter, BlockWriteFilter};


/// A read/write block filter.
pub struct BlowfishFilter<'a>(&'a Blowfish);

impl<'a> BlowfishFilter<'a> {

    /// Create a new blowfish filter for the given  
    #[inline]
    pub fn new(bf: &'a Blowfish) -> Self {
        Self(bf)
    }

}

impl BlockReadFilter for BlowfishFilter<'_> {

    #[inline]
    fn block_size(&self) -> usize {
        Blowfish::<BE>::block_size()
    }

    fn filter_read(&mut self, input: &[u8], output: &mut Vec<u8>) {
        let block_size = BlockReadFilter::block_size(self);
        let in_block = Block::<Blowfish>::from_slice(&input[..block_size]);
        output.reserve_exact(block_size);
        let out_block = Block::<Blowfish>::from_mut_slice(&mut output[..block_size]);
        self.0.decrypt_block_b2b(in_block, out_block);
    }

}

impl BlockWriteFilter for BlowfishFilter<'_> {

    #[inline]
    fn block_size(&self) -> usize {
        Blowfish::<BE>::block_size()
    }

    fn filter_write(&mut self, input: &[u8], output: &mut Vec<u8>) {
        let block_size = BlockWriteFilter::block_size(self);
        let in_block = Block::<Blowfish>::from_slice(&input[..block_size]);
        output.reserve_exact(block_size);
        let out_block = Block::<Blowfish>::from_mut_slice(&mut output[..block_size]);
        self.0.encrypt_block_b2b(in_block, out_block);
    }
}
