# codex-rs/codex-api/src/endpoint/

API endpoint client implementations.

## What this folder does

Contains typed client structs for each OpenAI API endpoint: Responses API (HTTP and WebSocket), Realtime WebSocket API, models listing, memory summarization, and context compaction.

## Key files

| File | Role |
|------|------|
| `mod.rs` | Module declarations |
| `responses.rs` | `ResponsesClient` / `ResponsesOptions` -- HTTP SSE client for the Responses API |
| `responses_websocket.rs` | `ResponsesWebsocketClient` / `ResponsesWebsocketConnection` -- WebSocket transport for the Responses API |
| `realtime_websocket/` | Realtime API WebSocket client with v1/v2 protocol support |
| `compact.rs` | `CompactClient` -- context compaction endpoint |
| `memories.rs` | `MemoriesClient` -- memory summarization endpoint |
| `models.rs` | `ModelsClient` -- models listing endpoint |
| `session.rs` | Session management helpers |
