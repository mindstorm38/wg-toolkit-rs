use std::io::{Read, Seek};

use super::{Section, SectionId, ReadSectionExt};


/// Terrain2 section, providing many information about `cdata_processed` files and many
/// settings for the terrain.
#[derive(Debug)]
pub struct BWT2 {
    pub settings1: TerrainSettings1,
    pub settings2: TerrainSettings2,
    pub chunks: Vec<TerrainChunk>,
    pub lod_distances: Vec<f32>,
    pub outland_cascades: Vec<OutlandCascade>,
    pub tiles_fnv: Vec<u32>
}

impl Section for BWT2 {

    const ID: &'static SectionId = b"BWT2";

    fn decode<R: Read + Seek>(read: &mut R) -> std::io::Result<Self> {

        let settings1_size = read.read_single_head()?;
        assert_eq!(settings1_size, 32);
        let settings1 = TerrainSettings1 {
            chunk_size: read.read_f32()?,
            min_x: read.read_i32()?,
            max_x: read.read_i32()?,
            min_y: read.read_i32()?,
            max_y: read.read_i32()?,
            normal_map_fnv: read.read_u32()?,
            global_map_fnv: read.read_u32()?,
            noise_texture_fnv: read.read_u32()?
        };

        let chunks = read.read_vector(|buf| {
            Ok(TerrainChunk {
                resource_fnv: buf.read_u32()?,
                loc_x: buf.read_i16()?,
                loc_y: buf.read_i16()?
            })
        })?;

        // currently unused
        let _3 = read.read_vector(|buf| buf.read_u32())?;

        let settings2_size = read.read_single_head()?;
        assert_eq!(settings2_size, 128);
        let terrain_version = read.read_u32()?;
        let terrain_flags = read.read_u32()?;
        let settings2 = TerrainSettings2 {
            terrain_version,
            blend_map_caching: terrain_flags & 0x1 != 0,
            normal_map_caching: terrain_flags & 0x2 != 0,
            enable_auto_rebuild_normal_map: terrain_flags & 0x8 != 0,
            enable_auto_rebuild_water_geometry: terrain_flags & 0x20 != 0,
            height_map_size: read.read_u32()?,
            normal_map_size: read.read_u32()?,
            hole_map_size: read.read_u32()?,
            shadow_map_size: read.read_u32()?,
            blend_map_size: read.read_u32()?,
            lod_texture_distance: read.read_f32()?,
            macro_lod_start: read.read_f32()?,
            start_bias: { read.skip::<4>()?; read.read_f32()? },
            end_bias: read.read_f32()?,
            direct_occlusion: read.read_f32()?,
            reverb_occlusion: read.read_f32()?,
            wrap_u: read.read_f32()?,
            wrap_v: read.read_f32()?,
            blend_macro_influence: {
                read.skip::<16>()?;
                read.read_f32()?
            },
            blend_global_threshold: read.read_f32()?,
            blend_height: read.read_f32()?,
            disabled_blend_height: read.read_f32()?,
            vt_lod_params: [
                read.read_f32()?,
                read.read_f32()?,
                read.read_f32()?,
                read.read_f32()?,
            ],
            bounding_box: [
                read.read_f32()?,
                read.read_f32()?,
                read.read_f32()?,
                read.read_f32()?,
            ]
        };

        let lod_distances = read.read_vector(|buf| buf.read_f32())?;

        // currently unused
        let _6 = read.read_vector(|buf| { buf.read_u32()?; buf.read_u32() })?;

        let outland_cascades = read.read_vector(|buf| {
            Ok(OutlandCascade {
                extent_min: [
                    buf.read_f32()?,
                    buf.read_f32()?,
                    buf.read_f32()?,
                ],
                extent_max: [
                    buf.read_f32()?,
                    buf.read_f32()?,
                    buf.read_f32()?,
                ],
                height_map_fnv: buf.read_u32()?,
                normal_map_fnv: buf.read_u32()?,
                tile_map_fnv: buf.read_u32()?,
                tile_scale: buf.read_f32()?
            })
        })?;

        let tiles_fnv = read.read_vector(|buf| buf.read_u32())?;

        Ok(BWT2 {
            settings1,
            settings2,
            chunks,
            lod_distances,
            outland_cascades,
            tiles_fnv
        })

    }

}


/// Terrain settings v1.
/// Decoded by [BWT2] section.
#[derive(Debug)]
pub struct TerrainSettings1 {
    /// space.settings/chunkSize or 100.0 by default
    pub chunk_size: f32,
    /// space.settings/bounds
    pub min_x: i32,
    /// space.settings/bounds
    pub max_x: i32,
    /// space.settings/bounds
    pub min_y: i32,
    /// space.settings/bounds
    pub max_y: i32,
    pub normal_map_fnv: u32,
    /// global_AM.dds, maybe tintTexture - global terrain albedo map
    pub global_map_fnv: u32,
    pub noise_texture_fnv: u32
}


/// Terrain chunk definition, each chunk has its own `cdata_processed` archive.
/// Each chunk has a size defined by [chunk_size](TerrainSettings1.chunk_size).
/// Decoded by [BWT2] section.
#[derive(Debug)]
pub struct TerrainChunk {
    /// Resource FNV hash, you can find the path to the `cdata_processed` archive by resolving
    /// this hash in the [BWST](super::BWST) section.
    pub resource_fnv: u32,
    pub loc_x: i16,
    pub loc_y: i16
}


/// Terrain settings v2.
/// Decoded by [BWT2] section.
#[derive(Debug)]
pub struct TerrainSettings2 {
    /// space.settings/terrain/version
    pub terrain_version: u32,
    /// terrain/blendMapCaching
    pub blend_map_caching: bool,
    /// terrain/normalMapCaching
    pub normal_map_caching: bool,
    /// terrain/editor/enableAutoRebuildNormalMap
    pub enable_auto_rebuild_normal_map: bool,
    /// terrain/editor/enableAutoRebuildWaterGeometry
    pub enable_auto_rebuild_water_geometry: bool,
    /// terrain/heightMapSize
    pub height_map_size: u32,
    /// terrain/normalMapSize
    pub normal_map_size: u32,
    /// terrain/holeMapSize
    pub hole_map_size: u32,
    /// terrain/shadowMapSize
    pub shadow_map_size: u32,
    /// terrain/blendMapSize
    pub blend_map_size: u32,
    /// terrain/lodInfo/lodTextureDistance
    pub lod_texture_distance: f32,
    /// terrain/lodInfo/macroLODStart
    pub macro_lod_start: f32,
    /// terrain/lodInfo/startBias
    pub start_bias: f32,
    /// terrain/lodInfo/endBias
    pub end_bias: f32,
    /// terrain/soundOcclusion/directOcclusion
    pub direct_occlusion: f32,
    /// terrain/soundOcclusion/reverbOcclusion
    pub reverb_occlusion: f32,
    /// terrain/detailNormal/wrapU
    pub wrap_u: f32,
    /// terrain/detailNormal/wrapV
    pub wrap_v: f32,
    /// terrain/blendMacroInfluence
    pub blend_macro_influence: f32,
    /// terrain/blendGlobalThreshold
    pub blend_global_threshold: f32,
    /// terrain/blendHeight
    pub blend_height: f32,
    /// terrain/disabledBlendHeight
    pub disabled_blend_height: f32,
    /// terrain/VTLodParams
    pub vt_lod_params: [f32; 4],
    pub bounding_box: [f32; 4],
}


/// Definition of a cascade in the terrain.
/// Decoded by [BWT2] section.
#[derive(Debug)]
pub struct OutlandCascade {
    pub extent_min: [f32; 3],
    pub extent_max: [f32; 3],
    pub height_map_fnv: u32,
    pub normal_map_fnv: u32,
    pub tile_map_fnv: u32,
    pub tile_scale: f32,
}
