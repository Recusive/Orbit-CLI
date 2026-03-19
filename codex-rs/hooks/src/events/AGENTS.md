# codex-rs/hooks/src/events/

This file applies to `codex-rs/hooks/src/events/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-hooks` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-hooks`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Per-event-type hook logic: request types, outcome types, and run/preview functions.

### What this folder does

Contains modules for each supported hook event type. Each module defines the typed request, outcome, and the `run()` / `preview()` functions that filter configured handlers by event name, build the command input, execute the commands, and aggregate results.

### Key files and their roles

- `mod.rs` -- Module declarations: `common`, `session_start`, `stop`, `user_prompt_submit`.
- `common.rs` -- Shared utilities used across event types (e.g., handler filtering, run summary construction).
- `session_start.rs` -- `SessionStartRequest` (session_id, transcript_path, cwd, model, permission_mode, source), `SessionStartSource` enum (Startup/Resume/Clear), `SessionStartOutcome` (summaries, context, stop_reason, system_message). `run()` and `preview()` functions.
- `user_prompt_submit.rs` -- `UserPromptSubmitRequest` (session_id, turn_id, transcript_path, cwd, model, permission_mode, prompt), `UserPromptSubmitOutcome` (summaries, blocked, block_reason, context, system_message). `run()` and `preview()` functions.
- `stop.rs` -- `StopRequest` (session_id, turn_id, transcript_path, cwd, model, permission_mode, stop_hook_active, last_assistant_message), `StopOutcome` (summaries, blocked, block_reason). `run()` and `preview()` functions.

### Imports from

- `crate::engine`: ConfiguredHandler, CommandShell, command execution
- `crate::schema`: wire format types for input/output
- `codex-protocol`: HookRunSummary, HookEventName

### Exports to

- Request/Outcome types re-exported through `src/lib.rs`
- `run()`/`preview()` called by `ClaudeHooksEngine` in `src/engine/mod.rs`
