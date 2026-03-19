# sdk/typescript/src/

TypeScript source for the `@openai/codex-sdk` package.

## Purpose

Contains all source modules for the TypeScript SDK. Built with `tsup` into `dist/` as ESM with declaration files.

## Key Files

| File | Role |
|------|------|
| `index.ts` | Package entry; re-exports all public types and the `Codex` and `Thread` classes |
| `codex.ts` | `Codex` class -- creates `Thread` instances; holds `CodexExec` and `CodexOptions` |
| `codexOptions.ts` | `CodexOptions` type definition (binary path override, API key, base URL, env, config overrides) |
| `thread.ts` | `Thread` class -- `run()` buffers events into a `Turn`; `runStreamed()` yields `ThreadEvent` via async generator |
| `threadOptions.ts` | Type definitions for thread-level options (model, sandbox, approval policy, web search, etc.) |
| `turnOptions.ts` | Type definitions for per-turn options (output schema, abort signal) |
| `exec.ts` | `CodexExec` class -- spawns `codex exec --experimental-json`, builds CLI args, yields stdout lines; resolves platform binary; serializes config overrides as TOML |
| `events.ts` | JSONL event types matching `codex-rs/exec/src/exec_events.rs` |
| `items.ts` | Thread item union types (`AgentMessageItem`, `CommandExecutionItem`, `FileChangeItem`, `McpToolCallItem`, `WebSearchItem`, etc.) |
| `outputSchemaFile.ts` | `createOutputSchemaFile()` writes JSON schema to a temp file, returns path and cleanup function |

## Imports From

- `@modelcontextprotocol/sdk/types.js` for `ContentBlock` type (used in `McpToolCallItem`)
- Node.js built-ins: `child_process`, `readline`, `module`, `fs`, `os`, `path`

## Exports To

- `dist/index.js` and `dist/index.d.ts` (built output consumed by package users)
- All public types and classes defined here are the full public API surface of `@openai/codex-sdk`
