# WG Toolkit
Open-source toolkit crate providing various codecs for binary and text formats distributed by [Wargaming.net](https://wargaming.net/). These formats are used by [Core](https://wotencore.net/) engine *(previously BigWorld)* notably used by World of Tanks.

[![Crates.io](https://img.shields.io/crates/v/wg-toolkit?style=flat-square)](https://crates.io/crates/wg-toolkit)&nbsp;&nbsp;[![Crates.io](https://img.shields.io/crates/d/wg-toolkit?style=flat-square)](https://crates.io/crates/wg-toolkit)

## Links
- [Crate page](https://crates.io/crates/wg-toolkit)
- [Crate documentation](https://docs.rs/wg-toolkit)

## Features
- Compiled spaces decoding *(not all sections are implemented)*.
- XML unpacking.
- FNV hashing algorithm.

## Missing
- Compiled spaces encoding and many sections decoding.
- Compiled model parsing *(for vehicles for example)*.
- XML packing.

## Credits
Thanks to SkepticalFox for [wot-space.bin-utils](https://bitbucket.org/SkepticalFox/wot-space.bin-utils/src/master/) python library, which directly inspired this crate.
