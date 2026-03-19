# codex-rs/protocol/src/prompts/

Embedded markdown prompt templates for the Codex CLI system instructions.

## What this folder does

Contains markdown files that are compiled into the Codex binary as system prompt components. These templates define the agent's personality, behavior guidelines, sandbox/permission instructions, and realtime conversation framing. They are assembled at runtime into the full system prompt based on session configuration.

## Directory structure

- `base_instructions/` -- core agent instructions (personality, capabilities, tool guidelines, formatting rules)
- `permissions/` -- sandbox and approval policy instructions
  - `approval_policy/` -- per-policy approval instructions (never, on_failure, on_request_rule, unless_trusted)
  - `sandbox_mode/` -- per-mode filesystem sandbox instructions (read_only, workspace_write, danger_full_access)
- `realtime/` -- realtime voice conversation framing instructions

## What it plugs into

These markdown files are included at compile time (via `include_str!` or similar) into the `codex-protocol` crate and assembled by `codex-core` when building the system prompt for a session.

## Exports to

The markdown content is accessed through the `protocol` module's prompt-related types and constants, consumed by `codex-core` during session initialization.
