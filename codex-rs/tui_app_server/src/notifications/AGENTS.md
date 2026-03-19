# codex-rs/tui_app_server/src/notifications/

This file applies to `codex-rs/tui_app_server/src/notifications/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

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

Desktop notification backends for the TUI.

### What this folder does

Provides a `DesktopNotificationBackend` abstraction that sends desktop notifications using terminal escape sequences. Supports two methods: OSC 9 (for terminals like WezTerm and Ghostty) and BEL (universal audible/visual bell). The backend auto-detects the best method based on the terminal environment, or can be explicitly configured.

### What it plugs into

- **../tui.rs**: `Tui` creates and owns the notification backend at startup based on user configuration.
- **../app.rs**: `App` triggers notifications when the agent completes a turn or requires attention.
- **codex_core::config::types::NotificationMethod**: Configuration enum (`Auto`, `Osc9`, `Bel`) that drives backend selection.

### Key files

| File | Role |
|------|------|
| `mod.rs` | `DesktopNotificationBackend` enum and factory; `detect_backend()` function; OSC 9 capability detection via terminal environment variables. |
| `osc9.rs` | `Osc9Backend` -- sends OSC 9 escape sequences for native toast notifications in supported terminals. |
| `bel.rs` | `BelBackend` -- sends the BEL character (`\x07`) for audible/visual bell notifications. |

### Imports from

- `codex_core::config::types::NotificationMethod` -- notification method preference from config.
- Standard library (`std::env`, `std::io`).

### Exports to

- **crate::tui**: `DesktopNotificationBackend`, `detect_backend()`.
