# codex-rs/environment/src/

Source code for the `codex-environment` crate.

## What this folder does

Contains the filesystem abstraction trait and its local implementation.

## Key files

- `lib.rs` -- Module declaration and public re-exports:
  - Declares `pub mod fs`
  - Defines `Environment` struct with `get_filesystem()` method that returns a `LocalFileSystem`
  - Re-exports all public types from `fs.rs`

- `fs.rs` -- Core filesystem implementation:
  - **Constants**: `MAX_READ_FILE_BYTES` (512 MB)
  - **Option types**: `CreateDirectoryOptions`, `RemoveOptions`, `CopyOptions`
  - **Data types**: `FileMetadata`, `ReadDirectoryEntry`
  - **Trait**: `ExecutorFileSystem` -- async trait with methods: `read_file`, `write_file`, `create_directory`, `get_metadata`, `read_directory`, `remove`, `copy`
  - **Implementation**: `LocalFileSystem` -- implements `ExecutorFileSystem` using `tokio::fs` for async operations
  - **Helper functions**:
    - `copy_dir_recursive()` -- Recursive directory copy preserving symlinks
    - `destination_is_same_or_descendant_of_source()` -- Safety check for copy operations
    - `resolve_copy_destination_path()` -- Normalizes paths for destination resolution
    - `copy_symlink()` -- Platform-specific symlink copying (Unix/Windows)
    - `system_time_to_unix_ms()` -- Timestamp conversion

## Imports from / exports to

**Imports:**
- `async_trait::async_trait`
- `codex_utils_absolute_path::AbsolutePathBuf`
- `tokio::fs`, `tokio::io`
- `std::path::{Path, PathBuf, Component}`

**Exports:**
- All public types are re-exported through `lib.rs`
