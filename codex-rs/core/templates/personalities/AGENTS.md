# codex-rs/core/templates/personalities/

This file applies to `codex-rs/core/templates/personalities/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-core` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-core`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Agent personality definitions that control communication style.

### What this folder does

Defines personality templates that shape how the agent communicates with the user. Personalities affect tone, verbosity, and interaction style without changing functional behavior.

### Key files

| File | Purpose |
|------|---------|
| `gpt-5.2-codex_friendly.md` | Friendly personality: warm, encouraging, explains reasoning, uses accessible language |
| `gpt-5.2-codex_pragmatic.md` | Pragmatic personality: concise, direct, focuses on results, minimal commentary |

### Where it plugs into

- Loaded via `include_str!()` in personality configuration
- Selected based on the `personality` config setting
- Injected into the system prompt via `crate::config`
- Can be changed per-turn in the TUI
