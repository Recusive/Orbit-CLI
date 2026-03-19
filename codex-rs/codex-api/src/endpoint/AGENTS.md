# codex-rs/codex-api/src/endpoint/

This file applies to `codex-rs/codex-api/src/endpoint/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-api` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-api`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

API endpoint client implementations.

### What this folder does

Contains typed client structs for each OpenAI API endpoint: Responses API (HTTP and WebSocket), Realtime WebSocket API, models listing, memory summarization, and context compaction.

### Key files

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
