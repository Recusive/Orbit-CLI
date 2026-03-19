# codex-rs/core/src/agent/builtins/

This file applies to `codex-rs/core/src/agent/builtins/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-core` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-core`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Built-in agent role definitions shipped with the Codex binary.

### What this folder does

Contains TOML configuration files that define pre-packaged agent roles. These roles are loaded by the role resolution system in `agent/role.rs` and can be selected when spawning sub-agents.

### Key files

| File | Purpose |
|------|---------|
| `awaiter.toml` | "Awaiter" role: a specialized agent that polls and waits for command/task completion, using low reasoning effort and long terminal timeouts |
| `explorer.toml` | "Explorer" role: placeholder for a code exploration agent (currently empty) |

### Where it plugs into

- **Loaded by**: `agent/role.rs` via `resolve_role_config()` which checks built-in roles before user-defined ones.
- **Referenced by**: `agent/control.rs` when spawning sub-agents with a specified `agent_type`.
- **Format**: Standard `config.toml` TOML structure with fields like `background_terminal_max_timeout`, `model_reasoning_effort`, and `developer_instructions`.
