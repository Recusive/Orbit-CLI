# codex-rs/backend-client/tests/

This file applies to `codex-rs/backend-client/tests/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-backend-client` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-backend-client`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Test data for the `codex-backend-client` crate.

### What this folder does

Contains JSON fixture files used by the unit tests in `src/types.rs` to verify deserialization and the `CodeTaskDetailsResponseExt` trait methods (unified diff extraction, assistant text messages, user prompt extraction, error message parsing).

### Where it plugs in

- Fixtures are loaded via `include_str!` in the `#[cfg(test)]` module of `src/types.rs`
- Tests verify the hand-rolled response models against realistic backend JSON payloads

### Key files

| File | Role |
|------|------|
| `fixtures/` | Directory containing JSON test fixtures |
| `task_details_with_diff.json` | Symlink/copy in older layout (may duplicate `fixtures/` content) |
| `task_details_with_error.json` | Symlink/copy in older layout (may duplicate `fixtures/` content) |
