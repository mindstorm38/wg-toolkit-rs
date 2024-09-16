//! Client application implemented by the client.

pub mod element;


/// This modules defines numerical identifiers for client app elements.
pub mod id {

    // use super::ElementIdRange;

    pub const UPDATE_FREQUENCY_NOTIFICATION: u8 = 0x02;
    pub const SET_GAME_TIME: u8                 = 0x03;
    pub const RESET_ENTITIES: u8                = 0x04;
    pub const CREATE_BASE_PLAYER: u8            = 0x05;
    pub const CREATE_CELL_PLAYER: u8            = 0x06;
    // TODO: 0x07: DummyPacket
    // TODO: 0x08: SpaceProperty
    // TODO: 0x09: AddSpaceGeometryMapping
    // TODO: 0x0A: RemoveSpaceGeometryMapping
    // TODO: 0x0B: CreateEntity
    // TODO: 0x0C: CreateEntityDetailed
    pub const TICK_SYNC: u8                     = 0x13;
    pub const SELECT_PLAYER_ENTITY: u8          = 0x1A;
    pub const FORCED_POSITION: u8               = 0x1B;

    // pub const ENTITY_METHOD: ElementIdRange     = ElementIdRange::new(0xA7, 0xFE);

}
