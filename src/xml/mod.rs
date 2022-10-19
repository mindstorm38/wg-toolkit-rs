//! Module for Wargaming packed XML codec.

mod unpack;
mod pack;

pub use unpack::*;
pub use pack::*;

pub mod de;

/// Re-export of xmltree dependency.
pub use xmltree;

use serde::{Deserialize};
use serde::de::{Visitor, Unexpected};

use glam::{Affine3A, Vec3A};
use xmltree::Element;


/// Signature of a packed XML file.
pub const PACKED_SIGNATURE: &[u8; 4] = b"\x45\x4E\xA1\x62";


/// Represent a packed XML untyped value.
pub enum Value {
    Element(Vec<(String, Value)>),
    String(String),
    Integer(i64),
    Bool(bool),
    Vec3(Vec3A),
    Affine3(Affine3A),
    Blob(Vec<u8>),
}


/// Wrapper type for `Vec3A type to be (de)serialize in Wargaming packed XML.
#[derive(Debug)]
pub struct XmlVec3(pub Vec3A);

/// Wrapper type for `Affine3A` type to be (de)serialize in Wargaming packed XML.
#[derive(Debug)]
pub struct XmlAffine3(pub Affine3A);


/// Internal visitor used to read a vec3 element.
struct Vec3Visitor;
impl<'de> Visitor<'de> for Vec3Visitor {

    type Value = XmlVec3;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a vector-3 string representation 'x y z'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error, 
    {

        fn from_str(s: &str) -> Option<XmlVec3> {
            let mut parts = s.split(' ');
            Some(XmlVec3(Vec3A::new(
                parts.next()?.parse().ok()?,
                parts.next()?.parse().ok()?,
                parts.next()?.parse().ok()?,
            )))
        }

        from_str(v).ok_or(E::invalid_value(Unexpected::Str(v), &self))
        
    }

}

impl<'de> Deserialize<'de> for XmlVec3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        deserializer.deserialize_str(Vec3Visitor)
    }
}




/// Extension to the [`xmltree::Element`] to read matrices
/// and vectors as encoded in XML.
pub trait XmlExt {

    /// Read a 4x4 matrix from a XML 4-rows matrix encoding.
    fn to_affine3(&self) -> Option<Affine3A>;

    /// Read a vec3 from the current element interpreted 
    fn to_vec3(&self) -> Option<Vec3A>;

}

impl XmlExt for Element {

    fn to_affine3(&self) -> Option<Affine3A> {

        let mut transform = Affine3A::ZERO;
        let mut rows = 0;

        for row_elt in &self.children {
            if let Some(row_elt) = row_elt.as_element() {
                if row_elt.name.len() == 4 && row_elt.name.starts_with("row") {

                    let row_index = row_elt.name.as_bytes()[3] - b'0';
                    let mat_row = match row_index {
                        0 => &mut transform.matrix3.x_axis,
                        1 => &mut transform.matrix3.y_axis,
                        2 => &mut transform.matrix3.z_axis,
                        3 => &mut transform.translation,
                        _ => continue
                    };

                    if let Some(vec) = row_elt.to_vec3() {
                        *mat_row = vec;
                        rows |= 1 << row_index;
                        if rows == 0b1111 {
                            return Some(transform);
                        }
                    }

                }
            }
        }

        None

    }

    fn to_vec3(&self) -> Option<Vec3A> {
        
        fn from_str(s: &str) -> Option<Vec3A> {
            let mut parts = s.split(' ');
            Some(Vec3A::new(
                parts.next()?.parse().ok()?,
                parts.next()?.parse().ok()?,
                parts.next()?.parse().ok()?,
            ))
        }

        for text_elt in &self.children {
            if let Some(text) = text_elt.as_text() {
                return from_str(text);
            }
        }

        None

    }
    
}
