# codex-rs/execpolicy-legacy/tests/

This file applies to `codex-rs/execpolicy-legacy/tests/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-execpolicy-legacy` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-execpolicy-legacy`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Tests for the `codex-execpolicy-legacy` crate.

### What this folder does

Contains integration tests for the legacy exec policy engine, organized as a single binary (`all.rs`) that aggregates test modules from the `suite/` subdirectory.

### Key files and their roles

- `all.rs` -- Test binary entry point; imports `mod suite`.
- `suite/` -- Individual test modules.

### Imports from

- `codex_execpolicy_legacy`: Policy, ExecCall, MatchedExec, ValidExec, and related types
