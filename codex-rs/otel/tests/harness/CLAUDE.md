# codex-rs/otel/tests/harness/

Shared test helpers for `codex-otel` integration tests.

## What this folder does

Provides utility functions used across multiple test suites to build in-memory metrics clients and inspect exported metric data.

## Key files

- `mod.rs` -- helper functions:
  - `build_metrics_with_defaults(default_tags)` -- creates a `MetricsClient` backed by `InMemoryMetricExporter` with optional default tags
  - `latest_metrics(exporter)` -- extracts the last `ResourceMetrics` from an in-memory exporter
  - `find_metric(resource_metrics, name)` -- locates a specific `Metric` by name in exported data
  - `attributes_to_map(attributes)` -- converts `KeyValue` iterator to `BTreeMap<String, String>` for assertions
  - `histogram_data(resource_metrics, name)` -- extracts bounds, bucket counts, sum, and count from a histogram metric

## Imports from

- `codex_otel::metrics` -- `MetricsClient`, `MetricsConfig`, `Result`
- `opentelemetry_sdk::metrics` -- `InMemoryMetricExporter`, metric data types

## Exports to

All functions are `pub(crate)` and used by test modules in `suite/`.
