# codex-rs/utils/json-to-toml/src/

Source directory for the `codex-utils-json-to-toml` crate.

## Key files

- `lib.rs` -- single-file implementation containing:
  - `json_to_toml(v: JsonValue) -> TomlValue` -- recursive match converting each JSON variant:
    - `Null` -> `TomlValue::String("")`
    - `Bool(b)` -> `TomlValue::Boolean(b)`
    - `Number(n)` -> `TomlValue::Integer` (if i64) or `TomlValue::Float` (if f64) or `TomlValue::String` (fallback)
    - `String(s)` -> `TomlValue::String(s)`
    - `Array(arr)` -> `TomlValue::Array` (recursive)
    - `Object(map)` -> `TomlValue::Table` (recursive)
  - Tests for numbers, arrays, booleans, floats, null, and nested objects
