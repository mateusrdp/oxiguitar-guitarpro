# Credits

Reading a closed format well is only possible because other people did the hard work of documenting it in the open. Thank you to every project below.

## Code we use

| Project | Authors | What it does here | Licence |
| --- | --- | --- | --- |
| [OxiGuitar's plugin SDK](https://github.com/mateusrdp/oxiguitar) (`oxiguitar-plugin`, `oxiguitar-plugin-api`, `oxiguitar-model`, and `oxiguitar-plugin-test` for the tests) | Mateus Rocha de Paula and the OxiGuitar contributors | The plugin interface and the score model | MIT OR Apache-2.0 for the interface and SDK; GPL-3.0-or-later for the model |
| [zip](https://github.com/zip-rs/zip2) | Mathijs van de Nes, Marli Frost, Ryan Levick, Chris Hennick and contributors | Guitar Pro 7 and 8 archives | MIT |
| [zlib-rs](https://github.com/trifectatechfoundation/zlib-rs) | The Trifecta Tech Foundation | Deflate compression, in pure Rust, under zip | Zlib |
| [quick-xml](https://github.com/tafia/quick-xml) | Johann Tuffe, Mingun and contributors | GPIF, the XML inside Guitar Pro 6, 7 and 8 files | MIT |
| [serde_json](https://github.com/serde-rs/json) | David Tolnay, Erick Tryzelaar and contributors | Tests and tools | MIT OR Apache-2.0 |
| [Rust](https://www.rust-lang.org/) and its standard library | The Rust Project | Everything else | MIT OR Apache-2.0 |

The readers and writers are our own code.

## Projects we learned the formats from

| Project | Authors | What we took | Licence |
| --- | --- | --- | --- |
| [alphaTab](https://github.com/CoderLine/alphaTab) | Danielku15 (CoderLine) and contributors | How Guitar Pro 6's container is compressed and laid out, what GPIF's elements mean, and a second reader to check our files against | MPL-2.0 |
| [PyGuitarPro](https://github.com/Perlence/PyGuitarPro) | Sviatoslav Abakumov (Perlence) and contributors | The byte layout of Guitar Pro 3 to 5, and a cross-check of our readers | LGPL-3.0 |
| [TuxGuitar](https://github.com/helge17/tuxguitar) | Julian Casadesus, helge17 and contributors | Its Guitar Pro readers, to settle what an effect's bytes mean | LGPL-2.1 |
| [MuseScore Studio](https://github.com/musescore/MuseScore) | Muse Group and contributors | A second program to check that what we write reads as we mean it | GPL-3.0 |

Each was used as documentation and as an independent check. No code from them is in these plugins.

## Tools

| Tool | Use |
| --- | --- |
| [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) and [libfuzzer-sys](https://github.com/rust-fuzz/libfuzzer) | Fuzzing the readers and writers (`fuzz/`) |
| [Node.js](https://nodejs.org/) and [Python](https://www.python.org/) | The interoperability checks against alphaTab and MuseScore (`scripts/interop/`) |
| [mdBook](https://github.com/rust-lang/mdBook) | Builds this manual |
| [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) | Checks the licences of everything we build with |

## Thanks

To Danielku15 and the alphaTab project, to Perlence and the PyGuitarPro contributors, and to the TuxGuitar team: years of patient reverse engineering, shared with everyone.

Guitar Pro is a trademark of Arobas Music. MuseScore is a trademark of Muse Group. These plugins are not affiliated with either.
