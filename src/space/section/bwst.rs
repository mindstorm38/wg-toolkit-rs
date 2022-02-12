//! Module for `StringTable` section.

use std::io::{Read, Seek, SeekFrom};

use super::{Section, SectionId, ReadSectionExt};


#[derive(Debug)]
pub struct BWST {
    pub strings: Vec<String>
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

        let mut strings = Vec::with_capacity(entries.len());

        for (_key, off, len) in entries {
            read.seek(SeekFrom::Start(strings_off + off))?;
            let mut buf = Vec::with_capacity(len);
            buf.resize(len, 0);
            read.read_exact(&mut buf[..])?;
            strings.push(String::from_utf8(buf).unwrap());
        }

        read.seek(SeekFrom::Start(strings_off + strings_len))?;

        Ok(BWST { strings })

    }

}
