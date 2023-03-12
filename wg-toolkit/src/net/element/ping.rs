//! Definition of the ping codec for loginapp.

use std::io::{self, Write, Read};

use crate::util::io::*;

use super::{SimpleElement, TopElement, ElementLength};


/// A ping sent from the client to the login app or replied from the
/// login app to the client.
#[derive(Debug)]
pub struct Ping {
    /// The number of the ping, the same number must be sent back to
    /// the client when login app receives it.
    pub num: u8,
}

impl Ping {
    pub const ID: u8 = 0x02;
}

impl SimpleElement for Ping {

    fn encode<W: Write>(&self, mut write: W) -> io::Result<()> {
        write.write_u8(self.num)
    }

    fn decode<R: Read>(mut read: R, len: usize) -> io::Result<Self> {
        Ok(Self { num: read.read_u8()? })
    }

}

impl TopElement for Ping {
    const LEN: ElementLength = ElementLength::Fixed(1);
}
