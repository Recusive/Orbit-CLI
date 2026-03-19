# 007_rejects_missing_file_delete

Tests that deleting a non-existent file fails, leaving the filesystem unchanged.

## Files

- `patch.txt` -- Patch that tries to delete `missing.txt`, which does not exist.
- `input/foo.txt` -- Pre-existing unrelated file.
- `expected/foo.txt` -- Same as input; verifies no changes were made.
