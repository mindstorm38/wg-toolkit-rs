//! Packed XML codec. This codec is widely use in Wargaming games' files.
//! 
//! This is basically a binary compression of an XML file, excepting that
//! some pattern can't be reproduced into. This is why a custom set of
//! structures are introduced in this module to handle this case, such as
//! [`Value`] and [`Element`].

use glam::{Vec3A, Affine3A};
use smallvec::SmallVec;

mod de;
mod ser;

pub use de::{from_reader, from_bytes, DeError};
pub use ser::to_writer;


/// Magic of a packed XML file.
pub const MAGIC: &[u8; 4] = b"\x45\x4E\xA1\x62";


/// A packed XML untyped value.
#[derive(Debug, Clone)]
pub enum Value {
    Element(Box<Element>),
    String(String),
    Integer(i64),
    Boolean(bool),
    Float(f32),
    Vec3(Vec3A),
    Affine3(Affine3A),
}

/// A packed element.
#[derive(Debug, Clone)]
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
            value: Value::Boolean(false), 
            children: SmallVec::new(),
        }
    }

    pub fn iter_children_all(&self) -> impl Iterator<Item = &'_ (String, Value)> {
        self.children.iter()
    }

    pub fn add_children<S: Into<String>>(&mut self, key: S, value: Value) {
        self.children.push((key.into(), value));
    }

    pub fn iter_children<'a, 'b: 'a>(&'a self, key: &'b str) -> impl Iterator<Item = &'a Value> + 'a {
        self.children.iter().filter_map(move |(k, v)| (k == key).then_some(v))
    }

    pub fn iter_children_mut<'a, 'b: 'a>(&'a mut self, key: &'b str) -> impl Iterator<Item = &'a mut Value> + 'a {
        self.children.iter_mut().filter_map(move |(k, v)| (k == key).then_some(v))
    }

    pub fn get_child<'a, 'b>(&'a self, key: &'b str) -> Option<&'a Value> {
        self.children.iter().find_map(|(k, v)| (k == key).then_some(v))
    }

    pub fn get_child_mut<'a, 'b>(&'a mut self, key: &'b str) -> Option<&'a mut Value> {
        self.children.iter_mut().find_map(|(k, v)| (k == key).then_some(v))
    }

}

impl Value {

    /// Try to get this value as an element if possible.
    #[inline]
    pub fn as_element(&self) -> Option<&Element> {
        if let Self::Element(elt) = self { Some(&**elt) } else { None }
    }

    /// Try to get this value as a string if possible.
    #[inline]
    pub fn as_string(&self) -> Option<&String> {
        if let Self::String(s) = self { Some(s) } else { None }
    }

    /// Try to get this value as an integer if possible.
    #[inline]
    pub fn as_integer(&self) -> Option<i64> {
        if let Self::Integer(n) = *self { Some(n) } else { None }
    }

    /// Try to get this value as a boolean is possible.
    #[inline]
    pub fn as_boolean(&self) -> Option<bool> {
        if let Self::Boolean(b) = *self { Some(b) } else { None }
    }

    /// Try to get this value as a float is possible.
    #[inline]
    pub fn as_float(&self) -> Option<f32> {
        if let Self::Float(n) = *self { Some(n) } else { None }
    }

    /// Try to get this value as a vec3 is possible.
    #[inline]
    pub fn as_vec3(&self) -> Option<Vec3A> {
        if let Self::Vec3(n) = *self { Some(n) } else { None }
    }

    /// Try to get this value as an affine3 is possible.
    #[inline]
    pub fn as_affine3(&self) -> Option<Affine3A> {
        if let Self::Affine3(n) = *self { Some(n) } else { None }
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
    /// This special kind act like a compressed string.
    /// This type is only used when the string to compress has a length
    /// that is a multiple of 4 and composed of the base64 charset. In 
    /// such case the string is base64-decoded, the resulting bytes
    /// are used instead of the string. To get the original string
    /// we need to encode the input.
    CompressedString = 5,
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
            5 => Self::CompressedString,
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
            DataType::CompressedString => 5
        }
    }

}