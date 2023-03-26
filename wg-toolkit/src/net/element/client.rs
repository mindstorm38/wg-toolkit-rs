//! Definition of the elements that can be sent from server to client
//! once connected to the base application..


use std::io::{self, Write, Read};

use glam::Vec3A;

use crate::util::io::*;

use super::{Element, SimpleElement, TopElement, EmptyElement, ElementLength};


/// The server informs us how frequently it is going to send update
/// the the client, and also give the server game time (exactly the
/// same as [`SetGameTime`] element, but inlined here).
#[derive(Debug)]
pub struct UpdateFrequencyNotification {
    /// The frequency in hertz.
    pub frequency: u8,
    /// The server game time.
    pub game_time: u32,
}

impl UpdateFrequencyNotification {
    pub const ID: u8 = 0x02;
}

impl SimpleElement for UpdateFrequencyNotification {

    fn encode<W: Write>(&self, mut write: W) -> io::Result<()> {
        write.write_u8(self.frequency)?;
        write.write_u16(1)?;
        write.write_u32(self.game_time)
    }

    fn decode<R: Read>(mut read: R, _len: usize) -> io::Result<Self> {
        Ok(Self { 
            frequency: read.read_u8()?,
            // Skip 2 bytes that we don't use.
            game_time: { read.skip::<2>()?; read.read_u32()? },
        })
    }
}

impl TopElement for UpdateFrequencyNotification {
    const LEN: ElementLength = ElementLength::Fixed(7);
}


/// The server informs us of the current (server) game time.
#[derive(Debug)]
pub struct SetGameTime {
    /// The server game time.
    pub game_time: u32,
}

impl SetGameTime {
    pub const ID: u8 = 0x03;
}

impl SimpleElement for SetGameTime {

    fn encode<W: Write>(&self, mut write: W) -> io::Result<()> {
        write.write_u32(self.game_time)
    }

    fn decode<R: Read>(mut read: R, _len: usize) -> io::Result<Self> {
        Ok(Self { game_time: read.read_u32()? })
    }

}

impl TopElement for SetGameTime {
    const LEN: ElementLength = ElementLength::Fixed(4);
}


/// The server wants to resets the entities in the Area of Interest (AoI).
#[derive(Debug)]
pub struct ResetEntities {
    pub keep_player_on_base: bool,
}

impl ResetEntities {
    pub const ID: u8 = 0x04;
}

impl SimpleElement for ResetEntities {

    fn encode<W: Write>(&self, mut write: W) -> io::Result<()> {
        write.write_u8(self.keep_player_on_base as _)
    }

    fn decode<R: Read>(mut read: R, _len: usize) -> io::Result<Self> {
        Ok(Self { keep_player_on_base: read.read_u8()? != 0 })
    }

}

impl TopElement for ResetEntities {
    const LEN: ElementLength = ElementLength::Fixed(1);
}


/// Sent from the base when a player should be created, the entity id
/// is given with its type.
/// 
/// The remaining data will later be decoded properly depending on the
/// entity type, it's used for initializing its properties (TODO).
/// For example the `Login` entity receive the account UID.
#[derive(Debug)]
pub struct CreateBasePlayer<E> {
    pub entity_id: u32,
    pub entity_type: u16,
    pub entity_data: E,
}

impl CreateBasePlayer<()> {
    pub const ID: u8 = 0x05;
}

impl<E: Element<Config = ()>> SimpleElement for CreateBasePlayer<E> {

    fn encode<W: Write>(&self, mut write: W) -> io::Result<()> {
        write.write_u32(self.entity_id)?;
        write.write_u16(self.entity_type)?;
        write.write_u8(0)?; // An (apparently) useless byte here.
        self.entity_data.encode(write, &())
    }

    fn decode<R: Read>(mut read: R, len: usize) -> io::Result<Self> {
        Ok(Self {
            entity_id: read.read_u32()?,
            entity_type: read.read_u16()?,
            entity_data: {
                let _ = read.read_u8()?;
                E::decode(read, len - 7, &())?
            }
        })
    }
}

impl<E: Element<Config = ()>> TopElement for CreateBasePlayer<E> {
    const LEN: ElementLength = ElementLength::Variable16;
}


// TODO: 0x06: CreateCellPlayer
// TODO: 0x07: DummyPacket
// TODO: 0x08: SpaceProperty
// TODO: 0x09: AddSpaceGeometryMapping
// TODO: 0x0A: RemoveSpaceGeometryMapping
// TODO: 0x0B: CreateEntity
// TODO: 0x0C: CreateEntityDetailed


/// It is used as a timestamp for the elements in a bundle.
#[derive(Debug)]
pub struct TickSync {
    pub tick: u8,
}

impl TickSync {
    pub const ID: u8 = 0x13;
}

impl SimpleElement for TickSync {

    fn encode<W: Write>(&self, mut write: W) -> io::Result<()> {
        write.write_u8(self.tick)
    }

    fn decode<R: Read>(mut read: R, _len: usize) -> io::Result<Self> {
        Ok(Self { tick: read.read_u8()? })
    }

}

impl TopElement for TickSync {
    const LEN: ElementLength = ElementLength::Fixed(1);
}


/// Sent by the server to inform that subsequent elements will target
/// the player entity.
#[derive(Debug, Default)]
pub struct SelectPlayerEntity;

impl SelectPlayerEntity {
    pub const ID: u8 = 0x1A;
}

impl EmptyElement for SelectPlayerEntity {}


/// This is when an update is being forced back for an (ordinarily)
/// client controlled entity, including for the player. Usually this is
/// due to a physics correction from the server, but it could be for any
/// reason decided by the server (e.g. server-initiated teleport).
#[derive(Debug)]
pub struct ForcedPosition {
    pub entity_id: u32,
    pub space_id: u32,
    pub vehicle_entity_id: u32,
    pub position: Vec3A,
    pub direction: Vec3A,
}

impl ForcedPosition {
    pub const ID: u8 = 0x1B;
}

impl SimpleElement for ForcedPosition {

    fn encode<W: Write>(&self, mut write: W) -> io::Result<()> {
        write.write_u32(self.entity_id)?;
        write.write_u32(self.space_id)?;
        write.write_u32(self.vehicle_entity_id)?;
        write.write_vec3(self.position)?;
        write.write_vec3(self.direction)
    }

    fn decode<R: Read>(mut read: R, _len: usize) -> io::Result<Self> {
        Ok(Self {
            entity_id: read.read_u32()?,
            space_id: read.read_u32()?,
            vehicle_entity_id: read.read_u32()?,
            position: read.read_vec3()?,
            direction: read.read_vec3()?,
        })
    }

}


/// A call to a selected entity's method.
#[derive(Debug)]
pub struct EntityMethod {
    
}

impl EntityMethod {
    
    pub const FIRST_ID: u8 = 0x4E;
    pub const LAST_ID: u8 = 0xA6;

    /// Convert a method index to a message id.
    pub const fn index_to_id(index: u8) -> u8 {
        Self::FIRST_ID + index
    }

    /// Convert a message id to method index.
    pub const fn id_to_index(id: u8) -> u8 {
        id - Self::FIRST_ID
    }

}


/// Setting a selected entity's property value.
#[derive(Debug)]
pub struct EntityProperty {

}

impl EntityProperty {

    pub const FIRST_ID: u8 = 0xA7;
    pub const LAST_ID: u8 = 0xFE;

    /// Convert a property index to a message id.
    pub const fn index_to_id(index: u8) -> u8 {
        Self::FIRST_ID + index
    }

    /// Convert a message id to property index.
    pub const fn id_to_index(id: u8) -> u8 {
        id - Self::FIRST_ID
    }

}
