# 011_add_overwrites_existing_file

Tests that `*** Add File` overwrites a file that already exists at the target path rather than failing.

## Files

- `patch.txt` -- Patch that adds `duplicate.txt` with "new content".
- `input/duplicate.txt` -- Pre-existing file with "old content".
- `expected/duplicate.txt` -- File with "new content" (overwritten).
