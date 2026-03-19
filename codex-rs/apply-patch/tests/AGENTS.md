# codex-rs/apply-patch/tests/

This file applies to `codex-rs/apply-patch/tests/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-apply-patch` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-apply-patch`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Integration tests for the `codex-apply-patch` crate. Tests are compiled as a single binary via `all.rs`.

### What this folder does

Contains end-to-end tests that exercise the `apply_patch` binary as a subprocess (CLI tests) and scenario-based filesystem tests that verify patch application against expected output directories.

### What it plugs into

- Invoked by `cargo test` / `cargo nextest` from the workspace root.
- Uses `codex-utils-cargo-bin` to locate the compiled `apply_patch` binary.
- Uses `assert_cmd` for CLI assertion helpers and `tempfile` for isolated test directories.

### Key files

| File | Role |
|------|------|
| `all.rs` | Test binary entry point; declares `mod suite` to aggregate all test modules. |
| `suite/` | Test module directory containing CLI and scenario test implementations. |
| `fixtures/` | Test fixture data (scenario directories with input, expected output, and patch files). |
