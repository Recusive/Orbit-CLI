# app-server-protocol/src/bin

## Purpose

Contains the binary entry point for the schema fixture generation tool.

## Key Files

| File | Role |
|------|------|
| `write_schema_fixtures.rs` | CLI binary (`write-schema-fixtures`) that regenerates all schema fixtures under the `schema/` directory. Uses `clap` for argument parsing. Accepts `--schema-root` (defaults to `schema/` relative to the crate), `--prettier` (optional Prettier binary path for formatting TypeScript), and `--experimental` (include experimental API types). Delegates to `codex_app_server_protocol::write_schema_fixtures_with_options()`. |

## What It Plugs Into

- Invoked manually or via `just write-app-server-schema` to regenerate `schema/json/` and `schema/typescript/` fixtures.
- The generated fixtures are committed to version control and validated by `tests/schema_fixtures.rs`.

## Imports From

- `codex_app_server_protocol` -- `write_schema_fixtures_with_options()`, `SchemaFixtureOptions`.
- `anyhow` -- Error handling.
- `clap` -- CLI argument parsing.

## Exports To

- Produces files in `schema/json/` and `schema/typescript/`. No library exports.
