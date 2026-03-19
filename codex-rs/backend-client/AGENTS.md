# codex-rs/backend-client/

This file applies to `codex-rs/backend-client/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-backend-client` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-backend-client`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

HTTP client for the Codex backend API (both Codex API and ChatGPT backend-api styles).

### What this folder does

Provides a `Client` that communicates with the Codex backend server for task management, rate limit queries, configuration retrieval, and sibling turn lookups. Supports two URL path styles: `/api/codex/...` (Codex API) and `/wham/...` (ChatGPT backend-api), auto-detecting based on the base URL.

### Where it plugs in

- Consumed by the CLI and cloud-tasks crates for interacting with the Codex backend
- Depends on `codex-backend-openapi-models` for generated OpenAPI model types
- Depends on `codex-client` for the custom-CA-aware reqwest client builder
- Depends on `codex-core` for authentication (`CodexAuth`) and user agent
- Depends on `codex-protocol` for protocol types (`RateLimitSnapshot`, `CreditsSnapshot`, `AccountPlanType`)

### Imports from

- `codex-backend-openapi-models` -- generated model structs (re-exported through `types.rs`)
- `codex-client` -- `build_reqwest_client_with_custom_ca`
- `codex-core` -- `CodexAuth`, `get_codex_user_agent`
- `codex-protocol` -- `AccountPlanType`, `CreditsSnapshot`, `RateLimitSnapshot`, `RateLimitWindow`
- `reqwest` -- HTTP client with JSON and rustls-tls
- `serde` / `serde_json` -- JSON deserialization
- `anyhow` -- error handling

### Exports to

Public API from `lib.rs`:

- `Client` -- the HTTP client with methods: `get_rate_limits`, `list_tasks`, `get_task_details`, `list_sibling_turns`, `get_config_requirements_file`, `create_task`
- `RequestError` -- structured error type with status code access and `is_unauthorized()` check
- `CodeTaskDetailsResponse` / `CodeTaskDetailsResponseExt` -- task details with helper trait for extracting diffs, messages, prompts, and errors
- `ConfigFileResponse` -- backend config file response
- `PaginatedListTaskListItem` / `TaskListItem` -- task listing types
- `TurnAttemptsSiblingTurnsResponse` -- sibling turns response

### Key files

| File | Role |
|------|------|
| `Cargo.toml` | Crate manifest; depends on `codex-backend-openapi-models`, `codex-client`, `codex-core`, `codex-protocol` |
| `src/lib.rs` | Module declarations and public re-exports |
| `src/client.rs` | `Client` struct with HTTP methods for all backend endpoints; `PathStyle` enum for URL routing; rate limit mapping helpers |
| `src/types.rs` | Hand-rolled task detail models (`CodeTaskDetailsResponse`, `Turn`, `TurnItem`, `ContentFragment`, etc.) with the `CodeTaskDetailsResponseExt` trait; re-exports generated OpenAPI models |
| `tests/` | Test data directory |
