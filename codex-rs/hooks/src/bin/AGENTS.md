# codex-rs/hooks/src/bin/

This file applies to `codex-rs/hooks/src/bin/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-hooks` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-hooks`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Binary utilities for the hooks crate.

### What this folder does

Contains a utility binary for regenerating the JSON Schema fixture files from Rust type definitions.

### Key files and their roles

- `write_hooks_schema_fixtures.rs` -- Standalone binary that calls `codex_hooks::write_schema_fixtures()`. Accepts an optional schema root path argument (defaults to `<crate>/schema/`). Generates all six JSON Schema files (input + output for SessionStart, UserPromptSubmit, Stop) into the `generated/` subdirectory.

### How to use

```bash
cargo run --bin write_hooks_schema_fixtures
# or with a custom output directory:
cargo run --bin write_hooks_schema_fixtures -- /path/to/schema
```
