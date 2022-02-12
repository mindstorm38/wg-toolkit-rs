//! Module for `Terrain2SceneFeature`.

use std::io::{Read, Seek};

use super::{Section, SectionId, ReadSectionExt};


#[derive(Debug)]
pub struct BWT2 {
    settings1: TerrainSettings1,
    settings2: TerrainSettings2,
    cdata: Vec<TerrainChunk>,
    lod_distances: Vec<f32>,
    outland_cascades: Vec<OutlandCascade>,
    tiles_fnv: Vec<u32>
}

impl Section for BWT2 {

    const ID: &'static SectionId = b"BWT2";

    fn decode<R: Read + Seek>(read: &mut R) -> std::io::Result<Self> {
        todo!()
    }

}


#[derive(Debug)]
pub struct TerrainSettings1 {
    /// space.settings/chunkSize or 100.0 by default
    chunk_size: f32,
    /// space.settings/bounds
    min_x: i32,
    /// space.settings/bounds
    max_x: i32,
    /// space.settings/bounds
    min_y: i32,
    /// space.settings/bounds
    max_y: i32,
    normal_map_fnv: u32,
    /// global_AM.dds, maybe tintTexture - global terrain albedo map
    global_map_fnv: u32,
    noise_texture_fnv: u32
}


#[derive(Debug)]
pub struct TerrainChunk {
    resource_fnv: u32,
    loc_x: i16,
    loc_y: i16
}


#[derive(Debug)]
pub struct TerrainSettings2 {
    /// space.settings/terrain/version
    terrain_version: u32,
    /// terrain/blendMapCaching
    blend_map_caching: bool,
    /// terrain/normalMapCaching
    normal_map_caching: bool,
    /// terrain/editor/enableAutoRebuildNormalMap
    enable_auto_rebuild_normal_map: bool,
    /// terrain/editor/enableAutoRebuildWaterGeometry
    enable_auto_rebuild_water_geometry: bool,
    /// terrain/heightMapSize
    height_map_size: u32,
    /// terrain/normalMapSize
    normal_map_size: u32,
    /// terrain/holeMapSize
    hole_map_size: u32,
    /// terrain/shadowMapSize
    shadow_map_size: u32,
    /// terrain/blendMapSize
    blend_map_size: u32,
    /// terrain/lodInfo/lodTextureDistance
    lod_texture_distance: f32,
    /// terrain/lodInfo/macroLODStart
    macro_lod_start: f32,
    /// terrain/lodInfo/startBias
    start_bias: f32,
    /// terrain/lodInfo/endBias
    end_bias: f32,
    /// terrain/soundOcclusion/directOcclusion
    direct_occlusion: f32,
    /// terrain/soundOcclusion/reverbOcclusion
    reverb_occlusion: f32,
    /// terrain/detailNormal/wrapU
    wrap_u: f32,
    /// terrain/detailNormal/wrapV
    wrap_v: f32,
    /// terrain/blendMacroInfluence
    blend_macro_influence: f32,
    /// terrain/blendGlobalThreshold
    blend_global_threshold: f32,
    /// terrain/blendHeight
    blend_height: f32,
    /// terrain/disabledBlendHeight
    disabled_blend_height: f32,
    /// terrain/VTLodParams
    vt_lod_params: [f32; 4],
    bounding_box: [f32; 4],
}


#[derive(Debug)]
pub struct OutlandCascade {
    extent_min: [f32; 3],
    extent_max: [f32; 3],
    height_map_fnv: u32,
    normal_map_fnv: u32,
    tile_map_fnv: u32,
    tile_scale: f32,
}