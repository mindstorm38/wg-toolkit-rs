//! RSA asymmetric encryption filter.

use std::io::{self, Read, Write};

use rsa::{RsaPrivateKey, PublicKeyParts, RsaPublicKey, PublicKey, Oaep};
use rand::rngs::OsRng;
use sha1::Sha1;


pub struct RsaReader<'a, R: Read> {
    /// Underlying reader.
    inner: R,
    /// The RSA key used for reading.
    key: &'a RsaPrivateKey,
    /// This buffer is temporarily used each time the clear block is
    /// exhausted in order to read and then decrypt incoming data.
    cipher_block: Box<[u8]>,
    /// The clear block contains the data after decryption.
    clear_block: Vec<u8>,
    /// The position of the cursor within the clear block.
    clear_pos: usize,
}

impl<'a, R: Read> RsaReader<'a, R> {

    #[inline]
    pub fn new(inner: R, key: &'a RsaPrivateKey) -> Self {
        Self {
            inner,
            key,
            cipher_block: vec![0; key.size()].into_boxed_slice(),
            clear_block: Vec::new(),
            clear_pos: 0,
        }
    }

}

impl<'a, R: Read> Read for RsaReader<'a, R> {

    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {

        if self.clear_pos >= self.clear_block.len() {
            // If the current pos is not a valid index for clear block,
            // read next cipher block and decrypt it.
            // Note: we need to read exactly the block's length.
            match self.inner.read_exact(&mut self.cipher_block) {
                Ok(()) => {
                    self.clear_block = self.key.decrypt(Oaep::new::<Sha1>(), &self.cipher_block).unwrap();
                    self.clear_pos = 0;
                }
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                    // If we can't read a cipher block for now, return 0 and 
                    // hope it will be possible later if the user need it.
                    return Ok(0);
                }
                Err(e) => return Err(e)
            }
        }

        // Here we know that cipher_block is ready.
        let remaining = self.clear_block.len() - self.clear_pos;
        let len = buf.len().min(remaining);
        buf[..len].copy_from_slice(&self.clear_block[self.clear_pos..][..len]);
        self.clear_pos += len;
        Ok(len)

    }

}



pub struct RsaWriter<'a, W: Write> {
    /// Underlying writer.
    inner: W,
    /// The RSA key used for reading.
    key: &'a RsaPublicKey,
    /// The clear block contains the data being written, when it reaches
    /// the capacity `clear_block_cap`, it is encrypted all at once and
    /// then written.
    clear_block: Vec<u8>,
    /// The (fixed) maximum capacity of the clear block, when the clear
    /// block reaches this capacity, it's encrypted and flushed.
    clear_block_cap: usize,
}

impl<'a, W: Write> RsaWriter<'a, W> {

    pub fn new(inner: W, key: &'a RsaPublicKey) -> Self {
        Self {
            inner,
            key,
            clear_block: Vec::new(),
            clear_block_cap: key.size() - 41 - 1,
        }
    }

}


impl<'a, W: Write> Write for RsaWriter<'a, W> {
        
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
        // Note that 'len' will never be greater than block size.
        let len = self.clear_block.len();
        if len > 0 {
            let cipher_block = self.key.encrypt(&mut OsRng, Oaep::new::<Sha1>(), &self.clear_block).unwrap();
            self.inner.write_all(&cipher_block)?;
            self.clear_block.clear();
        }
        Ok(())
    }

}

impl<'a, W: Write> Drop for RsaWriter<'a, W> {
    fn drop(&mut self) {
        let _ = Write::flush(self);
    }
}
