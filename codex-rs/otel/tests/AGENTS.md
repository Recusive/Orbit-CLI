# codex-rs/otel/tests/

This file applies to `codex-rs/otel/tests/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-otel` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-otel`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Integration tests for the `codex-otel` crate.

### What this folder does

Contains integration tests that exercise the metrics client, OTLP HTTP export, event routing policy, runtime metric snapshots, and session telemetry. Tests use in-memory OTEL exporters and local TCP loopback servers to verify behavior without external dependencies.

### Key files

- `tests.rs` -- test crate root; declares `harness` and `suite` modules
- `harness/` -- shared test helpers (build in-memory metrics client, extract metrics data)
- `suite/` -- individual test modules covering validation, send/receive, timing, snapshots, OTLP HTTP loopback, event routing, runtime summaries, and session telemetry metadata

### Imports from

- `codex_otel` -- all public types (`SessionTelemetry`, `OtelProvider`, `MetricsClient`, `MetricsConfig`, etc.)
- `codex_protocol` -- `ThreadId`, `SessionSource`, `SandboxPolicy`, `AskForApproval`, `UserInput`, etc.
- `opentelemetry_sdk` -- `InMemoryMetricExporter`, `InMemoryLogExporter`, `InMemorySpanExporter`, metric data types

### Exports to

These are test-only modules; they do not export anything to production code.
