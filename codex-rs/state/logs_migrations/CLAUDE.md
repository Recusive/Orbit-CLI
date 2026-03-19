# codex-rs/state/logs_migrations/

SQLite migration files for the dedicated logs database.

## What this folder does

Contains SQL migration scripts that define and evolve the schema of the logs SQLite database (`logs.db`). Migrations are embedded at compile time by `sqlx::migrate!()`.

## Key files

- `0001_logs.sql` -- initial logs table schema.
- `0002_logs_feedback_log_body.sql` -- adds feedback log body support.
