# codex-rs/core/src/packages/

This file applies to `codex-rs/core/src/packages/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-core` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-core`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Pinned package versions for runtime dependencies.

### What this folder does

Provides version constants for packages that are installed via package managers at runtime. This ensures consistent, reproducible installations across different environments.

### Key files

| File | Purpose |
|------|---------|
| `mod.rs` | Module declaration for `versions` |
| `versions.rs` | `ARTIFACT_RUNTIME` constant (currently `"2.5.6"`) -- pinned version for the artifact runtime package |

### Imports from

None (leaf module).

### Exports to

- `crate::mcp::skill_dependencies` -- uses version constants when installing MCP server dependencies
- `crate::tools::js_repl` -- may reference runtime versions for REPL setup
