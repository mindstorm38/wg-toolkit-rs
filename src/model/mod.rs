//! Compiled model memory representation, encoding and decoding.

pub mod primitives;
pub mod visual;

use std::io::{self, Read, Seek};

use glam::{Vec3A, Affine3A};

use crate::pxml::from_reader;
use crate::pxml::de::DeError as PxmlDeError;
use self::primitives::PrimitivesReader;


// /// Decode and resolve a compiled model.
// pub fn read_model<Rv, Rp>(visual: Rv, primitives: Rp) -> ModelResult<Model>
// where
//     Rv: Read + Seek,
//     Rp: Read + Seek,
// {

//     let root_elt = from_reader(visual)?;
//     let root_node_elt = root_elt.get_child("node").ok_or(ModelError::MalformedVisual)?;

//     let primitives_reader = PrimitivesReader::open(primitives)?;
//     for meta in primitives_reader.iter_sections_meta() {
//         println!("- {meta:?}");
//     }

//     // Decode the root node.
//     let mut root_node = Node::default();
//     read_node(root_node_elt, &mut root_node);

//     let mut model = Model {
//         root_node,
//         render_sets: Vec::new(),
//         bounding_box: BoundingBox::default(),
//         geometry_size: 0,
//         min_uv_density: 0.0,
//     };
    
//     for raw in &root_elt.children {
//         if let Some(elt) = raw.as_element() {
//             match &elt.name[..] {
//                 "renderSet" => {
//                     read_render_set(elt, &mut model);
//                 }
//                 "boundingBox" => {
//                     let min = elt.get_child("min").ok_or(ModelError::MalformedVisual)?;
//                     let max = elt.get_child("max").ok_or(ModelError::MalformedVisual)?;
//                     model.bounding_box.min = min.to_vec3().ok_or(ModelError::MalformedVisual)?;
//                     model.bounding_box.max = max.to_vec3().ok_or(ModelError::MalformedVisual)?;
//                 }
//                 a => {
//                     println!("unhandled root element: {a}")
//                 }
//             }
//         }
//     }

//     Ok(model)

// }


// /// Internal recursive function used to read single layer of node,
// /// and recurse reading its children.
// fn read_node(elt: &Element, node: &mut Node) {
//     for raw in &elt.children {
//         if let Some(elt) = raw.as_element() {
//             match &elt.name[..] {
//                 "identifier" => {
//                     if let Some(text) = elt.get_text() {
//                         node.identifier.replace_range(.., &*text);
//                     }
//                 }
//                 "transform" => {
//                     if let Some(transform) = elt.to_affine3() {
//                         node.transform = transform;
//                     }
//                 }
//                 "node" => {
//                     node.children.push(Node::default());
//                     read_node(elt, &mut node.children.last_mut().unwrap());
//                 }
//                 _ => {}
//             }
//         }
//     }
// }


// fn read_render_set(elt: &Element, model: &mut Model) {
    
//     for node in &elt.children {
//         if let Some(elt) = node.as_element() {
//             match &elt.name[..] {
//                 "geometry" => {
//                     if let Some(text) = elt.get_text() {
                        
//                     }
//                 }
//                 _ => {}
//             }
//         }
//     }

// }


// #[derive(Debug)]
// pub struct RenderSet {
//     pub node: String,
//     pub geometry: Geometry,
//     pub treat_as_world_space_object: bool,
// }

// #[derive(Debug)]
// pub struct Geometry {
//     pub vertices: String,
//     pub primitive: String,
// }

// #[derive(Debug, Default)]
// pub struct Node {
//     pub identifier: String,
//     pub transform: Affine3A,
//     pub children: Vec<Node>,
// }

// #[derive(Debug, Default)]
// pub struct BoundingBox {
//     pub min: Vec3A,
//     pub max: Vec3A,
// }

// #[derive(Debug)]
// pub struct Model {
//     pub render_sets: Vec<RenderSet>,
//     pub root_node: Node,
//     pub bounding_box: BoundingBox,
//     pub geometry_size: usize,
//     pub min_uv_density: f32,
// }


// /// Type alias for result with a generic ok type and an [`XmlError`] error type.
// pub type ModelResult<T> = Result<T, ModelError>;

// /// An error that can happen while reading a model.
// #[derive(Debug)]
// pub enum ModelError {
//     /// The XML visual data is malformed. 
//     MalformedVisual,
//     /// Underlying unhandled XML error, usually reading visual.
//     Pxml(PxmlDeError),
//     /// Underlying unhandled IO error.
//     Io(io::Error),
// }

// impl From<PxmlDeError> for ModelError {
//     fn from(err: PxmlDeError) -> Self {
//         Self::Pxml(err)
//     }
// }

// impl From<io::Error> for ModelError {
//     fn from(err: io::Error) -> Self {
//         Self::Io(err)
//     }
// }
