# codex-rs/tui/src/chatwidget/

This file applies to `codex-rs/tui/src/chatwidget/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-tui` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.
- Any user-visible TUI change needs matching snapshot coverage. Mirror behavior in the sibling TUI implementation when the same feature exists there.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-tui`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo insta pending-snapshots -p codex-tui`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Sub-modules for the `ChatWidget` main chat surface.

### What this folder does

Contains modules that support the `ChatWidget` defined in `../chatwidget.rs`. These handle agent lifecycle management, session header display, interrupt handling, realtime audio, skills UI, and the extensive test suite.

### What it plugs into

- **../chatwidget.rs**: The parent `ChatWidget` struct uses these modules for agent spawning, header rendering, interrupt handling, realtime audio state, and skills display.
- **../app.rs**: `App` interacts with `ChatWidget` to drive the main chat experience.
- **codex-core**: Uses `CodexThread`, `ThreadManager`, `Config` for agent lifecycle.
- **codex-protocol**: Uses `Event`, `EventMsg`, `Op` for protocol message handling.

### Key files

| File | Role |
|------|------|
| `agent.rs` | `spawn_agent()` -- bootstraps the agent thread, sets up the event forwarding loop, and returns an `UnboundedSender<Op>` for the UI to submit operations. Handles thread creation, config, and client name registration. |
| `session_header.rs` | `SessionHeader` -- simple widget that displays the current model name at the top of the chat area. |
| `interrupts.rs` | Interrupt handling logic for Ctrl+C behavior during agent turns. |
| `realtime.rs` | Realtime audio session state management (voice input/output, pending steer compare). |
| `skills.rs` | Skills display and management in the chat context. |
| `tests.rs` | Comprehensive test suite for `ChatWidget` covering approval modals, patch flows, streaming output, history rendering, and more. |

### Sub-directories

| Directory | Purpose |
|-----------|---------|
| `snapshots/` | Insta test snapshots for chatwidget tests. |
