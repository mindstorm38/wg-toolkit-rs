//! Definition of all predefined

use std::io::{self, Read, Seek, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use rsa::{RsaPrivateKey, RsaPublicKey};
use crate::net::element::ElementWriteExt;

use super::{ElementCodec, /*ElementEncoder, ElementDecoder,*/ ElementLength, ElementReadExt};
use crate::net::filter::{RsaReader, RsaWriter};


/// A login request, optionally encrypted.
#[derive(Debug, Default)]
pub struct LoginParams {
    pub version: u32,
    pub username: String,
    pub password: String,
    pub blowfish_key: Vec<u8>,
    pub context: String,
    pub digest: Option<[u8; 16]>,
    pub nonce: u32,
}

impl LoginParams {

    pub const ID: u8 = 0x00;

    /*fn decode_internal<R: Read>(&mut self, input: &mut R) -> io::Result<()> {

        let flags = input.read_u8()?;

        self.username.clear();
        self.password.clear();
        self.blowfish_key.clear();
        self.context.clear();

        input.read_rich_string(&mut self.username)?;
        input.read_rich_string(&mut self.password)?;
        input.read_rich_blob(&mut self.blowfish_key)?;
        input.read_rich_string(&mut self.context)?;

        self.digest = if flags & 0x01 != 0 {
            let mut digest = [0; 16];
            input.read_exact(&mut digest);
            Some(digest)
        } else {
            Option::None
        };

        self.nonce = input.read_u32::<LittleEndian>()?;

        Ok(())

    }

    fn encode_internal<W: Write>(&self, output: &mut W) -> io::Result<()> {

        output.write_u8(if self.digest.is_some() { 0x01 } else { 0x00 })?;
        output.write_rich_string(self.username.as_str())?;
        output.write_rich_string(self.password.as_str())?;
        output.write_rich_blob(&self.blowfish_key[..])?;
        output.write_rich_string(self.context.as_str())?;

        if let Some(digest) = self.digest {
            output.write_all(&digest[..])?;
        }

        output.write_u32::<LittleEndian>(self.nonce)

    }*/

}

/*impl ElementCodec for LoginElement {

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

}*/


/*/// Element encoder for login parameters. Optionally encrypted.
pub struct LoginEncoder<'a> {
    elt: LoginParams,
    key: Option<&'a RsaPublicKey>
}

impl<'a> LoginEncoder<'a> {

    pub fn new(elt: LoginParams, key: Option<&'a RsaPublicKey>) -> Self {
        Self { elt, key }
    }

    pub fn new_clear(elt: LoginParams) -> Self {
        Self::new(elt, None)
    }

    pub fn new_encrypted(elt: LoginParams, key: &'a RsaPublicKey) -> Self {
        Self::new(elt, Some(key))
    }

    fn encode_internal<W: Write>(&self, mut output: W) -> io::Result<()> {

        let elt = &self.elt;

        output.write_u8(if elt.digest.is_some() { 0x01 } else { 0x00 })?;
        output.write_rich_string(elt.username.as_str())?;
        output.write_rich_string(elt.password.as_str())?;
        output.write_rich_blob(&elt.blowfish_key[..])?;
        output.write_rich_string(elt.context.as_str())?;

        if let Some(digest) = elt.digest {
            output.write_all(&digest[..])?;
        }

        output.write_u32::<LittleEndian>(elt.nonce)

    }

}

impl ElementEncoder for LoginEncoder<'_> {
    const LEN: ElementLength = ElementLength::Variable16;
    fn encode<W: Write>(self, mut write: W) -> io::Result<()> {
        write.write_u32::<LittleEndian>(self.elt.version)?;
        if let Some(key) = self.key {
            write.write_u8(1)?;
            self.encode_internal(RsaWriter::new(write, key))
        } else {
            write.write_u8(0)?;
            self.encode_internal(write)
        }
    }
}


/// Element decoder for login parameters.
pub struct LoginDecoder<'a> {
    key: &'a RsaPrivateKey
}

impl<'a> LoginDecoder<'a> {

    pub fn new(key: &'a RsaPrivateKey) -> Self {
        Self { key }
    }

    fn decode_internal<R: Read>(mut input: R, version: u32) -> io::Result<LoginParams> {
        let flags = input.read_u8()?;
        Ok(LoginParams {
            version,
            username: input.read_rich_string()?,
            password: input.read_rich_string()?,
            blowfish_key: input.read_rich_blob()?,
            context: input.read_rich_string()?,
            digest: if flags & 0x01 != 0 {
                let mut digest = [0; 16];
                input.read_exact(&mut digest)?;
                Some(digest)
            } else {
                Option::None
            },
            nonce: input.read_u32::<LittleEndian>()?
        })
    }

}

impl ElementDecoder for LoginDecoder<'_> {
    const LEN: ElementLength = ElementLength::Variable16;
    type Output = LoginParams;
    fn decode<R: Read + Seek>(self, mut read: R, _len: u64) -> io::Result<Self::Output> {
        let version = read.read_u32::<LittleEndian>()?;
        if read.read_u8()? != 0 {
            Self::decode_internal(RsaReader::new(read, self.key), version)
        } else {
            Self::decode_internal(read, version)
        }
    }
}*/



pub struct LoginCodec<'ek, 'dk> {
    encode_key: Option<&'ek RsaPublicKey>,
    decode_key: &'dk RsaPrivateKey
}

impl<'ek, 'dk> LoginCodec<'ek, 'dk> {

    pub const ID: u8 = 0x00;

    pub fn new(encode_key: Option<&'ek RsaPublicKey>, decode_key: &'dk RsaPrivateKey) -> Self {
        Self { encode_key, decode_key }
    }

    pub fn new_clear(decode_key: &'dk RsaPrivateKey) -> Self {
        Self::new(None, decode_key)
    }

    pub fn new_encrypted(encode_key: &'ek RsaPublicKey, decode_key: &'dk RsaPrivateKey) -> Self {
        Self::new(Some(encode_key), decode_key)
    }

    fn encode_internal<W: Write>(mut write: W, input: LoginParams) -> io::Result<()> {
        write.write_u8(if input.digest.is_some() { 0x01 } else { 0x00 })?;
        write.write_rich_string(input.username.as_str())?;
        write.write_rich_string(input.password.as_str())?;
        write.write_rich_blob(&input.blowfish_key[..])?;
        write.write_rich_string(input.context.as_str())?;
        if let Some(digest) = input.digest {
            write.write_all(&digest[..])?;
        }
        write.write_u32::<LittleEndian>(input.nonce)
    }

    fn decode_internal<R: Read>(mut input: R, version: u32) -> io::Result<LoginParams> {
        let flags = input.read_u8()?;
        Ok(LoginParams {
            version,
            username: input.read_rich_string()?,
            password: input.read_rich_string()?,
            blowfish_key: input.read_rich_blob()?,
            context: input.read_rich_string()?,
            digest: if flags & 0x01 != 0 {
                let mut digest = [0; 16];
                input.read_exact(&mut digest)?;
                Some(digest)
            } else {
                Option::None
            },
            nonce: input.read_u32::<LittleEndian>()?
        })
    }

}

impl ElementCodec for LoginCodec<'_, '_> {

    const LEN: ElementLength = ElementLength::Variable16;
    type Element = LoginParams;

    fn encode<W: Write>(&self, mut write: W, input: Self::Element) -> io::Result<()> {
        write.write_u32::<LittleEndian>(input.version)?;
        if let Some(key) = self.encode_key {
            write.write_u8(1)?;
            Self::encode_internal(RsaWriter::new(write, key), input)
        } else {
            write.write_u8(0)?;
            Self::encode_internal(write, input)
        }
    }

    fn decode<R: Read + Seek>(&self, mut read: R, _len: u64) -> io::Result<Self::Element> {
        let version = read.read_u32::<LittleEndian>()?;
        if read.read_u8()? != 0 {
            Self::decode_internal(RsaReader::new(read, self.decode_key), version)
        } else {
            Self::decode_internal(read, version)
        }
    }

}



pub struct PingCodec;

impl PingCodec {
    pub const ID: u8 = 0x02;
}

impl ElementCodec for PingCodec {
    const LEN: ElementLength = ElementLength::Fixed(1);
    type Element = u8;
    fn encode<W: Write>(&self, mut write: W, input: Self::Element) -> io::Result<()> {
        write.write_u8(input)
    }
    fn decode<R: Read + Seek>(&self, mut read: R, _len: u64) -> io::Result<Self::Element> {
        read.read_u8()
    }
}

/*impl ElementEncoder for PingCodec {
    const LEN: ElementLength = ElementLength::Fixed(1);
    type Input = u8;
    fn encode<W: Write>(&self, mut write: W, input: Self::Input) -> io::Result<()> {
        write.write_u8(input)
    }
}*/


/*/// A ping request or response.
#[derive(Debug, Default)]
pub struct PingElement(pub u8);

impl PingElement {
    pub const ID: u8 = 0x02;
}

impl ElementEncoder for PingElement {
    const LEN: ElementLength = ElementLength::Fixed(1);
    fn encode<W: Write>(self, mut write: W) -> io::Result<()> {
        write.write_u8(self.0)
    }
}

impl ElementDecoder for PingElement {
    const LEN: ElementLength = ElementLength::Fixed(1);
    type Output = Self;
    fn decode<R: Read + Seek>(mut self, mut read: R, _len: u64) -> io::Result<Self::Output> {
        self.0 = read.read_u8()?;
        Ok(self)
    }
}*/

/*impl ElementCodec for PingElement {

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

}*/
