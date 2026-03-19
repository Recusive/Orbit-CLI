# codex-rs/tui_app_server/tests/fixtures/

This file applies to `codex-rs/tui_app_server/tests/fixtures/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Treat this directory as golden data or generated/static support material. Keep filenames and relative paths stable unless the owning test, renderer, or generator changes first.
- Prefer updating the producer or the corresponding test scenario before editing files here by hand.
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-tui-app-server` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.
- Any user-visible TUI change needs matching snapshot coverage. Mirror behavior in the sibling TUI implementation when the same feature exists there.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-tui-app-server`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo insta pending-snapshots -p codex-tui-app-server`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Test fixture data for integration tests.

### What this folder does

Contains static data files used by the integration test suite. These fixtures provide pre-recorded session data (JSONL rollout logs, etc.) that tests replay to verify rendering, history parsing, and session management behavior without requiring live agent connections.

### What it plugs into

- **../suite/**: Integration test modules load these fixtures to drive test scenarios.
- **../all.rs**: The test binary reads fixtures at runtime via file path references.

### Key files

| File | Role |
|------|------|
| `oss-story.jsonl` | A pre-recorded JSONL session rollout used to test history rendering and session replay. |
