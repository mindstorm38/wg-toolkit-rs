//! Module for packed XML codec.

mod unpack;
pub use unpack::*;


pub const PACKED_SIGNATURE: &[u8; 4] = b"\x45\x4E\xA1\x62";

