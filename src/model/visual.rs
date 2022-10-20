//! Utilities to parse visual processed files.

use std::collections::HashMap;
use std::io::{Seek, Read};

use glam::{Affine3A, Vec3A, Vec4};
use smallvec::SmallVec;

use crate::pxml::{self, Value, Element};


/// Try to read a visual processed file.
pub fn from_reader<R: Read + Seek>(reader: R) -> Result<Box<Visual>, DeError> {

    let root_elt = pxml::from_reader(reader)?;
    println!("{root_elt:#?}");
    let root_node_value = root_elt.get_child("node").ok_or(DeError::MissingRootNode)?;
    let root_node_elt = root_node_value.as_element().ok_or(DeError::MissingRootNode)?;
    let root_node = read_node(root_node_elt).ok_or(DeError::InvalidNode)?;

    let bb_elt = root_elt
        .get_child("boundingBox").ok_or(DeError::MissingBoundingBox)?
        .as_element().ok_or(DeError::MissingBoundingBox)?;

    let bb_min = bb_elt
        .get_child("min").ok_or(DeError::MissingBoundingBox)?
        .as_vec3().ok_or(DeError::MissingBoundingBox)?;

    let bb_max = bb_elt
        .get_child("max").ok_or(DeError::MissingBoundingBox)?
        .as_vec3().ok_or(DeError::MissingBoundingBox)?;

    let geometry_size = root_elt
        .get_child("geometrySize").ok_or(DeError::MissingGeometrySize)?
        .as_integer().ok_or(DeError::MissingGeometrySize)? as u32;

    let min_uv_density = root_elt
        .get_child("minUVDensity").ok_or(DeError::MissingUvDensity)?
        .as_float().ok_or(DeError::MissingUvDensity)?;

    let mut render_sets = SmallVec::new();
    for child in root_elt.iter_children("renderSet") {
        if let Value::Element(child_elt) = child {
            render_sets.push(read_render_set(&**&child_elt).ok_or(DeError::InvalidRenderSet)?);
        }
    }

    Ok(Box::new(Visual {
        root_node,
        render_sets,
        bb_min,
        bb_max,
        geometry_size,
        min_uv_density,
    }))

}


fn read_node(element: &Element) -> Option<Node> {
    
    let identifier = element.get_child("identifier")?.as_string()?;
    let transform = element.get_child("transform")?.as_affine3()?;
    
    let mut children = Vec::new();
    for child in element.iter_children("node") {
        if let Value::Element(child_elt) = child {
            children.push(read_node(&**child_elt)?);
        }
    }

    Some(Node {
        identifier: identifier.clone(),
        transform,
        children,
    })
    
}


fn read_render_set(element: &Element) -> Option<RenderSet> {

    let node = element.get_child("node")?.as_string()?;
    let treat_as_world_space_object = element.get_child("treatAsWorldSpaceObject")?.as_boolean()?;

    let geometry_elt = element.get_child("geometry")?.as_element()?;
    let geometry_vertices = geometry_elt.get_child("vertices")?.as_string()?;
    let geometry_indices = geometry_elt.get_child("primitive")?.as_string()?;

    let mut primitive_groups = SmallVec::new();
    for group_val in geometry_elt.iter_children("primitiveGroup") {
        if let Value::Element(group_elt) = group_val {
            
            let group_index = group_elt.value.as_integer()? as u32;
            let group_origin = group_elt.get_child("groupOrigin")?.as_vec3()?;

            let mat_elt = group_elt.get_child("material")?.as_element()?;
            let mat_identifier = mat_elt.get_child("identifier")?.as_string()?;
            let mat_collision_flags = mat_elt.get_child("collisionFlags")?.as_integer()? as u32;
            let mat_kind = mat_elt.get_child("materialKind")?.as_integer()? as u32;
            let mat_fx = mat_elt.get_child("fx")?.as_string()?;

            let mut mat_properties = HashMap::new();
            for prop_val in mat_elt.iter_children("property") {
                if let Value::Element(prop_elt) = prop_val {

                    let prop_name = prop_elt.value.as_string()?;
                    let prop_value = if let Some(val) = prop_elt.get_child("Texture") {
                        MaterialProperty::Texture(val.as_string()?.clone())
                    } else if let Some(val) = prop_elt.get_child("Bool") {
                        MaterialProperty::Boolean(val.as_boolean()?)
                    } else if let Some(val) = prop_elt.get_child("Int") {
                        MaterialProperty::Integer(val.as_integer()?)
                    } else if let Some(val) = prop_elt.get_child("Float") {
                        // Float properties are weird, because they might be present
                        // in many formats, integers or strings that needs to be parsed.
                        let n = match val {
                            &Value::Integer(n) => n as f32,
                            &Value::Float(n) => n,
                            Value::String(s) => s.parse().ok()?,
                            _ => return None
                        };
                        MaterialProperty::Float(n)
                    } else if let Some(val) = prop_elt.get_child("Vector4") {
                        // A vector 4 property is a string that needs to be parsed.
                        let mut raw = val.as_string()?
                            .splitn(4, ' ')
                            .filter_map(|s| s.parse::<f32>().ok());
                        let x = raw.next()?;
                        let y = raw.next()?;
                        let z = raw.next()?;
                        let w = raw.next()?;
                        MaterialProperty::Vec4(Vec4::new(x, y, z, w))
                    } else {
                        return None;
                    };

                    mat_properties.insert(prop_name.clone(), prop_value);
                    
                }
            }

            primitive_groups.push(PrimitiveGroup {
                index: group_index,
                origin: group_origin,
                material: Material { 
                    identifier: mat_identifier.clone(), 
                    properties: mat_properties,
                    collision_flags: mat_collision_flags,
                    material_kind: mat_kind,
                    fx: mat_fx.clone(),
                },
            })

        }
    }

    Some(RenderSet { 
        node: node.clone(), 
        geometry: Geometry { 
            vertices_section: geometry_vertices.clone(), 
            indices_section: geometry_indices.clone(), 
            primitive_groups,
        }, 
        treat_as_world_space_object,
    })

}


/// Represent an entire visual processed file.
#[derive(Debug)]
pub struct Visual {
    /// The root node.
    pub root_node: Node,
    /// Render sets.
    pub render_sets: SmallVec<[RenderSet; 4]>,
    /// Bounding box minimum.
    pub bb_min: Vec3A,
    /// Bounding box maximum.
    pub bb_max: Vec3A,
    /// Geometry size.
    pub geometry_size: u32,
    /// Minimum U/V density.
    pub min_uv_density: f32,
}

/// Represent a node in the visual tree.
#[derive(Debug)]
pub struct Node {
    /// Identifier of the node.
    pub identifier: String,
    /// Affine transformation applied to this node and its children.
    pub transform: Affine3A,
    /// Children nodes.
    pub children: Vec<Node>,
}

/// Represent a render set for a model's visual.
#[derive(Debug)]
pub struct RenderSet {
    /// Name of the target node for this render set.
    pub node: String,
    /// Geometry definition for this render set.
    pub geometry: Geometry,
    /// Unknown meaning.
    pub treat_as_world_space_object: bool,
}

/// Represent the geometry of a render set.
#[derive(Debug)]
pub struct Geometry {
    /// Identifier of the vertices section in the primitive binary file.
    pub vertices_section: String,
    /// Identifier of the indices section in the primitive binary file.
    pub indices_section: String,
    /// Primitive groups of the geometry.
    pub primitive_groups: SmallVec<[PrimitiveGroup; 1]>,
}

/// Prititive group of a geometry.
#[derive(Debug)]
pub struct PrimitiveGroup {
    /// Index of the primitive group.
    pub index: u32,
    /// Origin of the primitive group.
    pub origin: Vec3A,
    /// Material of the primitive group.
    pub material: Material,
}

#[derive(Debug)]
pub struct Material {
    pub identifier: String,
    pub properties: HashMap<String, MaterialProperty>,
    pub collision_flags: u32,
    pub material_kind: u32,
    pub fx: String,
}

#[derive(Debug)]
pub enum MaterialProperty {
    /// Integer property.
    Texture(String),
    /// A boolean property.
    Boolean(bool),
    /// An integer property.
    Integer(i64),
    /// A float property.
    Float(f32),
    /// A vec4 property.
    Vec4(Vec4),
}

/// Errors that can happend while deserializing a visual processed data.
#[derive(Debug)]
pub enum DeError {
    /// The root node is missing.
    MissingRootNode,
    /// A node is malformed.
    InvalidNode,
    /// A render set is malformed.
    InvalidRenderSet,
    /// The bounding box is missing.
    MissingBoundingBox,
    /// The geometry size if missing.
    MissingGeometrySize,
    /// The U/V density is missing.
    MissingUvDensity,
    /// Underlying Packed XML deserialization error.
    Pxml(pxml::DeError),
}

impl From<pxml::DeError> for DeError {
    fn from(e: pxml::DeError) -> Self {
        Self::Pxml(e)
    }
}
