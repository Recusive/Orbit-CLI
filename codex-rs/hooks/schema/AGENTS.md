# codex-rs/hooks/schema/

This file applies to `codex-rs/hooks/schema/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Treat this directory as golden data or generated/static support material. Keep filenames and relative paths stable unless the owning test, renderer, or generator changes first.
- Prefer updating the producer or the corresponding test scenario before editing files here by hand.
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-hooks` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-hooks`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

JSON Schema definition files for hook input/output wire formats.

### What this folder does

Contains JSON Schema files that define the expected input (stdin) and output (stdout) formats for hook commands at each lifecycle event. These schemas serve as both documentation and validation contracts.

### Key files and their roles

- `session-start.command.input.schema.json` -- Schema for SessionStart hook input (session_id, transcript_path, cwd, model, permission_mode, source).
- `session-start.command.output.schema.json` -- Schema for SessionStart hook output (continue, stop_reason, suppress_output, system_message, hook_specific_output).
- `user-prompt-submit.command.input.schema.json` -- Schema for UserPromptSubmit hook input (session_id, turn_id, transcript_path, cwd, model, permission_mode, prompt).
- `user-prompt-submit.command.output.schema.json` -- Schema for UserPromptSubmit hook output (continue, decision, reason, hook_specific_output).
- `stop.command.input.schema.json` -- Schema for Stop hook input (session_id, turn_id, transcript_path, cwd, model, permission_mode, stop_hook_active, last_assistant_message).
- `stop.command.output.schema.json` -- Schema for Stop hook output (continue, decision, reason).

### Subfolders

- `generated/` -- Auto-generated JSON Schema files from Rust types.

### What it plugs into

- Schema files are loaded by `src/engine/schema_loader.rs` at startup
- Generated fixtures are validated in unit tests to ensure Rust types match the schema files
