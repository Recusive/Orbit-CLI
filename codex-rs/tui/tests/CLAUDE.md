# codex-rs/tui/tests/

Integration tests for the `codex-tui` crate.

## What this folder does

Contains integration tests that verify TUI behavior at a higher level than unit tests. Tests are aggregated into a single binary via `all.rs` for faster compilation. The suite includes VT100 emulator-based rendering tests (gated behind the `vt100-tests` feature flag), model availability NUX tests, startup safety tests, and status indicator tests.

## What it plugs into

- **codex-tui**: Tests exercise the crate's public API (`custom_terminal`, `insert_history`, `ComposerInput`, etc.) and internal modules.
- **codex-cli**: Dev-dependency; some tests spawn the `codex` binary to test end-to-end behavior.
- **vt100**: Dev-dependency; provides a VT100 terminal emulator for rendering verification.

## Key files

| File | Role |
|------|------|
| `all.rs` | Single integration test entry point that aggregates all test modules from `suite/`. Also conditionally includes the `test_backend` module when the `vt100-tests` feature is enabled. |
| `test_backend.rs` | Re-exports `VT100Backend` from `../src/test_backend.rs` for use by integration tests. |
| `mod.rs` | Does not exist at this level -- `all.rs` serves as the entry point. |

## Sub-directories

| Directory | Purpose |
|-----------|---------|
| `suite/` | Individual test modules. |
| `fixtures/` | Test fixture data files. |

## Feature gates

- `vt100-tests`: Enables VT100 terminal emulator-based tests that verify actual rendered terminal output. These tests use `VT100Backend` to simulate a terminal and verify pixel-level rendering.
