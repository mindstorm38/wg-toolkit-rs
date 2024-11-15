//! Definition of elements related to base application.
//! 
//! Such elements are sent from the client to the base application and also
//! replies to such elements if they are requests.

use std::io::{self, Read, Write};

use crate::net::element::{SimpleElement, ElementLength};
use crate::util::io::*;


/// Internal module containing all raw elements numerical ids.
pub mod id {

    use crate::net::element::ElementIdRange;

    pub const CLIENT_AUTH: u8           = 0x00;
    pub const CLIENT_SESSION_KEY: u8    = 0x01;

    // pub const CELL_ENTITY_METHOD: ElementIdRange = ElementIdRange::new(0x0F, 0x87);
    pub const BASE_ENTITY_METHOD: ElementIdRange = ElementIdRange::new(0x87, 0xFE);

}


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

impl SimpleElement for ClientAuth {

    const ID: u8 = id::CLIENT_AUTH;
    const LEN: ElementLength = ElementLength::Fixed(7);
    
    fn encode(&self, write: &mut dyn Write) -> io::Result<()> {
        write.write_u32(self.login_key)?;
        write.write_u8(self.attempt_num)?;
        write.write_u16(self.unk)
    }

    fn decode(read: &mut dyn Read, _len: usize) -> io::Result<Self> {
        Ok(Self {
            login_key: read.read_u32()?, 
            attempt_num: read.read_u8()?,
            unk: read.read_u16()?,
        })
    }
    

}


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

impl SimpleElement for ClientSessionKey {

    const ID: u8 = id::CLIENT_SESSION_KEY;
    const LEN: ElementLength = ElementLength::Fixed(4);
    
    fn encode(&self, write: &mut dyn Write) -> io::Result<()> {
        write.write_u32(self.session_key)
    }

    fn decode(read: &mut dyn Read, _len: usize) -> io::Result<Self> {
        Ok(Self { session_key: read.read_u32()? })
    }

}


// /// Sent by the client to the base app to call a cell method for the given
// /// entity ID.
// #[derive(Debug, Clone)]
// pub struct CellEntityMethod<M: MethodCall> {
//     /// The entity ID on which we'll call the method, must be set to 0 if
//     /// the current player is targeted.
//     pub entity_id: u32,
//     /// The method call.
//     pub method: M,
// }

// impl<M: MethodCall> CellEntityMethod<M> {

//     /// Write this cell entity method call to the given bundle.
//     pub fn write(self, writer: BundleElementWriter) {
//         MethodCallWrapper::new(self.method, CellEntityMethodExt {
//             entity_id: self.entity_id,
//         }).write(writer);
//     }

//     /// Read this cell entity method call from the given top element reader.
//     pub fn read(reader: TopElementReader) -> io::Result<BundleElement<Self>> {
//         MethodCallWrapper::<M, CellEntityMethodExt>::read(reader).map(|res| {
//             res.map(|wrapper| Self {
//                 entity_id: wrapper.ext.entity_id,
//                 method: wrapper.method,
//             })
//         })
//     }

// }

// struct CellEntityMethodExt {
//     entity_id: u32,
// }

// impl MethodCallExt for CellEntityMethodExt {
//     const ID_RANGE: ElementIdRange = id::CELL_ENTITY_METHOD;
// }

// impl SimpleElement for CellEntityMethodExt {

//     fn encode(&self, write: &mut impl Write) -> io::Result<()> {
//         write.write_u32(self.entity_id)
//     }

//     fn decode(read: &mut impl Read, _len: usize) -> io::Result<Self> {
//         Ok(Self { entity_id: read.read_u32()? })
//     }

// }

// impl TopElement for CellEntityMethodExt {
//     const LEN: ElementLength = ElementLength::Variable16;
// }


// /// Sent by the client to the base app to call a base method for the 
// /// currently connected entity.
// #[derive(Debug, Clone)]
// pub struct BaseEntityMethod<M: MethodCall> {
//     pub method: M,
// }

// impl<M: MethodCall> BaseEntityMethod<M> {

//     /// Write this base entity method call to the given bundle.
//     pub fn write(self, writer: BundleElementWriter) {
//         MethodCallWrapper::new(self.method, BaseEntityMethodExt).write(writer);
//     }

//     /// Read this base entity method call from the given top element reader.
//     pub fn read(reader: TopElementReader) -> io::Result<BundleElement<Self>> {
//         MethodCallWrapper::<M, BaseEntityMethodExt>::read(reader).map(|res| {
//             res.map(|wrapper| Self {
//                 method: wrapper.method,
//             })
//         })
//     }

// }

// #[derive(Default)]
// struct BaseEntityMethodExt;

// impl MethodCallExt for BaseEntityMethodExt {
//     const ID_RANGE: ElementIdRange = id::BASE_ENTITY_METHOD;
// }

// impl NoopElement for BaseEntityMethodExt {}

// impl TopElement for BaseEntityMethodExt {
//     const LEN: ElementLength = ElementLength::Variable16;
// }
