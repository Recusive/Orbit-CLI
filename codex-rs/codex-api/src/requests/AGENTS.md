# codex-rs/codex-api/src/requests/

This file applies to `codex-rs/codex-api/src/requests/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-api` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-api`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Request construction and header building for API calls.

### What this folder does

Contains helpers for constructing HTTP request headers and building Responses API request payloads.

### Key files

| File | Role |
|------|------|
| `mod.rs` | Module declarations |
| `headers.rs` | `build_conversation_headers` -- constructs HTTP headers for API requests (auth, content-type, tracing) |
| `responses.rs` | Responses API request body construction helpers |
