# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project
follows [Semantic Versioning](https://semver.org/). A release's number is
the plugins' own; the plugin interface they use is in each plugin's
`manifest.json` and in the release notes.

## [Unreleased]

## [1.0.0] - Unreleased

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
