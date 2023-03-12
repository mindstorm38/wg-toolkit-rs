//! Definition of elements related to base application.
//! 
//! This modules defines 

use std::io::{self, Read, Write};

use byteorder::{ReadBytesExt, LE, WriteBytesExt};

use super::{ElementCodec, TopElementCodec, ElementLength};


/// Sent by the client to the server without encryption in order to authenticate,
/// the server then compares with its internal session keys from past successful
/// logins.
/// 
/// This element is usually a request, in such case a [`ServerSessionKey`] must be 
/// sent as a reply.
#[derive(Debug)]
pub struct ClientAuth {
    /// The login key that was sent by the login application, part of the  element
    /// [`super::login::LoginSuccess`].
    pub login_key: u32,
    /// The current number of attempts.
    pub attempts_count: u8,
    /// Unknown 16-bits value at the end.
    pub unk: u16,
}

/// Replied by the server to the client when receiving a [`ClientAuth`] request 
/// element. The key must be a new session 
#[derive(Debug)]
pub struct ServerSessionKey {
    /// The server session key, should not be the same as the login session key.
    pub session_key: u32,
}

/// Sent by the client on login (and apparently randomly after login) by the client
/// to return the session key that was sent by the server in the [`ServerSessionKey`]
/// reply.
#[derive(Debug)]
pub struct ClientSessionKey {
    /// The server session key
    pub session_key: u32,
}


/// Codec for [`ClientAuth`].
pub struct ClientAuthCodec;

impl ClientAuthCodec {
    pub const ID: u8 = 0x00;
}

impl ElementCodec for ClientAuthCodec {

    type Element = ClientAuth;

    fn encode<W: Write>(&self, mut write: W, input: Self::Element) -> io::Result<()> {
        write.write_u32::<LE>(input.login_key)?;
        write.write_u8(input.attempts_count)?;
        write.write_u16::<LE>(input.unk)
    }

    fn decode<R: Read>(&self, mut read: R, _len: usize) -> io::Result<Self::Element> {
        Ok(ClientAuth { 
            login_key: read.read_u32::<LE>()?, 
            attempts_count: read.read_u8()?,
            unk: read.read_u16::<LE>()?,
        })
    }

}

impl TopElementCodec for ClientAuthCodec {
    const LEN: ElementLength = ElementLength::Fixed(7);
}


/// Codec for [`ServerSessionKey`].
pub struct ServerSessionKeyCodec;

impl ElementCodec for ServerSessionKeyCodec {

    type Element = ServerSessionKey;

    fn encode<W: Write>(&self, mut write: W, input: Self::Element) -> io::Result<()> {
        write.write_u32::<LE>(input.session_key)
    }

    fn decode<R: Read>(&self, mut read: R, _len: usize) -> io::Result<Self::Element> {
        Ok(ServerSessionKey { session_key: read.read_u32::<LE>()? })
    }

}


/// Codec for [`ClientSessionKey`].
pub struct ClientSessionKeyCodec;

impl ClientSessionKeyCodec {
    pub const ID: u8 = 0x01;
}

impl ElementCodec for ClientSessionKeyCodec {

    type Element = ClientSessionKey;

    fn encode<W: Write>(&self, mut write: W, input: Self::Element) -> io::Result<()> {
        write.write_u32::<LE>(input.session_key)
    }

    fn decode<R: Read>(&self, mut read: R, _len: usize) -> io::Result<Self::Element> {
        Ok(ClientSessionKey { session_key: read.read_u32::<LE>()? })
    }

}

impl TopElementCodec for ClientSessionKeyCodec {
    const LEN: ElementLength = ElementLength::Fixed(4);
}


/// Codec for update frequency (Hz) sent from the server to the client
/// to update the expected update frequency.
pub struct UpdateFrequencyCodec;

impl UpdateFrequencyCodec {
    pub const ID: u8 = 0x02;
}

impl ElementCodec for UpdateFrequencyCodec {

    type Element = u8;

    fn encode<W: Write>(&self, mut write: W, input: Self::Element) -> io::Result<()> {
        write.write_u8(input)
    }

    fn decode<R: Read>(&self, mut read: R, _len: usize) -> io::Result<Self::Element> {
        read.read_u8()
    }

}

impl TopElementCodec for UpdateFrequencyCodec {
    // Length known from RE.
    const LEN: ElementLength = ElementLength::Fixed(7);
}
