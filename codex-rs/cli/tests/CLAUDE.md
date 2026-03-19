# codex-rs/cli/tests/

Integration tests for the `codex-cli` binary.

## What this folder does

Contains integration tests that exercise the `codex` binary via `assert_cmd` and verify various subcommand behaviors including feature flag listing, MCP server management, exec policy checking, and debug memory clearing.

## Where it plugs in

- Tests run the `codex` binary as an external process via `assert_cmd`
- Use `codex-utils-cargo-bin` to locate the compiled binary
- Some tests use `sqlx` and `codex-state` for database verification

## Key files

| File | Role |
|------|------|
| `features.rs` | Tests `codex features list` output and `codex features enable/disable` for known and unknown feature keys |
| `mcp_add_remove.rs` | Tests `codex mcp add` and `codex mcp remove` subcommands |
| `mcp_list.rs` | Tests `codex mcp list` output format |
| `execpolicy.rs` | Tests `codex execpolicy check` against policy files |
| `debug_clear_memories.rs` | Tests `codex debug clear-memories` clears state DB and memory directory |
