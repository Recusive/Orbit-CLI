# codex-rs/exec-server/src/client/

This file applies to `codex-rs/exec-server/src/client/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-exec-server` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-exec-server`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Client-side local (in-process) backend for the exec-server.

### What this folder does

Implements the `LocalBackend` struct that allows `ExecServerClient` to communicate with an `ExecServerHandler` directly in-process, without any network transport. This is used when the client and server run in the same process.

### Key files and their roles

- `local_backend.rs` -- `LocalBackend`: wraps an `Arc<ExecServerHandler>` and provides async `initialize()`, `initialized()`, and `shutdown()` methods that delegate directly to the handler.

### Imports from

- `crate::protocol::InitializeResponse`
- `crate::server::ExecServerHandler`
- `crate::client::ExecServerError`

### Exports to

- Used internally by `ExecServerClient` (in `client.rs`) as the `ClientBackend::InProcess` variant.
