# WG Toolkit
Toolkit crate providing various implementations for codecs distributed by [Wargaming.net](https://wargaming.net/). 
These codecs are part of the [Core](https://wotencore.net/) engine *(previously known as BigWorld)* notably used by 
World of Tanks. This crate also provides an implementation of the network protocol.

[![Crates.io](https://img.shields.io/crates/v/wg-toolkit?style=flat-square)](https://crates.io/crates/wg-toolkit)&nbsp;&nbsp;[![Crates.io](https://img.shields.io/crates/d/wg-toolkit?style=flat-square)](https://crates.io/crates/wg-toolkit)

## Links
- [Crate page](https://crates.io/crates/wg-toolkit)
- [Crate documentation](https://docs.rs/wg-toolkit)

## Features
- Packed XML codec
  - Deserialization
  - Serialization
- Tank model codec
  - Deserialization of visual tree
  - Deserialization of vertices/indices
- Compiled space codec *(WIP)*
  - Deserialization of some sections
    - BWTB (header table)
    - BWST (string table)
    - BWT2 (terrain2)
    - BWSG (static geometry)
    - BWCS (compiled space settings)
    - BWAL (asset list)
- Network protocol *(WIP, feature `network`)*
  - Packets encoding and decoding *(partial flags support)*
  - Appending elements to bundles
  - Assemble received packet in bundles
  - Iterate elements in a bundle

## Upcomming features
- Command line utility crate
- Game's resource file system (automatic opening of packages)

## Contributing guidelines
When contributing to the code base, some rules should be followed:
1. Each major feature should have its own directory module;
2. Each side-feature, used internally by core features should be located under `util` module in its own file module.
3. When working on a custom reader and/or writer, but only implement one of the two, please anticipate how your module would be built with both implemented;
4. Catch all errors and forward them toward the public interface;
5. Custom error types should all be defined using `thiserror::Error` derivation;
6. When working on a custom reader and/or writer that doesn't provide lazy read/write operations, please make simple public functions that directly output, like `from_reader(reader: impl Read)` or `to_writer(writer: impl Write)`.

## Credits
Thanks to SkepticalFox for [wot-space.bin-utils](https://bitbucket.org/SkepticalFox/wot-space.bin-utils/src/master/) python library, which directly inspired this crate.

Thanks to SkaceKamen for [wot-model-converter](https://github.com/SkaceKamen/wot-model-converter) python library, for its open-source work on processed model file format.
