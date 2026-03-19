# codex-rs/state/src/bin/

Standalone binary for tailing Codex logs.

## What this folder does

Contains the `codex-state-logs` CLI binary that tails structured logs from the Codex logs SQLite database with filtering and colorized output.

## Key files

- `logs_client.rs` -- full CLI implementation using `clap` for argument parsing. Supports filters by log level, timestamp range, module path, file path, thread ID, and body substring search. Outputs colorized logs with diff-style formatting for `apply_patch` tool calls. Runs in a poll loop for live tailing.

## What it plugs into

- Reads from the logs SQLite DB managed by `StateRuntime`.
- Useful for debugging Codex sessions in development.

## Imports from

- `codex-state` -- `StateRuntime`, `LogQuery`, `LogRow`.
- `clap` -- CLI argument parsing.
- `chrono` -- timestamp formatting.
- `owo-colors` -- terminal color output.
- `dirs` -- home directory resolution.
