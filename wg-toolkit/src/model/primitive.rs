//! Utilities to read primitives processed files.

use std::io::{self, Read, Seek, SeekFrom};
use std::collections::HashMap;
use std::fmt;

use glam::{Vec3, Vec2};

use crate::util::io::WgReadExt;


/// Magic of a primitives processed files.
pub const MAGIC: &[u8; 4] = b"\x65\x4E\xA1\x42";


/// Trait to implement for section codecs.
pub trait Section: Sized {

    /// Read this section from the given reader, this reader
    /// is seekable and its cursor will be located at the 
    /// beginning of the section. The length of the section
    /// is also given.
    fn read<R: Read + Seek>(reader: R, len: usize) -> Result<Self, DeError>;

}


/// Primitive store reader utility.
pub struct PrimitiveReader<R> {
    inner: R,
    sections: HashMap<String, SectionMeta>,
}

impl<R> PrimitiveReader<R> {

    /// Consume the primitive reader and 
    #[inline]
    pub fn into_inner(self) -> R {
        self.inner
    }

}

impl<R: Read + Seek> PrimitiveReader<R> {

    /// Open and decode a primitives file's header, the reader is kept open and date 
    /// can be read.
    /// 
    /// *The position of the reader is not important because it will be forced to zero 
    /// before reading. It works like that because the inner reader will be read in 
    /// absolute positioning.*
    pub fn open(mut inner: R) -> Result<Self, DeError> {

        let mut sections = HashMap::new();

        inner.rewind()?;
        if !inner.check_exact(MAGIC)? {
            return Err(DeError::InvalidMagic);
        }

        inner.seek(SeekFrom::End(-4))?;
        let mut table_len = inner.read_u32()? as usize;
        inner.seek(SeekFrom::End(-4 - table_len as i64))?;

        let mut section_offset = 4;

        while table_len != 0 {

            let section_len = inner.read_u32()? as usize;
            inner.skip::<16>()?;
            let section_name_len = inner.read_u32()? as usize;
            let section_name = inner.read_string(section_name_len)?;

            sections.insert(section_name.clone(), SectionMeta {
                name: section_name,
                off: section_offset,
                len: section_len,
            });

            // Keep the alignment of the section offset.
            section_offset += section_len;
            if section_len % 4 != 0 {
                section_offset += 4 - section_len % 4;
            }
            
            // Keep the alignment of the table cursor.
            table_len -= 24; // Remove the two u32 and the 16 skept bytes.
            table_len -= section_name_len; // Remove the size of the name.
            if section_name_len % 4 != 0 {
                let pad = 4 - section_name_len % 4;
                let mut buf = [0; 4];
                inner.read_exact(&mut buf[..pad])?;
                table_len -= pad; // Also remove the padding from the current length.
            }

        }

        Ok(Self {
            inner,
            sections,
        })

    }

    #[inline]
    pub fn iter_sections_meta(&self) -> impl Iterator<Item = &'_ SectionMeta> {
        self.sections.values()
    }

    #[inline]
    pub fn get_section_meta(&self, name: &str) -> Option<&SectionMeta> {
        self.sections.get(name)
    }

    /// Read the given section using the given section type.
    pub fn read_section<S: Section>(&mut self, name: &str) -> Option<Result<S, DeError>> {
        let &SectionMeta { off, len, .. } = self.get_section_meta(name)?;
        match self.inner.seek(SeekFrom::Start(off as u64)) {
            Ok(_) => Some(S::read(&mut self.inner, len)),
            Err(e) => Some(Err(e.into()))
        }
    }

}


/// Metadata about a section in the primitive file.
#[derive(Debug)]
pub struct SectionMeta {
    pub name: String,
    pub off: usize,
    pub len: usize,
}


/// A section that contains vertices.
#[derive(Debug)]
pub struct Vertices {
    pub vertices: Vec<Vertex>,
}

impl Section for Vertices {

    fn read<R: Read + Seek>(mut reader: R, _len: usize) -> Result<Self, DeError> {
        
        // Read the type of vertex. This type is a null-terminated string
        // of a fixed length of 64 octets.
        let mut ty_name = reader.read_cstring(64)?;
        let mut count = reader.read_u32()?;

        // Modern types contains 'BPVT', in such cases the real vertex 
        // type is located after the first one.
        if ty_name.starts_with("BPVT") {
            ty_name = reader.read_cstring(64)?;
            count = reader.read_u32()?;
        }

        let mut vertices = Vec::new();

        // Set the following properties depending on the given vertex
        // type. This is used to know how to read individual vertex.
        let mut ty_new = false;
        let mut ty_skinned = false;
        let mut ty_tb = false;
        let mut ty_iiiww = false;

        match &ty_name[..] {
            "set3/xyznuvtbpc" => {
                ty_new = true;
                ty_tb = true;
            }
            "set3/xyznuvpc" => {
                ty_new = true;
            }
            "set3/xyznuviiiwwtbpc" => {
                ty_new = true;
                ty_skinned = true;
                ty_tb = true;
                ty_iiiww = true;
            }
            "xyznuviiiwwtb" => {
                ty_skinned = true;
                ty_tb = true;
                ty_iiiww = true;
            }
            "xyznuvtb" => {
                ty_tb = true;
            }
            "xyznuv" => {}
            _ => return Err(DeError::InvalidType(ty_name))
        }

        // Read all vertices.
        for _ in 0..count {

            let position = {
                let x = reader.read_f32()?;
                let y = reader.read_f32()?;
                let z = reader.read_f32()?;
                Vec3::new(x, if ty_skinned { -y } else { y }, z)
            };

            let normal = {
                let packed = reader.read_u32()?;
                if ty_new {

                    #[inline(always)]
                    fn p2f(n: u32) -> f32 {
                        if n > 0x7F {
                            -((n & 0x7F) as f32) / 0x7F as f32
                        } else {
                            (n ^ 0x7F) as f32 / 0x7F as f32
                        }
                    }

                    let pkz = (packed >> 16) & 0xFF ^ 0xFF;
                    let pky = (packed >> 8)  & 0xFF ^ 0xFF;
                    let pkx =  packed        & 0xFF ^ 0xFF;
                    Vec3::new(p2f(pkx), p2f(pky), p2f(pkz))

                } else {

                    #[inline(always)]
                    fn p2f(n: u32, a: u32) -> f32 {
                        if n > a {
                            -(((n & a ^ a) + 1) as f32) / a as f32
                        } else {
                            n as f32 / a as f32
                        }
                    }

                    let pkz = (packed >> 22) & 0x3FF;
                    let pky = (packed >> 11) & 0x7FF;
                    let pkx =  packed        & 0x7FF;
                    Vec3::new(p2f(pkx, 0x3FF), p2f(pky, 0x3FF), p2f(pkz, 0x1FF))

                }
            };

            let uv = {
                let u = reader.read_f32()?;
                let v = reader.read_f32()?;
                Vec2::new(u, 1.0 - v)
            };

            let mut index = [0; 3];
            let mut index2 = [0; 3];
            let mut weight = [0.0; 3];

            if ty_iiiww {

                // Read indices and divide by 3.
                reader.read_exact(&mut index[..])?;
                index[0] /= 3;
                index[1] /= 3;
                index[2] /= 3;

                if ty_new {
                    // New indices need to be swapped.
                    index.swap(0, 2);
                    // Unknown purpose.
                    reader.read_exact(&mut index2[..])?;
                }

                // Read 2 weights and compute third one. 
                weight[0] = reader.read_u8()? as f32 / 255.0;
                weight[1] = reader.read_u8()? as f32 / 255.0;
                weight[2] = 1.0 - weight[0] - weight[1];

            }

            let tangent = if ty_tb { reader.read_u32()? } else { 0 };
            let binormal = if ty_tb { reader.read_u32()? } else { 0 };

            vertices.push(Vertex {
                position,
                normal,
                uv,
                index,
                index2,
                weight,
                tangent,
                binormal,
            });

        }

        Ok(Self { vertices })

    }

}

pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
    pub index: [u8; 3],
    pub index2: [u8; 3],
    pub weight: [f32; 3],
    pub tangent: u32,
    pub binormal: u32,
}

impl fmt::Debug for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vertex {{ pos: {}/{}/{}, norm: {}/{}/{}, uv: {}/{}, index: {:?}, weight: {:?}, tan: {}, binorm: {} }}",
            self.position.x, self.position.y, self.position.z,
            self.normal.x, self.normal.y, self.normal.z,
            self.uv.x, self.uv.y,
            self.index,
            self.weight,
            self.tangent,
            self.binormal,
        )
    }
}


/// A section that contains indices and groups.
#[derive(Debug)]
pub struct Indices {
    /// Listing of all primitives (triangles).
    pub primitives: Vec<Primitive>,
    /// Listing of all groups of primitives.
    pub groups: Vec<Group>,
}

impl Section for Indices {

    fn read<R: Read + Seek>(mut reader: R, _len: usize) -> Result<Self, DeError> {
        
        // Get the type name and the indices' width.
        let ty_name = reader.read_cstring(64)?;
        let ty_long = match &ty_name[..] {
            "list" => false,
            "list32" => true,
            _ => return Err(DeError::InvalidType(ty_name))
        };

        // Read number of vertices and groups.
        let vertices_count = reader.read_u32()? / 3;
        let groups_count = reader.read_u32()?;

        // Read all indices.
        let mut indices = Vec::new();
        if ty_long {
            for _ in 0..vertices_count {
                indices.push(Primitive {
                    a: reader.read_u32()?,
                    b: reader.read_u32()?,
                    c: reader.read_u32()?,
                });
            }
        } else {
            for _ in 0..vertices_count {
                indices.push(Primitive {
                    a: reader.read_u16()? as u32,
                    b: reader.read_u16()? as u32,
                    c: reader.read_u16()? as u32,
                });
            }
        }

        let mut groups = Vec::new();
        for _ in 0..groups_count {
            groups.push(Group {
                primitives_offset: reader.read_u32()?,
                primitives_count: reader.read_u32()?,
                vertices_offset: reader.read_u32()?,
                vertices_count: reader.read_u32()?,
            });
        }

        Ok(Self { primitives: indices, groups })

    }

}

/// A primitive (triangle) of indices, referencing vertices.
#[derive(Debug)]
pub struct Primitive {
    pub a: u32,
    pub b: u32,
    pub c: u32,
}

#[derive(Debug)]
pub struct Group {
    /// Offset of the first primitive of this group.
    pub primitives_offset: u32,
    /// Number of primitives in the group.
    pub primitives_count: u32,
    /// Offset of the first vertex of this group.
    pub vertices_offset: u32,
    /// Number of vertices in the group.
    pub vertices_count: u32,
}


/// Deserialization errors that can happen while deserializing sections.
#[derive(Debug)]
pub enum DeError {
    /// Invalid magic signature for the file.
    InvalidMagic,
    /// Any section's type begins with a type describing data layout for
    /// the section. This error is returned if such type cannot be resolved.
    InvalidType(String),
    /// Unhandled underlying I/O error.
    Io(io::Error),
}

impl fmt::Display for DeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::InvalidMagic => write!(f, "invalid magic"),
            Self::InvalidType(ref s) => write!(f, "invalid type '{s}'"),
            Self::Io(ref err) => write!(f, "io error: {err}"),
        }
    }
}

impl std::error::Error for DeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            _ => None
        }
    }
}

impl From<io::Error> for DeError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}
