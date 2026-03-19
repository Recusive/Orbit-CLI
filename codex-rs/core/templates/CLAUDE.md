# codex-rs/core/templates/

Prompt templates, system instructions, and configuration presets used by the Codex agent.

## What this folder does

Contains all the markdown and XML template files that are embedded into the Codex binary via `include_str!()` macros. These templates define system prompts, collaboration mode instructions, memory pipeline prompts, personality configurations, tool descriptions, and review output formats.

## Subdirectories

| Directory | Purpose |
|-----------|---------|
| `agents/` | Multi-agent orchestration prompt templates |
| `collab/` | Experimental collaboration mode prompts |
| `collaboration_mode/` | Collaboration mode presets (default, execute, pair programming, plan) |
| `compact/` | Context compaction prompt and summary prefix |
| `memories/` | Memory extraction and consolidation prompts |
| `model_instructions/` | Model-specific instruction templates |
| `personalities/` | Agent personality definitions (friendly, pragmatic) |
| `review/` | Code review output format templates |
| `search_tool/` | Tool search and tool suggest description templates |
| `tools/` | Tool-specific prompt templates |

## Where it plugs into

Templates are loaded at compile time via `include_str!()` in various modules:
- `crate::memories` -- memory pipeline prompts
- `crate::compact` -- compaction prompts
- `crate::skills` -- skill-related templates
- `crate::tools::handlers` -- tool descriptions
- `crate::models_manager::collaboration_mode_presets` -- collaboration mode definitions
- `crate::config` -- personality and model instruction templates
