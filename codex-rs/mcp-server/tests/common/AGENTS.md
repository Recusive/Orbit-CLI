# codex-rs/mcp-server/tests/common/

This file applies to `codex-rs/mcp-server/tests/common/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `mcp_test_support` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p mcp_test_support`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Shared test utilities for MCP server integration tests. This is a separate helper crate (`mcp_test_support`) used by the test suite.

### What this folder does

Provides reusable infrastructure for spawning the MCP server process, sending/receiving JSON-RPC messages, and creating mock model server responses.

### Key files

| File | Purpose |
|------|---------|
| `lib.rs` | Re-exports all helpers: `McpProcess`, `create_mock_responses_server`, response helpers, `to_response()` generic deserializer, and shell formatting utilities from `core_test_support` |
| `mcp_process.rs` | `McpProcess`: spawns `codex-mcp-server` as a child process, manages stdin/stdout communication for sending JSON-RPC messages and reading responses |
| `mock_model_server.rs` | `create_mock_responses_server()`: sets up a wiremock server that simulates the model API, returning SSE-formatted responses |
| `responses.rs` | Helper functions for creating mock SSE responses: `create_shell_command_sse_response()`, `create_apply_patch_sse_response()`, `create_final_assistant_message_sse_response()` |

### Where it plugs in

- **Consumed by**: `tests/suite/` test modules
- **Depends on**: `rmcp` (JSON-RPC types), `core_test_support`, `wiremock`, `serde_json`
