# codex-rs/hooks/schema/generated/

Auto-generated JSON Schema fixtures for hook wire formats.

## What this folder does

Contains JSON Schema files that are automatically generated from the Rust type definitions in `src/schema.rs` using `schemars`. These files are written by the `write_hooks_schema_fixtures` binary and are validated in unit tests to ensure the Rust structs and the schema files stay in sync.

## Key files and their roles

- `session-start.command.input.schema.json` -- Generated schema for `SessionStartCommandInput`
- `session-start.command.output.schema.json` -- Generated schema for `SessionStartCommandOutputWire`
- `user-prompt-submit.command.input.schema.json` -- Generated schema for `UserPromptSubmitCommandInput`
- `user-prompt-submit.command.output.schema.json` -- Generated schema for `UserPromptSubmitCommandOutputWire`
- `stop.command.input.schema.json` -- Generated schema for `StopCommandInput`
- `stop.command.output.schema.json` -- Generated schema for `StopCommandOutputWire`

## How to regenerate

Run the `write_hooks_schema_fixtures` binary:
```bash
cargo run --bin write_hooks_schema_fixtures -- [optional-schema-root-path]
```
If no path is given, it defaults to the `schema/` directory in the crate root.
