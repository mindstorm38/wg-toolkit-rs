//! Module for `AssetList` section.

use std::io::{Read, Seek};

use super::{Section, SectionId, ReadSectionExt};


#[derive(Debug)]
pub struct BWAL {
    assets: Vec<AssetInfo>
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


#[derive(Debug)]
pub struct AssetInfo {
    asset_type: AssetType,
    string_fnv: u32
}

#[derive(Debug)]
pub enum AssetType {
    ParticlesResource,
    WaterReflectionTexture,
    ControlPointRadiusPath,
    ModelResource
}
