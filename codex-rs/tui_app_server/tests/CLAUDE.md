# codex-rs/tui_app_server/tests/

Integration tests for the `codex-tui-app-server` crate.

## What this folder does

Contains integration tests that exercise the TUI at a higher level than unit tests. The tests include VT100 emulator-based rendering tests (verifying actual terminal output), startup regression tests, and model availability NUX tests. Tests are aggregated into a single binary via `all.rs` for faster compilation.

## What it plugs into

- **../src/**: Tests import from `codex_tui_app_server` (the library) and exercise its public and crate-visible APIs.
- **codex-cli** (dev-dependency): Some tests spawn the actual `codex` binary to verify end-to-end behavior.
- **codex-core** / **codex-utils-pty**: Used for test configuration and pseudo-terminal simulation.
- **vt100** (dev-dependency): Terminal emulator for verifying rendered output matches expectations.
- **insta** (dev-dependency): Snapshot testing for rendered output comparisons.

## Key files

| File | Role |
|------|------|
| `all.rs` | Single integration test binary that aggregates all test modules from `suite/`. Conditionally includes `test_backend` for VT100 tests. |
| `test_backend.rs` | VT100-based test backend (feature-gated behind `vt100-tests`) for rendering verification. |
| `suite/` | Subdirectory containing individual test modules. |
| `fixtures/` | Test fixture data files (e.g., JSONL session logs). |

## Features

- `vt100-tests`: Feature flag that enables the VT100 emulator test backend and related integration tests.
