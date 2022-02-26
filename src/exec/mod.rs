//! Automatic disassembly of important information from game's executable.

use std::path::Path;

use pelite::pe64::{Pe, PeFile};
use pelite::FileMap;

use iced_x86::Decoder;



pub struct Disassembly<'a> {
    file: PeFile<'a>
}

impl<'a> Disassembly<'a> {

    pub fn new(data: &'a [u8]) -> Option<Self> {
        Some(Self {
            file: PeFile::from_bytes(data).ok()?
        })
    }

}
