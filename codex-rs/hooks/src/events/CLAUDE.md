# codex-rs/hooks/src/events/

Per-event-type hook logic: request types, outcome types, and run/preview functions.

## What this folder does

Contains modules for each supported hook event type. Each module defines the typed request, outcome, and the `run()` / `preview()` functions that filter configured handlers by event name, build the command input, execute the commands, and aggregate results.

## Key files and their roles

- `mod.rs` -- Module declarations: `common`, `session_start`, `stop`, `user_prompt_submit`.
- `common.rs` -- Shared utilities used across event types (e.g., handler filtering, run summary construction).
- `session_start.rs` -- `SessionStartRequest` (session_id, transcript_path, cwd, model, permission_mode, source), `SessionStartSource` enum (Startup/Resume/Clear), `SessionStartOutcome` (summaries, context, stop_reason, system_message). `run()` and `preview()` functions.
- `user_prompt_submit.rs` -- `UserPromptSubmitRequest` (session_id, turn_id, transcript_path, cwd, model, permission_mode, prompt), `UserPromptSubmitOutcome` (summaries, blocked, block_reason, context, system_message). `run()` and `preview()` functions.
- `stop.rs` -- `StopRequest` (session_id, turn_id, transcript_path, cwd, model, permission_mode, stop_hook_active, last_assistant_message), `StopOutcome` (summaries, blocked, block_reason). `run()` and `preview()` functions.

## Imports from

- `crate::engine`: ConfiguredHandler, CommandShell, command execution
- `crate::schema`: wire format types for input/output
- `codex-protocol`: HookRunSummary, HookEventName

## Exports to

- Request/Outcome types re-exported through `src/lib.rs`
- `run()`/`preview()` called by `ClaudeHooksEngine` in `src/engine/mod.rs`
