# 016_pure_addition_update_chunk

Tests that an Update File chunk with only `+` lines (no `-` or context lines) appends content to the file.

## Files

- `patch.txt` -- Patch that updates `input.txt` with a pure-addition chunk adding two new lines.
- `input/input.txt` -- Original file.
- `expected/input.txt` -- File with the two new lines appended.
