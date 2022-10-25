//! Definition of all predefined

use std::io::{self, Read, Seek, SeekFrom, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use rsa::{RsaPrivateKey, RsaPublicKey};

use super::{ElementCodec, ElementLength, ElementReadExt, ElementWriteExt};
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
    //pub data: Vec<u8>
}

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
        // write.write_all(&input.data[..])
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
        /*Ok(LoginParams {
            version,
            data: {
                let mut data = Vec::new();
                input.read_to_end(&mut data)?;
                data
            }
        })*/
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

    fn decode<R: Read + Seek>(&self, mut read: R, len: u64) -> io::Result<Self::Element> {
        println!("decode login len: {len}");
        let version = read.read_u32::<LittleEndian>()?;
        if read.read_u8()? != 0 {
            Self::decode_internal(RsaReader::new(read, self.decode_key), version)
        } else {
            println!("decode ciphered login...");
            let pos = read.stream_position().unwrap();
            match Self::decode_internal(&mut read, version) {
                Ok(elt) => Ok(elt),
                Err(e) => {
                    println!("=> error: {:?}", e);
                    read.seek(SeekFrom::Start(pos)).unwrap();
                    let mut data = Vec::new();
                    read.read_to_end(&mut data).unwrap();
                    println!("=> raw data: ({}) {}", data.len(), crate::util::str_from_escaped(&data[..]));
                    Err(e)
                }
            }
        }
    }

}


#[derive(Debug)]
pub struct Challenge {
    pub kind: String,
    pub key: String
}

pub struct ChallengeCodec;

impl ElementCodec for ChallengeCodec {

    const LEN: ElementLength = ElementLength::Fixed(0);
    type Element = Challenge;

    fn encode<W: Write>(&self, _write: W, _input: Self::Element) -> io::Result<()> {
        todo!()
    }

    fn decode<R: Read + Seek>(&self, mut read: R, _len: u64) -> io::Result<Self::Element> {
        let _unk0 = read.read_u8()?;
        let kind = read.read_rich_string()?;
        let key = read.read_rich_string()?;
        Ok(Challenge {
            kind,
            key
        })
    }

}


pub struct ChallengeResponseCodec;

impl ChallengeResponseCodec {
    pub const ID: u8 = 0x03;
}

impl ElementCodec for ChallengeResponseCodec {

    const LEN: ElementLength = ElementLength::Variable16;
    type Element = ();

    fn encode<W: Write>(&self, write: W, input: Self::Element) -> io::Result<()> {
        todo!()
    }

    fn decode<R: Read + Seek>(&self, read: R, len: u64) -> io::Result<Self::Element> {
        todo!()
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
