# codex-rs/tui_app_server/tests/fixtures/

Test fixture data for integration tests.

## What this folder does

Contains static data files used by the integration test suite. These fixtures provide pre-recorded session data (JSONL rollout logs, etc.) that tests replay to verify rendering, history parsing, and session management behavior without requiring live agent connections.

## What it plugs into

- **../suite/**: Integration test modules load these fixtures to drive test scenarios.
- **../all.rs**: The test binary reads fixtures at runtime via file path references.

## Key files

| File | Role |
|------|------|
| `oss-story.jsonl` | A pre-recorded JSONL session rollout used to test history rendering and session replay. |
