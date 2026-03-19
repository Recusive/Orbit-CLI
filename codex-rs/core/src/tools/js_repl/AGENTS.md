# codex-rs/core/src/tools/js_repl/

This file applies to `codex-rs/core/src/tools/js_repl/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-core` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-core`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

JavaScript REPL tool with persistent kernel and code analysis.

### What this folder does

Provides a JavaScript REPL (Read-Eval-Print Loop) tool that the AI agent can use to execute JavaScript code interactively. Unlike shell commands, the REPL maintains state between executions within a turn.

Key components:
- **Kernel management** (`mod.rs`): Manages the Node.js REPL kernel lifecycle -- spawning, restarting, and per-turn isolation.
- **Code execution**: Sends JavaScript code to the kernel, captures output (including images via base64), and returns results.
- **Parser** (`meriyah.umd.min.js`): Bundled JavaScript parser for static analysis of code snippets.
- **Kernel script** (`kernel.js`): The Node.js REPL kernel that executes code and communicates results.

### Key files

| File | Purpose |
|------|---------|
| `mod.rs` | REPL manager, kernel lifecycle, tool spec definitions |
| `mod_tests.rs` | Tests for REPL execution |
| `kernel.js` | Node.js REPL kernel script |
| `meriyah.umd.min.js` | Bundled JavaScript parser (UMD format) |

### Imports from

- `crate::codex` -- `Session`, `TurnContext`
- `crate::tools::context` -- `ToolPayload`, `FunctionToolOutput`
- `crate::exec` -- Execution infrastructure

### Exports to

- `crate::tools::handlers::js_repl` -- `JsReplHandler`, `JsReplResetHandler` handlers
- `crate::state::SessionServices` -- REPL state persisted across turns
