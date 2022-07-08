use std::collections::HashMap;
use std::io::{Read, Seek};

use super::{Section, SectionId, BWST};
use crate::util::io::WgReadExt;


/// StaticGeometry section, defines models and positions.
#[derive(Debug)]
pub struct BWSG {
    pub strings: HashMap<u32, String>,
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


/// A model information with its resources.
/// Decoded by [BWSG] section.
#[derive(Debug)]
pub struct ModelInfo {
    pub vertices_fnv: u32,
    pub id_from: u32,
    pub id_to: u32,
    pub vertices_count: u32,
    pub vertices_type_fnv: u32
}


/// A position information.
/// Decoded by [BWSG] section.
#[derive(Debug)]
pub struct PositionInfo {
    pub typ: u64,
    /// Size of vertices block from .primitives
    pub size: u32,
    /// Index data_sizes
    pub data_sizes_id: u32,
    /// Start position in `BSGD`
    pub position: u32
}
