use std::io::{self, Read, Write};

use super::data::DataType;


/// This macro can be used to generate an enumeration capable of encoding and decoding
/// an arbitrary number of entities.
#[macro_export]
macro_rules! enum_entity {
    (
        $(
            $(#[$attr:meta])* 
            $enum_vis:vis enum $enum_name:ident {
                $( $entity_name:ident = $entity_type_id:literal ),*
                $(,)?
            }
        )*
    ) => {
        $(
            $(#[$attr])* 
            $enum_vis enum $enum_name {
                $( $entity_name ( $entity_name ),)*
            }

            impl $crate::net::app::common::entity::Entity for $enum_name {
                fn type_id(&self) -> u16 {
                    match self {
                        $( Self::$entity_name (_) => $entity_type_id, )*
                    }
                }
                fn encode(&self, write: &mut impl std::io::Write) -> std::io::Result<()> {
                    use $crate::net::app::common::entity::Entity;
                    match self {
                        $( Self::$entity_name (e) => Entity::encode(e, write), )*
                    }
                }
                fn decode(read: &mut impl std::io::Read, type_id: u16) -> std::io::Result<Self> {
                    use $crate::net::app::common::entity::Entity;
                    Ok(match type_id {
                        $( $entity_type_id => Self::$entity_name(Entity::decode(read, type_id)?), )*
                        _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("invalid entity type id: 0x{type_id:02X}")))
                    })
                }
            }
        )*
    };
}


/// Abstract type representing an enumeration of all entity types.
pub trait Entity: Sized {

    fn type_id(&self) -> u16;

    fn encode(&self, write: &mut impl Write) -> io::Result<()>;

    fn decode(read: &mut impl Read, type_id: u16) -> io::Result<Self>;

}

/// An alternative to implementing the [`Entity`] trait that only requires the type to
/// already implement the [`DataType`] trait and also provides a const TYPE ID.
pub trait DataTypeEntity: DataType {

    /// Constant type id to return for this data type.
    const TYPE_ID: u16;

}

impl<E: DataTypeEntity> Entity for E {

    #[inline]
    fn type_id(&self) -> u16 {
        Self::TYPE_ID
    }

    #[inline]
    fn encode(&self, write: &mut impl Write) -> io::Result<()> {
        DataType::write(self, write)
    }

    #[inline]
    fn decode(read: &mut impl Read, _type_id: u16) -> io::Result<Self> {
        DataType::read(read)
    }

}


// /// Abstract type for an entity, each entity has its own type.
// pub trait Entity: Sized {

//     /// The entity type id.
//     const ID: u16;

//     /// Type for the client method.
//     type ClientMethod: Method;

//     /// Type for the base method.
//     type BaseMethod: Method;

//     /// Type for the cell method.
//     type CellMethod: Method;

//     /// Encode the entity with its initial properties with the given writer.
//     fn encode(&self, write: &mut impl Write) -> io::Result<()>;

//     /// Decode the entity with its initial properties from the given reader.
//     fn decode(read: &mut impl Read, len: usize) -> io::Result<Self>;

// }

/// Abstract entity method type, usually implemented as an enumeration.
pub trait Method: Sized {

    /// The id exposed for the network protocol.
    fn id(&self) -> u32;

    /// Encode the entity with its initial properties with the given writer.
    fn encode(&self, write: &mut impl Write) -> io::Result<()>;

    /// Decode the entity with its initial properties from the given reader.
    fn decode(read: &mut impl Read, len: usize, id: u32) -> io::Result<Self>;

}
