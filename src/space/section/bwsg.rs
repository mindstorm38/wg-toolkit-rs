//! Module for `StaticGeometry` section.

use std::io::{Read, Seek};
use super::{Section, SectionId, ReadSectionExt, BWST};


#[derive(Debug)]
pub struct BWSG {
    pub strings: Vec<String>,
    pub models: Vec<ModelInfo>,
    pub positions: Vec<PositionInfo>,
}

impl Section for BWSG {

    const ID: &'static SectionId = b"BWSG";

    fn decode<R: Read + Seek>(read: &mut R) -> std::io::Result<Self> {

        // Reuse BWST decoding for strings stored in BWSG.
        let strings = BWST::decode(read)?.strings;

        let models = read.read_vector(|buf| {
            Ok(ModelInfo {
                vertices_fnv: buf.read_u32()?,
                id_from: buf.read_u32()?,
                id_to: buf.read_u32()?,
                vertices_count: buf.read_u32()?,
                vertices_type_fnv: buf.read_u32()?
            })
        })?;

        let positions = read.read_vector(|buf| {
            Ok(PositionInfo {
                typ: buf.read_u64()?,
                size: buf.read_u32()?,
                data_sizes_id: buf.read_u32()?,
                position: buf.read_u32()?
            })
        })?;

        Ok(BWSG {
            strings,
            models,
            positions
        })

    }

}


#[derive(Debug)]
pub struct ModelInfo {
    vertices_fnv: u32,
    id_from: u32,
    id_to: u32,
    vertices_count: u32,
    vertices_type_fnv: u32
}


#[derive(Debug)]
pub struct PositionInfo {
    typ: u64,
    /// Size of vertices block from .primitives
    size: u32,
    /// Index data_sizes
    data_sizes_id: u32,
    /// Start position in `BSGD`
    position: u32
}
