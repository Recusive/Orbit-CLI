# codex-rs/rmcp-client/src/bin/

This file applies to `codex-rs/rmcp-client/src/bin/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-rmcp-client` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-rmcp-client`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Test MCP server binaries used by integration tests and manual testing.

### What this folder does

Contains standalone MCP server binaries that the rmcp-client integration tests spawn as child processes. They are not shipped in production builds.

### Key files

- `rmcp_test_server.rs` -- minimal stdio MCP server with a single `echo` tool. Used by the process-group cleanup test.
- `test_stdio_server.rs` -- richer stdio MCP server with `echo`, `echo-tool`, `image`, and `image_scenario` tools, plus MCP resources (`memo://codex/example-note`) and resource templates. Used by resource listing/reading integration tests and manual TUI image-rendering tests.
- `test_streamable_http_server.rs` -- axum-based streamable HTTP MCP server with `echo` tool, resources, OAuth discovery endpoint, bearer token middleware, and a control endpoint (`/test/control/session-post-failure`) that can inject 404/401/500 failures for session-recovery testing.

### What it plugs into

- `tests/process_group_cleanup.rs` spawns `rmcp_test_server`.
- `tests/resources.rs` spawns `test_stdio_server`.
- `tests/streamable_http_recovery.rs` spawns `test_streamable_http_server`.

### Imports from

- `rmcp` SDK server-side handler traits and model types.
- `axum` (streamable HTTP server only).
- `serde_json`, `tokio`.
