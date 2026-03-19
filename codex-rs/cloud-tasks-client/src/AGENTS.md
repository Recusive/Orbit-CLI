# codex-rs/cloud-tasks-client/src/

This file applies to `codex-rs/cloud-tasks-client/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-cloud-tasks-client` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-cloud-tasks-client`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source directory for the `codex-cloud-tasks-client` crate.

### What this folder does

Contains the `CloudBackend` trait definition, shared types, and feature-gated implementations for cloud task operations.

### Key files

| File | Role |
|------|------|
| `lib.rs` | Module declarations; public re-exports of all types and feature-gated clients |
| `api.rs` | `CloudBackend` async trait with methods for listing, getting details, applying diffs, creating tasks, listing attempts; type definitions: `TaskId`, `TaskStatus`, `TaskSummary`, `DiffSummary`, `TurnAttempt`, `AttemptStatus`, `ApplyOutcome`, `ApplyStatus`, `TaskText`, `TaskListPage`, `CreatedTask`, `CloudTaskError` |
| `http.rs` | `HttpClient` (requires `online` feature) -- wraps `codex-backend-client::Client`; maps backend response models to crate types; handles diff extraction from multiple turn formats |
| `mock.rs` | `MockClient` (requires `mock` feature) -- returns deterministic mock data with environment-based variation for testing |
