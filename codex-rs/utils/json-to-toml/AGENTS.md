# codex-rs/utils/json-to-toml/

This file applies to `codex-rs/utils/json-to-toml/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-utils-json-to-toml` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-utils-json-to-toml`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Crate `codex-utils-json-to-toml` -- convert JSON values to TOML values.

### What this folder does

Provides a single recursive conversion function that transforms a `serde_json::Value` tree into a semantically equivalent `toml::Value` tree. Used for configuration merging where JSON-sourced overrides need to be applied to TOML configuration.

### Key types and functions

- `json_to_toml(v: JsonValue) -> TomlValue` -- recursive conversion mapping: Null -> empty string, Bool -> Bool, Number -> Integer/Float, String -> String, Array -> Array, Object -> Table

### Imports from

- `serde_json` -- source value type
- `toml` -- target value type

### Exports to

Used by `codex-config` for merging JSON-based configuration overrides into the TOML config tree.

### Key files

- `Cargo.toml` -- crate metadata; depends on `serde_json` and `toml`
- `src/lib.rs` -- `json_to_toml` function and tests covering all JSON value types
