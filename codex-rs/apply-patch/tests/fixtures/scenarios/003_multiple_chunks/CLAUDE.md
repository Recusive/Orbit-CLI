# 003_multiple_chunks

Tests that an Update File hunk with multiple `@@` chunks can modify different parts of the same file in a single patch.

## Files

- `patch.txt` -- Patch that updates `multi.txt` with two chunks: replaces "line2" with "changed2" and "line4" with "changed4".
- `input/multi.txt` -- Original file with four lines.
- `expected/multi.txt` -- File after both replacements are applied.
