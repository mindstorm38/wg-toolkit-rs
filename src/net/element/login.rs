//! Definition of all predefined

use std::io::{Read, Seek, Write};
use byteorder::{ReadBytesExt, WriteBytesExt};
use crate::net::element::ElementLength;
use super::ElementCodec;


pub struct LoginElement {

}

impl LoginElement {
    pub const ID: u8 = 0x00;
}

impl ElementCodec for LoginElement {

    const LEN: ElementLength = ElementLength::Variable16;

    fn encode<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        todo!()
    }

    fn decode<R: Read + Seek>(input: &mut R) -> std::io::Result<Self> {
        todo!()
    }

}


#[derive(Debug)]
pub struct PingElement(pub u8);

impl PingElement {
    pub const ID: u8 = 0x02;
}

impl ElementCodec for PingElement {

    const LEN: ElementLength = ElementLength::Fixed(1);

    fn encode<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        out.write_u8(self.0)
    }

    fn decode<R: Read + Seek>(input: &mut R) -> std::io::Result<Self> {
        input.read_u8().map(PingElement)
    }

}
