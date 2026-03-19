# codex-rs/chatgpt/tests/suite/

Test modules for the chatgpt integration test binary.

## What this folder does

Contains individual test modules aggregated by `mod.rs` into the single `all.rs` test binary. Currently has one module for apply command end-to-end testing.

## Where it plugs in

- Included via `mod suite` in `tests/all.rs`
- Tests import from `codex_chatgpt::apply_command` and `codex_chatgpt::get_task`

## Key files

| File | Role |
|------|------|
| `mod.rs` | Aggregates test modules; currently includes `apply_command_e2e` |
| `apply_command_e2e.rs` | Two tests: (1) `test_apply_command_creates_fibonacci_file` -- applies a fixture diff to a temp git repo and verifies the created file contents; (2) `test_apply_command_with_merge_conflicts` -- verifies that applying a diff to a repo with conflicting content produces merge conflict markers |
