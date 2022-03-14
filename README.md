# WG Toolkit
Toolkit crate providing various implementations for codecs distributed by [Wargaming.net](https://wargaming.net/). 
These codecs are part of the [Core](https://wotencore.net/) engine *(previously known as BigWorld)* notably used by 
World of Tanks. This crate also provides an implementation of the network protocol.

[![Crates.io](https://img.shields.io/crates/v/wg-toolkit?style=flat-square)](https://crates.io/crates/wg-toolkit)&nbsp;&nbsp;[![Crates.io](https://img.shields.io/crates/d/wg-toolkit?style=flat-square)](https://crates.io/crates/wg-toolkit)

## Links
- [Crate page](https://crates.io/crates/wg-toolkit)
- [Crate documentation](https://docs.rs/wg-toolkit)

## Features
- Network protocol *(WIP)*
  - Packets encoding and decoding *(partial flags support)*
  - Appending elements to bundles
  - Assemble received packet in bundles
  - Iterate elements in a bundle
- Compiled spaces decoding *(partial sections support)*
- XML unpacking
- FNV hashing algorithm

## Missing
- Compiled spaces encoding and many sections decoding
- Compiled model parsing *(for vehicles for example)*
- XML packing

## Credits
Thanks to SkepticalFox for [wot-space.bin-utils](https://bitbucket.org/SkepticalFox/wot-space.bin-utils/src/master/) python library, which directly inspired this crate.
