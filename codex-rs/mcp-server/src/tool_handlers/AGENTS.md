# codex-rs/mcp-server/src/tool_handlers/

This file applies to `codex-rs/mcp-server/src/tool_handlers/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-mcp-server` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-mcp-server`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Modular tool call handler implementations for the MCP server.

### What this folder does

Contains individual handler modules for MCP tool calls. Each module implements the logic for a specific tool exposed by the Codex MCP server.

### Key files

| File | Purpose |
|------|---------|
| `mod.rs` | Module declarations; re-exports `create_conversation` and `send_message` submodules as `pub(crate)` |
| `create_conversation.rs` | Handler for creating a new Codex conversation/thread |
| `send_message.rs` | Handler for sending a message to an existing conversation |

### Where it plugs in

- **Called by**: `message_processor.rs` when dispatching `tools/call` requests
- **Uses**: `codex-core` thread management, `codex-protocol` operations
