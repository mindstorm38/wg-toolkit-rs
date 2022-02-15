use std::io::{Read, Seek};

use super::{Section, SectionId, ReadSectionExt};


/// CompiledSpaceSettings section.
#[derive(Debug)]
pub struct BWCS {
    pub values: [f32; 6]
}

impl Section for BWCS {

    const ID: &'static SectionId = b"BWCS";

    fn decode<R: Read + Seek>(read: &mut R) -> std::io::Result<Self> {

        let size = read.read_single_head()?;
        assert_eq!(size, 24);

        let mut values = [0.0; 6];
        for value in &mut values {
            *value = read.read_f32()?;
        }

        Ok(BWCS { values })

    }

}
