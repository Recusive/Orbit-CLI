# codex-rs/exec-server/tests/

This file applies to `codex-rs/exec-server/tests/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-exec-server` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-exec-server`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Integration tests for the `codex-exec-server` crate.

### What this folder does

Contains integration tests that spawn the `codex-exec-server` binary, connect via WebSocket, and verify the JSON-RPC protocol behavior.

### Key files and their roles

- `initialize.rs` -- Tests the initialize/initialized handshake: spawns the server binary, sends an `initialize` request with `InitializeParams`, verifies the response matches `InitializeResponse {}`, then shuts down.
- `process.rs` -- Tests for WebSocket message processing.
- `websocket.rs` -- Tests for WebSocket transport behavior.
- `common/` -- Shared test infrastructure.

### Imports from

- `codex_exec_server`: `InitializeParams`, `InitializeResponse`
- `codex_app_server_protocol`: `JSONRPCMessage`, `JSONRPCResponse`
- `common::exec_server`: test harness

### Platform notes

- Tests are gated with `#![cfg(unix)]` -- they only run on Unix platforms.
