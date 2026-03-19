# codex-rs/exec-server/src/bin/

This file applies to `codex-rs/exec-server/src/bin/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-exec-server` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-exec-server`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Binary entry point for the `codex-exec-server` standalone server.

### What this folder does

Contains the main function for the `codex-exec-server` binary.

### Key files and their roles

- `codex-exec-server.rs` -- Binary entry point. Parses a `--listen URL` argument (default: `ws://127.0.0.1:0`) using clap, then calls `codex_exec_server::run_main_with_listen_url()` to start the WebSocket server.

### Imports from

- `codex_exec_server`: `run_main_with_listen_url`, `DEFAULT_LISTEN_URL`
- `clap`: CLI argument parsing
- `tokio`: async runtime (`#[tokio::main]`)
