# codex-rs/core/templates/model_instructions/

Model-specific instruction templates.

## What this folder does

Contains instruction templates tailored to specific model versions, providing model-specific behavioral guidance.

## Key files

| File | Purpose |
|------|---------|
| `gpt-5.2-codex_instructions_template.md` | Instructions template for GPT-5.2 Codex models, defining coding behavior, output format preferences, and tool usage guidelines |

## Where it plugs into

- Loaded via `include_str!()` in model instruction configuration
- Selected based on the active model and applied to the system prompt
- Used by `crate::config` when constructing model-specific instructions
