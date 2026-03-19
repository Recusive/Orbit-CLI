# codex-rs/utils/cargo-bin/

This file applies to `codex-rs/utils/cargo-bin/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-utils-cargo-bin` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-utils-cargo-bin`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Crate `codex-utils-cargo-bin` -- locate test binaries and resources across Cargo and Bazel.

### What this folder does

Provides helpers to find compiled binary targets and test resources at runtime, transparently supporting both `cargo test` (where `CARGO_BIN_EXE_*` env vars are absolute paths) and `bazel test` (where they are rlocationpaths resolved via runfiles).

### Key types and functions

- `cargo_bin(name)` -- returns the absolute path to a binary built for the current test run
- `find_resource!` -- macro that resolves a test resource path using either Cargo's `CARGO_MANIFEST_DIR` or Bazel's runfiles
- `repo_root()` -- locate the repository root directory
- `runfiles_available()` -- check if running under Bazel
- `CargoBinError` -- error type for binary resolution failures

### Imports from

- `assert_cmd` -- fallback binary discovery via Cargo
- `runfiles` -- Bazel runfiles resolution
- `thiserror` -- error derivation

### Exports to

Used exclusively in test code across the workspace for locating test binaries and fixture files.

### Key files

- `Cargo.toml` -- crate metadata and dependencies
- `src/lib.rs` -- `cargo_bin`, `find_resource!` macro, `repo_root`, runfile helpers
