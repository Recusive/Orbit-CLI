# codex-rs/tui_app_server/tests/

This file applies to `codex-rs/tui_app_server/tests/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-tui-app-server` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.
- Any user-visible TUI change needs matching snapshot coverage. Mirror behavior in the sibling TUI implementation when the same feature exists there.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-tui-app-server`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo insta pending-snapshots -p codex-tui-app-server`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Integration tests for the `codex-tui-app-server` crate.

### What this folder does

Contains integration tests that exercise the TUI at a higher level than unit tests. The tests include VT100 emulator-based rendering tests (verifying actual terminal output), startup regression tests, and model availability NUX tests. Tests are aggregated into a single binary via `all.rs` for faster compilation.

### What it plugs into

- **../src/**: Tests import from `codex_tui_app_server` (the library) and exercise its public and crate-visible APIs.
- **codex-cli** (dev-dependency): Some tests spawn the actual `codex` binary to verify end-to-end behavior.
- **codex-core** / **codex-utils-pty**: Used for test configuration and pseudo-terminal simulation.
- **vt100** (dev-dependency): Terminal emulator for verifying rendered output matches expectations.
- **insta** (dev-dependency): Snapshot testing for rendered output comparisons.

### Key files

| File | Role |
|------|------|
| `all.rs` | Single integration test binary that aggregates all test modules from `suite/`. Conditionally includes `test_backend` for VT100 tests. |
| `test_backend.rs` | VT100-based test backend (feature-gated behind `vt100-tests`) for rendering verification. |
| `suite/` | Subdirectory containing individual test modules. |
| `fixtures/` | Test fixture data files (e.g., JSONL session logs). |

### Features

- `vt100-tests`: Feature flag that enables the VT100 emulator test backend and related integration tests.
