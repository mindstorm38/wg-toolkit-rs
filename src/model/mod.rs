//! Compiled model memory representation, encoding and decoding.

use std::io::{Read, Seek};

pub mod primitive;
pub mod visual;

use self::visual::{Visual, RenderSet};
use self::primitive::{PrimitiveReader, Vertices, Indices, Vertex, Primitive, Group};


/// Decode and resolve a compiled model.
pub fn from_readers<Rv, Rp>(visual_reader: Rv, primitive_reader: Rp) -> Result<Model, ()>
where
    Rv: Read + Seek,
    Rp: Read + Seek,
{

    let visual = visual::from_reader(visual_reader).unwrap();
    let mut primitive_reader = PrimitiveReader::open(primitive_reader).unwrap();
    let mut render_sets_data = Vec::new();

    for render_set in &visual.render_sets {

        let vertices = primitive_reader
            .read_section::<Vertices>(&render_set.geometry.vertices_section)
            .unwrap().unwrap();

        let indices = primitive_reader
            .read_section::<Indices>(&render_set.geometry.indices_section)
            .unwrap().unwrap();

        render_sets_data.push(RenderSetData {
            vertices: vertices.vertices,
            primitives: indices.primitives,
            groups: indices.groups,
        });

    }

    Ok(Model {
        visual, 
        render_sets_data,
    })

}


#[derive(Debug)]
pub struct Model {
    /// Description of the visual components of the model.
    pub visual: Box<Visual>,
    /// Decoded data for each render set.
    pub render_sets_data: Vec<RenderSetData>,
}

#[derive(Debug)]
pub struct RenderSetData {
    /// All vertices for the model. To access correct vertices,
    /// use correct method of the model to get access to them.
    pub vertices: Vec<Vertex>,
    /// Indices of the model, linking all vertices.
    pub primitives: Vec<Primitive>,
    /// Groups of indices.
    pub groups: Vec<Group>,
}

impl Model {

    /// Shortcut method to get a render set metadata together with its data.
    pub fn get_render_set(&self, index: usize) -> (&RenderSet, &RenderSetData) {
        (
            &self.visual.render_sets[index],
            &self.render_sets_data[index],
        )
    }

}

impl RenderSetData {

    pub fn get_group(&self, index: usize) -> (&[Vertex], &[Primitive]) {
        let group = &self.groups[index];
        (
            &self.vertices[group.vertices_offset as usize..][..group.vertices_count as usize],
            &self.primitives[group.primitives_offset as usize..][..group.primitives_count as usize],
        )
    }

}


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
