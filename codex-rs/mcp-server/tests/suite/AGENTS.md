# codex-rs/mcp-server/tests/suite/

This file applies to `codex-rs/mcp-server/tests/suite/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-mcp-server` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-mcp-server`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Test module directory aggregated by `tests/all.rs`.

### What this folder does

Contains integration test modules that exercise the MCP server end-to-end by spawning the server process and sending JSON-RPC messages.

### Key files

| File | What it tests |
|------|---------------|
| `mod.rs` | Aggregates `codex_tool` module |
| `codex_tool.rs` | Integration tests for the `codex` MCP tool: verifies tool invocation, session creation, event streaming, tool response format with `threadId` and `content` in structured content, and interaction with mock model servers |

### Test patterns

- Tests spawn the MCP server via `McpProcess` from the `common/` crate
- A wiremock mock server simulates the model API with SSE responses
- JSON-RPC messages are sent via stdin and responses are read from stdout
- Tests validate both the MCP protocol compliance and the Codex-specific structured content format
