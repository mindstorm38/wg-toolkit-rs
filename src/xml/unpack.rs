use byteorder::{ReadBytesExt, LittleEndian};
use std::io::{self, Read, Seek, SeekFrom};
use xmltree::{self, Element, XMLNode};

use super::PACKED_HEADER;


pub fn unpack_xml<R: Read + Seek>(read: &mut R) -> Result<Element, XmlError> {

    let pos = read.stream_position()?;

    let mut buf = [0; 4];
    read.read_exact(&mut buf)?;
    let packed = &buf == PACKED_HEADER;

    if !packed {
        read.seek(SeekFrom::Start(pos))?;
        Ok(Element::parse(read)?)
    } else {
        XmlUnpacker::new(read).decode()
    }

}


/// Internal temporary structure for unpacking XML.
struct XmlUnpacker<R> {
    read: R,
    dict: Vec<String>
}

impl<R: Read + Seek> XmlUnpacker<R> {

    fn new(read: R) -> Self {
        Self {
            read,
            dict: Vec::new()
        }
    }

    fn decode(mut self) -> Result<Element, XmlError> {

        let _ = self.read.read_u8()?;
        self.read_dictionary()?;

        let mut root = Element::new("root");
        self.read_element(&mut root)?;
        Ok(root)

    }

    fn read_dictionary(&mut self) -> io::Result<()> {
        loop {
            let string = read_null_string(&mut self.read)?;
            if string.is_empty() {
                return Ok(());
            }
            self.dict.push(string);
        }
    }

    fn read_element(&mut self, elt: &mut Element) -> io::Result<()> {

        let children_count = self.read.read_u16::<LittleEndian>()? as usize;
        let descriptor = self.read_data_descriptor()?;
        let mut children = Vec::with_capacity(children_count);
        for _ in 0..children_count {
            children.push(self.read_element_descriptor()?);
        }

        self.read_data(elt, 0, &descriptor, false);  // This data should not return an element.
        let mut offset = descriptor.end_addr;

        for child in children {
            let mut child_elt = Element::new(self.dict[child.name_idx].as_str());
            self.read_data(&mut child_elt, offset, &child.data, true);
            offset = child.data.end_addr;
            elt.children.push(XMLNode::Element(child_elt));
        }

        Ok(())

    }

    fn read_data_descriptor(&mut self) -> io::Result<DataDescriptor> {
        let data_descriptor = self.read.read_u32::<LittleEndian>()?;
        Ok(DataDescriptor {
            data_type: DataType::from_raw((data_descriptor >> 28) as u8).unwrap(), // TODO: remove unwrap
            end_addr: data_descriptor & 0x00FFFFFFF,
            start_addr: self.read.stream_position()? as u32,
        })
    }

    fn read_element_descriptor(&mut self) -> io::Result<ElementDescriptor> {
        Ok(ElementDescriptor {
            name_idx: self.read.read_u16::<LittleEndian>()? as usize,
            data: self.read_data_descriptor()?,
        })
    }

    fn read_data(&mut self, elt: &mut Element, offset: u32, descriptor: &DataDescriptor, allow_element: bool) -> io::Result<()> {
        let len = (descriptor.end_addr - offset) as usize;
        match descriptor.data_type {
            DataType::Element => {
                if !allow_element {
                    panic!("unexpected element");
                }
                self.read_element(elt)?
            },
            DataType::String => elt.children.push(XMLNode::Text(self.read_string(len)?)),
            DataType::Integer => elt.children.push(XMLNode::Text(self.read_number(len)?.to_string())),
            DataType::Float => {
                let n = len / 4;
                for _ in 0..n {
                    let _ = self.read.read_u32::<LittleEndian>()?;
                }
                elt.children.push(XMLNode::Text("todo matrices".to_string()))
            },
            DataType::Boolean => {
                let string = if self.read_bool(len)? { "true" } else { "false" };
                elt.children.push(XMLNode::Text(string.to_string()));
            }
            DataType::Blob => {
                let mut data = vec![0; len];
                self.read.read_exact(&mut data[..])?;
                elt.children.push(XMLNode::Text(base64::encode(data)));
            }
        }
        Ok(())
    }

    fn read_string(&mut self, len: usize) -> io::Result<String> {
        let mut buf = vec![0; len];
        self.read.read_exact(&mut buf[..])?;
        Ok(String::from_utf8(buf).unwrap()) // TODO: remove unwrap
    }

    fn read_number(&mut self, len: usize) -> io::Result<i64> {
        match len {
            0 => Ok(0),
            1 => self.read.read_i8().map(|n| n as i64),
            2 => self.read.read_i16::<LittleEndian>().map(|n| n as i64),
            4 => self.read.read_i32::<LittleEndian>().map(|n| n as i64),
            8 => self.read.read_i64::<LittleEndian>(),
            _ => panic!("illegal number size")
        }
    }

    fn read_bool(&mut self, len: usize) -> io::Result<bool> {
        match len {
            0 => Ok(false),
            1 => self.read.read_u8().map(|n| n == 1),
            _ => panic!("illegal bool size")
        }
    }

}


struct DataDescriptor {
    data_type: DataType,
    start_addr: u32,
    end_addr: u32
}


struct ElementDescriptor {
    data: DataDescriptor,
    name_idx: usize
}


#[repr(u8)]
enum DataType {
    Element = 0x0,
    String = 0x1,
    Integer = 0x2,
    Float = 0x3,
    Boolean = 0x4,
    Blob = 0x5
}

impl DataType {

    pub fn from_raw(raw: u8) -> Option<DataType> {
        if raw <= 0x5 {
            Some(unsafe { std::mem::transmute(raw) })
        } else {
            println!("unknown type: {}", raw);
            None
        }
    }

}


/// Internal fast reading for null-terminated strings. Requires a seekable reader.
fn read_null_string<R: Read + Seek>(read: &mut R) -> io::Result<String> {

    let mut cursor = read.stream_position()?;
    let mut buf = [0; 32];
    let mut string = Vec::new();

    'e: loop {

        let mut len = match read.read(&mut buf) {
            Ok(len) => len,
            Err(e) if e.kind() != io::ErrorKind::Interrupted => return Err(e),
            _ => continue
        };

        for &c in &buf[..len] {
            cursor += 1;
            len -= 1;
            if c == 0 {
                if len != 0 { // Only seek if bytes remains.
                    read.seek(SeekFrom::Start(cursor))?;
                }
                break 'e;
            }
            string.push(c);
        }

    }

    Ok(String::from_utf8(string).unwrap()) // remove unwrap

}


#[derive(Debug)]
pub enum XmlError {
    Io(io::Error),
    Xml(xmltree::ParseError),
}

impl From<io::Error> for XmlError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<xmltree::ParseError> for XmlError {
    fn from(e: xmltree::ParseError) -> Self {
        Self::Xml(e)
    }
}