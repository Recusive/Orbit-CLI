# codex-rs/windows-sandbox-rs/src/conpty/

This file applies to `codex-rs/windows-sandbox-rs/src/conpty/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-windows-sandbox` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-windows-sandbox`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

ConPTY (Console Pseudo Terminal) helpers for Windows sandbox.

### What this folder does

Encapsulates Windows ConPTY creation and process spawning with the `PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE` plumbing. Shared by both the legacy restricted-token path and the elevated runner path when `tty=true`.

### Key files

- `mod.rs` -- `ConptyInstance` struct (owns the PTY handle and backing pipes), `create_conpty()` function (creates a ConPTY with specified dimensions), and `spawn_conpty_process_as_user()` (the main entry point: creates a ConPTY, sets up proc-thread attributes, and calls `CreateProcessAsUserW` with the restricted token and environment block).
- `proc_thread_attr.rs` -- `ProcThreadAttributeList` wrapper for `PROC_THREAD_ATTRIBUTE_LIST` initialization and pseudoconsole attribute setting.

### What it plugs into

- Called by `run_windows_sandbox_capture()` in `lib.rs` and by the elevated command runner when PTY-based execution is needed.
- Uses `codex-utils-pty::RawConPty` for the low-level PTY handle creation.

### Imports from

- `windows-sys` for `CreateProcessAsUserW`, `ClosePseudoConsole`, and related Win32 APIs.
- `codex-utils-pty` for `RawConPty`.
