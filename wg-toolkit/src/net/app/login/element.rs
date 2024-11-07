//! Definition of elements related to login application.
//! 
//! When a client send a login request to the login app, it might be encrypted with RSA, 
//! the server then decide which response to return depending on the input, it might 
//! send a challenge that is required. When the login succeed, the server sends a login 
//! key that is used by the client when first connecting to the base app.
//! 
//! This app also provides a way to ping test the server.

use std::io::{self, Read, Write};
use std::net::SocketAddrV4;
use std::sync::Arc;
use std::time::Duration;

use rsa::{RsaPrivateKey, RsaPublicKey};
use blowfish::Blowfish;

use crate::net::filter::{RsaWriter, RsaReader, BlowfishWriter, BlowfishReader};
use crate::net::element::{Element, SimpleElement, ElementLength};
use crate::util::io::*;


/// Internal module containing all raw elements numerical ids.
pub mod id {
    pub const LOGIN_REQUEST: u8         = 0x00;
    pub const PING: u8                  = 0x02;
    pub const CHALLENGE_RESPONSE: u8    = 0x03;
}


/// A ping sent from the client to the login app or replied from the
/// login app to the client.
#[derive(Debug, Clone, Copy)]
pub struct Ping {
    /// The number of the ping, the same number must be sent back to
    /// the client when login app receives it.
    pub num: u8,
}

impl SimpleElement for Ping {

    const ID: u8 = id::PING;
    const LEN: ElementLength = ElementLength::Fixed(1);

    fn encode(&self, write: &mut impl Write) -> io::Result<()> {
        write.write_u8(self.num)
    }

    fn decode(read: &mut impl Read, _len: usize) -> io::Result<Self> {
        Ok(Self { num: read.read_u8()? })
    }

}


/// A login request to be sent with [`LoginCodec`], sent from client to 
/// server when it wants to log into and gain access to a base app.
#[derive(Debug, Default, Clone)]
pub struct LoginRequest {
    /// The protocol used, currently undocumented.
    pub protocol: u32,
    /// The username used to login.
    pub username: String,
    /// The password used to login, only used when using password to login.
    pub password: String,
    /// The blowfish key to initialize the blowfish cipher, its size can be between 
    /// 4 and 56, both included, the full key is 56 bytes long.
    pub blowfish_key: Vec<u8>,
    pub context: String,
    pub digest: Option<[u8; 16]>,
    pub nonce: u32,
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

impl LoginRequest {
    pub const ID: u8 = id::LOGIN_REQUEST;
}

impl Element for LoginRequest {

    type Config = LoginRequestEncryption;
    const LEN: ElementLength = ElementLength::Variable16;

    fn encode(&self, write: &mut impl Write, config: &Self::Config) -> io::Result<u8> {
        write.write_u32(self.protocol)?;
        match config {
            LoginRequestEncryption::Clear => {
                write.write_u8(0)?;
                encode_login_params(write, self)?;
            }
            LoginRequestEncryption::Client(key) => {
                write.write_u8(1)?;
                encode_login_params(RsaWriter::new(write, &key), self)?;
            }
            LoginRequestEncryption::Server(_) => panic!("missing client public encryption key to encode the login request"),
        }
        Ok(Self::ID)
    }

    fn decode(read: &mut impl Read, _len: usize, config: &Self::Config, id: u8) -> io::Result<Self> {
        debug_assert_eq!(id, Self::ID);
        let protocol = read.read_u32()?;
        if read.read_u8()? != 0 {
            if let LoginRequestEncryption::Server(key) = config {
                decode_login_params(RsaReader::new(read, &key), protocol)
            } else {
                Err(io::Error::new(io::ErrorKind::InvalidData, "missing server private encryption key to decode the login request"))
            }
        } else {
            decode_login_params(read, protocol)
        }
    }

}

fn encode_login_params(mut write: impl Write, input: &LoginRequest) -> io::Result<()> {
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

fn decode_login_params(mut input: impl Read, protocol: u32) -> io::Result<LoginRequest> {
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


/// Describe all kinds of responses returned from server to client when
/// the client attempt to login. This includes challenge or error codes.
#[derive(Debug, Clone)]
pub enum LoginResponse {
    /// The login is successful.
    Success(LoginSuccess),
    /// A challenge must be completed in order to have a response.
    Challenge(LoginChallenge),
    /// An error happened server-side and the login process cannot succeed. This data
    /// is expected to be a JSON string on modern version of the game.
    Error(LoginError, String),
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
        /// This prefix string to the key used to initialize the Cuckoo Cycle context,
        /// it's given to a SHA-256 before being used, so it can be any size.
        key_prefix: Vec<u8>,
        max_nonce: u32,
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
    /// This error is handled in a specific way, the client will put the client in a
    /// waiting mode where it will automatically retry login.
    RateLimited = 83,
    /// This error expect the message to be a JSON containing a single key 'bans' that
    /// is itself a string representation of a JSON element containing 'expiryTime' 
    /// and 'reason'.
    Banned = 84,
    ChallengeError = 85,
}

/// Describe if the login response has to be encrypted or not. This must be 
/// provided as configuration when writing or reading the element.
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
    const LEN: ElementLength = ElementLength::Undefined;  // It's a reply.

    fn encode(&self, write: &mut impl Write, config: &Self::Config) -> io::Result<u8> {
        
        match self {
            Self::Success(success) => {
                
                write.write_u8(1)?; // Logged-on
                
                if let LoginResponseEncryption::Encrypted(bf) = config {
                    encode_login_success(BlowfishWriter::new(write, &bf), success)?;
                } else {
                    encode_login_success(write, success)?;
                }

            }
            Self::Challenge(challenge) => {

                write.write_u8(66)?;
                
                match challenge {
                    LoginChallenge::CuckooCycle { key_prefix: prefix, max_nonce } => {
                        write.write_string_variable(CHALLENGE_CUCKOO_CYCLE)?;
                        write.write_blob_variable(&prefix)?;
                        write.write_u64(*max_nonce as u64)?;
                    }
                }
                
            }
            Self::Error(err, message) => {
                write.write_u8(*err as _)?;
                write.write_string_variable(&message)?;
            }
            Self::Unknown(code) => write.write_u8(*code)?
        }

        Ok(0)

    }

    fn decode(read: &mut impl Read, _len: usize, config: &Self::Config, _id: u8) -> io::Result<Self> {
        
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
                        let prefix = read.read_blob_variable()?;
                        let max_nonce = read.read_u64()? as u32;
                        LoginChallenge::CuckooCycle { 
                            key_prefix: prefix, 
                            max_nonce,
                        }
                    }
                    name => return Err(io::Error::new(io::ErrorKind::InvalidData, format!("invalid challenge name: {name}")))
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


/// Describe a generic challenge response of a given generic type. This is a top element
/// that's not expecting any reply, because the client sends a new login request just
/// after.
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
    /// The full key used to initialize the Cuckoo Cycle context, this should start with
    /// the issued [`LoginChallenge::CuckooCycle::key_prefix`].
    pub key: Vec<u8>,
    /// Contains each solution found for the problem.
    pub solution: Vec<u32>,
}

impl<E: Element> Element for ChallengeResponse<E> {

    type Config = E::Config;
    const LEN: ElementLength = ElementLength::Variable16;

    fn encode(&self, write: &mut impl Write, config: &Self::Config) -> io::Result<u8> {
        write.write_f32(self.duration.as_secs_f32())?;
        self.data.encode(write, config)
    }

    fn decode(read: &mut impl Read, len: usize, config: &Self::Config, id: u8) -> io::Result<Self> {
        Ok(ChallengeResponse { 
            duration: Duration::from_secs_f32(read.read_f32()?), 
            data: E::decode(read, len - 4, config, id)?
        })
    }

}

impl SimpleElement for CuckooCycleResponse {
    
    const ID: u8 = id::CHALLENGE_RESPONSE;
    const LEN: ElementLength = ElementLength::Undefined;

    fn encode(&self, write: &mut impl Write) -> io::Result<()> {
        write.write_blob_variable(&self.key)?;
        for &nonce in &self.solution {
            write.write_u32(nonce)?;
        }
        Ok(())
    }

    fn decode(read: &mut impl Read, _len: usize) -> io::Result<Self> {

        let key = read.read_blob_variable()?;
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
