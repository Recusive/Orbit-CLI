# codex-rs/execpolicy-legacy/tests/

Tests for the `codex-execpolicy-legacy` crate.

## What this folder does

Contains integration tests for the legacy exec policy engine, organized as a single binary (`all.rs`) that aggregates test modules from the `suite/` subdirectory.

## Key files and their roles

- `all.rs` -- Test binary entry point; imports `mod suite`.
- `suite/` -- Individual test modules.

## Imports from

- `codex_execpolicy_legacy`: Policy, ExecCall, MatchedExec, ValidExec, and related types
