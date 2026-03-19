# codex-rs/environment/

This file applies to `codex-rs/environment/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-environment` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-environment`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Crate: `codex-environment` -- Filesystem abstraction layer for the Codex executor.

### What this crate does

Provides a trait-based filesystem abstraction (`ExecutorFileSystem`) that the Codex core executor uses for all file operations. The default implementation (`LocalFileSystem`) delegates to Tokio's async filesystem APIs. This abstraction enables testing with mock filesystems and enforces consistent behavior (e.g., file size limits, recursive copy safety checks).

### Main types

- `Environment` -- Simple struct that provides access to the filesystem implementation via `get_filesystem()`
- `ExecutorFileSystem` trait -- Async trait defining all file operations:
  - `read_file`, `write_file`, `create_directory`, `get_metadata`, `read_directory`, `remove`, `copy`
- `LocalFileSystem` -- Default implementation using `tokio::fs` and `std::fs`
- `FileMetadata` -- File metadata (is_directory, is_file, timestamps)
- `ReadDirectoryEntry` -- Directory listing entry
- `CreateDirectoryOptions`, `RemoveOptions`, `CopyOptions` -- Operation option structs
- `FileSystemResult<T>` -- Type alias for `io::Result<T>`

### Key behaviors

- **Read limit**: Files larger than 512 MB are rejected
- **Recursive copy safety**: Prevents copying a directory into itself or a descendant
- **Symlink handling**: Copies symlinks as symlinks on Unix/Windows
- **Cross-platform**: Handles Unix symlinks, Windows symlink directories, and non-UTF-8 paths

### What it plugs into

- Used by `codex-core` as the filesystem backend for tool execution (file read/write/copy/remove operations)
- The `Environment` struct is passed through the core execution pipeline

### Imports from / exports to

**Dependencies:**
- `async-trait` -- For async trait definitions
- `codex-utils-absolute-path` -- `AbsolutePathBuf` for type-safe absolute paths
- `tokio` -- Async filesystem operations

**Exports:**
- `Environment`, `ExecutorFileSystem`, `FileMetadata`, `ReadDirectoryEntry`, `CopyOptions`, `CreateDirectoryOptions`, `RemoveOptions`, `FileSystemResult`

### Key files

- `Cargo.toml` -- Crate manifest
- `src/lib.rs` -- `Environment` struct and module re-exports
- `src/fs.rs` -- `ExecutorFileSystem` trait definition and `LocalFileSystem` implementation
