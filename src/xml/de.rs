//! Deserializing modules for Wargaming packed XML.

use std::io::{self, Read, Seek, SeekFrom};
use std::string::FromUtf8Error;
use std::fmt::Display;

use smallvec::{SmallVec, smallvec};
use serde::de::{self, SeqAccess};

use crate::util::io::WgReadExt;


/// A deserializer implementation for packed XML format.
struct Deserializer<R: Read + Seek> {
    /// Internal reader.
    reader: R,
    /// Internal dictionary of strings, referenced later by
    /// elements for their names.
    dict: Vec<String>,
    /// Stack of elements being deserialized.
    /// It initially contains the root element and only the last element 
    /// is the one being deserialized.
    stack: SmallVec<[ElementFullDescriptor; 8]>,
}

struct DeserializerSeq<'a, R: Read + Seek> {
    inner: &'a mut Deserializer<R>,
}

impl<R: Read + Seek> Deserializer<R> {

    fn new(mut reader: R) -> Result<Self, DeError> {

        let mut dict = Vec::new();

        let _ = reader.read_u8()?;

        loop {
            let string = read_null_string(&mut reader)?;
            if string.is_empty() {
                break
            }
            dict.push(string);
        }

        let root_element = read_element(&mut reader)?;

        Ok(Self {
            reader,
            dict,
            stack: smallvec![root_element],
        })

    }

    fn current_element(&self) -> &ElementFullDescriptor {
        &self.stack[self.stack.len() - 1]
    }

    fn current_typed_data(&self, ty: DataType) -> Result<&DataDescriptor, DeError> {
        let elt = self.current_element();
        if ty == elt.data.ty {
            Ok(&elt.data)
        } else {
            Err(DeError::UnexpectedDataType(elt.data.ty))
        }
    }

}

impl<'de, 'a, R: Read + Seek> de::Deserializer<'de> for &'a mut Deserializer<R> {

    type Error = DeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>
    {
        let data = self.current_typed_data(DataType::Boolean)?;
        match data.length() {
            0 => visitor.visit_bool(false),
            1 => {
                self.reader.seek(SeekFrom::Start(data.start_addr as u64))?;
                visitor.visit_bool(self.reader.read_u8()? == 1)
            }
            len => Err(DeError::InvalidBoolSize(len))
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>
    {
        let data = self.current_typed_data(DataType::Integer)?;
        visitor.visit_i64(read_integer(&mut self.reader, data)?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> 
    {
        let data = self.current_typed_data(DataType::Integer)?;
        visitor.visit_i64(read_integer(&mut self.reader, data)?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>
    {
        let data = self.current_typed_data(DataType::Integer)?;
        visitor.visit_i64(read_integer(&mut self.reader, data)?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> 
    {
        let data = self.current_typed_data(DataType::Integer)?;
        visitor.visit_i64(read_integer(&mut self.reader, data)?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
    V: de::Visitor<'de> {
        let data = self.current_typed_data(DataType::Integer)?;
        visitor.visit_u64(read_integer(&mut self.reader, data)? as u64)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> 
    {
        let data = self.current_typed_data(DataType::Integer)?;
        visitor.visit_u64(read_integer(&mut self.reader, data)? as u64)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> 
    {
        let data = self.current_typed_data(DataType::Integer)?;
        visitor.visit_u64(read_integer(&mut self.reader, data)? as u64)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> 
    {
        let data = self.current_typed_data(DataType::Integer)?;
        visitor.visit_u64(read_integer(&mut self.reader, data)? as u64)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        todo!()
    }

}

impl<'a, 'de, R: Read + Seek> SeqAccess<'de> for DeserializerSeq<'a, R> {

    type Error = DeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de> {
        todo!()
        // seed.deserialize(&mut *self.inner)
    }

}


/// Internal data descriptor.
struct DataDescriptor {
    ty: DataType,
    /// Start of the data in the file's address.
    start_addr: u32,
    /// end of the data in the file's address.
    end_addr: u32,
}

impl DataDescriptor {

    /// Compute data length.
    #[inline]
    fn length(&self) -> usize {
        (self.end_addr - self.start_addr) as usize
    }

}


/// Internal descriptor for children elements of an element.
struct ElementDescriptor {
    data: DataDescriptor,
    name_idx: usize
}


/// Internal descriptor for a full element and its children.
struct ElementFullDescriptor {
    data: DataDescriptor,
    children: SmallVec<[ElementDescriptor; 8]>,
}


/// Internal possible data types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Element = 0,
    String = 1,
    Integer = 2,
    Vector = 3,
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
            3 => Self::Vector,
            4 => Self::Boolean,
            5 => Self::Blob,
            _ => return None
        })
    }

}


fn read_data_descriptor<R: Read + Seek>(reader: &mut R) -> Result<DataDescriptor, DeError> {
    let data_descriptor = reader.read_u32()?;
    let raw_data_type = data_descriptor >> 28;
    Ok(DataDescriptor {
        ty: DataType::from_raw(raw_data_type)
            .ok_or(DeError::InvalidDataType(raw_data_type))?,
        end_addr: data_descriptor & 0x00FFFFFFF,
        start_addr: reader.stream_position()? as u32,
    })
}


fn read_element_descriptor<R: Read + Seek>(reader: &mut R) -> Result<ElementDescriptor, DeError> {
    Ok(ElementDescriptor {
        name_idx: reader.read_u16()? as usize,
        data: read_data_descriptor(&mut *reader)?,
    })
}


fn read_element<R: Read + Seek>(reader: &mut R) -> Result<ElementFullDescriptor, DeError> {
    
    let children_count = reader.read_u16()? as usize;
    let mut full_descriptor = ElementFullDescriptor {
        data: read_data_descriptor(&mut *reader)?,
        children: SmallVec::new(),
    };

    for _ in 0..children_count {
        full_descriptor.children.push(read_element_descriptor(&mut *reader)?);
    }

    Ok(full_descriptor)

}


fn read_integer<R: Read + Seek>(reader: &mut R, data: &DataDescriptor) -> Result<i64, DeError> {
    match data.length() {
        0 => Ok(0),
        1 => Ok(reader.read_i8()? as i64),
        2 => Ok(reader.read_i16()? as i64),
        4 => Ok(reader.read_i32()? as i64),
        8 => Ok(reader.read_i64()?),
        len => Err(DeError::InvalidNumberSize(len))
    }
}


/// Internal fast reading for null-terminated strings. 
/// Requires a seekable reader because multiple bytes are read at the
/// same time and the position often need to be rolled back in order to
/// align to the end of read string.
fn read_null_string<R: Read + Seek>(mut reader: R) -> Result<String, DeError> {

    let mut cursor = reader.stream_position()?;
    let mut buf = [0; 32];
    let mut string = Vec::new();

    'e: loop {

        let mut len = match reader.read(&mut buf) {
            Ok(len) => len,
            Err(e) if e.kind() != io::ErrorKind::Interrupted => return Err(e.into()),
            _ => continue
        };

        for &c in &buf[..len] {
            cursor += 1;
            len -= 1;
            if c == 0 {
                if len != 0 { // Only seek if bytes remains.
                    reader.seek(SeekFrom::Start(cursor))?;
                }
                break 'e;
            }
            string.push(c);
        }

    }

    Ok(String::from_utf8(string)?)

}


#[derive(Debug)]
pub enum DeError {
    /// Invalid data type while parsing.
    InvalidDataType(u32),
    /// Unexpected `DataType::Element`.
    UnexpectedDataType(DataType),
    /// Invalid data size for a number.
    InvalidNumberSize(usize),
    /// Invalid data size for a boolean.
    InvalidBoolSize(usize),
    /// Custom error.
    Custom(String),
    /// Invalid string utf8.
    Utf8(FromUtf8Error),
    /// IO error will unpacking.
    Io(io::Error),
    /// XML parsing error while parsing a non-packed input.
    Xml(xmltree::ParseError),
}

impl Display for DeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DeError::InvalidDataType(n) => write!(f, "invalid data type id {n}"),
            DeError::UnexpectedDataType(ty) => write!(f, "unexpected data type {ty:?}"),
            DeError::InvalidNumberSize(n) => write!(f, "invalid data size of {n} bytes for a number"),
            DeError::InvalidBoolSize(n) => write!(f, "invalid data size of {n} bytes for a boolean"),
            DeError::Custom(ref msg) => write!(f, "custom deserialization error: {msg}"),
            DeError::Utf8(ref err) => write!(f, "utf8 error: {err:?}"),
            DeError::Io(ref err) => write!(f, "io error: {err:?}"),
            DeError::Xml(ref err) => write!(f, "xml parsing error: {err:?}"),
        }
    }
}

impl de::Error for DeError {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Custom(format!("{msg}"))
    }
}

impl std::error::Error for DeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DeError::Utf8(err) => Some(err),
            DeError::Io(err) => Some(err),
            DeError::Xml(err) => Some(err),
            _ => None
        }
    }
}

impl From<FromUtf8Error> for DeError {
    fn from(e: FromUtf8Error) -> Self {
        Self::Utf8(e)
    }
}

impl From<io::Error> for DeError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<xmltree::ParseError> for DeError {
    fn from(e: xmltree::ParseError) -> Self {
        Self::Xml(e)
    }
}
