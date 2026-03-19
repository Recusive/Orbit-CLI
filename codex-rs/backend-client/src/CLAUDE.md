# codex-rs/backend-client/src/

Source directory for the `codex-backend-client` crate.

## What this folder does

Contains the HTTP client implementation and hand-rolled response types for the Codex backend API. The client handles authentication, request construction, JSON decoding, and mapping between backend-specific models and the `codex-protocol` types used by the rest of the CLI.

## Where it plugs in

- `lib.rs` is the crate entry point, re-exporting `Client`, `RequestError`, and the response types
- `client.rs` uses `codex-client` for building a custom-CA-aware reqwest client, `codex-core` for auth, and `codex-protocol` for rate limit / account types
- `types.rs` re-exports generated OpenAPI models from `codex-backend-openapi-models` and defines hand-rolled task detail types

## Imports from

- `codex_backend_openapi_models::models` -- generated structs for rate limits, config, task lists, plan types
- `codex_client` -- `build_reqwest_client_with_custom_ca`
- `codex_core::auth` -- `CodexAuth`
- `codex_core::default_client` -- `get_codex_user_agent`
- `codex_protocol::protocol` -- `CreditsSnapshot`, `RateLimitSnapshot`, `RateLimitWindow`
- `codex_protocol::account` -- `PlanType`

## Exports to

All public types re-exported through `lib.rs` to downstream consumers.

## Key files

| File | Role |
|------|------|
| `lib.rs` | Module declarations and public re-exports |
| `client.rs` | `Client` -- HTTP client supporting Codex API and ChatGPT backend-api path styles; methods for rate limits, task CRUD, sibling turns, config; `PathStyle` enum; `RequestError` type |
| `types.rs` | Re-exports OpenAPI models; defines `CodeTaskDetailsResponse`, `Turn`, `TurnItem`, `ContentFragment`, `Worklog`, `TurnError` with the `CodeTaskDetailsResponseExt` trait for extracting diffs, messages, prompts, and errors from task responses |
