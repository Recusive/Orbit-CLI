# codex-rs/tui/tests/suite/

Individual integration test modules for the TUI.

## What this folder does

Contains the test modules aggregated by `../all.rs` into a single integration test binary. Each module tests a specific aspect of TUI behavior.

## What it plugs into

- **../all.rs**: Declares all modules here as `mod suite;` which then includes each test file.
- **codex-tui**: Tests exercise the crate's public and internal APIs.
- **../test_backend.rs**: VT100-based tests use the `VT100Backend` for terminal emulation.

## Key files

| File | Role |
|------|------|
| `mod.rs` | Module root; declares all test sub-modules (`model_availability_nux`, `no_panic_on_startup`, `status_indicator`, `vt100_history`, `vt100_live_commit`). |
| `no_panic_on_startup.rs` | Smoke test ensuring the TUI binary starts without panicking. Spawns the actual `codex-tui` binary and verifies it exits cleanly. |
| `vt100_history.rs` | VT100 emulator-based tests for history line insertion and scrolling. Verifies that `insert_history_lines()` correctly renders content in both inline and alternate-screen viewports. Gated behind `vt100-tests` feature. |
| `vt100_live_commit.rs` | VT100 emulator-based tests for live streaming commit animation rendering. Verifies the streaming pipeline's visual output. Gated behind `vt100-tests` feature. |
| `status_indicator.rs` | Tests for the status indicator widget rendering. |
| `model_availability_nux.rs` | Tests for the model availability NUX (new user experience) flow. |
