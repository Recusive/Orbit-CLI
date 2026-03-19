# 015_failure_after_partial_success_leaves_changes

Tests that when a patch has multiple hunks and a later hunk fails, earlier successful changes are still persisted to the filesystem (no rollback).

## Files

- `patch.txt` -- Patch that first adds `created.txt` (succeeds), then tries to update `missing.txt` (fails because it does not exist).
- `expected/created.txt` -- The successfully created file remains on disk despite the subsequent failure.

This scenario has no `input/` directory since only an add operation succeeds.
