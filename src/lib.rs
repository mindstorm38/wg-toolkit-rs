//! Toolkit for various binary and text formats distributed by Wargaming.net (BigWorld, Core engine).
//!
//! Credits to SkepticalFox for its works at
//! https://bitbucket.org/SkepticalFox/wot-space.bin-utils/src/master/

pub mod util;

#[cfg(feature = "space")]
pub mod space;

#[cfg(feature = "xml")]
pub mod xml;

#[cfg(feature = "network")]
pub mod net;
