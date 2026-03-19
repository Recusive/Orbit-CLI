# codex-rs/state/src/bin/

This file applies to `codex-rs/state/src/bin/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-state` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-state`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Standalone binary for tailing Codex logs.

### What this folder does

Contains the `codex-state-logs` CLI binary that tails structured logs from the Codex logs SQLite database with filtering and colorized output.

### Key files

- `logs_client.rs` -- full CLI implementation using `clap` for argument parsing. Supports filters by log level, timestamp range, module path, file path, thread ID, and body substring search. Outputs colorized logs with diff-style formatting for `apply_patch` tool calls. Runs in a poll loop for live tailing.

### What it plugs into

- Reads from the logs SQLite DB managed by `StateRuntime`.
- Useful for debugging Codex sessions in development.

### Imports from

- `codex-state` -- `StateRuntime`, `LogQuery`, `LogRow`.
- `clap` -- CLI argument parsing.
- `chrono` -- timestamp formatting.
- `owo-colors` -- terminal color output.
- `dirs` -- home directory resolution.
