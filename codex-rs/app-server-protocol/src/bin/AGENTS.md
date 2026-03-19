# codex-rs/app-server-protocol/src/bin/

This file applies to `codex-rs/app-server-protocol/src/bin/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-app-server-protocol` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-app-server-protocol`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just write-app-server-schema`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

### Purpose

Contains the binary entry point for the schema fixture generation tool.

### Key Files

| File | Role |
|------|------|
| `write_schema_fixtures.rs` | CLI binary (`write-schema-fixtures`) that regenerates all schema fixtures under the `schema/` directory. Uses `clap` for argument parsing. Accepts `--schema-root` (defaults to `schema/` relative to the crate), `--prettier` (optional Prettier binary path for formatting TypeScript), and `--experimental` (include experimental API types). Delegates to `codex_app_server_protocol::write_schema_fixtures_with_options()`. |

### What It Plugs Into

- Invoked manually or via `just write-app-server-schema` to regenerate `schema/json/` and `schema/typescript/` fixtures.
- The generated fixtures are committed to version control and validated by `tests/schema_fixtures.rs`.

### Imports From

- `codex_app_server_protocol` -- `write_schema_fixtures_with_options()`, `SchemaFixtureOptions`.
- `anyhow` -- Error handling.
- `clap` -- CLI argument parsing.

### Exports To

- Produces files in `schema/json/` and `schema/typescript/`. No library exports.
