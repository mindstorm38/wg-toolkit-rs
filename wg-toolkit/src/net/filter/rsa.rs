//! RSA asymmetric encryption filter.

use rsa::{RsaPrivateKey, PublicKeyParts, RsaPublicKey, PublicKey, Oaep};
use rand::rngs::OsRng;
use sha1::Sha1;

use super::{BlockReadFilter, BlockWriteFilter};


/// A block filter for reading a stream of RSA encrypted data.
pub struct RsaReadFilter<'a>(&'a RsaPrivateKey);

impl<'a> RsaReadFilter<'a> {

    #[inline]
    pub fn new(key: &'a RsaPrivateKey) -> Self {
        Self(key)
    }

}

impl<'a> BlockReadFilter for RsaReadFilter<'a> {

    #[inline]
    fn block_size(&self) -> usize {
        self.0.size()
    }

    fn filter_read(&mut self, input: &[u8], output: &mut Vec<u8>) {
        output.extend(self.0.decrypt(Oaep::new::<Sha1>(), input).unwrap())
    }

}


/// A block filter for writing RSA encrypted data.
pub struct RsaWriteFilter<'a>(&'a RsaPublicKey);

impl<'a> RsaWriteFilter<'a> {

    #[inline]
    pub fn new(key: &'a RsaPublicKey) -> Self {
        Self(key)
    }

}

impl<'a> BlockWriteFilter for RsaWriteFilter<'a> {

    #[inline]
    fn block_size(&self) -> usize {
        self.0.size() - 41 - 1
    }

    fn filter_write(&mut self, input: &[u8], output: &mut Vec<u8>) {
        output.extend(self.0.encrypt(&mut OsRng, Oaep::new::<Sha1>(), input).unwrap())
    }

}
