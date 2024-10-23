use std::io::{self, Read, Write};


/// Abstract type for an entity, each entity has its own type.
pub trait Entity: Sized {

    /// The entity type id.
    const ID: u16;

    /// Type for the client method.
    type ClientMethod: Method;

    /// Type for the base method.
    type BaseMethod: Method;

    /// Type for the cell method.
    type CellMethod: Method;

    /// Encode the entity with its initial properties with the given writer.
    fn encode(&self, write: &mut impl Write) -> io::Result<()>;

    /// Decode the entity with its initial properties from the given reader.
    fn decode(read: &mut impl Read, len: usize) -> io::Result<Self>;

}

/// Abstract entity method type, usually implemented as an enumeration.
pub trait Method: Sized {

    /// The id exposed for the network protocol.
    fn id(&self) -> u32;

    /// Encode the entity with its initial properties with the given writer.
    fn encode(&self, write: &mut impl Write) -> io::Result<()>;

    /// Decode the entity with its initial properties from the given reader.
    fn decode(read: &mut impl Read, len: usize, id: u32) -> io::Result<Self>;

}

/// Represent an element data type
pub trait DataType {

}
