# codex-rs/hooks/

Hook system for the Codex agent. Provides lifecycle hooks that run external commands at key points in the agent session: session start, user prompt submission, and stop.

## What this folder does

Implements a hook discovery, dispatch, and execution engine. Hooks are configured in the config layer stack and are implemented as external commands that receive JSON input on stdin and return JSON output on stdout. The system supports three event types: `SessionStart`, `UserPromptSubmit`, and `Stop`. Hooks can provide context to the agent, block operations, or inject system messages. Also supports legacy notify hooks (fire-and-forget external commands after agent turns).

## What it plugs into

- **codex-core** -- the agent engine calls hooks at session start, prompt submission, and stop events
- **codex-config** -- hooks are discovered from the `ConfigLayerStack` (global, project, local)
- **codex-protocol** -- hook run summaries are emitted as protocol events (HookStarted, HookCompleted)

## Imports from

- `codex-config`: `ConfigLayerStack` for hook configuration discovery
- `codex-protocol`: `HookRunSummary`, `HookEventName`, `HookRunStatus`, `HookOutputEntryKind`, `ThreadId`, `SandboxPermissions`
- `schemars`: JSON Schema generation for hook input/output wire formats
- `chrono`: timestamp serialization
- `tokio`: async command execution
- `regex`: matcher patterns for hook filtering

## Exports to

- `Hooks` -- main entry point: `new(config)`, `dispatch(payload)`, `run_session_start()`, `run_user_prompt_submit()`, `run_stop()`, and preview methods
- `HooksConfig` -- configuration: legacy_notify_argv, feature_enabled, config_layer_stack, shell_program/args
- `Hook`, `HookPayload`, `HookEvent`, `HookResult`, `HookResponse` -- core types
- `HookEventAfterAgent`, `HookEventAfterToolUse`, `HookToolInput`, `HookToolInputLocalShell`, `HookToolKind` -- event detail types
- `SessionStartRequest/Outcome`, `UserPromptSubmitRequest/Outcome`, `StopRequest/Outcome` -- typed request/response for each event
- `legacy_notify_json()`, `notify_hook()` -- legacy notification support
- `write_schema_fixtures()` -- generates JSON Schema files for hook input/output formats
- `command_from_argv()` -- utility for building `tokio::process::Command` from argv

## Key files

- `Cargo.toml` -- crate metadata; library `codex_hooks`
- `src/lib.rs` -- module declarations and public re-exports
- `src/registry.rs` -- `Hooks` struct (main API) and `HooksConfig`
- `src/types.rs` -- `Hook`, `HookPayload`, `HookEvent`, `HookResult`, `HookResponse`, tool input types
- `src/schema.rs` -- JSON Schema definitions and fixture generation for hook wire formats
- `src/legacy_notify.rs` -- legacy fire-and-forget notification support (backward compat)
- `src/user_notification.rs` -- `UserNotification` serialization for legacy notify
- `schema/` -- JSON Schema definition files
