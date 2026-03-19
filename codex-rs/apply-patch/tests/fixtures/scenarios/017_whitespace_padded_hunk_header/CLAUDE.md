# 017_whitespace_padded_hunk_header

Tests that a hunk header (`*** Update File:`) with leading whitespace is tolerated and parsed correctly.

## Files

- `patch.txt` -- Patch with `  *** Update File: foo.txt` (leading spaces on the hunk header).
- `input/foo.txt` -- Original file with "old".
- `expected/foo.txt` -- Updated file with "new".
