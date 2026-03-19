# 005_rejects_empty_patch

Tests that an empty patch (Begin Patch / End Patch with no hunks) is rejected and leaves the filesystem unchanged.

## Files

- `patch.txt` -- Empty patch with no file operations.
- `input/foo.txt` -- Pre-existing file that should remain unchanged.
- `expected/foo.txt` -- Same as input; verifies no changes were made.
