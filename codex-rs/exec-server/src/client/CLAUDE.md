# codex-rs/exec-server/src/client/

Client-side local (in-process) backend for the exec-server.

## What this folder does

Implements the `LocalBackend` struct that allows `ExecServerClient` to communicate with an `ExecServerHandler` directly in-process, without any network transport. This is used when the client and server run in the same process.

## Key files and their roles

- `local_backend.rs` -- `LocalBackend`: wraps an `Arc<ExecServerHandler>` and provides async `initialize()`, `initialized()`, and `shutdown()` methods that delegate directly to the handler.

## Imports from

- `crate::protocol::InitializeResponse`
- `crate::server::ExecServerHandler`
- `crate::client::ExecServerError`

## Exports to

- Used internally by `ExecServerClient` (in `client.rs`) as the `ClientBackend::InProcess` variant.
