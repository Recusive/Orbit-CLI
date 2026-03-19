# codex-rs/otel/tests/

Integration tests for the `codex-otel` crate.

## What this folder does

Contains integration tests that exercise the metrics client, OTLP HTTP export, event routing policy, runtime metric snapshots, and session telemetry. Tests use in-memory OTEL exporters and local TCP loopback servers to verify behavior without external dependencies.

## Key files

- `tests.rs` -- test crate root; declares `harness` and `suite` modules
- `harness/` -- shared test helpers (build in-memory metrics client, extract metrics data)
- `suite/` -- individual test modules covering validation, send/receive, timing, snapshots, OTLP HTTP loopback, event routing, runtime summaries, and session telemetry metadata

## Imports from

- `codex_otel` -- all public types (`SessionTelemetry`, `OtelProvider`, `MetricsClient`, `MetricsConfig`, etc.)
- `codex_protocol` -- `ThreadId`, `SessionSource`, `SandboxPolicy`, `AskForApproval`, `UserInput`, etc.
- `opentelemetry_sdk` -- `InMemoryMetricExporter`, `InMemoryLogExporter`, `InMemorySpanExporter`, metric data types

## Exports to

These are test-only modules; they do not export anything to production code.
