# 008_rejects_empty_update_hunk

Tests that an Update File hunk with no diff lines (no `@@` section or change lines) is rejected as a parse error.

## Files

- `patch.txt` -- Patch with `*** Update File: foo.txt` but no `@@` or change lines.
- `input/foo.txt` -- Pre-existing file.
- `expected/foo.txt` -- Same as input; verifies the file was not modified.
