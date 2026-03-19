# codex-rs/exec-server/src/bin/

Binary entry point for the `codex-exec-server` standalone server.

## What this folder does

Contains the main function for the `codex-exec-server` binary.

## Key files and their roles

- `codex-exec-server.rs` -- Binary entry point. Parses a `--listen URL` argument (default: `ws://127.0.0.1:0`) using clap, then calls `codex_exec_server::run_main_with_listen_url()` to start the WebSocket server.

## Imports from

- `codex_exec_server`: `run_main_with_listen_url`, `DEFAULT_LISTEN_URL`
- `clap`: CLI argument parsing
- `tokio`: async runtime (`#[tokio::main]`)
