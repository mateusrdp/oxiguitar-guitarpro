# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project
follows [Semantic Versioning](https://semver.org/). A release's number is
the plugins' own; the plugin interface they use is in each plugin's
`manifest.json` and in the release notes.

## [Unreleased]

## [0.3.0] - 2026-11-25

### Changed

- **The plugin file is a tar archive** (mateusrdp/oxiguitar#347): `build-oxp.sh` packs `manifest.json` first and the library second (`plugin.so`, `plugin.dll` or `plugin.dylib`), as OxiGuitar 0.23 and later read plugin files, which older OxiGuitars can't load. The SDK is at OxiGuitar v0.23.0 (interface 1.16); the readers give the score model's newer fields (plugins' data) their defaults, and the host tests name no display system (`NO_DISPLAY`), as the SDK now has them. `scripts/test-all.py` 1.1.0, as in every repository.

### Added

- **Every test with one command** (mateusrdp/oxiguitar#273): `scripts/test-all.py` builds and runs all of this repository's tests on your own corpus (`--corpus DIR` or `OXIGUITAR_CORPUS`), and tests the plugin against the newest OxiGuitar release, with a progress bar, a summary to paste into an issue and a log to diagnose from. The manual has a Features page naming the tests of every feature (CI checks it) and a Testing page.

### Changed

- **CI outlasts a slow Ubuntu mirror**: it installs packages with `scripts/apt-install.sh`, the same file as in every OxiGuitar repository. apt is stopped only when its downloads have not grown for 2 minutes, then tried again on the next mirror; while they grow, it may take as long as it needs (24 Nov 2026: fixed 5-minute tries failed jobs on mirrors at 80 to 185 kB/s).
- **The manual explains the ideas behind the plugin** (its overview, with pictures), shows how to use it with screenshots, says in Architecture and strategy what it can't do yet, what is left out on purpose and where help is wanted, and has a Credits page listing every project it uses, with links and thanks. The README has a plain-words introduction and a Credits section.

## [0.2.2] - 2026-11-16

### Changed

- **`oxiguitar.gpx` now declares interface 1.6 and answers with score
  version 2**, as the Guitar Pro 7–8 and 5 plugins do. It declared 1.0, so
  OxiGuitar read a Guitar Pro 6 file into score version 1 and dropped the
  palette's marks (the rest of Guitar Pro 8's notation, Multivoice and
  Design Mode's bars) that a .gpx file holds.

## [0.2.1] - 2026-11-09

### Fixed

- **Notes in flat keys no longer gain a spelling when saved as .gp.** A note
  with no chosen spelling was written with a fixed table of sharps, so in a
  flat key a B flat was saved as A sharp and came back as a chosen
  spelling. It is now written as OxiGuitar spells it in the bar's key (as
  Guitar Pro itself would). Found by OxiGuitar's edit-and-save test, which
  failed once 0.2.0 was published.
- **Guitar Pro 7 and 8 files no longer open with forced line breaks.**
  Guitar Pro writes a bars-per-line layout in every file whether or not
  anyone chose one (its default, 3 or 4 bars a line), and 0.2.0 read it as
  a Design Mode choice. A layout that is only Guitar Pro's default now
  opens with OxiGuitar's own layout; a chosen one is still kept, and the
  writer makes sure a chosen layout never looks like the default. Found by
  OxiGuitar's Guitar Pro 7.6 comparison test.

## [0.2.0] - 2026-11-08

### Added

- **The rest of Guitar Pro 8's palette** (OxiGuitar v0.17, plugin
  interface 1.6): the Guitar Pro 7-8 plugin reads and writes fermatas,
  simile (repeat) bars, slurs, hairpins, 8va and 15ma lines, barres, slashed
  beats, beaming breaks, the sustain pedal, fade out and volume swell, wide
  and slight vibrato, wah, arpeggios, rasgueado, golpe, dead slaps,
  ornaments, string numbers, chosen spellings and Design Mode's bars per
  system. The Guitar Pro 5 writer names what Guitar Pro 5 can't hold.
  Pick scrape and forced tuplet groups have no form in a .gp file and are
  named as losses. `oxiguitar.gp` and `oxiguitar.gp5` now declare interface
  1.6, so OxiGuitar passes them every mark (they declared 1.2, so OxiGuitar
  0.16's free time and the other 1.3 to 1.5 marks were dropped before
  writing).

## [0.1.0] - 2026-11-02

First release in a repository of its own (mateusrdp/oxiguitar#165). The
plugins are those of OxiGuitar 0.11.0, with their history; their ids, file
names and behaviour are unchanged.

### Added

- Guitar Pro 3, 4, 5, 6 (.gpx) and 7-8 (.gp) readers, and Guitar Pro 5 and
  7-8 writers, as the plugins `oxiguitar.gp3`, `oxiguitar.gp4`,
  `oxiguitar.gp5`, `oxiguitar.gpx` and `oxiguitar.gp` (plugin interface 1.0
  to 1.4).
- `build-oxp.sh`, which builds the five `.oxp` files for the system it runs
  on, and a contract test in each plugin that checks the built file through
  OxiGuitar's plugin host.
- `release.yml` and `publish.yml`: test builds on every pull request, and on a `v*` tag a draft release with a zip per system, `.deb` and `.rpm` packages, a per-user Windows installer, a macOS `.pkg` and `SHA256SUMS`, made by OxiGuitar's reusable workflows.
- `crates/oxiguitar-formats/tests/host.rs`, which reads every public fixture
  through the five files and compares with reading in the same process.

### Changed

- The plugin SDK and the score model are git dependencies on the `v0.11.0`
  tag of mateusrdp/oxiguitar instead of workspace members.
- `scripts/guitar-pro-check.sh` converts with `oxiguitar-cli` installed from
  that tag.

### Removed

- The `plugin_tab` fuzz target, which tests the core's display lists and
  stays in OxiGuitar.
