//! BigWorld/Core network protocol.

pub mod packet;
pub mod element;
pub mod bundle;
pub mod interface;


/// Packet's flags.
#[repr(transparent)]
pub struct PacketFlags(());

impl PacketFlags {
    const HAS_REQUESTS: u16        = 0x0001;
    const HAS_PIGGYBACKS: u16      = 0x0002;
    const HAS_ACKS: u16            = 0x0004;
    const ON_CHANNEL: u16          = 0x0008;
    const IS_RELIABLE: u16         = 0x0010;
    const IS_FRAGMENT: u16         = 0x0020;
    const HAS_SEQUENCE_NUMBER: u16 = 0x0040;
    const INDEXED_CHANNEL: u16     = 0x0080;
    const HAS_CHECKSUM: u16        = 0x0100;
    const CREATE_CHANNEL: u16      = 0x0200;
    const HAS_CUMULATIVE_ACK: u16  = 0x0400;
}
