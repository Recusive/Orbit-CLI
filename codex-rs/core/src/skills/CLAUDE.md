# codex-rs/core/src/skills/

Skill loading, management, rendering, injection, and invocation for the Codex agent.

## What this folder does

Skills are reusable, composable instruction sets that extend the agent's capabilities. This module handles the complete skill lifecycle:

- **Loading** (`loader.rs`): Discovers and loads skill definitions from `.codex/skills/` directories at project, user, and system levels.
- **Management** (`manager.rs`): `SkillsManager` -- central coordinator for skill lifecycle, caching, and availability queries.
- **Model** (`model.rs`): `SkillMetadata`, `SkillLoadOutcome`, `SkillPolicy`, `SkillError` -- data types for skill definitions.
- **Rendering** (`render.rs`): `render_skills_section()` -- generates the system prompt section listing available skills.
- **Injection** (`injection.rs`): `build_skill_injections()` -- injects skill instructions into conversation context when skills are invoked.
- **Invocation** (`invocation_utils.rs`): Handles implicit skill invocation (auto-detecting when a skill should be triggered from context).
- **Environment** (`env_var_dependencies.rs`): Collects and resolves environment variable dependencies declared by skills.
- **Remote** (`remote.rs`): Fetches remote skill definitions.
- **System** (`system.rs`): Built-in system skill definitions.

## Key files

| File | Purpose |
|------|---------|
| `mod.rs` | Module declarations and public re-exports |
| `manager.rs` | `SkillsManager` -- skill lifecycle coordination |
| `loader.rs` | Skill file discovery and loading |
| `model.rs` | Skill data types and metadata |
| `render.rs` | System prompt skill section rendering |
| `injection.rs` | Skill instruction injection into context |
| `invocation_utils.rs` | Implicit skill invocation detection |
| `env_var_dependencies.rs` | Environment variable dependency management |

## Imports from

- `crate::config` -- `Config`, `SkillsConfig` for skill settings
- `crate::instructions` -- `SkillInstructions` for injection format
- `crate::plugins` -- Plugin-provided skills integration

## Exports to

- `crate::codex` -- `render_skills_section()` for prompt construction
- `crate::state` -- `SkillsManager` held in `SessionServices`
- `crate::tools::runtimes` -- `SkillMetadata` for execve-based skill execution
- Public API: `SkillsManager`, `SkillMetadata`, `SkillLoadOutcome`, `SkillError`
