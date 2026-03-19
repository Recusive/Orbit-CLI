# 012_delete_directory_fails

Tests that `*** Delete File` fails when the target is a directory rather than a regular file.

## Files

- `patch.txt` -- Patch that tries to delete `dir` (a directory, not a file).
- `input/dir/foo.txt` -- File inside the directory.
- `expected/dir/foo.txt` -- Same as input; verifies the directory and its contents are unchanged.
