# 014_update_file_appends_trailing_newline

Tests that after an update, the output file always ends with a trailing newline, even if the original file did not.

## Files

- `patch.txt` -- Patch that replaces "no newline at end" with "first line" and "second line" in `no_newline.txt`.
- `input/no_newline.txt` -- File without a trailing newline.
- `expected/no_newline.txt` -- Updated file with trailing newline appended.
