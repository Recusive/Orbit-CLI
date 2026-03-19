# codex-rs/codex-api/src/requests/

Request construction and header building for API calls.

## What this folder does

Contains helpers for constructing HTTP request headers and building Responses API request payloads.

## Key files

| File | Role |
|------|------|
| `mod.rs` | Module declarations |
| `headers.rs` | `build_conversation_headers` -- constructs HTTP headers for API requests (auth, content-type, tracing) |
| `responses.rs` | Responses API request body construction helpers |
