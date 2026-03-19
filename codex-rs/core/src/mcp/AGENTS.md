# codex-rs/core/src/mcp/

This file applies to `codex-rs/core/src/mcp/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-core` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-core`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

MCP (Model Context Protocol) server management, tool collection, and authentication.

### What this folder does

Manages the lifecycle and integration of MCP servers that provide external tools to the Codex agent. This is the high-level orchestration layer that sits above the lower-level `mcp_connection_manager.rs`.

Key responsibilities:
- **Server configuration**: Builds the effective set of MCP servers from config, plugins, and the built-in Codex Apps server.
- **Codex Apps integration**: Special handling for the `codex_apps` MCP server (the apps/connectors system), including URL construction, bearer token management, and HTTP header injection.
- **Tool collection**: `collect_mcp_snapshot()` connects to all configured MCP servers and gathers their tools, resources, and resource templates.
- **Tool naming**: Qualified tool names use the format `mcp__<server>__<tool>` with `split_qualified_tool_name()` and `group_tools_by_server()` utilities.
- **Auth status**: Computes authentication statuses for MCP servers with OAuth requirements.
- **Plugin provenance**: Tracks which plugins provide which MCP servers/connectors.
- **Skill dependencies**: `maybe_prompt_and_install_mcp_dependencies()` handles installing required packages for MCP servers.

### Key files

| File | Purpose |
|------|---------|
| `mod.rs` | `McpManager` struct, server configuration, tool collection, qualified name utilities |
| `mod_tests.rs` | Tests for MCP management |
| `auth.rs` | MCP server authentication status computation |
| `skill_dependencies.rs` | Dependency installation prompting for MCP servers |
| `skill_dependencies_tests.rs` | Tests for dependency installation |

### Imports from

- `crate::config` -- `Config`, `McpServerConfig`, `McpServerTransportConfig`
- `crate::mcp_connection_manager` -- `McpConnectionManager` for actual server connections
- `crate::plugins` -- `PluginsManager`, `PluginCapabilitySummary`
- `crate::auth` -- `AuthManager`, `CodexAuth`
- `codex_protocol::mcp` -- `Tool`, `Resource`, `ResourceTemplate`

### Exports to

- `crate::codex` -- `McpManager` is held in session services
- `crate::state` -- MCP connection manager stored in `SessionServices`
- `crate::tools::handlers::mcp` -- MCP tool handler uses connection manager
- Public API for `codex-app-server` MCP tool listing
