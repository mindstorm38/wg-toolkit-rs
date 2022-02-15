use std::io::{self, Read, Seek, SeekFrom};
use std::fmt::Write;

use byteorder::{ReadBytesExt, LittleEndian};
use xmltree::{self, Element, XMLNode};

use super::PACKED_SIGNATURE;


/// Unpack or parse XML from an input `Read` implementor. This function will
/// simply parse the input if it happen to be an already unpacked XML.
pub fn unpack_xml<R: Read + Seek>(read: &mut R) -> Result<Element, XmlError> {

    let pos = read.stream_position()?;

    let mut buf = [0; 4];
    read.read_exact(&mut buf)?;

    if &buf != PACKED_SIGNATURE {
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

    fn read_dictionary(&mut self) -> Result<(), XmlError> {
        loop {
            let string = read_null_string(&mut self.read)?;
            if string.is_empty() {
                return Ok(());
            }
            self.dict.push(string);
        }
    }

    fn read_element(&mut self, elt: &mut Element) -> Result<(), XmlError> {

        let children_count = self.read.read_u16::<LittleEndian>()? as usize;
        let descriptor = self.read_data_descriptor()?;
        let mut children = Vec::with_capacity(children_count);
        for _ in 0..children_count {
            children.push(self.read_element_descriptor()?);
        }

        self.read_data(elt, 0, &descriptor, false)?;  // This data should not return an element.
        let mut offset = descriptor.end_addr;

        for child in children {
            let mut child_elt = Element::new(self.dict[child.name_idx].as_str());
            self.read_data(&mut child_elt, offset, &child.data, true)?;
            offset = child.data.end_addr;
            elt.children.push(XMLNode::Element(child_elt));
        }

        Ok(())

    }

    fn read_data_descriptor(&mut self) -> Result<DataDescriptor, XmlError> {
        let data_descriptor = self.read.read_u32::<LittleEndian>()?;
        let raw_data_type = data_descriptor >> 28;
        Ok(DataDescriptor {
            data_type: DataType::from_raw(raw_data_type)
                .ok_or(XmlError::InvalidDataType(raw_data_type))?,
            end_addr: data_descriptor & 0x00FFFFFFF,
            start_addr: self.read.stream_position()? as u32,
        })
    }

    fn read_element_descriptor(&mut self) -> Result<ElementDescriptor, XmlError> {
        Ok(ElementDescriptor {
            name_idx: self.read.read_u16::<LittleEndian>()? as usize,
            data: self.read_data_descriptor()?,
        })
    }

    fn read_data(&mut self, elt: &mut Element, offset: u32, descriptor: &DataDescriptor, allow_element: bool) -> Result<(), XmlError> {
        let len = (descriptor.end_addr - offset) as usize;
        let string;
        match descriptor.data_type {
            DataType::Element if !allow_element => return Err(XmlError::UnexpectedElement),
            DataType::Element => return self.read_element(elt),
            DataType::String => string = self.read_string(len)?,
            DataType::Integer => string = self.read_number(len)?.to_string(),
            DataType::Vector => {
                let floats = self.read_vector(len)?;
                if floats.len() == 12 {
                    // Display as as 3x4 matrix, because it's done like this in SkepticalFox's
                    // implementation.
                    for (i, arr) in floats.chunks_exact(3).enumerate() {
                        let mut row_elt = Element::new(format!("row{}", i).as_str());
                        let string = format!("{} {} {}", arr[0], arr[1], arr[2]);
                        row_elt.children.push(XMLNode::Text(string));
                        elt.children.push(XMLNode::Element(row_elt));
                    }
                    return Ok(());
                } else {
                    let mut tmp = String::new();
                    for (i, f) in floats.into_iter().enumerate() {
                        if i != 0 {
                            tmp.write_char(' ').unwrap();
                        }
                        tmp.write_fmt(format_args!("{}", f)).unwrap();
                    }
                    string = tmp;
                }
            },
            DataType::Boolean => {
                string = (if self.read_bool(len)? { "true" } else { "false" }).to_string();
            }
            DataType::Blob => {
                let mut data = vec![0; len];
                self.read.read_exact(&mut data[..])?;
                string = base64::encode(data);
            }
        }
        elt.children.push(XMLNode::Text(string));
        Ok(())
    }

    fn read_string(&mut self, len: usize) -> Result<String, XmlError> {
        if len == 0 {
            Ok("".to_string())
        } else {
            let mut buf = vec![0; len];
            self.read.read_exact(&mut buf[..])?;
            // TODO: Remove unwrap, or assume that XML strings will always be ASCII.
            Ok(String::from_utf8(buf).unwrap())
        }
    }

    fn read_number(&mut self, len: usize) -> Result<i64, XmlError> {
        match len {
            0 => Ok(0),
            1 => Ok(self.read.read_i8()? as i64),
            2 => Ok(self.read.read_i16::<LittleEndian>()? as i64),
            4 => Ok(self.read.read_i32::<LittleEndian>()? as i64),
            8 => Ok(self.read.read_i64::<LittleEndian>()?),
            _ => Err(XmlError::InvalidNumberSize(len))
        }
    }

    fn read_vector(&mut self, len: usize) -> Result<Vec<f32>, XmlError> {
        let n = len / 4;
        let mut res = Vec::with_capacity(n);
        for _ in 0..n {
            res.push(self.read.read_f32::<LittleEndian>()?);
        }
        Ok(res)
    }

    fn read_bool(&mut self, len: usize) -> Result<bool, XmlError> {
        match len {
            0 => Ok(false),
            1 => Ok(self.read.read_u8()? == 1),
            _ => Err(XmlError::InvalidBoolSize(len))
        }
    }

}


struct DataDescriptor {
    data_type: DataType,
    #[allow(unused)] // Currently unused because children are packed and in the right order.
    start_addr: u32,
    end_addr: u32
}


struct ElementDescriptor {
    data: DataDescriptor,
    name_idx: usize
}


enum DataType {
    Element,
    String,
    Integer,
    Vector,
    Boolean,
    Blob
}

impl DataType {

    pub fn from_raw(raw: u32) -> Option<Self> {
        Some(match raw {
            0 => Self::Element,
            1 => Self::String,
            2 => Self::Integer,
            3 => Self::Vector,
            4 => Self::Boolean,
            5 => Self::Blob,
            _ => return None
        })
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

    // TODO: Remove unwrap, or assume that XML names will always be ASCII.
    Ok(String::from_utf8(string).unwrap())

}


#[derive(Debug)]
pub enum XmlError {
    /// Invalid data type while parsing.
    InvalidDataType(u32),
    /// Unexpected `DataType::Element`.
    UnexpectedElement,
    /// Invalid data size for a number.
    InvalidNumberSize(usize),
    /// Invalid data size for a boolean.
    InvalidBoolSize(usize),
    /// IO error will unpacking.
    Io(io::Error),
    /// XML parsing error while parsing a non-packed input.
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