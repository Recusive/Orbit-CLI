# codex-rs/hooks/src/

This file applies to `codex-rs/hooks/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-hooks` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-hooks`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source code for the `codex-hooks` crate.

### What this folder does

Implements the hook system: configuration discovery, command execution, output parsing, and lifecycle event dispatching.

### Key files and their roles

- `lib.rs` -- Module declarations and public re-exports for all hook types.
- `registry.rs` -- `Hooks` struct: main API. `new(config)` builds the engine from `HooksConfig`. `dispatch(payload)` runs legacy hooks. `run_session_start()`, `run_user_prompt_submit()`, `run_stop()` run the engine hooks. `preview_*` methods return `HookRunSummary` without executing. Also has `command_from_argv()` utility.
- `types.rs` -- Core types: `Hook` (name + async function), `HookPayload` (session_id, cwd, client, triggered_at, hook_event), `HookEvent` (AfterAgent/AfterToolUse), `HookResult` (Success/FailedContinue/FailedAbort), `HookResponse`, `HookToolInput`, `HookToolInputLocalShell`, `HookToolKind`.
- `schema.rs` -- JSON Schema wire types and generation. Defines input/output structs for each event type (SessionStartCommandInput, UserPromptSubmitCommandInput, StopCommandInput, *OutputWire). Has `write_schema_fixtures()` for generating schema JSON files. Also defines `NullableString`, `BlockDecisionWire`, `HookEventNameWire`.
- `legacy_notify.rs` -- `legacy_notify_json()` and `notify_hook()`: backward-compatible fire-and-forget notification hooks.
- `user_notification.rs` -- `UserNotification` enum and serialization for legacy notify payloads (agent-turn-complete).

### Subfolders

- `bin/` -- Binary for generating schema fixtures
- `engine/` -- Hook discovery, configuration, command execution, and output parsing
- `events/` -- Per-event-type request/outcome types and run/preview logic
