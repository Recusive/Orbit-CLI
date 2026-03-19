# codex-rs/codex-client/src/

This file applies to `codex-rs/codex-client/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-client` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-client`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source directory for the `codex-client` crate.

### What this folder does

Contains the HTTP transport layer, custom CA loading, SSE processing, retry logic, and telemetry for Codex API communication.

### Key files

| File | Role |
|------|------|
| `lib.rs` | Module declarations and public re-exports |
| `transport.rs` | `HttpTransport` async trait; `ReqwestTransport` -- reqwest-backed implementation; `ByteStream` / `StreamResponse` types |
| `default_client.rs` | `CodexHttpClient` -- pre-configured reqwest client with defaults; `CodexRequestBuilder` -- builder pattern |
| `custom_ca.rs` | Loads custom CA certs from `NODE_EXTRA_CA_CERTS` env var; merges with system certs; builds rustls `ClientConfig`; `build_reqwest_client_with_custom_ca` |
| `request.rs` | `Request` struct with URL, headers, body, optional zstd compression; `Response` wrapper |
| `retry.rs` | `RetryPolicy` (max retries, delay, jitter); `RetryOn` enum; `run_with_retry` -- async retry executor with exponential `backoff` |
| `sse.rs` | `sse_stream` -- converts HTTP response stream into parsed SSE events |
| `error.rs` | `TransportError` (HTTP, stream, timeout variants); `StreamError` (SSE-specific) |
| `telemetry.rs` | `RequestTelemetry` -- captures request/response metadata for OpenTelemetry spans |
