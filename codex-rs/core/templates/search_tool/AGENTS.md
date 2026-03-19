# codex-rs/core/templates/search_tool/

This file applies to `codex-rs/core/templates/search_tool/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-core` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-core`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Tool search and tool suggest description templates.

### What this folder does

Provides the description templates for the `tool_search` and `tool_suggest` meta-tools that help the agent discover available tools.

### Key files

| File | Purpose |
|------|---------|
| `tool_description.md` | Description template for the `tool_search` tool -- explains how to search for tools by keyword or description |
| `tool_suggest_description.md` | Description template for the `tool_suggest` tool -- explains how to get tool suggestions based on context |

### Where it plugs into

- Loaded via `include_str!()` in `crate::tools::handlers::tool_search` and `crate::tools::handlers::tool_suggest`
- Defines the tool descriptions shown in the tool registry
