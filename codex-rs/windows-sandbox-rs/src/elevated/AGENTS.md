# codex-rs/windows-sandbox-rs/src/elevated/

This file applies to `codex-rs/windows-sandbox-rs/src/elevated/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-windows-sandbox` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-windows-sandbox`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Elevated sandbox runner IPC protocol and implementation.

### What this folder does

Implements the framed IPC protocol used between the parent CLI process and the elevated command runner (`codex-command-runner`). The parent sends spawn requests over named pipes; the runner creates restricted tokens, spawns processes, and streams output back.

### Key files

- `ipc_framed.rs` -- framed IPC protocol definition. Message types: `SpawnRequest` (command, env, cwd, policy, timeouts), `SpawnReady` (acknowledgment with PID), `Output` (stdout/stderr chunks), `Stdin` (input forwarding), `Exit` (exit code), `Error` (error description), `Terminate` (kill signal). Includes length-prefixed frame encoding/decoding, base64 helpers for binary data, and read/write frame functions.
- `command_runner_win.rs` -- Windows command runner implementation. Connects to IPC pipes, reads the `SpawnRequest`, derives a restricted token based on the sandbox policy, spawns the child via ConPTY or pipes, streams output frames back to the parent, handles stdin/terminate frames, and emits a final exit frame.
- `runner_pipe.rs` -- named pipe helpers for the runner IPC channel.
- `cwd_junction.rs` -- NTFS junction point creation for CWD isolation.

### What it plugs into

- The parent CLI process (`elevated_impl.rs`) sends `SpawnRequest` frames and reads `Output`/`Exit` frames.
- The `codex-command-runner` binary implements the runner side of this protocol.
- Used only in the elevated sandbox path, not the legacy restricted-token path.

### Imports from

- `serde`, `serde_json` for message serialization.
- `base64` for binary data encoding in output frames.
- Parent crate's ACL, token, process, ConPTY, and policy modules.
