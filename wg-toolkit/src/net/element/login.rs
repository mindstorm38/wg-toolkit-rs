//! Definition of element related to login application.

use std::io::{self, Read, Write};
use std::net::SocketAddrV4;
use std::sync::Arc;
use std::time::Duration;

use rsa::{RsaPrivateKey, RsaPublicKey};
use blowfish::Blowfish;

use crate::net::filter::{BlockReader, BlockWriter, rsa::{RsaReadFilter, RsaWriteFilter}};
use crate::net::filter::blowfish::{BlowfishWriter, BlowfishReader};
use crate::util::io::*;

use super::{Element, SimpleElement, TopElement, ElementLength};


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

impl LoginRequest {
    pub const ID: u8 = 0x00;
}


/// Describe all kinds of responses returned from server to client when
/// the client attempt to login. This includes challenge or error codes.
#[derive(Debug, Clone)]
pub enum LoginResponse {
    /// The login is successful.
    Success(LoginSuccess),
    /// An error happened server-side and the login process cannot succeed.
    Error(LoginError, String),
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
    /// Session key, it's used to authenticate to the base app.
    pub login_key: u32,
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
    // ChallengeIssued = 66, handled by a specific variant of LoginResponse.
    InvalidUser = 67,
    InvalidPassword = 68,
    AlreadyLoggedIn = 69,
    BadDigest = 70,
    DatabaseGeneralFailure = 71,
    DatabaseNotReady = 72,
    IllegalCharacters = 73,
    ServerNotReady = 74,
    UpdaterNotReady = 75, // No longer used
    NoBaseApp = 76,
    BaseAppOverload = 77,
    CellAppOverload = 78,
    BaseAppTimeout = 79,
    BaseAppManagerTimeout = 80,
    DatabaseAppOverload = 81,
    LoginNotAllowed = 82,
    RateLimited = 83,
    Banned = 84,
    ChallengeError = 85,
}


/// Describe a generic challenge response of a given generic type.
#[derive(Debug, Clone)]
pub struct ChallengeResponse<T> {
    /// Resolve duration of the challenge.
    pub duration: Duration,
    /// Inner data of the challenge response.
    pub data: T,
}

impl ChallengeResponse<()> {
    pub const ID: u8 = 0x03;
}

/// Describe a challenge response for cuckoo cycle challenge type.
#[derive(Debug, Clone)]
pub struct CuckooCycleResponse {
    pub key: String,
    pub solution: Vec<u32>,
}


/// Describe the type of encryption to use for encoding/decoding
/// a login request. This must be provided as configuration when
/// writing or reading the element.
#[derive(Debug)]
pub enum LoginRequestEncryption {
    /// Clear transmission between server and client.
    Clear,
    /// Encrypted encoding.
    Client(Arc<RsaPublicKey>),
    /// Encrypted decoding.
    Server(Arc<RsaPrivateKey>),
}

impl Element for LoginRequest {

    type Config = LoginRequestEncryption;

    fn encode<W: Write>(&self, mut write: W, config: &Self::Config) -> io::Result<()> {
        write.write_u32(self.protocol)?;
        match config {
            LoginRequestEncryption::Clear => {
                write.write_u8(0)?;
                encode_login_params(write, self)
            }
            LoginRequestEncryption::Client(key) => {
                write.write_u8(1)?;
                encode_login_params(BlockWriter::new(write, RsaWriteFilter::new(&key)), self)
            }
            LoginRequestEncryption::Server(_) => panic!("cannot encode with server login codec"),
        }
    }

    fn decode<R: Read>(mut read: R, _len: usize, config: &Self::Config) -> io::Result<Self> {
        let protocol = read.read_u32()?;
        if read.read_u8()? != 0 {
            if let LoginRequestEncryption::Server(key) = config {
                decode_login_params(BlockReader::new(read, RsaReadFilter::new(&key)), protocol)
            } else {
                Err(io::Error::new(io::ErrorKind::InvalidData, "cannot decode without server login codec"))
            }
        } else {
            decode_login_params(&mut read, protocol)
        }
    }

}

impl TopElement for LoginRequest {
    const LEN: ElementLength = ElementLength::Variable16;
}

fn encode_login_params<W: Write>(mut write: W, input: &LoginRequest) -> io::Result<()> {
    write.write_u8(if input.digest.is_some() { 0x01 } else { 0x00 })?;
    write.write_string_variable(&input.username)?;
    write.write_string_variable(&input.password)?;
    write.write_blob_variable(&input.blowfish_key)?;
    write.write_string_variable(&input.context)?;
    if let Some(digest) = input.digest {
        write.write_all(&digest)?;
    }
    write.write_u32(input.nonce)
}

fn decode_login_params<R: Read>(mut input: R, protocol: u32) -> io::Result<LoginRequest> {
    let flags = input.read_u8()?;
    Ok(LoginRequest {
        protocol,
        username: input.read_string_variable()?,
        password: input.read_string_variable()?,
        blowfish_key: input.read_blob_variable()?,
        context: input.read_string_variable()?,
        digest: if flags & 0x01 != 0 {
            let mut digest = [0; 16];
            input.read_exact(&mut digest)?;
            Some(digest)
        } else {
            Option::None
        },
        nonce: input.read_u32()?
    })
}


/// Codec for [`LoginResponse`].
#[derive(Debug)]
pub enum LoginResponseEncryption {
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

impl Element for LoginResponse {

    type Config = LoginResponseEncryption;

    fn encode<W: Write>(&self, mut write: W, config: &Self::Config) -> io::Result<()> {
        
        match self {
            Self::Success(success) => {
                
                write.write_u8(1)?; // Logged-on
                
                if let LoginResponseEncryption::Encrypted(bf) = config {
                    encode_login_success(BlowfishWriter::new(write, &bf), success)?;
                } else {
                    encode_login_success(write, success)?;
                }

            }
            Self::Error(err, message) => {
                write.write_u8(*err as _)?;
                write.write_string_variable(&message)?;
            }
            Self::Challenge(challenge) => {

                write.write_u8(66)?;
                
                match challenge {
                    LoginChallenge::CuckooCycle { prefix, max_nonce } => {
                        write.write_string_variable(CHALLENGE_CUCKOO_CYCLE)?;
                        write.write_string_variable(&prefix)?;
                        write.write_u64(*max_nonce)?;
                    }
                }
                
            }
            Self::Unknown(code) => write.write_u8(*code)?
        }

        Ok(())

    }

    fn decode<R: Read>(mut read: R, _len: usize, config: &Self::Config) -> io::Result<Self> {
        
        let error = match read.read_u8()? {
            1 => {
                
                let success = 
                if let LoginResponseEncryption::Encrypted(bf) = config {
                    decode_login_success(BlowfishReader::new(read, &bf))?
                } else {
                    decode_login_success(read)?
                };

                return Ok(LoginResponse::Success(success));

            }
            66 => {
                
                let challenge_name = read.read_string_variable()?;
                let challenge = match &challenge_name[..] {
                    CHALLENGE_CUCKOO_CYCLE => {
                        let prefix = read.read_string_variable()?;
                        let max_nonce = read.read_u64()?;
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
            // TODO: Implement other variants
            code => return Ok(LoginResponse::Unknown(code))
        };

        let message = match read.read_string_variable() {
            Ok(msg) => msg,
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => String::new(),
            Err(e) => return Err(e),
        };

        Ok(LoginResponse::Error(error, message))

    }

}

/// Internal function for encoding login success. It is extracted here
/// in order to be usable with optional encryption.
fn encode_login_success<W: Write>(mut write: W, success: &LoginSuccess) -> io::Result<()> {
    write.write_sock_addr_v4(success.addr)?;
    write.write_u32(success.login_key)?;
    if !success.server_message.is_empty() {
        write.write_string_variable(&success.server_message)?;
    }
    Ok(())
}

/// Internal function for decoding login success. It is extracted here
/// in order to be usable with optional encryption.
fn decode_login_success<R: Read>(mut read: R) -> io::Result<LoginSuccess> {
    Ok(LoginSuccess { 
        addr: read.read_sock_addr_v4()?, 
        login_key: read.read_u32()?, 
        server_message: match read.read_string_variable() {
            Ok(msg) => msg,
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => String::new(),
            Err(e) => return Err(e),
        },
    })
}


impl<E: Element> Element for ChallengeResponse<E> {

    type Config = E::Config;

    fn encode<W: Write>(&self, mut write: W, config: &Self::Config) -> io::Result<()> {
        write.write_f32(self.duration.as_secs_f32())?;
        self.data.encode(write, config)
    }

    fn decode<R: Read>(mut read: R, len: usize, config: &Self::Config) -> io::Result<Self> {
        Ok(ChallengeResponse { 
            duration: Duration::from_secs_f32(read.read_f32()?), 
            data: E::decode(read, len - 4, config)?
        })
    }

}

impl<E: Element> TopElement for ChallengeResponse<E> {
    const LEN: ElementLength = ElementLength::Variable16;
}

impl SimpleElement for CuckooCycleResponse {

    fn encode<W: Write>(&self, mut write: W) -> io::Result<()> {
        write.write_string_variable(&self.key)?;
        for &nonce in &self.solution {
            write.write_u32(nonce)?;
        }
        Ok(())
    }

    fn decode<R: Read>(mut read: R, _len: usize) -> io::Result<Self> {

        let key = read.read_string_variable()?;
        let mut solution = Vec::with_capacity(42);

        loop {
            solution.push(match read.read_u32() {
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
