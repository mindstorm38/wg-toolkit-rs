//! Definition of element related to login application.

use std::io::{self, Read, Write};
use std::net::SocketAddrV4;
use std::sync::Arc;
use std::time::Duration;

use byteorder::{LE, ReadBytesExt, WriteBytesExt};
use rsa::{RsaPrivateKey, RsaPublicKey};
use blowfish::Blowfish;

use crate::net::filter::{BlockReader, BlockWriter, rsa::{RsaReadFilter, RsaWriteFilter}};
use crate::net::filter::blowfish::BlowfishFilter;

use super::{TopElementCodec, ElementCodec, ElementLength, ElementReadExt, ElementWriteExt};


/// A login request to be sent with [`LoginCodec`], send from client to 
/// server when it wants to log into and gain access to a base app.
#[derive(Debug, Default, Clone)]
pub struct LoginRequest {
    pub protocol: u32,
    pub username: String,
    pub password: String,
    pub blowfish_key: Vec<u8>,
    pub context: String,
    pub digest: Option<[u8; 16]>,
    pub nonce: u32,
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
    /// Unknown response code.
    Unknown(u8),
}

/// Describe a login success response. It provides the client with the
/// address of the base app to connect, session key and an optional
/// server message.
#[derive(Debug, Clone)]
pub struct LoginSuccess {
    /// The socket address of the base app server to connect after successful
    /// login.
    pub addr: SocketAddrV4,
    /// Session key.
    pub session_key: u32,
    /// Server message for successful login.
    pub server_message: String,
}

/// Describe an issued challenge as a response to a login request.
#[derive(Debug, Clone)]
pub enum LoginChallenge {
    /// Cuckoo cycle challenge.
    CuckooCycle {
        prefix: String,
        max_nonce: u64,
    },
}

/// Describe a login error as a response to a login request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LoginError {
    MalformedRequest = 64,
    BadProtocolVersion = 65,
    InvalidUser = 67,
    InvalidPassword = 68,
}

/// Describe a generic challenge response of a given generic type.
#[derive(Debug, Clone)]
pub struct ChallengeResponse<T> {
    /// Resolve duration of the challenge.
    pub duration: Duration,
    /// Inner data of the challenge response.
    pub data: T,
}

/// Describe a challenge response for cuckoo cycle challenge type.
#[derive(Debug, Clone)]
pub struct CuckooCycleResponse {
    pub key: String,
    pub solution: Vec<u32>,
}


/// The codec for sending a login request to the server.
#[derive(Debug)]
pub enum LoginRequestCodec {
    /// Clear transmission between server and client.
    Clear,
    /// Encrypted encoding.
    Client(Arc<RsaPublicKey>),
    /// Encrypted decoding.
    Server(Arc<RsaPrivateKey>),
}

impl LoginRequestCodec {
    pub const ID: u8 = 0x00;
}

impl ElementCodec for LoginRequestCodec {

    type Element = LoginRequest;

    fn encode<W: Write>(&self, mut write: W, input: Self::Element) -> io::Result<()> {
        write.write_u32::<LE>(input.protocol)?;
        match self {
            LoginRequestCodec::Clear => {
                write.write_u8(0)?;
                encode_login_params(write, input)
            }
            LoginRequestCodec::Client(key) => {
                write.write_u8(1)?;
                encode_login_params(BlockWriter::new(write, RsaWriteFilter::new(&key)), input)
            }
            LoginRequestCodec::Server(_) => panic!("cannot encode with server login codec"),
        }
    }

    fn decode<R: Read>(&self, mut read: R, _len: usize) -> io::Result<Self::Element> {
        let protocol = read.read_u32::<LE>()?;
        if read.read_u8()? != 0 {
            if let LoginRequestCodec::Server(key) = self {
                decode_login_params(BlockReader::new(read, RsaReadFilter::new(&key)), protocol)
            } else {
                Err(io::Error::new(io::ErrorKind::InvalidData, "cannot decode without server login codec"))
            }
        } else {
            decode_login_params(&mut read, protocol)
        }
    }

}

impl TopElementCodec for LoginRequestCodec {
    const LEN: ElementLength = ElementLength::Variable16;
}

fn encode_login_params<W: Write>(mut write: W, input: LoginRequest) -> io::Result<()> {
    write.write_u8(if input.digest.is_some() { 0x01 } else { 0x00 })?;
    write.write_rich_string(&input.username)?;
    write.write_rich_string(&input.password)?;
    write.write_rich_blob(&input.blowfish_key)?;
    write.write_rich_string(&input.context)?;
    if let Some(digest) = input.digest {
        write.write_all(&digest)?;
    }
    write.write_u32::<LE>(input.nonce)
}

fn decode_login_params<R: Read>(mut input: R, protocol: u32) -> io::Result<LoginRequest> {
    let flags = input.read_u8()?;
    Ok(LoginRequest {
        protocol,
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


/// Codec for [`LoginResponse`].
#[derive(Debug)]
pub enum LoginResponseCodec {
    /// The login response is not encrypted. This should be selected if the
    /// login request contains an empty blowfish key.
    Clear,
    /// The login response is encrypted with the given blowfish key.
    /// This blowfish key should be created from the key provided by the client
    /// in the login request.
    /// 
    /// *The blowfish key is only actually used when encoding or decoding a
    /// login success, other statuses do not require the key.*
    Encrypted(Arc<Blowfish>),
}

/// Text identifier of the cuckoo cycle challenge type.
const CHALLENGE_CUCKOO_CYCLE: &'static str = "cuckoo_cycle";

impl ElementCodec for LoginResponseCodec {

    type Element = LoginResponse;

    fn encode<W: Write>(&self, mut write: W, input: Self::Element) -> io::Result<()> {

        match input {
            LoginResponse::Success(success) => {
                
                write.write_u8(1)?; // Logged-on
                
                if let Self::Encrypted(bf) = self {
                    encode_login_success(BlockWriter::new(write, BlowfishFilter::new(&bf)), &success)?;
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

    fn decode<R: Read>(&self, mut read: R, _len: usize) -> io::Result<Self::Element> {
        
        let error = match read.read_u8()? {
            1 => {
                
                let success = if let Self::Encrypted(bf) = self {
                    decode_login_success(BlockReader::new(read, BlowfishFilter::new(&bf)))?
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
    if !success.server_message.is_empty() {
        write.write_rich_string(&success.server_message)?;
    }
    Ok(())
}

/// Internal function for decoding login success. It is extracted here
/// in order to be usable with optional encryption.
fn decode_login_success<R: Read>(mut read: R) -> io::Result<LoginSuccess> {
    Ok(LoginSuccess { 
        addr: read.read_sock_addr_v4()?, 
        session_key: read.read_u32::<LE>()?, 
        server_message: match read.read_rich_string() {
            Ok(msg) => msg,
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => String::new(),
            Err(e) => return Err(e),
        },
    })
}


/// Base codec for challenge responses. This codec is generic because decoding 
/// vary depending on the type of challenge.
#[derive(Debug, Clone, Copy)]
pub struct ChallengeResponseCodec<C> {
    codec: C,
}

impl ChallengeResponseCodec<()> {
    pub const ID: u8 = 0x03;
}

impl<C> ChallengeResponseCodec<C> {

    #[inline]
    pub fn new(codec: C) -> Self {
        Self { codec }
    }

}

impl<C: ElementCodec> ElementCodec for ChallengeResponseCodec<C> {

    type Element = ChallengeResponse<C::Element>;

    fn encode<W: Write>(&self, mut write: W, input: Self::Element) -> io::Result<()> {
        write.write_f32::<LE>(input.duration.as_secs_f32())?;
        self.codec.encode(write, input.data)?;
        Ok(())
    }

    fn decode<R: Read>(&self, mut read: R, len: usize) -> io::Result<Self::Element> {
        Ok(ChallengeResponse { 
            duration: Duration::from_secs_f32(read.read_f32::<LE>()?), 
            data: self.codec.decode(read, len - 4)?
        })
    }
    
}

impl<C: ElementCodec> TopElementCodec for ChallengeResponseCodec<C> {
    const LEN: ElementLength = ElementLength::Variable16;
}


pub struct CuckooCycleResponseCodec;

impl ElementCodec for CuckooCycleResponseCodec {

    type Element = CuckooCycleResponse;

    fn encode<W: Write>(&self, mut write: W, input: Self::Element) -> io::Result<()> {
        write.write_rich_string(&input.key)?;
        for &nonce in &input.solution {
            write.write_u32::<LE>(nonce)?;
        }
        Ok(())
    }

    fn decode<R: Read>(&self, mut read: R, _len: usize) -> io::Result<Self::Element> {

        let key = read.read_rich_string()?;
        let mut solution = Vec::with_capacity(42);

        loop {
            solution.push(match read.read_u32::<LE>() {
                Ok(n) => n,
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e),
            });
        }
        
        Ok(CuckooCycleResponse { 
            key, 
            solution,
        })

    }

}
