# codex-rs/core/src/tools/

This file applies to `codex-rs/core/src/tools/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-core` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-core`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Tool registry, routing, execution, sandboxing, and output formatting for the Codex agent.

### What this folder does

This is the tool system -- the layer between the AI model's tool calls and actual execution. It handles tool registration, routing, approval workflows, sandbox enforcement, and output formatting.

#### Architecture

```
Model tool call
  -> ToolRouter (registry + dispatch)
    -> ToolHandler (parse args, validate)
      -> ToolOrchestrator (approval + sandbox + retry)
        -> ToolRuntime (platform-specific execution)
          -> SandboxManager (command transformation)
            -> exec (process spawning)
```

#### Key subsystems

- **Registry** (`registry.rs`): Registers all available tools with their specs (name, description, schema).
- **Router** (`router.rs`): `ToolRouter` dispatches incoming tool calls to the correct handler based on tool name.
- **Orchestrator** (`orchestrator.rs`): Central approval + sandbox selection + retry logic. Handles the full flow: approval (bypass/cache/prompt) -> select sandbox -> attempt -> retry on denial.
- **Sandboxing** (`sandboxing.rs`): Shared approval and sandbox traits (`ApprovalCtx`, `Approvable`, `Sandboxable`, `ToolRuntime`, `SandboxAttempt`).
- **Handlers** (`handlers/`): Concrete handler implementations for each tool.
- **Runtimes** (`runtimes/`): Platform-specific execution backends.
- **Parallel** (`parallel.rs`): Parallel tool call execution.
- **Spec** (`spec.rs`): Tool specification parsing and schema validation.
- **Code mode** (`code_mode/`): Interactive code execution mode.
- **JS REPL** (`js_repl/`): JavaScript REPL tool.

#### Output formatting
- `format_exec_output_for_model_structured()` -- JSON output with metadata
- `format_exec_output_for_model_freeform()` -- Freeform text output with exit code and timing

### Key files

| File | Purpose |
|------|---------|
| `mod.rs` | Output formatting functions, telemetry constants |
| `router.rs` | `ToolRouter` -- tool call dispatch |
| `registry.rs` | Tool registration and spec management |
| `orchestrator.rs` | `ToolOrchestrator` -- approval + sandbox + retry |
| `sandboxing.rs` | Shared approval/sandbox traits and utilities |
| `parallel.rs` | Parallel tool execution |
| `spec.rs` | Tool specification parsing |
| `context.rs` | Tool execution context types |
| `events.rs` | Tool event emission |
| `discoverable.rs` | Discoverable tools for tool_search/tool_suggest |
| `network_approval.rs` | Network access approval flow |
| `code_mode_description.rs` | Code mode tool description augmentation |

### Imports from

- `crate::codex` -- `Session`, `TurnContext`
- `crate::sandboxing` -- `SandboxManager`, `CommandSpec`, `ExecRequest`
- `crate::exec` -- `ExecToolCallOutput`
- `crate::truncate` -- Output truncation for model consumption

### Exports to

- `crate::codex` -- `ToolRouter` used during turn execution
- `crate::tasks::RegularTask` -- tool execution during regular turns
- Public: `ToolRouter`, `parse_tool_input_schema`
