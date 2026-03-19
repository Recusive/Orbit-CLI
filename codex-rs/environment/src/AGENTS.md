# codex-rs/environment/src/

This file applies to `codex-rs/environment/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-environment` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-environment`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source code for the `codex-environment` crate.

### What this folder does

Contains the filesystem abstraction trait and its local implementation.

### Key files

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

### Imports from / exports to

**Imports:**
- `async_trait::async_trait`
- `codex_utils_absolute_path::AbsolutePathBuf`
- `tokio::fs`, `tokio::io`
- `std::path::{Path, PathBuf, Component}`

**Exports:**
- All public types are re-exported through `lib.rs`
