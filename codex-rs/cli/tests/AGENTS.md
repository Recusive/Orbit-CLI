# codex-rs/cli/tests/

This file applies to `codex-rs/cli/tests/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-cli` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-cli`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Integration tests for the `codex-cli` binary.

### What this folder does

Contains integration tests that exercise the `codex` binary via `assert_cmd` and verify various subcommand behaviors including feature flag listing, MCP server management, exec policy checking, and debug memory clearing.

### Where it plugs in

- Tests run the `codex` binary as an external process via `assert_cmd`
- Use `codex-utils-cargo-bin` to locate the compiled binary
- Some tests use `sqlx` and `codex-state` for database verification

### Key files

| File | Role |
|------|------|
| `features.rs` | Tests `codex features list` output and `codex features enable/disable` for known and unknown feature keys |
| `mcp_add_remove.rs` | Tests `codex mcp add` and `codex mcp remove` subcommands |
| `mcp_list.rs` | Tests `codex mcp list` output format |
| `execpolicy.rs` | Tests `codex execpolicy check` against policy files |
| `debug_clear_memories.rs` | Tests `codex debug clear-memories` clears state DB and memory directory |
