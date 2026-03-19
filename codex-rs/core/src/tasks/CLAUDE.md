# codex-rs/core/src/tasks/

Session task types that drive different Codex workflows (chat, review, compact, undo, etc.).

## What this folder does

Defines the `SessionTask` trait and its concrete implementations. Each task type encapsulates a specific Codex workflow that runs as a background Tokio task owned by a `Session`.

### SessionTask trait
- `kind()` -- identifies the task type for telemetry/UI
- `span_name()` -- tracing span name
- `run()` -- executes the task until completion or cancellation
- `abort()` -- cleanup after cancellation

### Task lifecycle (managed in `mod.rs`)
1. `Session::spawn_task()` -- aborts existing tasks, creates cancellation token, spawns background task
2. Task runs with a `SessionTaskContext` providing access to session, auth, and models
3. On completion: emits `TurnComplete` event with optional final agent message
4. On abort: emits `TurnAborted` event, records abort marker in history
5. Token usage metrics are computed and emitted per-turn

### Concrete task types

| Task | File | Purpose |
|------|------|---------|
| `RegularTask` | `regular.rs` | Standard chat turn -- sends user input to model, processes tool calls |
| `ReviewTask` | `review.rs` | Code review turn with specialized prompt and output format |
| `CompactTask` | `compact.rs` | Context compaction -- summarizes history to fit context window |
| `GhostSnapshotTask` | `ghost_snapshot.rs` | Creates ghost commits for turn-level version tracking |
| `UndoTask` | `undo.rs` | Reverts changes from a previous turn |
| `UserShellCommandTask` | `user_shell.rs` | Executes user-initiated shell commands with approval flow |

## Key files

| File | Purpose |
|------|---------|
| `mod.rs` | `SessionTask` trait, `Session::spawn_task()`, abort/finish lifecycle, metrics |
| `regular.rs` | `RegularTask` -- main agent loop |
| `review.rs` | `ReviewTask` -- code review workflow |
| `compact.rs` | `CompactTask` -- context compaction |
| `ghost_snapshot.rs` | `GhostSnapshotTask` -- git ghost commits |
| `undo.rs` | `UndoTask` -- turn reversal |
| `user_shell.rs` | `UserShellCommandTask` -- user shell command execution |

## Imports from

- `crate::codex` -- `Session`, `TurnContext`
- `crate::state` -- `ActiveTurn`, `RunningTask`, `TaskKind`
- `crate::protocol` -- Event types (`TurnComplete`, `TurnAborted`, etc.)
- `crate::features` -- Feature flag checks
- `codex_otel` -- Telemetry metrics and timers

## Exports to

- `crate::codex` -- `Session` uses task types for all turn execution
- `crate::state` -- `TaskKind` used in `RunningTask`
