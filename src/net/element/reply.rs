//! Reply elements, for builtin support of request/replies.
//! These structures are used in `Bundle` structure and sub structures.

use std::io::{self, Read, Seek, Write};

use byteorder::{ReadBytesExt, WriteBytesExt, LE};

use super::{ElementCodec, ElementLength};


pub const REPLY_ID: u8 = 0xFF;


/// A codec just to read the request ID of a reply element. This is used internally
/// by bundle readers.
pub struct ReplyHeaderCodec;

impl ElementCodec for ReplyHeaderCodec {

    const LEN: ElementLength = ElementLength::Variable32;
    type Element = u32;

    fn encode<W: Write>(&self, mut write: W, input: Self::Element) -> io::Result<()> {
        write.write_u32::<LE>(input)
    }

    fn decode<R: Read + Seek>(&self, mut read: R, len: u64) -> io::Result<Self::Element> {
        debug_assert!(len >= 4);
        read.read_u32::<LE>()
    }

}


/// A generic element codec for reply messages.
#[derive(Debug)]
pub struct ReplyCodec<'a, C> {
    codec: &'a C
}

impl<'a, C> ReplyCodec<'a, C> {
    pub fn new(codec: &'a C) -> Self {
        Self { codec }
    }
}

impl<C: ElementCodec> ElementCodec for ReplyCodec<'_, C> {

    const LEN: ElementLength = ElementLength::Variable32;
    type Element = Reply<C::Element>;

    fn encode<W: Write>(&self, mut write: W, input: Self::Element) -> io::Result<()> {
        write.write_u32::<LE>(input.request_id)?;
        self.codec.encode(write, input.element)
    }

    fn decode<R: Read + Seek>(&self, mut read: R, len: u64) -> io::Result<Self::Element> {
        Ok(Reply {
            request_id: read.read_u32::<LE>()?,
            element: self.codec.decode(read, len - 4)? // FIXME: Use a sub cursor to limit seek start.
        })
    }

}


/// A wrapper for a reply element, with the request ID.
#[derive(Debug)]
pub struct Reply<E> {
    /// The request ID this reply is for.
    pub request_id: u32,
    /// The inner reply element.
    pub element: E
}

impl<E> Reply<E> {
    pub fn new(request_id: u32, element: E) -> Self {
        Self { request_id, element }
    }
}