# codex-rs/utils/json-to-toml/

Crate `codex-utils-json-to-toml` -- convert JSON values to TOML values.

## What this folder does

Provides a single recursive conversion function that transforms a `serde_json::Value` tree into a semantically equivalent `toml::Value` tree. Used for configuration merging where JSON-sourced overrides need to be applied to TOML configuration.

## Key types and functions

- `json_to_toml(v: JsonValue) -> TomlValue` -- recursive conversion mapping: Null -> empty string, Bool -> Bool, Number -> Integer/Float, String -> String, Array -> Array, Object -> Table

## Imports from

- `serde_json` -- source value type
- `toml` -- target value type

## Exports to

Used by `codex-config` for merging JSON-based configuration overrides into the TOML config tree.

## Key files

- `Cargo.toml` -- crate metadata; depends on `serde_json` and `toml`
- `src/lib.rs` -- `json_to_toml` function and tests covering all JSON value types
