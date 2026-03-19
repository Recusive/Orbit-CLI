# codex-rs/cloud-requirements/src/

This file applies to `codex-rs/cloud-requirements/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-cloud-requirements` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-cloud-requirements`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source directory for the `codex-cloud-requirements` crate.

### What this folder does

Contains the single-file implementation of the cloud requirements loading system. All logic is in `lib.rs`.

### Key files

| File | Role |
|------|------|
| `lib.rs` | Complete implementation: `CloudRequirementsService` (fetch with retries, HMAC-signed cache read/write, background refresh); `BackendRequirementsFetcher` (HTTP fetch via `codex-backend-client`); `RequirementsFetcher` trait for testability; cache types (`CloudRequirementsCacheFile`, `CloudRequirementsCacheSignedPayload`); auth recovery on 401; metrics emission; comprehensive test suite with `StaticFetcher`, `PendingFetcher`, `SequenceFetcher`, `TokenFetcher`, `UnauthorizedFetcher` mocks |
