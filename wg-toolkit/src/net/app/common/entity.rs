use std::io::{self, Read, Write};

use crate::net::element::ElementLength;

use super::data::DataType;


// /// This macro can be used to generate an enumeration capable of encoding and decoding
// /// an arbitrary number of entities, the enumeration itself implements the [`Entity`]
// /// trait, so the enumeration acts as a single generic entity.
// #[macro_export]
// macro_rules! __bootstrap_enum_entities {
//     (
//         $(
//             $(#[$attr:meta])* 
//             $enum_vis:vis enum $enum_name:ident : $client_method_name:ident , $base_method_name:ident , $cell_method_name:ident {
//                 $( $entity_name:ident = $entity_type_id:literal ),*
//                 $(,)?
//             }
//         )*
//     ) => {
//         $(
//             $(#[$attr])* 
//             $enum_vis enum $enum_name {
//                 $( $entity_name ( $entity_name ),)*
//             }

//             $(#[$attr])* 
//             $enum_vis enum $client_method_name {
//                 $( $entity_name ( <$entity_name as $crate::net::app::common::entity::Entity>::ClientMethod ),)*
//             }

//             $(#[$attr])* 
//             $enum_vis enum $base_method_name {
//                 $( $entity_name ( <$entity_name as $crate::net::app::common::entity::Entity>::BaseMethod ),)*
//             }

//             $(#[$attr])* 
//             $enum_vis enum $cell_method_name {
//                 $( $entity_name ( <$entity_name as $crate::net::app::common::entity::Entity>::CellMethod ),)*
//             }

//             impl $crate::net::app::common::entity::Method for $client_method_name {
//                 fn type_id(&self) -> u16 {
//                     match self {
//                         $( Self::$entity_name (_) => $entity_type_id, )*
//                         _ => unreachable!()
//                     }
//                 }
//                 fn exposed_id(&self) -> u16 {
//                     match self {
//                         $( Self::$entity_name (m) => m.exposed_id(), )*
//                         _ => unreachable!()
//                     }
//                 }
//                 fn encode(&self, write: &mut impl std::io::Write) -> std::io::Result<()> {
//                     use $crate::net::app::common::entity::Method;
//                     match self {
//                         $( Self::$entity_name (m) => m.encode(write), )*
//                         _ => unreachable!()
//                     }
//                 }
//                 fn decode(read: &mut impl std::io::Read, type_id: u16, exposed_id: u16) -> std::io::Result<Self> {
//                     use $crate::net::app::common::entity::Method;
//                     Ok(match type_id {
//                         $( $entity_type_id => Self::$entity_name(Method::decode(read, type_id, exposed_id)?), )*
//                         _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("invalid method type id: 0x{type_id:02X}")))
//                     })
//                 }
//             }

//             impl $crate::net::app::common::entity::Method for $base_method_name {
//                 fn type_id(&self) -> u16 {
//                     match self {
//                         $( Self::$entity_name (_) => $entity_type_id, )*
//                         _ => unreachable!()
//                     }
//                 }
//                 fn exposed_id(&self) -> u16 {
//                     match self {
//                         $( Self::$entity_name (m) => m.exposed_id(), )*
//                         _ => unreachable!()
//                     }
//                 }
//                 fn encode(&self, write: &mut impl std::io::Write) -> std::io::Result<()> {
//                     use $crate::net::app::common::entity::Method;
//                     match self {
//                         $( Self::$entity_name (m) => m.encode(write), )*
//                         _ => unreachable!()
//                     }
//                 }
//                 fn decode(read: &mut impl std::io::Read, type_id: u16, exposed_id: u16) -> std::io::Result<Self> {
//                     use $crate::net::app::common::entity::Method;
//                     Ok(match type_id {
//                         $( $entity_type_id => Self::$entity_name(Method::decode(read, type_id, exposed_id)?), )*
//                         _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("invalid method type id: 0x{type_id:02X}")))
//                     })
//                 }
//             }

//             impl $crate::net::app::common::entity::Method for $cell_method_name {
//                 fn type_id(&self) -> u16 {
//                     match self {
//                         $( Self::$entity_name (_) => $entity_type_id, )*
//                         _ => unreachable!()
//                     }
//                 }
//                 fn exposed_id(&self) -> u16 {
//                     match self {
//                         $( Self::$entity_name (m) => m.exposed_id(), )*
//                         _ => unreachable!()
//                     }
//                 }
//                 fn encode(&self, write: &mut impl std::io::Write) -> std::io::Result<()> {
//                     use $crate::net::app::common::entity::Method;
//                     match self {
//                         $( Self::$entity_name (m) => m.encode(write), )*
//                         _ => unreachable!()
//                     }
//                 }
//                 fn decode(read: &mut impl std::io::Read, type_id: u16, exposed_id: u16) -> std::io::Result<Self> {
//                     use $crate::net::app::common::entity::Method;
//                     Ok(match type_id {
//                         $( $entity_type_id => Self::$entity_name(Method::decode(read, type_id, exposed_id)?), )*
//                         _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("invalid method type id: 0x{type_id:02X}")))
//                     })
//                 }
//             }

//             impl $crate::net::app::common::entity::Entity for $enum_name {
//                 type ClientMethod = $client_method_name;
//                 type BaseMethod = $base_method_name;
//                 type CellMethod = $cell_method_name;
//                 fn type_id(&self) -> u16 {
//                     match self {
//                         $( Self::$entity_name (_) => $entity_type_id, )*
//                         _ => unreachable!()
//                     }
//                 }
//                 fn encode(&self, write: &mut impl std::io::Write) -> std::io::Result<()> {
//                     use $crate::net::app::common::entity::Entity;
//                     match self {
//                         $( Self::$entity_name (e) => Entity::encode(e, write), )*
//                         _ => unreachable!()
//                     }
//                 }
//                 fn decode(read: &mut impl std::io::Read, type_id: u16) -> std::io::Result<Self> {
//                     use $crate::net::app::common::entity::Entity;
//                     Ok(match type_id {
//                         $( $entity_type_id => Self::$entity_name(Entity::decode(read, type_id)?), )*
//                         _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("invalid entity type id: 0x{type_id:02X}")))
//                     })
//                 }
//             }
//         )*
//     };
// }

/// This macro can be used to generate an enumeration capable of encoding and decoding
/// an arbitrary number of methods, the enumeration implements the [`Method`] trait, and
/// all methods should 
#[macro_export]
macro_rules! __bootstrap_enum_methods {
    (__length; $length:literal) => { $crate::net::element::ElementLength::Fixed($length) };
    (__length; var8 ) => { $crate::net::element::ElementLength::Variable8 };
    (__length; var16 ) => { $crate::net::element::ElementLength::Variable16 };
    (__length; var24 ) => { $crate::net::element::ElementLength::Variable24 };
    (__length; var32 ) => { $crate::net::element::ElementLength::Variable32 };
    (
        $(
            $(#[$attr:meta])* 
            $enum_vis:vis enum $enum_name:ident {
                $( $method_name:ident ( $method_exposed_id:literal, $method_length:tt ) ),*
                $(,)?
            }
        )*
    ) => {
        $(
            $(#[$attr])* 
            $enum_vis enum $enum_name {
                $( $method_name ( $method_name ),)*
            }

            impl $crate::net::app::common::entity::Method for $enum_name {
                // fn exposed_id(&self) -> u16 {
                //     match self {
                //         $( Self::$method_name (_) => $method_exposed_id, )*
                //         _ => unreachable!()
                //     }
                // }
                fn encode_length(&self) -> $crate::net::element::ElementLength {
                    match self {
                        $( Self::$method_name (_) => $crate::__bootstrap_enum_methods!(__length; $method_length), )*
                        _ => unreachable!()
                    }
                }
                fn encode(&self, write: &mut dyn std::io::Write) -> std::io::Result<u16> {
                    use $crate::net::app::common::data::DataType;
                    match self {
                        $( Self::$method_name (m) => DataType::write(m, write).map(|()| $method_exposed_id), )*
                        _ => unreachable!()
                    }
                }
                fn decode_length(exposed_id: u16) -> $crate::net::element::ElementLength {
                    match exposed_id {
                        $( $method_exposed_id => $crate::__bootstrap_enum_methods!(__length; $method_length), )*
                        _ => panic!()
                    }
                }
                fn decode(read: &mut dyn std::io::Read, exposed_id: u16) -> std::io::Result<Self> {
                    use $crate::net::app::common::data::DataType;
                    Ok(match exposed_id {
                        $( $method_exposed_id => Self::$method_name(DataType::read(read)?), )*
                        _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("invalid method exposed id: 0x{exposed_id:02X}")))
                    })
                }
            }
        )*
    };
}


/// Abstract type representing an entity type.
pub trait Entity: Sized {

    /// The client method enum type associated to this entity.
    type ClientMethod: Method;
    /// The base method enum type associated to this entity.
    type BaseMethod: Method;
    /// The cell method enum type associated to this entity.
    type CellMethod: Method;

    fn encode(&self, write: &mut dyn Write) -> io::Result<()>;

    fn decode(read: &mut dyn Read) -> io::Result<Self>;

}

/// Abstract type representing a method for an entity.
pub trait Method: Sized {
    
    // /// Return the exposed id for this method.
    // fn exposed_id(&self) -> u16;

    /// Return the preferred encoding length of this method, when sub message id is used
    /// this is just ignored.
    fn encode_length(&self) -> ElementLength;

    /// Encode the method call into the given writer.
    fn encode(&self, write: &mut dyn Write) -> io::Result<u16>;

    /// Return the decode length for the given exposed method id.
    fn decode_length(exposed_id: u16) -> ElementLength;

    /// Decode the given method from the given reader and its exposed id.
    fn decode(read: &mut dyn Read, exposed_id: u16) -> io::Result<Self>;

}


/// An alternative to implementing the [`Entity`] trait that only requires the type to
/// already implement the [`DataType`] trait and also provides a const TYPE ID.
pub trait DataTypeEntity: DataType {

    /// The client method enum type associated to this entity.
    type ClientMethod: Method;
    /// The base method enum type associated to this entity.
    type BaseMethod: Method;
    /// The cell method enum type associated to this entity.
    type CellMethod: Method;
    
}

impl<E: DataTypeEntity> Entity for E {

    type ClientMethod = <E as DataTypeEntity>::ClientMethod;
    type BaseMethod = <E as DataTypeEntity>::BaseMethod;
    type CellMethod = <E as DataTypeEntity>::CellMethod;

    #[inline]
    fn encode(&self, write: &mut dyn Write) -> io::Result<()> {
        DataType::write(self, write)
    }

    #[inline]
    fn decode(read: &mut dyn Read) -> io::Result<Self> {
        DataType::read(read)
    }

}
