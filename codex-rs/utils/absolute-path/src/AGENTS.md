# codex-rs/utils/absolute-path/src/

This file applies to `codex-rs/utils/absolute-path/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-utils-absolute-path` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-utils-absolute-path`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source directory for the `codex-utils-absolute-path` crate.

### Key files

- `lib.rs` -- single-file implementation containing:
  - `AbsolutePathBuf` struct with methods: `resolve_path_against_base`, `from_absolute_path`, `current_dir`, `join`, `parent`, conversion traits (`AsRef<Path>`, `From`, `TryFrom`)
  - `AbsolutePathBufGuard` -- thread-local RAII guard for setting the base path used during `Deserialize` of relative paths
  - `maybe_expand_home_directory` -- tilde expansion helper (non-Windows only)
  - Custom `Deserialize` impl that reads the thread-local base path to resolve relative paths
  - Unit tests for resolution, deserialization, and tilde expansion
