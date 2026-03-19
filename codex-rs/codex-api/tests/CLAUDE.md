# codex-rs/codex-api/tests/

Integration and end-to-end tests for the `codex-api` crate.

## What this folder does

Contains tests that verify API client behavior, model types, SSE stream processing, and WebSocket protocol handling.

## Key files

| File | Role |
|------|------|
| `clients.rs` | Tests for API client construction and configuration |
| `models_integration.rs` | Tests for model type serialization/deserialization |
| `sse_end_to_end.rs` | End-to-end SSE stream processing tests |
| `realtime_websocket_e2e.rs` | End-to-end Realtime WebSocket protocol tests |
