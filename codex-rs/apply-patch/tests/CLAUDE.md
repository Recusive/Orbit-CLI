# codex-rs/apply-patch/tests/

Integration tests for the `codex-apply-patch` crate. Tests are compiled as a single binary via `all.rs`.

## What this folder does

Contains end-to-end tests that exercise the `apply_patch` binary as a subprocess (CLI tests) and scenario-based filesystem tests that verify patch application against expected output directories.

## What it plugs into

- Invoked by `cargo test` / `cargo nextest` from the workspace root.
- Uses `codex-utils-cargo-bin` to locate the compiled `apply_patch` binary.
- Uses `assert_cmd` for CLI assertion helpers and `tempfile` for isolated test directories.

## Key files

| File | Role |
|------|------|
| `all.rs` | Test binary entry point; declares `mod suite` to aggregate all test modules. |
| `suite/` | Test module directory containing CLI and scenario test implementations. |
| `fixtures/` | Test fixture data (scenario directories with input, expected output, and patch files). |
