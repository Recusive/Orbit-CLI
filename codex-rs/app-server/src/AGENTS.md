# codex-rs/app-server/src/

This file applies to `codex-rs/app-server/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-app-server` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-app-server`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

### Purpose

Contains all source code for the `codex-app-server` crate. This directory houses the server's entry points, transport layer, request processing pipeline, and domain-specific API handlers.

### Module Structure

The crate is organized into a flat module layout declared in `lib.rs`:

#### Entry Points

- **`main.rs`** -- Binary entry point. Parses `--listen` CLI arg (stdio or ws://), resolves managed config path, and calls `run_main_with_transport`.
- **`lib.rs`** -- Library root. Boots config, tracing, telemetry, and runs the two-task event loop (processor + outbound router). Exports `run_main`, `run_main_with_transport`, `AppServerTransport`.

#### Transport Layer

- **`transport.rs`** -- Defines `AppServerTransport` (Stdio / WebSocket), `TransportEvent`, connection state types. Implements stdio line-delimited JSON and axum-based WebSocket acceptor with health/readyz endpoints.
- **`in_process.rs`** -- In-memory channel-based transport for embedding the server without process boundaries. Provides `InProcessClientHandle`, `InProcessStartArgs`, `start()`.
- **`outgoing_message.rs`** -- `OutgoingMessage` enum (Response, Error, Request, Notification, AppServerNotification), `OutgoingMessageSender` for routing, `ConnectionId`, `ConnectionRequestId`.

#### Request Processing

- **`message_processor.rs`** -- Top-level `MessageProcessor`. Handles initialize handshake, routes requests to config/FS/external-agent-config APIs or delegates to `CodexMessageProcessor`.
- **`codex_message_processor.rs`** -- Core domain logic handler. Processes thread/start, turn/start, turn/interrupt, model/list, plugin operations, auth, fuzzy file search, MCP, review, analytics, and more.

#### Domain Handlers

- **`config_api.rs`** -- `ConfigApi` for config/read, config/valueWrite, config/batchWrite, config/requirementsRead.
- **`fs_api.rs`** -- `FsApi` for filesystem CRUD (read, write, mkdir, metadata, readdir, remove, copy).
- **`external_agent_config_api.rs`** -- `ExternalAgentConfigApi` for detecting and importing external agent configurations.
- **`command_exec.rs`** -- `CommandExecManager` for PTY-backed command execution with resize/write/terminate support.
- **`fuzzy_file_search.rs`** -- Fuzzy file search session management.
- **`dynamic_tools.rs`** -- Dynamic tool call execution.
- **`models.rs`** -- Model listing helpers and supported model enumeration.

#### Supporting Modules

- **`thread_state.rs`** -- Per-thread runtime state tracking.
- **`thread_status.rs`** -- `ThreadWatchManager` for thread status change notifications.
- **`filters.rs`** -- Notification filtering and routing logic.
- **`error_code.rs`** -- JSON-RPC error code constants (INVALID_PARAMS, INPUT_TOO_LARGE, OVERLOADED, etc.).
- **`bespoke_event_handling.rs`** -- Custom event transformation logic for specific notification types.
- **`server_request_error.rs`** -- Server request error mapping.
- **`app_server_tracing.rs`** -- Tracing span construction for request instrumentation.

### Imports From

- `codex-app-server-protocol` -- All JSON-RPC and typed protocol types.
- `codex-core` -- Agent runtime, config, auth, thread management.
- `codex-protocol` -- Shared lower-level types.
- Various utility crates: `codex-feedback`, `codex-state`, `codex-file-search`, `codex-login`, `codex-chatgpt`, etc.

### Exports To

- The crate's public API (consumed by `codex-app-server-client`, the TUI, and tests) comes from `lib.rs`.
