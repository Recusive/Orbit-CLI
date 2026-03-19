# codex-rs/rmcp-client/tests/

This file applies to `codex-rs/rmcp-client/tests/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-rmcp-client` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-rmcp-client`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Integration tests for the `codex-rmcp-client` crate.

### What this folder does

End-to-end tests that spawn real MCP server processes and verify client behavior including process lifecycle, resource operations, and session recovery.

### Key files

- `process_group_cleanup.rs` -- Unix-only test verifying that dropping an `RmcpClient` terminates the entire process group of the spawned MCP server (including grandchild processes).
- `resources.rs` -- tests `list_resources`, `list_resource_templates`, and `read_resource` against the `test_stdio_server` binary.
- `streamable_http_recovery.rs` -- tests session recovery behavior for streamable HTTP transport: verifies that a 404 (session expired) triggers automatic re-initialization with a single retry, that 401/500 errors do not trigger recovery, and that a double-404 fails but the client can recover on the next call.

### What it plugs into

- Depends on binaries in `src/bin/` (`test_stdio_server`, `test_streamable_http_server`).
- Uses `codex-utils-cargo-bin` to locate compiled test binaries.

### Imports from

- `codex-rmcp-client` (the library under test).
- `rmcp` model types for assertions.
- `codex-utils-cargo-bin`, `futures`, `reqwest`, `tokio`.
