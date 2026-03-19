# codex-rs/codex-api/src/sse/

This file applies to `codex-rs/codex-api/src/sse/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-api` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-api`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Server-Sent Events (SSE) stream processing for the Responses API.

### What this folder does

Processes SSE streams from the Responses API, parsing events into typed `ResponseEvent` values. Provides both a direct processing function and a spawn-based stream that runs on a background task.

### Key files

| File | Role |
|------|------|
| `mod.rs` | Module declarations; re-exports `process_sse`, `spawn_response_stream`, `stream_from_fixture` |
| `responses.rs` | `process_sse` -- processes raw SSE byte stream into `ResponseEvent` channel; `spawn_response_stream` -- spawns SSE processing on a background task; `stream_from_fixture` -- creates a stream from test fixture data |
