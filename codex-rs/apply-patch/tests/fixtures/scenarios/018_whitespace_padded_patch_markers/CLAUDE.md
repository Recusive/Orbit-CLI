# 018_whitespace_padded_patch_markers

Tests that `*** Begin Patch` and `*** End Patch` markers with leading/trailing whitespace are tolerated.

## Files

- `patch.txt` -- Patch with a leading space on `*** Begin Patch` and a trailing space on `*** End Patch`.
- `input/file.txt` -- Original file with "one".
- `expected/file.txt` -- Updated file with "two".
