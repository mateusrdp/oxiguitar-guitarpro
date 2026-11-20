# Requirements and assumptions

## Requirements

- OxiGuitar 0.17 or later for the Guitar Pro 5, 6 and 7-8 plugins (plugin interface 1.6). The Guitar Pro 3 and 4 plugins ask for less. See [the OxiGuitar manual](https://mateusrdp.github.io/oxiguitar/).
- Windows, macOS or Linux. Linux and Windows builds exist for x86_64 and ARM64.
- To hear a song you also need a plugin that plays scores and an audio output plugin. These plugins don't make sound.

## Assumptions

- The file really is Guitar Pro. Anything else is not opened.
- You are allowed to use the files you open. The plugins read what you give them and nothing else.
- Saving to .gp5 or .gp is enough for your purposes. There is no writer for Guitar Pro 3, 4 or 6.
