# 009_requires_existing_file_for_update

Tests that updating a non-existent file fails with an I/O error, leaving the filesystem unchanged.

## Files

- `patch.txt` -- Patch that tries to update `missing.txt`, which does not exist.
- `input/foo.txt` -- Pre-existing unrelated file.
- `expected/foo.txt` -- Same as input; verifies no changes were made.
