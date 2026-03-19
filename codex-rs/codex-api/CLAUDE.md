# codex-rs/codex-api/

Low-level API client library for OpenAI endpoints (Responses API, Realtime WebSocket, SSE, models, memories, compaction).

## What this folder does

Provides typed Rust clients for communicating with OpenAI's API endpoints. Supports multiple transport mechanisms: HTTP SSE streaming for the Responses API, WebSocket connections for the Realtime API (v1 and v2 protocols), and standard REST endpoints for models listing, memory summarization, and context compaction.

## Where it plugs in

- Used by `codex-core` as the primary API communication layer
- Built on top of `codex-client` for HTTP transport, retry, and custom CA support
- Uses `codex-protocol` for shared types (`RealtimeEvent`, `RealtimeAudioFrame`)

## Imports from

- `codex-client` -- `ReqwestTransport`, `RequestTelemetry`, `TransportError`, HTTP transport abstractions
- `codex-protocol` -- protocol types, realtime events
- `codex-utils-rustls-provider` -- TLS configuration
- `tokio-tungstenite` / `tungstenite` -- WebSocket connections
- `eventsource-stream` -- SSE stream parsing
- `serde` / `serde_json` -- JSON serialization

## Exports to

Public API from `lib.rs`:

- `ResponsesClient` / `ResponsesOptions` -- Responses API HTTP client
- `ResponsesWebsocketClient` / `ResponsesWebsocketConnection` -- Responses API over WebSocket
- `RealtimeWebsocketClient` / `RealtimeWebsocketConnection` / `RealtimeEventParser` / `RealtimeSessionConfig` -- Realtime API WebSocket client
- `CompactClient` -- context compaction endpoint
- `MemoriesClient` -- memory summarization endpoint
- `ModelsClient` -- models listing endpoint
- `Provider` / `AuthProvider` -- API provider configuration and authentication
- `ApiError` -- error types
- `SseTelemetry` / `WebsocketTelemetry` -- telemetry types
- SSE streaming helpers: `stream_from_fixture`, `process_sse`, `spawn_response_stream`
- Common types: `ResponsesApiRequest`, `ResponseEvent`, `ResponseStream`, `CompactionInput`, etc.

## Key files

| File | Role |
|------|------|
| `Cargo.toml` | Crate manifest; depends on `codex-client`, `codex-protocol`, `tokio-tungstenite` |
| `src/lib.rs` | Module declarations and public re-exports |
| `src/endpoint/` | API endpoint clients |
| `src/requests/` | Request construction and header helpers |
| `src/sse/` | SSE stream processing |
| `src/auth.rs` | `AuthProvider` trait for API authentication |
| `src/common.rs` | Shared types across endpoints |
| `src/error.rs` | `ApiError` enum |
| `src/provider.rs` | `Provider` struct for API base URL and configuration |
| `src/rate_limits.rs` | Rate limit header parsing |
| `src/telemetry.rs` | SSE and WebSocket telemetry types |
| `tests/` | Integration and end-to-end tests |
