# codex-rs/codex-api/src/endpoint/realtime_websocket/

Realtime API WebSocket client with support for v1 and v2 protocols.

## What this folder does

Implements the WebSocket client for OpenAI's Realtime API, supporting both protocol versions. Handles connection establishment, event parsing, message serialization, and bidirectional streaming of realtime events (including audio frames).

## Where it plugs in

- Used by `codex-core` for real-time agent communication
- Builds on `tokio-tungstenite` for WebSocket connections
- Uses `codex-protocol` for `RealtimeEvent` and `RealtimeAudioFrame` types

## Key files

| File | Role |
|------|------|
| `mod.rs` | Module declarations and re-exports |
| `methods.rs` | `RealtimeWebsocketClient` -- connection establishment; `RealtimeWebsocketConnection` -- read/write split; `RealtimeWebsocketEvents` / `RealtimeWebsocketWriter` |
| `methods_common.rs` | Shared connection helpers across protocol versions |
| `methods_v1.rs` | V1 protocol-specific connection methods |
| `methods_v2.rs` | V2 protocol-specific connection methods |
| `protocol.rs` | `RealtimeEventParser` -- parses JSON messages into `RealtimeEvent`; `RealtimeSessionConfig` / `RealtimeSessionMode` |
| `protocol_common.rs` | Shared protocol parsing logic |
| `protocol_v1.rs` | V1 protocol event parsing |
| `protocol_v2.rs` | V2 protocol event parsing |
