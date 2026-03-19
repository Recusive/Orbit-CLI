# 019_unicode_simple

Tests that patches containing Unicode characters (accented characters, emoji) are applied correctly.

## Files

- `patch.txt` -- Patch that updates `foo.txt`, replacing a line with accented characters and adding an emoji.
- `input/foo.txt` -- Original file with Unicode content including accented characters.
- `expected/foo.txt` -- Updated file with the modified Unicode line.
