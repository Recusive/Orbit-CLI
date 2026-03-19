# codex-rs/core/templates/collab/

This file applies to `codex-rs/core/templates/collab/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-core` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-core`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Experimental collaboration mode prompt templates.

### What this folder does

Contains experimental prompt templates for collaborative interaction modes where the agent works more interactively with the user.

### Key files

| File | Purpose |
|------|---------|
| `experimental_prompt.md` | Experimental collaboration prompt defining interactive pair-programming style behavior |

### Where it plugs into

- Loaded via `include_str!()` when the `Collab` feature flag is enabled
- Used by collaboration mode configuration in `crate::models_manager::collaboration_mode_presets`
