# codex-rs/chatgpt/src/

This file applies to `codex-rs/chatgpt/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-chatgpt` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-chatgpt`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source directory for the `codex-chatgpt` crate.

### What this folder does

Contains the ChatGPT backend API integration code: HTTP client with token management, task fetching, diff application, and connector (app) listing/merging logic.

### Where it plugs in

- `lib.rs` declares modules and controls public visibility
- `chatgpt_client.rs` and `chatgpt_token.rs` are internal helpers used by the public modules
- `connectors.rs`, `get_task.rs`, and `apply_command.rs` are the public-facing modules

### Imports from

- `codex_core` -- `Config`, `AuthManager`, `create_client`, `TokenData`, connectors, plugins
- `codex_connectors` -- `AllConnectorsCacheKey`, `DirectoryListResponse`, caching functions
- `codex_git` -- `ApplyGitRequest`, `apply_git_patch`
- `codex_utils_cli` -- `CliConfigOverrides`
- `clap` -- CLI derive macros

### Exports to

All public items re-exported through `lib.rs`.

### Key files

| File | Role |
|------|------|
| `lib.rs` | Module declarations; `apply_command` and `connectors` are `pub`, `get_task` is `pub` |
| `chatgpt_client.rs` | `chatgpt_get_request` / `chatgpt_get_request_with_timeout` -- authenticated GET requests to the ChatGPT backend with bearer token and account ID headers |
| `chatgpt_token.rs` | Global `CHATGPT_TOKEN` static via `LazyLock<RwLock<Option<TokenData>>>`; `init_chatgpt_token_from_auth` loads token from auth manager |
| `connectors.rs` | Lists all/cached/accessible connectors from ChatGPT directory API and MCP tools; merges, filters disallowed, and annotates with plugin app state |
| `get_task.rs` | `get_task` fetches a task by ID from `/wham/tasks/{id}`; response types for extracting PR diffs |
| `apply_command.rs` | `ApplyCommand` clap struct for the `apply` subcommand; `apply_diff_from_task` extracts diff from task response and applies via `codex_git::apply_git_patch` |
