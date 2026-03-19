# sdk/

This file applies to `sdk/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Read the files listed below before changing behavior in this subtree; keep neighboring docs and call sites consistent with any structural change.

## Validate
- Run the nearest package or crate tests that exercise this subtree.

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Top-level directory containing official SDK packages for embedding the Codex agent into external workflows and applications.

### Purpose

Provides language-specific SDK wrappers around the `codex` CLI and `codex app-server` JSON-RPC protocol. Each sub-package spawns the Codex CLI binary and communicates over stdin/stdout (JSONL for TypeScript, JSON-RPC v2 for Python).

### Contents

| Directory | Package | Language | Protocol |
|-----------|---------|----------|----------|
| `python/` | `codex-app-server-sdk` | Python >=3.10 | JSON-RPC v2 over stdio |
| `python-runtime/` | `codex-cli-bin` | Python >=3.10 | N/A (binary distribution) |
| `typescript/` | `@openai/codex-sdk` | TypeScript/Node 18+ | JSONL over stdio |

### Architecture

All SDKs follow the same pattern:
1. Locate or resolve the `codex` CLI binary (platform-specific)
2. Spawn `codex exec` (TypeScript) or `codex app-server` (Python) as a child process
3. Exchange structured messages over stdin/stdout
4. Expose typed Thread/Turn abstractions to the consumer

### Relationship to Other Modules

- Depends on the Rust CLI binary built from `codex-rs/` (the `codex` executable)
- The Python SDK generated types come from the JSON schema at `codex-rs/app-server-protocol/schema/json/`
- The TypeScript SDK event types mirror `codex-rs/exec/src/exec_events.rs`
