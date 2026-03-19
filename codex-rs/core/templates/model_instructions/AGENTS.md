# codex-rs/core/templates/model_instructions/

This file applies to `codex-rs/core/templates/model_instructions/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-core` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-core`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Model-specific instruction templates.

### What this folder does

Contains instruction templates tailored to specific model versions, providing model-specific behavioral guidance.

### Key files

| File | Purpose |
|------|---------|
| `gpt-5.2-codex_instructions_template.md` | Instructions template for GPT-5.2 Codex models, defining coding behavior, output format preferences, and tool usage guidelines |

### Where it plugs into

- Loaded via `include_str!()` in model instruction configuration
- Selected based on the active model and applied to the system prompt
- Used by `crate::config` when constructing model-specific instructions
