# codex-rs/protocol/src/prompts/permissions/sandbox_mode/

Sandbox mode prompt templates -- one per `SandboxMode` variant.

## What this folder does

Contains markdown instructions that describe the active filesystem sandbox mode to the agent. Each file uses a `{network_access}` placeholder that is filled at runtime.

## Key files

- `read_only.md` -- `SandboxMode::ReadOnly`: sandbox permits reading files only; network access status is templated
- `workspace_write.md` -- `SandboxMode::WorkspaceWrite`: sandbox permits reading files and editing within `cwd` and `writable_roots`; editing outside requires approval; network access status is templated
- `danger_full_access.md` -- `SandboxMode::DangerFullAccess`: no filesystem sandboxing; all commands permitted; network access status is templated

## What it plugs into

Selected by `codex-core` based on the session's `SandboxPolicy` configuration. The chosen template is embedded in the system prompt with the `{network_access}` placeholder replaced.

## Exports to

Static markdown content consumed during system prompt assembly.
