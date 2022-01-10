# img-cmp
This will hopefully help with identifying and removing similar images in a large library.

## What does it do?
Usage:

```
img-cmp COMMAND ITEM1 [ITEM2 [...]]
```

## Commands

### rename
For consistent naming, rename file to sha-1 hash output:

```
a_very_cool_name.xyz => 12aa5b72b1022c5db40a4f4424e1bb18a488abfe.xyz
```

Just in case you accidentally rename something that should not be renamed, the terminal output will
contain the old and the new name.

BE CAREFUL WITH THIS COMMAND, IT WILL RENAME EVERY FILE YOU FEED IT.

For safer renaming give it individual files or a glob:

```
img-cmp rename *.png
```

### run
Check for similar images according to their image hash, and save a cache file to shell's CWD.

The program will always look for a cache file in your shell's CWD

The cache file is named "cache.json" (/I may move this to XDG\_CACHE\_HOME/)

### cache: you may want to
If you deleted files from your library run this command, otherwise the program will compare
non-existent files and maybe give false positives.
