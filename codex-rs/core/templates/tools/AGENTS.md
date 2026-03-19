# codex-rs/core/templates/tools/

This file applies to `codex-rs/core/templates/tools/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-core` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-core`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Tool-specific prompt templates.

### What this folder does

Contains prompt templates for specific tools that require detailed instructions beyond simple descriptions.

### Key files

| File | Purpose |
|------|---------|
| `presentation_artifact.md` | Template for the presentation artifact tool, defining the format and structure for generated presentation artifacts |

### Where it plugs into

- Loaded via `include_str!()` in `crate::tools::handlers::artifacts`
- Provides the detailed instructions for artifact creation tools
