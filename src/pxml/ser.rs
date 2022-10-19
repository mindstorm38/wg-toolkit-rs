//! Serialization module for Packed XML.

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::io::{self, Write, Seek, SeekFrom};

use glam::Vec3A;
use smallvec::SmallVec;

use crate::util::io::WgWriteExt;

use super::{MAGIC, Element, Value, DataType};


pub fn to_writer<W: Write + Seek>(mut writer: W, element: &Element) -> io::Result<()> {

    let mut dict = HashMap::new();
    let mut next_index = 0;
    fill_dict_recursively(element, &mut dict, &mut next_index);

    // Write the magic number.
    writer.write_all(MAGIC)?;
    writer.write_u8(0)?;

    // Write the dictionary, finished by an empty string.
    for k in dict.keys() {
        writer.write_cstring(k)?;
    }
    writer.write_cstring("")?;

    // Finally write the root element.
    write_element(&mut writer, element, &dict).map(|_| ())

}


/// Internal function to analyze and fill the node's name dictionary.
fn fill_dict_recursively<'a>(element: &'a Element, dict: &mut HashMap<&'a String, u16>, next_index: &mut u16) {
    
    for (k, v) in &element.children {

        if let Entry::Vacant(v) = dict.entry(k) {
            v.insert(*next_index);
            *next_index += 1;
        }

        if let Value::Element(child_element) = v {
            fill_dict_recursively(&*child_element, &mut *dict, &mut *next_index);
        }

    }

}


fn write_element<W: Write + Seek>(writer: &mut W, element: &Element, dict: &HashMap<&String, u16>) -> io::Result<usize> {

    let self_start_offset = writer.stream_position()?;

    writer.write_u16(element.children.len() as u16)?;

    // Here we write placeholder descriptors, that will be later written.
    // Save the start offset of the element.
    writer.write_u32(0)?;
    for _ in 0..element.children.len() {
        writer.write_u16(0)?;
        writer.write_u32(0)?;
    }

    // Write element's value.
    let (self_ty, self_len) = write_value(&mut *writer, &element.value, dict)?;
    let mut offset = self_len;

    // Save descriptors for future writes.
    let self_descriptor = calc_data_descriptor(self_ty, offset);
    let mut children_descriptors = SmallVec::<[(u16, u32); 16]>::new();

    // Write element's children.
    for (k, child_value) in &element.children {
        let (child_ty, child_len) = write_value(&mut *writer, &child_value, dict)?;
        offset += child_len;
        let child_descriptor = calc_data_descriptor(child_ty, offset);
        // NOTE: Dictionary fetching should not panic since we constructed the 
        // dictionary depending on 
        children_descriptors.push((dict[k], child_descriptor));
    }

    let self_end_offset = writer.stream_position()?;

    // Finally write descriptors 2 octets (children count) after start.
    writer.seek(SeekFrom::Start(self_start_offset + 2))?;
    writer.write_u32(self_descriptor)?;
    for (name_index, data) in children_descriptors {
        writer.write_u16(name_index)?;
        writer.write_u32(data)?;
    }

    // Jump back to the end of the element.
    writer.seek(SeekFrom::Start(self_end_offset))?;

    // Compute total length.
    Ok((self_end_offset - self_start_offset) as usize)

}


/// Internal function to write a value and return the size used to write it.
/// The returned data type is used to compute the data descriptor.
fn write_value<W: Write + Seek>(writer: &mut W, value: &Value, dict: &HashMap<&String, u16>) -> io::Result<(DataType, usize)> {

    #[inline]
    fn write_vec3<W: Write + Seek>(writer: &mut W, v: &Vec3A) -> io::Result<()> {
        writer.write_f32(v.x)?;
        writer.write_f32(v.y)?;
        writer.write_f32(v.z)
    }

    // Returned length should perfectly match written data.

    match value {
        Value::Element(child_element) => {
            write_element(writer, &*child_element, dict).map(|len| (DataType::Element, len))
        }
        Value::String(s) => {
            writer.write_string(s)?;
            Ok((DataType::String, s.len()))
        }
        &Value::Integer(n) => {
            // Zero is optimized out.
            let len = if n == 0 {
                0
            } else if let Ok(n) = i8::try_from(n) {
                writer.write_i8(n)?; 1
            } else if let Ok(n) = i16::try_from(n) {
                writer.write_i16(n)?; 2
            } else if let Ok(n) = i32::try_from(n) {
                writer.write_i32(n)?; 4
            } else {
                writer.write_i64(n)?; 8
            };
            Ok((DataType::Integer, len))
        },
        &Value::Bool(b) => {
            // Only write an octet if true.
            if b {
                writer.write_u8(1)?;
            }
            Ok((DataType::Boolean, if b { 1 } else { 0 }))
        }
        &Value::Float(n) => {
            writer.write_f32(n)?;
            Ok((DataType::Float, 4))
        }
        Value::Vec3(v) => {
            write_vec3(writer, v)?;
            Ok((DataType::Float, 4 * 3))
        }
        Value::Affine3(a) => {
            write_vec3(writer, &a.x_axis)?;
            write_vec3(writer, &a.y_axis)?;
            write_vec3(writer, &a.z_axis)?;
            write_vec3(writer, &a.w_axis)?;
            Ok((DataType::Float, 4 * 12))
        }
        Value::Blob(data) => {
            writer.write_all(&data[..])?;
            Ok((DataType::Blob, data.len()))
        }
    }

}


#[inline]
fn calc_data_descriptor(ty: DataType, offset: usize) -> u32 {
    (ty.to_raw() << 28) | (offset as u32 & 0x00FFFFFFF)
}
