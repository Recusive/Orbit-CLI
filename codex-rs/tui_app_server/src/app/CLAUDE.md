# codex-rs/tui_app_server/src/app/

App submodule containing the adapter layer between the TUI and the app-server backend.

## What this folder does

Houses the supplementary modules for the `App` state machine defined in `../app.rs`. These modules handle app-server event adaptation, pending request tracking, multi-agent navigation state, and interactive replay logic during thread switching.

## What it plugs into

- **../app.rs**: The parent `App` struct uses these modules for app-server communication, request resolution, agent navigation, and replay tracking.
- **../app_server_session.rs**: `AppServerSession` provides the transport; `app_server_adapter.rs` bridges its events into `AppEvent`s.
- **codex-app-server-protocol**: JSON-RPC types (`ServerNotification`, `ServerRequest`, `ClientRequest`, `RequestId`) used throughout.

## Key files

| File | Role |
|------|------|
| `app_server_adapter.rs` | Adapter layer that drains app-server events (`AppServerEvent`, `ServerNotification`, `ServerRequest`) and converts them into `AppEvent`s. This is the bridge between the app-server protocol and the TUI event loop. |
| `app_server_requests.rs` | `PendingAppServerRequests` -- tracks in-flight app-server requests (exec approvals, file change approvals, permissions, user inputs, MCP requests) and resolves them when the user responds. |
| `agent_navigation.rs` | `AgentNavigationState` -- manages multi-agent picker ordering, spawn-order traversal, and active-agent labels. Keeps pure navigation logic separate from `App` UI side effects. |
| `pending_interactive_replay.rs` | Tracks which interactive prompts (approvals, user input, MCP elicitations) are still pending during thread-event replay when switching agents/threads. |

## Imports from

- `crate::app_event` (`AppEvent`, `RealtimeAudioDeviceKind`)
- `crate::app_command` (`AppCommand`, `AppCommandView`)
- `crate::app_server_session` (`AppServerSession`, rate limit helpers)
- `codex_app_server_client` (`AppServerEvent`)
- `codex_app_server_protocol` (all JSON-RPC types)
- `codex_protocol` (`ThreadId`, MCP request IDs)

## Exports to

- **../app.rs**: All types are `pub(super)` or `pub(crate)`, consumed by the parent `App` implementation.
