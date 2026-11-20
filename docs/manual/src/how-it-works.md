# Architecture and strategy

The five plugins are thin. Each one is a small wrapper around a shared library that holds the file format detection, the readers and the writers. A plugin file carries only its own reader, which is why you can leave out the ones you don't need.

The strategy is to trust the file's contents over its name, and to check our output against other programs instead of our own tests alone. The repository keeps songs we wrote ourselves in every format, files written for opening in Guitar Pro itself, and checks that alphaTab and MuseScore read what we write the way we mean it. The readers and writers are also fuzzed, since the files come from anywhere.
