//! Packed XML codec. This codec is widely use in Wargaming games' files.
//! 
//! This is basically a binary compression of an XML file, excepting that
//! some pattern can't be reproduced into. This is why a custom set of
//! structures are introduced in this module to handle this case, such as
//! [`Value`] and [`Element`].

use glam::{Vec3A, Affine3A};
use smallvec::SmallVec;

pub mod de;
pub mod ser;

pub use de::{from_reader, from_bytes};
pub use ser::{to_writer};


/// Magic of a packed XML file.
pub const MAGIC: &[u8; 4] = b"\x45\x4E\xA1\x62";


/// A packed XML untyped value.
#[derive(Debug)]
pub enum Value {
    Element(Box<Element>),
    String(String),
    Integer(i64),
    Bool(bool),
    Float(f32),
    Vec3(Vec3A),
    Affine3(Affine3A),
    Blob(Vec<u8>),
}

/// A packed element.
#[derive(Debug)]
pub struct Element {
    /// Proper value of a element.
    pub value: Value,
    /// Children values, each value is mapped to a name that
    /// is not guaranteed to be unique.
    children: SmallVec<[(String, Value); 8]>,
}

impl Element {

    pub fn new() -> Self {
        Self { 
            value: Value::Bool(false), 
            children: SmallVec::new(),
        }
    }

    pub fn add_children<S: Into<String>>(&mut self, key: S, value: Value) {
        self.children.push((key.into(), value));
    }

    pub fn iter_children<'a, 'b: 'a>(&'a self, key: &'b str) -> impl Iterator<Item = &'a Value> + 'a {
        self.children.iter().filter_map(move |(k, v)| (k == key).then_some(v))
    }

    pub fn get_children<'a, 'b: 'a>(&'a self, key: &'b str) -> Option<&'a Value> {
        self.iter_children(key).next()
    }

}


/// Internally used data types for values.
#[derive(Debug, Clone, Copy)]
enum DataType {
    Element = 0,
    String = 1,
    Integer = 2,
    Float = 3,
    Boolean = 4,
    Blob = 5,
}

impl DataType {

    /// Return the data type from its raw 
    fn from_raw(raw: u32) -> Option<Self> {
        Some(match raw {
            0 => Self::Element,
            1 => Self::String,
            2 => Self::Integer,
            3 => Self::Float,
            4 => Self::Boolean,
            5 => Self::Blob,
            _ => return None
        })
    }

    fn to_raw(self) -> u32 {
        match self {
            DataType::Element => 0,
            DataType::String => 1,
            DataType::Integer => 2,
            DataType::Float => 3,
            DataType::Boolean => 4,
            DataType::Blob => 5
        }
    }

}