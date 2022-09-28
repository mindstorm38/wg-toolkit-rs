//! Module for packed XML codec.

mod unpack;
mod pack;

pub use unpack::*;
pub use pack::*;

/// Re-export of xmltree dependency.
pub use xmltree;

use glam::{Affine3A, Vec3A};
use xmltree::Element;


/// Signature of a packed XML file.
pub const PACKED_SIGNATURE: &[u8; 4] = b"\x45\x4E\xA1\x62";


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
