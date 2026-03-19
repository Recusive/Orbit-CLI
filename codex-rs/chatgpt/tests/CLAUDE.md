# codex-rs/chatgpt/tests/

Integration tests for the `codex-chatgpt` crate.

## What this folder does

Contains end-to-end tests for the apply command, verifying that diffs extracted from task responses are correctly applied to git repositories. Tests use JSON fixtures and temporary git repos.

## Where it plugs in

- `all.rs` is the single integration test binary entry point that aggregates all test modules from `suite/`
- Tests exercise `apply_diff_from_task` from the `apply_command` module against realistic task response payloads

## Key files

| File | Role |
|------|------|
| `all.rs` | Single integration test binary; includes `mod suite` |
| `task_turn_fixture.json` | JSON fixture with a task response containing a PR diff that creates `scripts/fibonacci.js` |
| `suite/` | Test module directory |
