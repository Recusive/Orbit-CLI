# codex-rs/windows-sandbox-rs/src/conpty/

ConPTY (Console Pseudo Terminal) helpers for Windows sandbox.

## What this folder does

Encapsulates Windows ConPTY creation and process spawning with the `PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE` plumbing. Shared by both the legacy restricted-token path and the elevated runner path when `tty=true`.

## Key files

- `mod.rs` -- `ConptyInstance` struct (owns the PTY handle and backing pipes), `create_conpty()` function (creates a ConPTY with specified dimensions), and `spawn_conpty_process_as_user()` (the main entry point: creates a ConPTY, sets up proc-thread attributes, and calls `CreateProcessAsUserW` with the restricted token and environment block).
- `proc_thread_attr.rs` -- `ProcThreadAttributeList` wrapper for `PROC_THREAD_ATTRIBUTE_LIST` initialization and pseudoconsole attribute setting.

## What it plugs into

- Called by `run_windows_sandbox_capture()` in `lib.rs` and by the elevated command runner when PTY-based execution is needed.
- Uses `codex-utils-pty::RawConPty` for the low-level PTY handle creation.

## Imports from

- `windows-sys` for `CreateProcessAsUserW`, `ClosePseudoConsole`, and related Win32 APIs.
- `codex-utils-pty` for `RawConPty`.
