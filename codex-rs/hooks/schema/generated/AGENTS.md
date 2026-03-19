# codex-rs/hooks/schema/generated/

This file applies to `codex-rs/hooks/schema/generated/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Treat this directory as golden data or generated/static support material. Keep filenames and relative paths stable unless the owning test, renderer, or generator changes first.
- Prefer updating the producer or the corresponding test scenario before editing files here by hand.
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-hooks` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.
- Do not hand-edit generated protocol or model artifacts unless the task is specifically about the generated output. Regenerate them from the owning schema/export command.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-hooks`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Auto-generated JSON Schema fixtures for hook wire formats.

### What this folder does

Contains JSON Schema files that are automatically generated from the Rust type definitions in `src/schema.rs` using `schemars`. These files are written by the `write_hooks_schema_fixtures` binary and are validated in unit tests to ensure the Rust structs and the schema files stay in sync.

### Key files and their roles

- `session-start.command.input.schema.json` -- Generated schema for `SessionStartCommandInput`
- `session-start.command.output.schema.json` -- Generated schema for `SessionStartCommandOutputWire`
- `user-prompt-submit.command.input.schema.json` -- Generated schema for `UserPromptSubmitCommandInput`
- `user-prompt-submit.command.output.schema.json` -- Generated schema for `UserPromptSubmitCommandOutputWire`
- `stop.command.input.schema.json` -- Generated schema for `StopCommandInput`
- `stop.command.output.schema.json` -- Generated schema for `StopCommandOutputWire`

### How to regenerate

Run the `write_hooks_schema_fixtures` binary:
```bash
cargo run --bin write_hooks_schema_fixtures -- [optional-schema-root-path]
```
If no path is given, it defaults to the `schema/` directory in the crate root.
