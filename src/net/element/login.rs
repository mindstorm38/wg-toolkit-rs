//! Definition of all predefined

use std::io::{self, Read, Seek, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use rsa::{RsaPrivateKey, RsaPublicKey};
use crate::net::element::ElementWriteExt;

use super::{ElementCodec, ElementLength, ElementReadExt};
use crate::net::filter::{RsaReader, RsaWriter};


/// A login request, optionally encrypted.
#[derive(Debug, Default)]
pub struct LoginElement {
    version: u32,
    username: String,
    password: String,
    blowfish_key: Vec<u8>,
    context: String,
    nonce: u32,
    has_digest: bool
}

impl LoginElement {

    pub const ID: u8 = 0x00;

    fn decode_internal<R: Read>(&mut self, input: &mut R) -> io::Result<()> {

        let flags = input.read_u8()?;
        self.has_digest = flags & 0x01 != 0;

        self.username.clear();
        self.password.clear();
        self.blowfish_key.clear();
        self.context.clear();

        input.read_rich_string(&mut self.username)?;
        input.read_rich_string(&mut self.password)?;
        input.read_rich_blob(&mut self.blowfish_key)?;
        input.read_rich_string(&mut self.context)?;

        if self.has_digest {
            let mut digest = [0; 16];
            input.read_exact(&mut digest);
        }

        self.nonce = input.read_u32::<LittleEndian>()?;

        Ok(())

    }

    fn encode_internal<W: Write>(&self, output: &mut W) -> io::Result<()> {

        output.write_u8(if self.has_digest { 0x01 } else { 0x00 })?;
        output.write_rich_string(self.username.as_str())?;
        output.write_rich_string(self.password.as_str())?;
        output.write_rich_blob(&self.blowfish_key[..])?;
        output.write_rich_string(self.context.as_str())?;

        if self.has_digest {
            let digest = [0; 16]; // TODO: Unknown for now.
            output.write_all(&digest[..])?;
        }

        output.write_u32::<LittleEndian>(self.nonce)

    }

}

impl ElementCodec for LoginElement {

    const LEN: ElementLength = ElementLength::Variable16;
    type EncodeCfg = Option<RsaPublicKey>;
    type DecodeCfg = RsaPrivateKey;

    fn encode<W: Write>(&self, output: &mut W, cfg: &Self::EncodeCfg) -> io::Result<()> {
        output.write_u32::<LittleEndian>(self.version)?;
        if let Some(key) = cfg {
            output.write_u8(1)?;
            self.encode_internal(&mut RsaWriter::new(output, key))
        } else {
            output.write_u8(0)?;
            self.encode_internal(output)
        }
    }

    fn decode<R: Read + Seek>(&mut self, input: &mut R, cfg: &Self::DecodeCfg) -> io::Result<()> {
        self.version = input.read_u32::<LittleEndian>()?;
        if input.read_u8()? != 0 {
            self.decode_internal(&mut RsaReader::new(input, cfg))
        } else {
            self.decode_internal(input)
        }
    }

}


/// A ping request or response.
#[derive(Debug, Default)]
pub struct PingElement(pub u8);

impl PingElement {
    pub const ID: u8 = 0x02;
}

impl ElementCodec for PingElement {

    const LEN: ElementLength = ElementLength::Fixed(1);
    type EncodeCfg = ();
    type DecodeCfg = ();

    fn encode<W: Write>(&self, out: &mut W, _cfg: &Self::EncodeCfg) -> io::Result<()> {
        out.write_u8(self.0)
    }

    fn decode<R: Read + Seek>(&mut self, input: &mut R, _cfg: &Self::DecodeCfg) -> io::Result<()> {
        self.0 = input.read_u8()?;
        Ok(())
    }

}
