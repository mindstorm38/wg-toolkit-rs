//! Definition of common data types that can be transferred as entity or method calls.

use std::borrow::Cow;
use std::io::{self, Read, Write};
use std::fmt;

pub use glam::{Vec2, Vec3, Vec4};

use crate::util::io::{WgReadExt, WgWriteExt, serde_pickle_de_options, serde_pickle_ser_options};
use crate::util::{AsciiFmt, TruncateFmt};


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


/// The string data type used by default for all STRING types, it will try to 
#[derive(Clone)]
pub enum AutoString {
    String(String),
    Python(serde_pickle::Value),
    Raw(Vec<u8>),
}

impl fmt::Debug for AutoString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(string) => f.debug_tuple("Utf8").field(string).finish(),
            Self::Python(value) => f.debug_tuple("Python").field(&format_args!("{}", TruncateFmt(value, 3000))).finish(),
            Self::Raw(bytes) => f.debug_tuple("Raw").field(&AsciiFmt(&bytes)).finish(),
        }
    }
}

impl DataType for AutoString {

    fn write(&self, write: &mut dyn Write) -> io::Result<()> {
        write.write_blob_variable(&*(match self {
            AutoString::String(v) => Cow::Borrowed(v.as_bytes()),
            AutoString::Python(v) => Cow::Owned(serde_pickle::value_to_vec(v, serde_pickle_ser_options()).unwrap()),
            AutoString::Raw(v) => Cow::Borrowed(&v[..]),
        }))
    }

    fn read(read: &mut dyn Read) -> io::Result<Self> {
        
        let raw = read.read_blob_variable()?;

        if let Ok(v) = serde_pickle::value_from_reader(&raw[..], serde_pickle_de_options()) {
            return Ok(Self::Python(v));
        }

        match String::from_utf8(raw) {
            Ok(s) => Ok(Self::String(s)),
            Err(e) => Ok(Self::Raw(e.into_bytes())),
        }

    }

}


/// The Python builtin data type.
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

impl fmt::Debug for Python {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Python")
            .field(&format_args!("{}", TruncateFmt(&self.value, 3000)))
            .finish()
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
                $( $(#[$field_attr:meta])* $field_vis:vis $field_name:ident : $field_ty:ty ),*
                $(,)?
            }
        )*
    ) => {
        $(
            $(#[$attr])* 
            $struct_vis struct $struct_name {
                $( $(#[$field_attr])* $field_vis $field_name : $field_ty,)*
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
