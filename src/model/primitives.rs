//! Primitives processed file.

use std::io::{self, Read, Seek, SeekFrom};
use std::collections::HashMap;

use crate::util::io::WgReadExt;


/// Primitives file reader utility.
pub struct ModelPrimitives<R> {
    pub inner: R,
    sections: HashMap<String, SectionMeta>,
}

impl<R: Read + Seek> ModelPrimitives<R> {

    pub fn new(mut inner: R) -> io::Result<Self> {

        let mut sections = HashMap::new();

        inner.seek(SeekFrom::End(-4))?;
        let mut table_len = inner.read_u32()? as usize;
        inner.seek(SeekFrom::End(-4 - table_len as i64))?;

        let mut section_offset = 4;

        while table_len != 0 {

            let section_len = inner.read_u32()? as usize;
            inner.skip::<16>()?;
            let section_name_len = inner.read_u32()? as usize;
            let section_name = inner.read_string(section_name_len)?;

            sections.insert(section_name.clone(), SectionMeta {
                name: section_name,
                off: section_offset,
                len: section_len,
            });
            
            // Keep the alignment of the section offset.
            section_offset += section_len;
            if section_len % 4 != 0 {
                section_offset += 4 - section_len % 4;
            }

            // Keep the alignment of the table cursor.
            table_len -= 24; // Remove the two u32 and the 16 skept bytes.
            if section_name_len % 4 != 0 {
                let pad = 4 - section_name_len % 4;
                let mut buf = [0; 4];
                inner.read_exact(&mut buf[..pad])?;
                table_len -= pad; // Also remove the padding from the current length.
            }

        }

        Ok(Self {
            inner,
            sections,
        })

    }

    #[inline]
    pub fn get_section(&self, name: &str) -> Option<&SectionMeta> {
        self.sections.get(name)
    }

}

/// 
pub struct SectionMeta {
    pub name: String,
    pub off: usize,
    pub len: usize,
}
