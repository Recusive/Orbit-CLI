# codex-rs/codex-api/src/

Source directory for the `codex-api` crate.

## What this folder does

Contains the implementation of all OpenAI API clients, organized into endpoint-specific modules, request construction helpers, SSE stream processing, and shared types.

## Key files

| File | Role |
|------|------|
| `lib.rs` | Module declarations and public re-exports |
| `auth.rs` | `AuthProvider` trait -- provides bearer tokens for API requests |
| `common.rs` | Shared types: `ResponsesApiRequest`, `ResponseEvent`, `ResponseStream`, `CompactionInput`, `MemorySummarizeInput/Output`, `RawMemory` |
| `error.rs` | `ApiError` enum with variants for transport, SSE, WebSocket, and API errors |
| `provider.rs` | `Provider` struct -- encapsulates base URL, auth, and API version; `is_azure_responses_wire_base_url` helper |
| `rate_limits.rs` | Parsing rate limit information from HTTP response headers |
| `telemetry.rs` | `SseTelemetry` and `WebsocketTelemetry` structs for tracking API call metrics |
| `endpoint/` | API endpoint client implementations |
| `requests/` | Request construction and header building |
| `sse/` | SSE stream processing |
