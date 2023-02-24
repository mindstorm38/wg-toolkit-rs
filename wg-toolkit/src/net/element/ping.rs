//! Definition of the ping codec for loginapp.

use std::io::{self, Write, Read};

use byteorder::{WriteBytesExt, ReadBytesExt};

use super::{ElementCodec, ElementLength, TopElementCodec};


/// Codec for ping echo-request exchange. 
/// 
/// This codec transfers a single byte, to validate a ping, this byte
/// must be echoed back to the sender of the ping request.
pub struct PingCodec;

impl PingCodec {
    pub const ID: u8 = 0x02;
}

impl ElementCodec for PingCodec {
    
    type Element = u8;
    
    fn encode<W: Write>(&self, mut write: W, input: Self::Element) -> io::Result<()> {
        write.write_u8(input)
    }
    
    fn decode<R: Read>(&self, mut read: R, _len: usize) -> io::Result<Self::Element> {
        read.read_u8()
    }

}

impl TopElementCodec for PingCodec {
    const LEN: ElementLength = ElementLength::Fixed(1);
}
