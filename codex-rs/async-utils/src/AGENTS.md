# codex-rs/async-utils/src/

This file applies to `codex-rs/async-utils/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-async-utils` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-async-utils`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source code for the `codex-async-utils` crate.

### What this folder does

Contains the single-file implementation of async cancellation utilities.

### Key files

- `lib.rs` -- Complete crate implementation:
  - `CancelErr` enum with single variant `Cancelled`
  - `OrCancelExt` async trait with `or_cancel()` method
  - Blanket implementation for all `Future + Send` types using `tokio::select!` to race the future against `CancellationToken::cancelled()`
  - Tests covering: future completing first, token cancelling first, and token already cancelled before the future starts

### Imports from / exports to

**Imports:**
- `async_trait::async_trait`
- `std::future::Future`
- `tokio_util::sync::CancellationToken`

**Exports:**
- `CancelErr`, `OrCancelExt`
