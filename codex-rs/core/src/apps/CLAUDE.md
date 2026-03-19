# codex-rs/core/src/apps/

Renders instruction text for the Apps (Connectors) feature that integrates external services via MCP.

## What this folder does

Provides a single function `render_apps_section()` that generates the system prompt section describing how the AI agent should interact with installed apps/connectors. Apps are backed by MCP tool servers and can be triggered explicitly via `[$app-name](app://connector_id)` syntax or implicitly through context.

## Key files

| File | Purpose |
|------|---------|
| `mod.rs` | Module declaration, re-exports `render_apps_section` |
| `render.rs` | `render_apps_section()` -- generates the XML-tagged apps instruction block |

## Imports from

- `crate::mcp::CODEX_APPS_MCP_SERVER_NAME` -- the MCP server name for apps
- `codex_protocol::protocol` -- XML tag constants for apps instructions

## Exports to

- `crate::codex` -- called during system prompt construction to include apps documentation
