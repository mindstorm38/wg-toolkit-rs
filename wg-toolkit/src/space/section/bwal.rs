use std::io::{Read, Seek};

use crate::util::io::WgReadExt;

use super::{Section, SectionId};


/// AssetList section, defines a list of assets for this space.
#[derive(Debug)]
pub struct BWAL {
    pub assets: Vec<AssetInfo>
}

impl Section for BWAL {

    const ID: &'static SectionId = b"BWAL";

    fn decode<R: Read + Seek>(read: &mut R) -> std::io::Result<Self> {

        let assets = read.read_vector(|buf| {

            let asset_type = match buf.read_u32()? {
                1 => AssetType::ParticlesResource,
                2 => AssetType::WaterReflectionTexture,
                5 => AssetType::ControlPointRadiusPath,
                6 => AssetType::ModelResource,
                _ => panic!("invalid asset type")
            };

            Ok(AssetInfo {
                asset_type,
                string_fnv: buf.read_u32()?
            })

        })?;

        Ok(BWAL { assets })

    }

}


/// An compiled space asset info.
/// Decoded by [BWAL] section.
#[derive(Debug)]
pub struct AssetInfo {
    pub asset_type: AssetType,
    pub string_fnv: u32
}


/// An asset type for an [AssetInfo].
/// Decoded by [BWAL] section.
#[derive(Debug)]
pub enum AssetType {
    ParticlesResource,
    WaterReflectionTexture,
    ControlPointRadiusPath,
    ModelResource
}
