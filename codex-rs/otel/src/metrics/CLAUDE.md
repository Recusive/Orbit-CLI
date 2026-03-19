# codex-rs/otel/src/metrics/

OpenTelemetry metrics subsystem for the Codex CLI.

## What this folder does

Provides a `MetricsClient` that wraps the OpenTelemetry `SdkMeterProvider` to emit counters, histograms, and duration histograms. Supports OTLP export (gRPC/HTTP) and in-memory export (for tests). Includes input validation, default tag merging, runtime metric snapshots, and a global singleton accessor.

## Key files

- `mod.rs` -- declares submodules; re-exports `MetricsClient`, `MetricsConfig`, `MetricsExporter`, `MetricsError`, `Result`; provides `install_global()` and `global()` for a process-wide `MetricsClient` singleton via `OnceLock`
- `client.rs` -- `MetricsClient` (cheaply cloneable via `Arc`):
  - `new(config)` -- builds OTLP or in-memory meter provider with OS resource attributes, optional runtime reader, and periodic export
  - `counter()` / `histogram()` / `record_duration()` -- record metrics with validated tags merged against defaults
  - `start_timer()` -- returns an RAII `Timer`
  - `snapshot()` -- collects runtime metrics via `ManualReader` without shutdown
  - `shutdown()` -- flushes and stops the provider
- `config.rs` -- `MetricsConfig` builder with `otlp()` and `in_memory()` constructors; `with_export_interval()`, `with_runtime_reader()`, `with_tag()` methods
- `error.rs` -- `MetricsError` enum covering validation errors, exporter build failures, provider shutdown errors, and snapshot unavailability
- `names.rs` -- all metric name constants: `codex.tool.call`, `codex.api_request`, `codex.sse_event`, `codex.websocket.*`, `codex.responses_api_*`, `codex.turn.*`, `codex.startup_prewarm.*`, `codex.thread.started`
- `runtime_metrics.rs` -- `RuntimeMetricTotals` and `RuntimeMetricsSummary` for aggregating per-turn metrics from OTEL snapshots; `from_snapshot()` extracts counter sums and histogram sums
- `tags.rs` -- `SessionMetricTagValues` struct and tag constants (`app.version`, `auth_mode`, `model`, `originator`, `service_name`, `session_source`); converts session metadata into validated tag tuples
- `timer.rs` -- `Timer` struct that records duration on drop; supports additional tags via `record()`
- `validation.rs` -- `validate_metric_name()`, `validate_tag_key()`, `validate_tag_value()`, `validate_tags()`; enforces alphanumeric + `.`/`_`/`-` for names and additionally `/` for tags

## Imports from

- `crate::config` -- `OtelExporter`, `OtelHttpProtocol`
- `crate::otlp` -- `build_header_map`, `build_grpc_tls_config`, `build_http_client`
- `opentelemetry`, `opentelemetry_sdk`, `opentelemetry-otlp`, `opentelemetry-semantic-conventions`
- `codex-utils-string` -- `sanitize_metric_tag_value`

## Exports to

- `MetricsClient`, `MetricsConfig`, `MetricsExporter`, `MetricsError`, `Result` re-exported through `crate::metrics` and `lib.rs`
- `RuntimeMetricsSummary`, `RuntimeMetricTotals`, `Timer` re-exported through `lib.rs`
- `names` and `tags` modules are `pub` for use by `session_telemetry.rs` and external crates
