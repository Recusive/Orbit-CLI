# codex-rs/utils/sleep-inhibitor/

This file applies to `codex-rs/utils/sleep-inhibitor/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-utils-sleep-inhibitor` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-utils-sleep-inhibitor`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Crate `codex-utils-sleep-inhibitor` -- prevent system idle sleep during active turns.

### What this folder does

Cross-platform helper that prevents the machine from going to sleep while Codex is actively processing a turn. Uses native platform APIs on macOS (IOKit power assertions), Linux (systemd-inhibit/gnome-session-inhibit), and Windows (PowerCreateRequest). Falls back to a no-op on unsupported platforms.

### Key types and functions

- `SleepInhibitor` -- public struct wrapping platform-specific implementation; tracks enabled state and turn-running state
- `set_turn_running(bool)` -- toggle sleep prevention on/off based on whether a turn is active
- `is_turn_running()` -- query current state

### Platform backends

- **macOS**: `IOPMAssertionCreateWithName` with `PreventUserIdleSystemSleep` assertion type
- **Linux**: spawns `systemd-inhibit` or `gnome-session-inhibit` subprocess
- **Windows**: `PowerCreateRequest` + `PowerSetRequest` with `PowerRequestSystemRequired`
- **Other**: no-op dummy implementation

### Imports from

- `tracing` -- warning logs for assertion failures
- `core-foundation` (macOS) -- CFString interop
- `libc` (Linux) -- process management
- `windows-sys` (Windows) -- power management APIs

### Exports to

Used by `codex-core` to keep the system awake during active agent turns.

### Key files

- `Cargo.toml` -- crate metadata with platform-specific dependencies
- `src/lib.rs` -- public `SleepInhibitor` wrapper and tests
- `src/macos.rs` -- macOS IOKit power assertion implementation
- `src/linux_inhibitor.rs` -- Linux systemd/gnome session inhibitor
- `src/windows_inhibitor.rs` -- Windows power request implementation
- `src/dummy.rs` -- no-op fallback for unsupported platforms
- `src/iokit_bindings.rs` -- generated IOKit FFI bindings (included by macos.rs)
