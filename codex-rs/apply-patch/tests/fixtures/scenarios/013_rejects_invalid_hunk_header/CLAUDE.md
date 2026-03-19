# 013_rejects_invalid_hunk_header

Tests that an invalid hunk header (e.g. `*** Frobnicate File:`) is rejected as a parse error.

## Files

- `patch.txt` -- Patch with `*** Frobnicate File: foo`, an unrecognized operation.
- `input/foo.txt` -- Pre-existing file.
- `expected/foo.txt` -- Same as input; verifies no changes were made.
