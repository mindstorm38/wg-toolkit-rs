//! Definition of elements related to base application.
//! 
//! This modules defines 

use std::io;

use byteorder::{ReadBytesExt, LE, WriteBytesExt};

use super::{ElementCodec, TopElementCodec, ElementLength};


#[derive(Debug)]
pub struct Authenticate {
    pub session_key: u32,
    pub attempts_count: u8,
}

pub struct AuthenticateCodec;

impl ElementCodec for AuthenticateCodec {

    type Element = Authenticate;

    fn encode<W: io::Write>(&self, mut write: W, input: Self::Element) -> io::Result<()> {
        write.write_u32::<LE>(input.session_key)?;
        write.write_u32::<LE>(input.session_key)
    }

    fn decode<R: io::Read>(&self, mut read: R, _len: usize) -> io::Result<Self::Element> {
        Ok(Authenticate { 
            session_key: read.read_u32::<LE>()?, 
            attempts_count: read.read_u8()?,
        })
    }

}

impl TopElementCodec for AuthenticateCodec {
    const LEN: ElementLength = ElementLength::Fixed(4 + 1 + 2);
}


#[derive(Debug)]
pub struct AuthenticateResponse {
    pub session_key: u32,
}
