# codex-rs/state/

SQLite-backed persistent state management for Codex.

## What this folder does

Provides `StateRuntime`, a high-level interface for storing and querying thread metadata, logs, backfill state, agent jobs, and memories in local SQLite databases. Extracts metadata from JSONL rollout files and mirrors it into structured tables.

## What it plugs into

- Used by `codex-core` for thread/session persistence across CLI restarts.
- The `log_db` module provides a `tracing_subscriber::Layer` for writing structured logs to SQLite.
- The `logs_client` binary provides a standalone log tailing CLI.

## Imports from

- `codex-protocol` -- `ThreadId`, `RolloutItem`, `EventMsg`, `ResponseItem`, and other protocol types.
- `sqlx` -- SQLite connection pooling, migrations, and queries.
- `chrono` -- date/time handling.
- `serde`, `serde_json` -- serialization.
- `tokio` -- async runtime.
- `tracing`, `tracing-subscriber` -- log layer integration.

## Exports to

- `StateRuntime` -- primary entrypoint: manages DB pools, runs migrations, provides query methods for threads, logs, backfill, agent jobs, and memories.
- Model types: `ThreadMetadata`, `ThreadMetadataBuilder`, `ThreadsPage`, `LogEntry`, `LogQuery`, `LogRow`, `AgentJob`, `AgentJobItem`, `BackfillState`, `Stage1Output`, etc.
- Path helpers: `state_db_path()`, `logs_db_path()`.
- Extraction: `apply_rollout_item()`, `rollout_item_affects_thread_metadata()`.
- Constants: `SQLITE_HOME_ENV`, `LOGS_DB_FILENAME`, `STATE_DB_FILENAME`, version numbers.

## Key files

- `Cargo.toml` -- crate manifest.
- `src/lib.rs` -- module declarations, re-exports, and metric constants.
- `src/runtime.rs` -- `StateRuntime` implementation with DB initialization and query methods.
- `src/model/` -- data model types.
- `src/runtime/` -- runtime submodules (threads, logs, backfill, agent_jobs, memories).
- `src/extract.rs` -- rollout item to thread metadata extraction.
- `src/log_db.rs` -- tracing layer that writes log events to SQLite.
- `src/migrations.rs` -- SQLx migration runner setup.
- `src/paths.rs` -- file path utilities.
- `migrations/` -- SQLite migration SQL files for the state DB.
- `logs_migrations/` -- SQLite migration SQL files for the logs DB.
