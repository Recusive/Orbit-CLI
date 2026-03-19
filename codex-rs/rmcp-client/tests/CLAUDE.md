# codex-rs/rmcp-client/tests/

Integration tests for the `codex-rmcp-client` crate.

## What this folder does

End-to-end tests that spawn real MCP server processes and verify client behavior including process lifecycle, resource operations, and session recovery.

## Key files

- `process_group_cleanup.rs` -- Unix-only test verifying that dropping an `RmcpClient` terminates the entire process group of the spawned MCP server (including grandchild processes).
- `resources.rs` -- tests `list_resources`, `list_resource_templates`, and `read_resource` against the `test_stdio_server` binary.
- `streamable_http_recovery.rs` -- tests session recovery behavior for streamable HTTP transport: verifies that a 404 (session expired) triggers automatic re-initialization with a single retry, that 401/500 errors do not trigger recovery, and that a double-404 fails but the client can recover on the next call.

## What it plugs into

- Depends on binaries in `src/bin/` (`test_stdio_server`, `test_streamable_http_server`).
- Uses `codex-utils-cargo-bin` to locate compiled test binaries.

## Imports from

- `codex-rmcp-client` (the library under test).
- `rmcp` model types for assertions.
- `codex-utils-cargo-bin`, `futures`, `reqwest`, `tokio`.
