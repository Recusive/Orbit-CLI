# codex-rs/state/src/

Source code for the `codex-state` crate.

## What this folder does

Implements SQLite-backed state management including thread metadata, structured logging, backfill orchestration, agent jobs, and memories.

## Key files

- `lib.rs` -- module declarations, public re-exports, environment variable and metric constants.
- `runtime.rs` -- `StateRuntime` struct: initializes SQLite connection pools (WAL mode, busy timeout), runs migrations, and coordinates query execution. Declares submodule imports for threads, logs, backfill, agent_jobs, and memories.
- `extract.rs` -- `apply_rollout_item()` transforms `RolloutItem` variants (SessionMeta, TurnContext, EventMsg, ResponseItem) into `ThreadMetadata` mutations. `rollout_item_affects_thread_metadata()` is a fast filter.
- `log_db.rs` -- `tracing_subscriber::Layer` implementation that captures log events and batch-inserts them into the logs SQLite DB via a background task.
- `migrations.rs` -- loads the `STATE_MIGRATOR` and `LOGS_MIGRATOR` from embedded SQL files.
- `paths.rs` -- file modification time utilities.

## Subdirectories

- `model/` -- data model types and database row mappings.
- `runtime/` -- query implementations organized by domain (threads, logs, backfill, agent_jobs, memories).
- `bin/` -- standalone binary (logs_client).

## Imports from

- `codex-protocol` for `ThreadId`, `RolloutItem`, `EventMsg`, etc.
- `sqlx` for SQLite operations.
- `chrono`, `serde`, `tokio`, `tracing`.
