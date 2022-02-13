//! Module for packed XML decoding/encoding.

mod unpack;
pub use unpack::*;


const PACKED_HEADER: &[u8; 4] = b"\x45\x4E\xA1\x62";

