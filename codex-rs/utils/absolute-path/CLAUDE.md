# codex-rs/utils/absolute-path/

Crate `codex-utils-absolute-path` -- a newtype wrapper guaranteeing paths are absolute and normalized.

## What this folder does

Provides `AbsolutePathBuf`, a path type that is always absolute and normalized (though not necessarily canonicalized or existing on disk). Supports tilde (`~`) expansion on non-Windows platforms, relative path resolution against a base, and serde deserialization with a thread-local base path guard.

## Key types and functions

- `AbsolutePathBuf` -- the core newtype wrapping `PathBuf`; implements `Serialize`, `Deserialize`, `JsonSchema`, and `TS`
- `AbsolutePathBufGuard` -- RAII guard that sets a thread-local base path for deserializing relative paths
- `resolve_path_against_base()` -- resolve a possibly-relative path against an explicit base
- `from_absolute_path()` -- construct from an already-absolute path
- `current_dir()` -- construct from the current working directory

## Imports from

- `dirs` -- home directory lookup for tilde expansion
- `path-absolutize` -- path normalization
- `schemars`, `serde`, `ts-rs` -- schema generation and serialization

## Exports to

Used extensively throughout the workspace wherever absolute paths are required in configuration, sandbox policies, and protocol types. Key consumers include `codex-protocol`, `codex-config`, `codex-core`, and `codex-utils-sandbox-summary`.

## Key files

- `Cargo.toml` -- crate metadata and dependencies
- `src/lib.rs` -- all implementation: `AbsolutePathBuf`, `AbsolutePathBufGuard`, trait impls, tests
