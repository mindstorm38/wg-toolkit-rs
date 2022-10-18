use std::io::{self, Write};

use xmltree::{Element, XMLNode};

use crate::util::io::WgWriteExt;
use super::PACKED_SIGNATURE;


/// Pack the given XML root element to a given `Write`
/// implementor. 
pub fn pack_xml<W: Write>(mut write: W, elt: &Element) -> io::Result<()> {

    write.write_all(PACKED_SIGNATURE)?;



    todo!()

}


struct XmlPacker<W> {
    write: W,
}

impl<W: Write> XmlPacker<W> {

    fn new(write: W) -> Self {
        Self {
            write,
        }
    }

    fn write(&mut self) {

    }

    fn write_element(&mut self, elt: &Element) -> io::Result<()> {

        self.write.write_u16(elt.children.len() as u16)?;
        // TODO: Write data descriptor
        for child in &elt.children {
            
        }


        Ok(())

    }

    fn write_data(&mut self, node: &XMLNode) {
        match node {
            XMLNode::Text(text) => {
                
            }
            XMLNode::Element(elt) => {

            }
            _ => {}
        }
    }

}
