# app-server-protocol/tests

## Purpose

Integration tests for the `codex-app-server-protocol` crate, focused on verifying that committed schema fixtures stay in sync with the Rust type definitions.

## Key Files

| File | Role |
|------|------|
| `schema_fixtures.rs` | Schema fixture validation tests. Contains two tests: `typescript_schema_fixtures_match_generated` (compares committed TypeScript fixtures against freshly generated ones) and `json_schema_fixtures_match_generated` (compares committed JSON Schema fixtures against freshly generated ones). When tests fail, the error message instructs the developer to run `just write-app-server-schema` to regenerate fixtures. Uses `similar::TextDiff` for readable diff output on mismatches. |

## What It Plugs Into

- Tests read from the `schema/` directory via `codex-utils-cargo-bin::find_resource!` for path resolution (supports both Cargo and Bazel runfiles).
- Tests use `codex_app_server_protocol::generate_json_with_experimental` and `generate_typescript_schema_fixture_subtree_for_tests` to produce fresh fixtures for comparison.
- Tests use `codex_app_server_protocol::read_schema_fixture_subtree` to read committed fixtures.

## Imports From

- `codex_app_server_protocol` -- Schema generation and fixture reading functions.
- `codex_utils_cargo_bin` -- `find_resource!` macro for locating schema fixtures in Cargo or Bazel environments.
- `similar` -- Text diffing library for readable test failure output.
- `tempfile` -- Temporary directory creation for generated fixture comparison.

## Exports To

- No exports; test-only.
