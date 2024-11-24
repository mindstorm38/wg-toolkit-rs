//! Definition of elements related to base application.
//! 
//! Such elements are sent from the client to the base application and also
//! replies to such elements if they are requests.

use std::io::{self, Read, Write};

use crate::net::element::{ElementLength, Element_, SimpleElement_};
use crate::net::app::common::entity::Method;


/// Internal module containing all raw elements numerical ids.
pub mod id {

    use crate::net::element::ElementIdRange;

    pub const CLIENT_AUTH: u8           = 0x00;
    pub const CLIENT_SESSION_KEY: u8    = 0x01;

    // pub const CELL_ENTITY_METHOD: ElementIdRange = ElementIdRange::new(0x0F, 0x87);
    pub const BASE_ENTITY_METHOD: ElementIdRange = ElementIdRange::new(0x87, 0xFE);

}


crate::__struct_simple_codec! {
    /// Sent by the client to the server without encryption in order to authenticate,
    /// the server then compares with its internal login keys from past successful
    /// logins on the login app.
    /// 
    /// This element is usually a request, in such case a [`ServerSessionKey`] must be 
    /// sent as a reply.
    #[derive(Debug, Clone)]
    pub struct ClientAuth {
        /// The login key that was sent by the login application, part of the  element
        /// [`super::login::LoginSuccess`].
        pub login_key: u32,
        /// The current number of attempts.
        pub attempt_num: u8,
        /// Unknown 16-bits value at the end.
        pub unk: u16,
    }
}

impl SimpleElement_ for ClientAuth {
    const ID: u8 = id::CLIENT_AUTH;
    const LEN: ElementLength = ElementLength::Fixed(7);
}


crate::__struct_simple_codec! {
    /// This element can be used in two cases:
    /// - As a reply to [`ClientAuth`] from the server to the client in order to give it
    ///   the initial session key.
    /// - Sent by the client on login (and apparently randomly after login) to return 
    ///   the session key that was sent by the server in the initial reply (first case).
    #[derive(Debug, Clone)]
    pub struct ClientSessionKey {
        /// The server session key
        pub session_key: u32,
    }
}

impl SimpleElement_ for ClientSessionKey {
    const ID: u8 = id::CLIENT_SESSION_KEY;
    const LEN: ElementLength = ElementLength::Fixed(4);
}


/// Codec for a base entity method call.
///
/// FIXME: For now, this doesn't support sub message id.
#[derive(Debug, Clone)]
pub struct BaseEntityMethod<M: Method> {
    pub inner: M,
}

impl<M: Method> Element_<()> for BaseEntityMethod<M> {

    fn write_length(&self, _config: &()) -> io::Result<ElementLength> {
        Ok(ElementLength::Variable16)
    }

    fn write(&self, write: &mut dyn Write, _config: &()) -> io::Result<u8> {
        let exposed_id = self.inner.write(write)?;
        if exposed_id >= id::BASE_ENTITY_METHOD.slots_count() as u16 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "missing support for sub-id"));
        }
        Ok(id::BASE_ENTITY_METHOD.first + exposed_id as u8)
    }

    fn read_length(_config: &(), _id: u8) -> io::Result<ElementLength> {
        Ok(ElementLength::Variable16)
    }

    fn read(read: &mut dyn Read, _config: &(), _len: usize, id: u8) -> io::Result<Self> {
        if !id::BASE_ENTITY_METHOD.contains(id) {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("unexpected base entity method element id: {id:02X}")));
        }
        let inner = M::read(read, (id - id::BASE_ENTITY_METHOD.first) as u16)?;
        Ok(Self {
            inner,
        })
    }

}
