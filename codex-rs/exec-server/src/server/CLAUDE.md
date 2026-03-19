# codex-rs/exec-server/src/server/

Server-side implementation for the exec-server.

## What this folder does

Implements the WebSocket server that listens for JSON-RPC connections, processes the initialize/initialized handshake, and dispatches incoming requests to the handler.

## Key files and their roles

- `mod.rs` (server.rs in parent) -- Module entry point. Re-exports `ExecServerHandler`, `DEFAULT_LISTEN_URL`, `ExecServerListenUrlParseError`. Provides `run_main()` and `run_main_with_listen_url()`.
- `handler.rs` -- `ExecServerHandler`: manages connection state (initialize_requested, initialized) with atomic booleans. Enforces single-initialize and ordered handshake.
- `jsonrpc.rs` -- JSON-RPC helper utilities (e.g., `invalid_request()` error constructor).
- `processor.rs` -- `run_connection()`: reads incoming JSON-RPC messages from a `JsonRpcConnection`, routes `initialize` requests and `initialized` notifications to the handler, and sends responses/errors back.
- `transport.rs` -- WebSocket transport: `parse_listen_url()`, `run_transport()`, and `run_websocket_listener()` that binds a `TcpListener`, accepts connections, upgrades to WebSocket, and spawns per-connection tasks.
- `transport_tests.rs` -- Tests for URL parsing and WebSocket transport behavior.

## Imports from

- `crate::connection::JsonRpcConnection`
- `crate::protocol`: InitializeResponse, method constants
- `codex-app-server-protocol`: JSONRPCErrorError
- `tokio`, `tokio-tungstenite`: async networking

## Exports to

- `ExecServerHandler` used by `LocalBackend` (in-process client)
- `DEFAULT_LISTEN_URL` and `run_main*` used by the binary and library consumers
