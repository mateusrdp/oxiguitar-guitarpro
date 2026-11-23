# Architecture and strategy

The five plugins are thin. Each one is a small wrapper around a shared library that holds the file format detection, the readers and the writers. A plugin file carries only its own reader, which is why you can leave out the ones you don't need.

The strategy is to trust the file's contents over its name, and to check our output against other programs instead of our own tests alone. The repository keeps songs we wrote ourselves in every format, files written for opening in Guitar Pro itself, and checks that alphaTab and MuseScore read what we write the way we mean it. The readers and writers are also fuzzed, since the files come from anywhere.

## Current limitations

- **No writers for Guitar Pro 3, 4 and 6.** You can open those files, and save as `.gp5` or `.gp`.
- **No automations.** Volume, pan and instrument changes in the middle of a song have no place in OxiGuitar before v1.0, so a file's automations don't come across.
- **No audio tracks.** Guitar Pro 7 and 8 can carry a recorded audio track; OxiGuitar doesn't play or keep it yet (mateusrdp/oxiguitar#240).
- **Guitar Pro's own sounds.** Its RSE sound settings and sound banks belong to Guitar Pro. OxiGuitar plays each track with its own sound plugins instead, by the track's General MIDI instrument.
- **Layout.** Guitar Pro's page layout and style settings aren't copied: OxiGuitar engraves the score its own way.
- **Stem direction overrides** are dropped: stems follow the voice (a decision, 19 October 2026).

When a file loses something on the way in, the footer says so; when a save would lose something, OxiGuitar asks first, and can keep it in the song's `.oxg` file.

## Left out on purpose

- **Anything of Guitar Pro's.** No code, sounds, icons, fonts or sample songs are taken from Guitar Pro. The formats are read from what open projects (alphaTab, PyGuitarPro, TuxGuitar) have documented, and checked against files we wrote ourselves.
- **Writers for the old formats.** `.gp5` opens in every Guitar Pro from 5 on and in TuxGuitar, and `.gp` in Guitar Pro 7 and 8, so older writers would add work and help almost nobody.
- **Trusting the extension.** Files on the internet are often misnamed; the content decides.

## Where you can help

- **Check our files in Guitar Pro.** The repository's `fixtures/guitar-pro-check/` holds files we wrote; if you own Guitar Pro 7 or 8, open them and tell us what looks wrong. That is the single most useful check we can't do ourselves.
- **Files that don't open**, or open wrong. Open an issue with what you see. Attach the file only if you are allowed to share it.
- **Automations**, if they come back into scope after v1.0.

Every change needs its tests and a row on the [Features](features.md) page; the [Testing](testing.md) page says how to run them.

