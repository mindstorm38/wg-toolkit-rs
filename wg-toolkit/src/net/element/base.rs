//! Definition of elements related to base application.
//! 
//! Such elements are sent from the client to the base application and also
//! replies to such elements if they are requests.

use std::io::{self, Read, Write};

use crate::util::io::*;

use super::{SimpleElement, TopElement, ElementLength};


/// Sent by the client to the server without encryption in order to authenticate,
/// the server then compares with its internal login keys from past successful
/// logins on the login app.
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

impl ClientAuth {
    pub const ID: u8 = 0x00;
}

impl SimpleElement for ClientAuth {

    fn encode(&self, write: &mut impl Write) -> io::Result<u8> {
        write.write_u32(self.login_key)?;
        write.write_u8(self.attempts_count)?;
        write.write_u16(self.unk)?;
        Ok(Self::ID)
    }

    fn decode(read: &mut impl Read, _len: usize, _id: u8) -> io::Result<Self> {
        Ok(Self {
            login_key: read.read_u32()?, 
            attempts_count: read.read_u8()?,
            unk: read.read_u16()?,
        })
    }

}

impl TopElement for ClientAuth {
    const LEN: ElementLength = ElementLength::Fixed(7);
}


/// Replied by the server to the client when receiving a [`ClientAuth`] request 
/// element. The key must be a new session 
#[derive(Debug)]
pub struct ServerSessionKey {
    /// The server session key, should not be the same as the login session key.
    pub session_key: u32,
}

impl SimpleElement for ServerSessionKey {

    fn encode(&self, write: &mut impl Write) -> io::Result<u8> {
        write.write_u32(self.session_key)?;
        Ok(0)
    }

    fn decode(read: &mut impl Read, _len: usize, _id: u8) -> io::Result<Self> {
        Ok(Self { session_key: read.read_u32()? })
    }
    
}


/// Sent by the client on login (and apparently randomly after login) to return 
/// the session key that was sent by the server in the [`ServerSessionKey`] 
/// reply.
#[derive(Debug)]
pub struct ClientSessionKey {
    /// The server session key
    pub session_key: u32,
}

impl ClientSessionKey {
    pub const ID: u8 = 0x01;
}

impl SimpleElement for ClientSessionKey {

    fn encode(&self, write: &mut impl Write) -> io::Result<u8> {
        write.write_u32(self.session_key)?;
        Ok(Self::ID)
    }

    fn decode(read: &mut impl Read, _len: usize, _id: u8) -> io::Result<Self> {
        Ok(Self { session_key: read.read_u32()? })
    }

}

impl TopElement for ClientSessionKey {
    const LEN: ElementLength = ElementLength::Fixed(4);
}


/// Sent by the client to the base app to call a cell method for the given
/// entity ID.
#[derive(Debug)]
pub struct CellEntityMethod {
    /// The entity ID on which we'll call the method, must be set to 0 if
    /// the current player is targeted.
    pub entity_id: u32,
    /// The raw data of the method call.
    pub data: Vec<u8>
}

impl CellEntityMethod {

    pub const FIRST_ID: u8 = 0x0F;
    pub const LAST_ID: u8  = 0x87;

    /// Convert a method index to a message id.
    pub const fn index_to_id(index: u8) -> u8 {
        Self::FIRST_ID + index
    }

    /// Convert a message id to method index.
    pub const fn id_to_index(id: u8) -> u8 {
        id - Self::FIRST_ID
    }

}

impl SimpleElement for CellEntityMethod {

    fn encode(&self, write: &mut impl Write) -> io::Result<u8> {
        write.write_u32(self.entity_id)?;
        write.write_blob(&self.data)?;
        Ok(0) // TODO:
    }

    fn decode(read: &mut impl Read, len: usize, _id: u8) -> io::Result<Self> {
        Ok(Self { // TODO: use id
            entity_id: read.read_u32()?,
            data: read.read_blob(len - 4)?,
        })
    }

}

impl TopElement for CellEntityMethod {
    const LEN: ElementLength = ElementLength::Variable16;
}


/// Sent by the client to the base app to call a base method for the 
/// currently connected entity.
#[derive(Debug)]
pub struct BaseEntityMethod {
    pub data: Vec<u8>,
}

impl BaseEntityMethod {

    pub const FIRST_ID: u8 = 0x88;
    pub const LAST_ID: u8  = 0xFE;

    /// Convert a method index to a message id.
    pub const fn index_to_id(index: u8) -> u8 {
        Self::FIRST_ID + index
    }

    /// Convert a message id to method index.
    pub const fn id_to_index(id: u8) -> u8 {
        id - Self::FIRST_ID
    }

}

impl SimpleElement for BaseEntityMethod {

    fn encode(&self, write: &mut impl Write) -> io::Result<u8> {
        write.write_blob(&self.data)?;
        Ok(0) // TODO:
    }

    fn decode(read: &mut impl Read, len: usize, _id: u8) -> io::Result<Self> {
        Ok(Self { // TODO: use id
            data: read.read_blob(len - 4)?,
        })
    }

}

impl TopElement for BaseEntityMethod {
    const LEN: ElementLength = ElementLength::Variable16;
}
