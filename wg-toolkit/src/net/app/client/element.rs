//! Definition of the elements that can be sent from server to client
//! once connected to the base application..

use std::io::{self, Read, Write};

use glam::Vec3A;

use crate::net::element::{ElementLength, NoopElement, SimpleElement, TopElement, DebugElementFixed, DebugElementVariable16};
use crate::util::io::*;


#[derive(Debug, Clone)]
pub struct Authenticate {
    pub key: u32,
}

impl SimpleElement for Authenticate {

    fn encode(&self, write: &mut impl Write) -> io::Result<()> {
        write.write_u32(self.key)
    }

    fn decode(read: &mut impl Read, _len: usize) -> io::Result<Self> {
        Ok(Self {
            key: read.read_u32()?,
        })
    }

}

#[derive(Debug, Clone)]
pub struct BandwidthNotification {
    pub bps: u32,
}

impl SimpleElement for BandwidthNotification {

    fn encode(&self, write: &mut impl Write) -> io::Result<()> {
        write.write_u32(self.bps)
    }

    fn decode(read: &mut impl Read, _len: usize) -> io::Result<Self> {
        Ok(Self {
            bps: read.read_u32()?,
        })
    }

}


/// The server informs us how frequently it is going to send update
/// the the client, and also give the server game time (exactly the
/// same as [`SetGameTime`] element, but inlined here).
#[derive(Debug, Clone)]
pub struct UpdateFrequencyNotification {
    /// The frequency in hertz.
    pub frequency: u8,
    /// The server game time.
    pub game_time: u32,
}

impl SimpleElement for UpdateFrequencyNotification {

    fn encode(&self, write: &mut impl Write) -> io::Result<()> {
        write.write_u8(self.frequency)?;
        write.write_u16(1)?;
        write.write_u32(self.game_time)
    }

    fn decode(read: &mut impl Read, _len: usize) -> io::Result<Self> {
        Ok(Self { 
            frequency: read.read_u8()?,
            // Skip 2 bytes that we don't use.
            game_time: { read.read_u16()?; read.read_u32()? },
        })
    }

}

impl TopElement for UpdateFrequencyNotification {
    const LEN: ElementLength = ElementLength::Fixed(7);
}


/// The server informs us of the current (server) game time.
#[derive(Debug, Clone)]
pub struct SetGameTime {
    /// The server game time.
    pub game_time: u32,
}

impl SimpleElement for SetGameTime {

    fn encode(&self, write: &mut impl Write) -> io::Result<()> {
        write.write_u32(self.game_time)
    }

    fn decode(read: &mut impl Read, _len: usize) -> io::Result<Self> {
        Ok(Self { game_time: read.read_u32()? })
    }

}

impl TopElement for SetGameTime {
    const LEN: ElementLength = ElementLength::Fixed(4);
}


/// The server wants to resets the entities in the Area of Interest (AoI).
#[derive(Debug, Clone)]
pub struct ResetEntities {
    pub keep_player_on_base: bool,
}

impl SimpleElement for ResetEntities {

    fn encode(&self, write: &mut impl Write) -> io::Result<()> {
        write.write_bool(self.keep_player_on_base)
    }

    fn decode(read: &mut impl Read, _len: usize) -> io::Result<Self> {
        Ok(Self { keep_player_on_base: read.read_bool()? })
    }

}

impl TopElement for ResetEntities {
    const LEN: ElementLength = ElementLength::Fixed(1);
}


pub type CreateBasePlayer = DebugElementVariable16;
pub type CreateCellPlayer = DebugElementVariable16;
pub type DummyPacket = DebugElementVariable16;
pub type SpaceProperty = DebugElementVariable16;
pub type AddSpaceGeometryMapping = DebugElementVariable16;
pub type RemoveSpaceGeometryMapping = DebugElementVariable16;
pub type CreateEntity = DebugElementVariable16;
pub type CreateEntityDetailed = DebugElementVariable16;

#[derive(Debug, Default, Clone, Copy)]
pub struct CellAppSuspended;
impl NoopElement for CellAppSuspended {}
impl TopElement for CellAppSuspended {
    const LEN: ElementLength = ElementLength::Fixed(0);
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CellAppResumed;
impl NoopElement for CellAppResumed {}
impl TopElement for CellAppResumed {
    const LEN: ElementLength = ElementLength::Fixed(0);
}

pub type ClientSuspensionDetectionEnabled = DebugElementFixed<4>;
pub type EnterAoi = DebugElementFixed<5>;
pub type EnterAoiOnVehicle = DebugElementFixed<9>;
pub type LeaveAoi = DebugElementVariable16;

// /// Sent from the base when a player should be created, the entity id
// /// is given with its type.
// /// 
// /// The remaining data will later be decoded properly depending on the
// /// entity type, it's used for initializing its properties (TODO).
// /// For example the `Login` entity receive the account UID.
// #[derive(Debug, Clone)]
// pub struct CreateBasePlayer<E> {
//     /// The unique identifier of the entity being created.
//     pub entity_id: u32,
//     /// The entity type identifier being created.
//     pub entity_type: u16,
//     /// This string's usage is currently unknown.
//     pub unk: String,
//     /// The actual data to be sent for creating the player's entity.
//     pub entity_data: E,
//     /// This integer describe the number of entity components composing
//     /// the entity, this value must be strictly equal to the same value
//     /// as the client.
//     /// 
//     /// TODO: This number is used to know how much entity components
//     /// must be parsed after this number. Components can be seen as
//     /// regular components. **It's not currently implemented.**
//     pub entity_components_count: u8,
// }

// impl<E: Element<Config = ()>> SimpleElement for CreateBasePlayer<E> {

//     fn encode(&self, write: &mut impl Write) -> io::Result<()> {
//         write.write_u32(self.entity_id)?;
//         write.write_u16(self.entity_type)?;
//         write.write_string_variable(&self.unk)?;
//         self.entity_data.encode(&mut *write, &())?;
//         write.write_u8(self.entity_components_count)
//     }

//     fn decode(read: &mut impl Read, len: usize) -> io::Result<Self> {
//         Ok(Self {
//             entity_id: read.read_u32()?,
//             entity_type: read.read_u16()?,
//             unk: read.read_string_variable()?,
//             entity_data: E::decode(&mut *read, len - 7, &())?,
//             entity_components_count: read.read_u8()?,
//         })
//     }
// }

// impl<E: Element<Config = ()>> TopElement for CreateBasePlayer<E> {
//     const LEN: ElementLength = ElementLength::Variable16;
// }


/// It is used as a timestamp for the elements in a bundle.
#[derive(Debug, Clone)]
pub struct TickSync {
    pub tick: u8,
}

impl SimpleElement for TickSync {

    fn encode(&self, write: &mut impl Write) -> io::Result<()> {
        write.write_u8(self.tick)
    }

    fn decode(read: &mut impl Read, _len: usize) -> io::Result<Self> {
        Ok(Self { tick: read.read_u8()? })
    }

}

impl TopElement for TickSync {
    const LEN: ElementLength = ElementLength::Fixed(1);
}

pub type TickSyncPeriodic = DebugElementFixed<2>;
pub type RelativePositionReference = DebugElementFixed<1>;
pub type RelativePosition = DebugElementFixed<12>;
pub type SetVehicle = DebugElementFixed<8>;
pub type SelectAliasedEntity = DebugElementFixed<1>;
pub type SelectEntity = DebugElementFixed<4>;


/// Sent by the server to inform that subsequent elements will target
/// the player entity.
#[derive(Debug, Default, Clone, Copy)]
pub struct SelectPlayerEntity;
impl NoopElement for SelectPlayerEntity { }
impl TopElement for SelectPlayerEntity {
    const LEN: ElementLength = ElementLength::Fixed(0);
}


/// This is when an update is being forced back for an (ordinarily)
/// client controlled entity, including for the player. Usually this is
/// due to a physics correction from the server, but it could be for any
/// reason decided by the server (e.g. server-initiated teleport).
#[derive(Debug, Clone)]
pub struct ForcedPosition {
    pub entity_id: u32,
    pub space_id: u32,
    pub vehicle_entity_id: u32,
    pub position: Vec3A,
    pub direction: Vec3A,
}

impl SimpleElement for ForcedPosition {

    fn encode(&self, write: &mut impl Write) -> io::Result<()> {
        write.write_u32(self.entity_id)?;
        write.write_u32(self.space_id)?;
        write.write_u32(self.vehicle_entity_id)?;
        write.write_vec3(self.position)?;
        write.write_vec3(self.direction)
    }

    fn decode(read: &mut impl Read, _len: usize) -> io::Result<Self> {
        Ok(Self {
            entity_id: read.read_u32()?,
            space_id: read.read_u32()?,
            vehicle_entity_id: read.read_u32()?,
            position: read.read_vec3()?,
            direction: read.read_vec3()?,
        })
    }

}

impl TopElement for ForcedPosition {
    const LEN: ElementLength = ElementLength::Fixed(38);
}

pub type AvatarUpdateNoAliasDetailed = DebugElementFixed<29>;
pub type AvatarUpdateAliasDetailed = DebugElementFixed<26>;
pub type AvatarUpdatePlayerDetailed = DebugElementFixed<25>;
pub type AvatarUpdateVolatileProperties = DebugElementVariable16;
pub type ChangeVolatilePackerType = DebugElementVariable16;

pub type NrlCreateNode = DebugElementVariable16;
pub type NrlUnlinkNode = DebugElementVariable16;
pub type NrlUpdateNode = DebugElementVariable16;
pub type NrlUnlinkTreeFlag = DebugElementFixed<0>;
pub type NrlUpdateNodeFlag = DebugElementFixed<0>;
pub type NrlData = DebugElementVariable16;
pub type NrlMsgToClient = DebugElementVariable16;
pub type NrlUnreliableMsgToClient = DebugElementVariable16;

// TODO: Avatar update

pub type ControlEntity = DebugElementFixed<5>;
pub type VoiceData = DebugElementVariable16;
pub type RestoreClient = DebugElementVariable16;
pub type SwitchBaseApp = DebugElementFixed<9>;

pub type ResourceHeader = DebugElementVariable16;
pub type ResourceFragment = DebugElementVariable16;

pub type LoggedOff = DebugElementFixed<1>;

pub type DetailedPosition = DebugElementFixed<24>;

pub type NestedEntityProperty = DebugElementVariable16;
pub type SliceEntityProperty = DebugElementVariable16;
pub type UpdateEntity = DebugElementVariable16;
pub type SetCellAppExtAddress = DebugElementVariable16;
pub type LastProxyMessageAfterDirectCellAppConnection = DebugElementVariable16;
