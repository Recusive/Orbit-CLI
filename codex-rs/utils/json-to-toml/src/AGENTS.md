# codex-rs/utils/json-to-toml/src/

This file applies to `codex-rs/utils/json-to-toml/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-utils-json-to-toml` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-utils-json-to-toml`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source directory for the `codex-utils-json-to-toml` crate.

### Key files

- `lib.rs` -- single-file implementation containing:
  - `json_to_toml(v: JsonValue) -> TomlValue` -- recursive match converting each JSON variant:
    - `Null` -> `TomlValue::String("")`
    - `Bool(b)` -> `TomlValue::Boolean(b)`
    - `Number(n)` -> `TomlValue::Integer` (if i64) or `TomlValue::Float` (if f64) or `TomlValue::String` (fallback)
    - `String(s)` -> `TomlValue::String(s)`
    - `Array(arr)` -> `TomlValue::Array` (recursive)
    - `Object(map)` -> `TomlValue::Table` (recursive)
  - Tests for numbers, arrays, booleans, floats, null, and nested objects
