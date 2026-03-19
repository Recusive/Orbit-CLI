# codex-rs/protocol/src/prompts/

This file applies to `codex-rs/protocol/src/prompts/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-protocol` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-protocol`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Embedded markdown prompt templates for the Codex CLI system instructions.

### What this folder does

Contains markdown files that are compiled into the Codex binary as system prompt components. These templates define the agent's personality, behavior guidelines, sandbox/permission instructions, and realtime conversation framing. They are assembled at runtime into the full system prompt based on session configuration.

### Directory structure

- `base_instructions/` -- core agent instructions (personality, capabilities, tool guidelines, formatting rules)
- `permissions/` -- sandbox and approval policy instructions
  - `approval_policy/` -- per-policy approval instructions (never, on_failure, on_request_rule, unless_trusted)
  - `sandbox_mode/` -- per-mode filesystem sandbox instructions (read_only, workspace_write, danger_full_access)
- `realtime/` -- realtime voice conversation framing instructions

### What it plugs into

These markdown files are included at compile time (via `include_str!` or similar) into the `codex-protocol` crate and assembled by `codex-core` when building the system prompt for a session.

### Exports to

The markdown content is accessed through the `protocol` module's prompt-related types and constants, consumed by `codex-core` during session initialization.
