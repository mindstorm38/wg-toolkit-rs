//! Module for `StringTable` section.

use std::io::{Read, Seek, SeekFrom};
use std::collections::HashMap;

use super::{Section, SectionId, ReadSectionExt};
use crate::fnv::fnv1a_64;


#[derive(Debug)]
pub struct BWST {
    pub strings: HashMap<u32, String>
}

impl Section for BWST {

    const ID: &'static SectionId = b"BWST";

    fn decode<R: Read + Seek>(read: &mut R) -> std::io::Result<Self> {

        let entries = read.read_vector(|buf| {
            Ok((buf.read_u32()?, buf.read_u32()? as u64, buf.read_u32()? as usize))
        })?;

        // Currently useless because entries should be valid.
        let strings_len = read.read_u32()? as u64;
        let strings_off = read.stream_position()?;

        let mut strings = HashMap::with_capacity(entries.len());

        for (_key, off, len) in entries {
            read.seek(SeekFrom::Start(strings_off + off))?;
            let mut buf = Vec::with_capacity(len);
            buf.resize(len, 0);
            read.read_exact(&mut buf[..])?;
            let fnv = get_hash(&buf[..]);
            strings.insert(fnv, String::from_utf8(buf).unwrap());
        }

        read.seek(SeekFrom::Start(strings_off + strings_len))?;

        Ok(BWST { strings })

    }

}

impl BWST {

    pub fn get_string(&self, hash: u32) -> Option<&str> {
        Some(self.strings.get(&hash)?.as_str())
    }

}


/// Get FNV hash for given data.
pub fn get_hash(data: &[u8]) -> u32 {
    (fnv1a_64(data) & 0xFFFFFFFF) as u32
}

/// Get FNV hash for given string.
pub fn get_hash_from_str(string: &str) -> u32 {
    get_hash(string.as_bytes())
}
