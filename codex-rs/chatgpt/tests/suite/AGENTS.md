# codex-rs/chatgpt/tests/suite/

This file applies to `codex-rs/chatgpt/tests/suite/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-chatgpt` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-chatgpt`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Test modules for the chatgpt integration test binary.

### What this folder does

Contains individual test modules aggregated by `mod.rs` into the single `all.rs` test binary. Currently has one module for apply command end-to-end testing.

### Where it plugs in

- Included via `mod suite` in `tests/all.rs`
- Tests import from `codex_chatgpt::apply_command` and `codex_chatgpt::get_task`

### Key files

| File | Role |
|------|------|
| `mod.rs` | Aggregates test modules; currently includes `apply_command_e2e` |
| `apply_command_e2e.rs` | Two tests: (1) `test_apply_command_creates_fibonacci_file` -- applies a fixture diff to a temp git repo and verifies the created file contents; (2) `test_apply_command_with_merge_conflicts` -- verifies that applying a diff to a repo with conflicting content produces merge conflict markers |
