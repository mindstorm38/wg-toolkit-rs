//! Toolkit for various binary and text formats distributed by Wargaming.net (BigWorld, Core engine).
//!
//! Credits to SkepticalFox for its work on compiled spaces:
//! https://bitbucket.org/SkepticalFox/wot-space.bin-utils/src/master/
//! 
//! Credits to SkaceKamen for its work on compiled model:
//! https://github.com/SkaceKamen/wot-model-converter

pub mod util;
pub mod pxml;

pub mod space;
pub mod model;

#[cfg(feature = "res")]
pub mod res;

#[cfg(feature = "net")]
pub mod net;
