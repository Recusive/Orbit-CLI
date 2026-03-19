# codex-rs/utils/absolute-path/src/

Source directory for the `codex-utils-absolute-path` crate.

## Key files

- `lib.rs` -- single-file implementation containing:
  - `AbsolutePathBuf` struct with methods: `resolve_path_against_base`, `from_absolute_path`, `current_dir`, `join`, `parent`, conversion traits (`AsRef<Path>`, `From`, `TryFrom`)
  - `AbsolutePathBufGuard` -- thread-local RAII guard for setting the base path used during `Deserialize` of relative paths
  - `maybe_expand_home_directory` -- tilde expansion helper (non-Windows only)
  - Custom `Deserialize` impl that reads the thread-local base path to resolve relative paths
  - Unit tests for resolution, deserialization, and tilde expansion
