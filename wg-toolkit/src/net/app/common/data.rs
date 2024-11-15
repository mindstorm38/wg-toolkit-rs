//! Definition of common data types that can be transferred as entity or method calls.

use std::io::{self, Read, Write};
use std::fmt;

pub use glam::{Vec2, Vec3, Vec4};

use crate::util::io::{WgReadExt, WgWriteExt};
use crate::util::AsciiFmt;


/// Represent an element data type
pub trait DataType: Sized {

    fn write(&self, write: &mut dyn Write) -> io::Result<()>;

    fn read(read: &mut dyn Read) -> io::Result<Self>;

}

impl DataType for () {

    #[inline(always)]
    fn write(&self, _write: &mut dyn Write) -> io::Result<()> {
        Ok(())
    }

    #[inline(always)]
    fn read(_read: &mut dyn Read) -> io::Result<Self> {
        Ok(())
    }

}

impl DataType for String {

    #[inline(always)]
    fn write(&self, write: &mut dyn Write) -> io::Result<()> {
        write.write_string_variable(self)
    }

    #[inline(always)]
    fn read(read: &mut dyn Read) -> io::Result<Self> {
        read.read_string_variable()
    }

}


/// A string data type that may or may not successfully be decoded, if invalid UTF-8.
#[derive(Clone, PartialEq, Eq)]
pub enum RelaxString {
    Utf8(String),
    Raw(Vec<u8>),
}

impl fmt::Debug for RelaxString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Utf8(string) => f.debug_tuple("Utf8").field(string).finish(),
            Self::Raw(bytes) => f.debug_tuple("Raw").field(&AsciiFmt(&bytes)).finish(),
        }
    }
}

impl DataType for RelaxString {

    fn write(&self, write: &mut dyn Write) -> io::Result<()> {
        write.write_blob_variable(match self {
            RelaxString::Utf8(s) => s.as_bytes(),
            RelaxString::Raw(vec) => &vec[..],
        })
    }

    fn read(read: &mut dyn Read) -> io::Result<Self> {
        Ok(match read.read_string_variable_fallback()? {
            Ok(string) => RelaxString::Utf8(string),
            Err(bytes) => RelaxString::Raw(bytes),
        })
    }

}

/// The Python builtin data type.
#[derive(Debug)]
pub struct Python {
    /// Internal pickle value.
    pub value: serde_pickle::Value,
}

impl DataType for Python {

    #[inline(always)]
    fn write(&self, write: &mut dyn Write) -> io::Result<()> {
        write.write_python_pickle(&self.value)
    }

    #[inline(always)]
    fn read(read: &mut dyn Read) -> io::Result<Self> {
        read.read_python_pickle().map(|value| Self { value })
    }

}

/// The mailbox type used sparingly in method calls.
#[derive(Debug)]
pub struct Mailbox {
    pub entity_id: u32,
    pub address: (), // TODO: 
}

impl DataType for Mailbox {

    #[inline]
    fn write(&self, _write: &mut dyn Write) -> io::Result<()> {
        Err(io::Error::new(io::ErrorKind::InvalidData, "mailbox write not yet supported"))
    }

    #[inline]
    fn read(_read: &mut dyn Read) -> io::Result<Self> {
        Err(io::Error::new(io::ErrorKind::InvalidData, "mailbox write not yet supported"))
    }

}

macro_rules! impl_builtin_copy {
    ($ty:ty, $write_method:ident, $read_method:ident) => {
        impl DataType for $ty {

            #[inline(always)]
            fn write(&self, write: &mut dyn Write) -> io::Result<()> {
                write.$write_method(*self)
            }
        
            #[inline(always)]
            fn read(read: &mut dyn Read) -> io::Result<Self> {
                read.$read_method()
            }
        
        }
    };
}

impl_builtin_copy!(u8, write_u8, read_u8);
impl_builtin_copy!(i8, write_i8, read_i8);
impl_builtin_copy!(u16, write_u16, read_u16);
impl_builtin_copy!(i16, write_i16, read_i16);
impl_builtin_copy!(u32, write_u32, read_u32);
impl_builtin_copy!(i32, write_i32, read_i32);
impl_builtin_copy!(u64, write_u64, read_u64);
impl_builtin_copy!(i64, write_i64, read_i64);
impl_builtin_copy!(f32, write_f32, read_f32);
impl_builtin_copy!(f64, write_f64, read_f64);
impl_builtin_copy!(Vec2, write_vec2, read_vec2);
impl_builtin_copy!(Vec3, write_vec3, read_vec3);
impl_builtin_copy!(Vec4, write_vec4, read_vec4);

impl<const LEN: usize, D: DataType> DataType for Box<[D; LEN]> {

    fn write(&self, write: &mut dyn Write) -> io::Result<()> {
        for comp in &**self {
            comp.write(&mut *write)?;
        }
        Ok(())
    }

    fn read(read: &mut dyn Read) -> io::Result<Self> {
        
        let mut tmp = Vec::with_capacity(LEN);
        for _ in 0..LEN {
            tmp.push(D::read(&mut *read)?);
        }

        let Ok(ret) = tmp.into_boxed_slice().try_into() else {
            unreachable!();
        };

        Ok(ret)

    }
    
}

impl<D: DataType> DataType for Vec<D> {

    fn write(&self, write: &mut dyn Write) -> io::Result<()> {
        write.write_packed_u24(self.len() as u32)?;
        for comp in &**self {
            comp.write(&mut *write)?;
        }
        Ok(())
    }

    fn read(read: &mut dyn Read) -> io::Result<Self> {
        let len = read.read_packed_u24()? as usize;
        let mut tmp = Vec::with_capacity(len);
        for _ in 0..len {
            tmp.push(D::read(&mut *read)?);
        }
        Ok(tmp)
    }

}


/// This macro can be used to create simple aggregation of structures with all fields of
/// type [`DataType`], the structure is both defined and trait is implemented.
#[macro_export]
macro_rules! __bootstrap_struct_data_type {
    (
        $(
            $(#[$attr:meta])* 
            $struct_vis:vis struct $struct_name:ident {
                $( $field_vis:vis $field_name:ident : $field_ty:ty ),*
                $(,)?
            }
        )*
    ) => {
        $(
            $(#[$attr])* 
            $struct_vis struct $struct_name {
                $( $field_vis $field_name : $field_ty,)*
            }

            impl $crate::net::app::common::data::DataType for $struct_name {
                fn write(&self, write: &mut dyn std::io::Write) -> std::io::Result<()> {
                    $( self.$field_name.write(&mut *write)?; )*
                    Ok(())
                }
                fn read(read: &mut dyn std::io::Read) -> std::io::Result<Self> {
                    Ok(Self {
                        $( $field_name: <$field_ty>::read(&mut *read)?, )*
                    })
                }
            }
        )*
    };
}
