# 006_rejects_missing_context

Tests that an update patch fails when the context lines do not match any content in the target file, leaving the file unchanged.

## Files

- `patch.txt` -- Patch that tries to replace "missing" with "changed" in `modify.txt`, but "missing" does not exist in the file.
- `input/modify.txt` -- File with content that does not contain the expected old lines.
- `expected/modify.txt` -- Same as input; verifies the file was not modified.
