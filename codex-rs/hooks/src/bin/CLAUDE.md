# codex-rs/hooks/src/bin/

Binary utilities for the hooks crate.

## What this folder does

Contains a utility binary for regenerating the JSON Schema fixture files from Rust type definitions.

## Key files and their roles

- `write_hooks_schema_fixtures.rs` -- Standalone binary that calls `codex_hooks::write_schema_fixtures()`. Accepts an optional schema root path argument (defaults to `<crate>/schema/`). Generates all six JSON Schema files (input + output for SessionStart, UserPromptSubmit, Stop) into the `generated/` subdirectory.

## How to use

```bash
cargo run --bin write_hooks_schema_fixtures
# or with a custom output directory:
cargo run --bin write_hooks_schema_fixtures -- /path/to/schema
```
