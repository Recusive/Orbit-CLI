# codex-rs/chatgpt/tests/

This file applies to `codex-rs/chatgpt/tests/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-chatgpt` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-chatgpt`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Integration tests for the `codex-chatgpt` crate.

### What this folder does

Contains end-to-end tests for the apply command, verifying that diffs extracted from task responses are correctly applied to git repositories. Tests use JSON fixtures and temporary git repos.

### Where it plugs in

- `all.rs` is the single integration test binary entry point that aggregates all test modules from `suite/`
- Tests exercise `apply_diff_from_task` from the `apply_command` module against realistic task response payloads

### Key files

| File | Role |
|------|------|
| `all.rs` | Single integration test binary; includes `mod suite` |
| `task_turn_fixture.json` | JSON fixture with a task response containing a PR diff that creates `scripts/fibonacci.js` |
| `suite/` | Test module directory |
