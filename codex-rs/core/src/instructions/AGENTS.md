# codex-rs/core/src/instructions/

This file applies to `codex-rs/core/src/instructions/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-core` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-core`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

User instruction loading and injection for AGENTS.md files and skill instructions.

### What this folder does

Manages the loading and serialization of user-provided instructions that get injected into the model context. Two instruction types are handled:

- **UserInstructions**: Content from `AGENTS.md` (or similar) files associated with a directory. Serialized with XML markers (`AGENTS_MD_FRAGMENT`) and injected as user-role messages into the conversation.
- **SkillInstructions**: Content from skill definition files, wrapped with skill-specific XML markers (`SKILL_FRAGMENT`) including the skill name and path.

### Key files

| File | Purpose |
|------|---------|
| `mod.rs` | Module declaration, re-exports `UserInstructions`, `SkillInstructions`, `USER_INSTRUCTIONS_PREFIX` |
| `user_instructions.rs` | `UserInstructions` and `SkillInstructions` structs with serialization to `ResponseItem` |
| `user_instructions_tests.rs` | Tests for instruction serialization |

### Imports from

- `codex_protocol` -- `ResponseItem` for conversion
- `crate::contextual_user_message` -- `AGENTS_MD_FRAGMENT`, `SKILL_FRAGMENT` XML tag helpers

### Exports to

- `crate::codex` -- injected into conversation history during turn preparation
- `crate::skills` -- `SkillInstructions` used when skills are invoked
- `crate::custom_prompts` -- coordinates with user instruction loading
