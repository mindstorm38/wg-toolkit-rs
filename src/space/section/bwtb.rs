use std::io::{ Read, Seek, SeekFrom};
use std::fmt::{self, Formatter};
use std::collections::HashMap;

use super::{Section, SectionId, ReadSectionExt};


/// Header section, defining all offsets for real sections. This section is a fake section
/// and doesn't implement the `Section` trait.
pub struct BWTB {
    pub root: SectionMeta,
    pub sections: Vec<SectionMeta>,
    sections_from_id: HashMap<SectionId, usize>
}

impl BWTB {

    pub fn decode<R: Read>(read: &mut R) -> std::io::Result<BWTB> {

        let root = SectionMeta::decode(read)?;
        assert_eq!(&root.id, b"BWTB");

        let mut sections = Vec::with_capacity(root.sections_count);
        for _ in 0..root.sections_count {
            sections.push(SectionMeta::decode(read)?);
        }

        Ok(BWTB {
            root,
            sections_from_id: sections.iter()
                .enumerate()
                .map(|(i, r)| (r.id.clone(), i))
                .collect(),
            sections,
        })

    }

    pub fn get_section_meta(&self, id: &SectionId) -> Option<&SectionMeta> {
        self.sections.get(*self.sections_from_id.get(id)?)
    }

}


/// Metadata for section, its offset and length. Sections count is an internal value only
/// used by the fake BWTB header section.
pub struct SectionMeta {
    pub id: SectionId,
    pub off: usize,
    pub len: usize,
    pub sections_count: usize
}

impl SectionMeta {

    fn decode<R: Read>(read: &mut R) -> std::io::Result<SectionMeta> {

        let mut id = [0; 4];
        read.read_exact(&mut id)?;

        read.read_u32()?;
        let off = read.read_u32()? as usize;
        read.read_u32()?;
        let len = read.read_u32()? as usize;
        let rows_count = read.read_u32()? as usize;

        Ok(SectionMeta {
            id,
            off,
            len,
            sections_count: rows_count
        })

    }

}

impl fmt::Debug for SectionMeta {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("SectionMeta")
            .field("id", &self.id.iter().map(|&c| c as char).collect::<String>())
            .field("off", &self.off)
            .field("len", &self.len)
            .finish()
    }

}