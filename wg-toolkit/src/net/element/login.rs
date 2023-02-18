//! Definition of all predefined

use std::io::{self, Read, Seek, SeekFrom, Write};
use std::net::SocketAddrV4;

use byteorder::{LE, ReadBytesExt, WriteBytesExt};
use rsa::{RsaPrivateKey, RsaPublicKey};
use blowfish::Blowfish;

use super::{ElementCodec, ElementLength, ElementReadExt, ElementWriteExt};
use crate::net::filter::{BlockReader, BlockWriter, rsa::{RsaReadFilter, RsaWriteFilter}};
use crate::net::filter::blowfish::BlowfishFilter;


/// Codec for ping echo-request exchange. The same value as echo must be returned.
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


/// A login request to be sent with [`LoginCodec`].
#[derive(Debug, Default, Clone)]
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

/// The codec for sending a login request to the server.
pub struct LoginCodec<'ek, 'dk> {
    encode_key: Option<&'ek RsaPublicKey>,
    decode_key: &'dk RsaPrivateKey
}

impl<'ek, 'dk> LoginCodec<'ek, 'dk> {

    pub const ID: u8 = 0x00;

    pub fn new(encode_key: Option<&'ek RsaPublicKey>, decode_key: &'dk RsaPrivateKey) -> Self {
        Self { encode_key, decode_key }
    }

    /// Create a new login codec for clear transmission, without encryption.
    pub fn new_clear(decode_key: &'dk RsaPrivateKey) -> Self {
        Self::new(None, decode_key)
    }

    /// Create a new login codec with encryption.
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
        write.write_u32::<LE>(input.nonce)
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
            nonce: input.read_u32::<LE>()?
        })
    }

}

impl ElementCodec for LoginCodec<'_, '_> {

    const LEN: ElementLength = ElementLength::Variable16;
    type Element = LoginParams;

    fn encode<W: Write>(&self, mut write: W, input: Self::Element) -> io::Result<()> {
        write.write_u32::<LE>(input.version)?;
        if let Some(key) = self.encode_key {
            write.write_u8(1)?;
            Self::encode_internal(BlockWriter::new(write, RsaWriteFilter::new(key)), input)
        } else {
            write.write_u8(0)?;
            Self::encode_internal(write, input)
        }
    }

    fn decode<R: Read + Seek>(&self, mut read: R, _len: u64) -> io::Result<Self::Element> {
        // println!("decode login len: {len}");
        let version = read.read_u32::<LE>()?;
        if read.read_u8()? != 0 {
            Self::decode_internal(BlockReader::new(read, RsaReadFilter::new(self.decode_key)), version)
        } else {
            // println!("decode ciphered login...");
            let pos = read.stream_position().unwrap();
            match Self::decode_internal(&mut read, version) {
                Ok(elt) => Ok(elt),
                Err(e) => {
                    // println!("=> error: {:?}", e);
                    read.seek(SeekFrom::Start(pos)).unwrap();
                    let mut data = Vec::new();
                    read.read_to_end(&mut data).unwrap();
                    // println!("=> raw data: ({}) {}", data.len(), crate::util::str_from_escaped(&data[..]));
                    Err(e)
                }
            }
        }
    }

}


/// Describe all kinds of responses returned from server to client when
/// the client attempt to login. This includes challenge or error codes.
#[derive(Debug, Clone)]
pub enum LoginResponse {
    /// The login is successful.
    Success(LoginSuccess),
    /// An error happened server-side and the login process cannot succeed.
    Error(LoginError),
    /// A challenge must be completed in order to have a response.
    Challenge(LoginChallenge),
    Unknown(u8),
}

const CHALLENGE_CUCKOO_CYCLE: &'static str = "cuckoo_cycle";

#[derive(Debug, Clone)]
pub struct LoginSuccess {
    /// The socket address of the base app server to connect after successful
    /// login.
    addr: SocketAddrV4,
    /// Blowfish session key.
    session_key: u32,
    /// Server message for successful login.
    server_message: String,
}

/// Describe an issued challenge.
#[derive(Debug, Clone)]
pub enum LoginChallenge {
    /// Cuckoo cycle challenge.
    CuckooCycle {
        prefix: String,
        max_nonce: u64,
    },
}

/// Describe a login error. 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LoginError {
    MalformedRequest = 64,
    BadProtocolVersion = 65,
    InvalidUser = 67,
    InvalidPassword = 68,
}

/// Codec for [`LoginResponse`].
pub struct LoginResponseCodec<'a> {
    blowfish: Option<&'a Blowfish>
}

impl<'a> LoginResponseCodec<'a> {

    #[inline]
    pub fn new(bf: Option<&'a Blowfish>) -> Self {
        Self {
            blowfish: bf
        }
    }

    #[inline]
    pub fn new_clear() -> Self {
        Self::new(None)
    }

    #[inline]
    pub fn new_encrypted(bf: &'a Blowfish) -> Self {
        Self::new(Some(bf))
    }

}

impl ElementCodec for LoginResponseCodec<'_> {

    const LEN: ElementLength = ElementLength::Fixed(0);
    type Element = LoginResponse;

    fn encode<W: Write>(&self, mut write: W, input: Self::Element) -> io::Result<()> {

        match input {
            LoginResponse::Success(success) => {
                
                write.write_u8(1)?; // Logged-on
                
                if let Some(bf) = &self.blowfish {
                    encode_login_success(BlockWriter::new(write, BlowfishFilter::new(*bf)), &success)?;
                } else {
                    encode_login_success(write, &success)?;
                }

            }
            LoginResponse::Error(err) => {
                write.write_u8(err as _)?;
            }
            LoginResponse::Challenge(challenge) => {

                write.write_u8(66)?;
                
                match challenge {
                    LoginChallenge::CuckooCycle { prefix, max_nonce } => {
                        write.write_rich_string(CHALLENGE_CUCKOO_CYCLE)?;
                        write.write_rich_string(&prefix)?;
                        write.write_u64::<LE>(max_nonce)?;
                    }
                }
                
            }
            LoginResponse::Unknown(code) => write.write_u8(code)?
        }

        Ok(())

    }

    fn decode<R: Read + Seek>(&self, mut read: R, _len: u64) -> io::Result<Self::Element> {
        
        let error = match read.read_u8()? {
            1 => {
                
                let success = if let Some(bf) = &self.blowfish {
                    decode_login_success(BlockReader::new(read, BlowfishFilter::new(*bf)))?
                } else {
                    decode_login_success(read)?
                };

                return Ok(LoginResponse::Success(success));

            }
            66 => {
                
                let challenge_name = read.read_rich_string()?;
                let challenge = match &challenge_name[..] {
                    CHALLENGE_CUCKOO_CYCLE => {
                        let prefix = read.read_rich_string()?;
                        let max_nonce = read.read_u64::<LE>()?;
                        LoginChallenge::CuckooCycle { prefix, max_nonce }
                    }
                    _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid challenge name"))
                };

                return Ok(LoginResponse::Challenge(challenge));

            }
            64 => LoginError::MalformedRequest,
            65 => LoginError::BadProtocolVersion,
            67 => LoginError::InvalidUser,
            68 => LoginError::InvalidPassword,
            code => return Ok(LoginResponse::Unknown(code))
        };

        Ok(LoginResponse::Error(error))

    }

}


/// Internal function for encoding login success. It is extracted here
/// in order to be usable with optional encryption.
fn encode_login_success<W: Write>(mut write: W, success: &LoginSuccess) -> io::Result<()> {
    write.write_sock_addr_v4(success.addr)?;
    write.write_u32::<LE>(success.session_key)?;
    write.write_rich_string(&success.server_message)?;
    Ok(())
}

/// Internal function for decoding login success. It is extracted here
/// in order to be usable with optional encryption.
fn decode_login_success<R: Read>(mut read: R) -> io::Result<LoginSuccess> {
    Ok(LoginSuccess { 
        addr: read.read_sock_addr_v4()?, 
        session_key: read.read_u32::<LE>()?, 
        server_message: read.read_rich_string()?,
    })
}
