# codex-rs/state/src/model/

This file applies to `codex-rs/state/src/model/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-state` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-state`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Data model types for the state and logs SQLite databases.

### What this folder does

Defines the Rust structs and enums that represent database rows and query parameters for threads, logs, backfill state, agent jobs, and memories.

### Key files

- `mod.rs` -- re-exports all model types and internal row types.
- `thread_metadata.rs` -- `ThreadMetadata`, `ThreadMetadataBuilder`, `ThreadsPage`, `ThreadRow`, `Anchor`, `SortKey`, `BackfillStats`, `ExtractionOutcome`. Maps between protocol-level thread data and SQLite rows.
- `log.rs` -- `LogEntry`, `LogQuery`, `LogRow`. Defines the structured log record and query filter types.
- `backfill_state.rs` -- `BackfillState`, `BackfillStatus`. Tracks progress of rollout-to-DB backfill.
- `agent_job.rs` -- `AgentJob`, `AgentJobCreateParams`, `AgentJobItem`, `AgentJobItemCreateParams`, `AgentJobStatus`, `AgentJobItemStatus`, `AgentJobProgress`, `AgentJobRow`, `AgentJobItemRow`.
- `memories.rs` -- `Stage1JobClaim`, `Stage1Output`, `Stage1OutputRef`, `Stage1StartupClaimParams`, `Phase2InputSelection`, `Phase2JobClaimOutcome`, and related types for the memory extraction pipeline.

### Exports to

- All types are re-exported through `src/lib.rs` for use by `codex-core` and the logs client binary.
