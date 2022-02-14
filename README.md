# WG Toolkit
Open-source toolkit crate providing various codecs for binary and text formats distributed by [Wargaming.net](https://wargaming.net/). These formats are used by [Core](https://wotencore.net/) engine *(previously BigWorld)* notably used by World of Tanks.

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
