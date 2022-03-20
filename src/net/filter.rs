//! Multiple IO filters (RSA, Blowfish, plain).

use std::io::{self, Read, Write};
use rand::rngs::OsRng;

use rsa::{RsaPrivateKey, PublicKeyParts, PaddingScheme, RsaPublicKey, PublicKey};
use sha1::Sha1;


/// A filter reader for RSA-encrypted data (its length must be
/// a multiple of the key's block size).
pub struct RsaReader<'a, R> {
    inner: R,
    key: &'a RsaPrivateKey,
    cipher_block: Vec<u8>,
    clear_block: Vec<u8>,
    pos: usize
}

impl<'a, R> RsaReader<'a, R> {
    pub fn new(inner: R, key: &'a RsaPrivateKey) -> Self {
        Self {
            inner,
            cipher_block: vec![0; key.size()],
            clear_block: Vec::new(),
            pos: 0,
            key,
        }
    }
}

impl<'a, R: Read> Read for RsaReader<'a, R> {

    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {

        if self.pos >= self.clear_block.len() {
            // If the current pos is not a valid index for clear block,
            // get next cipher block and
            match self.inner.read_exact(&mut self.cipher_block[..]) {
                Ok(()) => {
                    let scheme = PaddingScheme::new_oaep::<Sha1>();
                    self.clear_block = self.key.decrypt(scheme, &self.cipher_block[..]).unwrap();
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


/// A filter write for clear data to RSA-encrypted blocks.
/// Note that each flush call with non-empty internal buffer
/// will write a full RSA block.
pub struct RsaWriter<'a, O> {
    inner: O,
    key: &'a RsaPublicKey,
    clear_block: Vec<u8>,
    clear_block_cap: usize
}

impl<'a, O> RsaWriter<'a, O> {
    pub fn new(inner: O, key: &'a RsaPublicKey) -> Self {
        Self {
            inner,
            clear_block: Vec::new(),
            clear_block_cap: key.size() - 130,
            key,
        }
    }
}

impl<'a, O: Write> Write for RsaWriter<'a, O> {

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
            let padding = PaddingScheme::new_pkcs1v15_encrypt();
            let cipher_block = self.key.encrypt(&mut OsRng, padding, &self.clear_block[..]).unwrap();
            self.inner.write_all(&cipher_block[..])?;
            self.clear_block.clear();
        }
        Ok(())
    }

}
