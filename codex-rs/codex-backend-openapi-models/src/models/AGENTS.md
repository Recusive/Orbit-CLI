# codex-rs/codex-backend-openapi-models/src/models/

This file applies to `codex-rs/codex-backend-openapi-models/src/models/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-backend-openapi-models` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-backend-openapi-models`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Auto-generated OpenAPI model structs.

### What this folder does

Contains individual Rust files, each defining a single API model type generated from the Codex backend OpenAPI specification. These files are machine-generated and should not be hand-edited.

### Key files

| File | Role |
|------|------|
| `mod.rs` | Re-exports all model types |
| `rate_limit_status_payload.rs` | `RateLimitStatusPayload` -- top-level usage/rate-limit response |
| `rate_limit_status_details.rs` | `RateLimitStatusDetails` -- per-limit rate details |
| `rate_limit_window_snapshot.rs` | `RateLimitWindowSnapshot` -- windowed rate limit snapshot |
| `credit_status_details.rs` | `CreditStatusDetails` -- credit balance and status |
| `additional_rate_limit_details.rs` | `AdditionalRateLimitDetails` -- extra rate limit entries |
| `paginated_list_task_list_item_.rs` | `PaginatedListTaskListItem` -- paginated task list |
| `task_list_item.rs` | `TaskListItem` -- individual task in a list |
| `code_task_details_response.rs` | `CodeTaskDetailsResponse` -- full task details |
| `task_response.rs` | `TaskResponse` -- task creation response |
| `config_file_response.rs` | `ConfigFileResponse` -- backend config file |
| `external_pull_request_response.rs` | `ExternalPullRequestResponse` -- PR info |
| `git_pull_request.rs` | `GitPullRequest` -- git PR details |
