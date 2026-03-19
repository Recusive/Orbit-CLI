# codex-rs/codex-api/src/sse/

Server-Sent Events (SSE) stream processing for the Responses API.

## What this folder does

Processes SSE streams from the Responses API, parsing events into typed `ResponseEvent` values. Provides both a direct processing function and a spawn-based stream that runs on a background task.

## Key files

| File | Role |
|------|------|
| `mod.rs` | Module declarations; re-exports `process_sse`, `spawn_response_stream`, `stream_from_fixture` |
| `responses.rs` | `process_sse` -- processes raw SSE byte stream into `ResponseEvent` channel; `spawn_response_stream` -- spawns SSE processing on a background task; `stream_from_fixture` -- creates a stream from test fixture data |
