//! Definition of elements related to base application.
//! 
//! This modules defines 

use std::io;

use byteorder::{ReadBytesExt, LE, WriteBytesExt};

use super::{ElementCodec, TopElementCodec, ElementLength};


/// Sent by the client to the server without encryption in order to authenticate,
/// the server then compares with its internal session keys from past successful
/// logins.
/// 
/// This element is usually a reply, in such case a [`v`] must be sent as a reply.
#[derive(Debug)]
pub struct ClientAuth {
    /// The login session key that was sent by the login application, part of
    /// the [`LoginSuccess`] element.
    pub session_key: u32,
    /// The current number of attempts.
    pub attempts_count: u8,
    /// Unknown 16-bits value at the end.
    pub unk: u16,
}

/// Replied by the server to the client when receiving a [`ClientAuth`] request 
/// element. The key must be a new session 
#[derive(Debug)]
pub struct ServerAuth {
    /// The server session key, should not be the same as the login session key.
    pub session_key: u32,
}


/// Codec for [`ClientAuth`].
pub struct ClientAuthCodec;

impl ClientAuthCodec {
    pub const ID: u8 = 0x00;
}

impl ElementCodec for ClientAuthCodec {

    type Element = ClientAuth;

    fn encode<W: io::Write>(&self, mut write: W, input: Self::Element) -> io::Result<()> {
        write.write_u32::<LE>(input.session_key)?;
        write.write_u8(input.attempts_count)?;
        write.write_u16::<LE>(input.unk)
    }

    fn decode<R: io::Read>(&self, mut read: R, _len: usize) -> io::Result<Self::Element> {
        Ok(ClientAuth { 
            session_key: read.read_u32::<LE>()?, 
            attempts_count: read.read_u8()?,
            unk: read.read_u16::<LE>()?,
        })
    }

}

impl TopElementCodec for ClientAuthCodec {
    const LEN: ElementLength = ElementLength::Fixed(4 + 1 + 2);
}


/// Codec for [`ServerAuth`].
pub struct ServerAuthCodec;

impl ElementCodec for ServerAuthCodec {

    type Element = ServerAuth;

    fn encode<W: io::Write>(&self, mut write: W, input: Self::Element) -> io::Result<()> {
        write.write_u32::<LE>(input.session_key)
    }

    fn decode<R: io::Read>(&self, mut read: R, _len: usize) -> io::Result<Self::Element> {
        Ok(ServerAuth { session_key: read.read_u32::<LE>()? })
    }

}
