# codex-rs/codex-client/

Low-level HTTP transport, SSE streaming, retry logic, and custom CA support for Codex API clients.

## What this folder does

Provides the foundational HTTP client layer used by all Codex API crates. Handles reqwest-based transport with custom CA certificate support (via `NODE_EXTRA_CA_CERTS` and system certs), SSE stream processing, request retry with exponential backoff, zstd compression, and OpenTelemetry-based request telemetry.

## Where it plugs in

- Used by `codex-api` for Responses API and Realtime API communication
- Used by `codex-backend-client` for backend API calls (via `build_reqwest_client_with_custom_ca`)
- Used by `codex-chatgpt` indirectly through `codex-core`
- Provides the `HttpTransport` trait for mockable HTTP in tests

## Imports from

- `reqwest` -- HTTP client with JSON and streaming support
- `rustls` / `rustls-native-certs` / `rustls-pki-types` -- TLS with custom CA
- `codex-utils-rustls-provider` -- shared rustls provider
- `eventsource-stream` -- SSE parsing
- `opentelemetry` / `tracing-opentelemetry` -- distributed tracing
- `zstd` -- request body compression
- `tokio` -- async runtime

## Exports to

Public API from `lib.rs`:

- `ReqwestTransport` / `HttpTransport` / `ByteStream` / `StreamResponse` -- HTTP transport abstraction
- `CodexHttpClient` / `CodexRequestBuilder` -- high-level client with defaults
- `build_reqwest_client_with_custom_ca` / `maybe_build_rustls_client_config_with_custom_ca` -- custom CA support
- `Request` / `RequestCompression` / `Response` -- request/response types
- `RetryPolicy` / `RetryOn` / `run_with_retry` / `backoff` -- retry primitives
- `sse_stream` -- SSE stream helper
- `RequestTelemetry` -- per-request telemetry
- `TransportError` / `StreamError` -- error types

## Key files

| File | Role |
|------|------|
| `Cargo.toml` | Crate manifest; depends on `reqwest`, `rustls`, `eventsource-stream`, `opentelemetry`, `zstd` |
| `src/lib.rs` | Module declarations and public re-exports |
| `src/transport.rs` | `HttpTransport` trait, `ReqwestTransport` implementation, `ByteStream`, `StreamResponse` |
| `src/default_client.rs` | `CodexHttpClient` / `CodexRequestBuilder` -- pre-configured HTTP client |
| `src/custom_ca.rs` | Custom CA certificate loading from `NODE_EXTRA_CA_CERTS` env var and system certs |
| `src/request.rs` | `Request` / `Response` types with zstd compression support |
| `src/retry.rs` | `RetryPolicy`, `RetryOn`, `run_with_retry` with exponential backoff |
| `src/sse.rs` | SSE stream processing helpers |
| `src/error.rs` | `TransportError` / `StreamError` enums |
| `src/telemetry.rs` | `RequestTelemetry` for OpenTelemetry span instrumentation |
| `src/bin/custom_ca_probe.rs` | Test binary for verifying custom CA behavior |
| `tests/` | Tests for CA handling |
