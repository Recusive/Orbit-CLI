# codex-rs/async-utils/src/

Source code for the `codex-async-utils` crate.

## What this folder does

Contains the single-file implementation of async cancellation utilities.

## Key files

- `lib.rs` -- Complete crate implementation:
  - `CancelErr` enum with single variant `Cancelled`
  - `OrCancelExt` async trait with `or_cancel()` method
  - Blanket implementation for all `Future + Send` types using `tokio::select!` to race the future against `CancellationToken::cancelled()`
  - Tests covering: future completing first, token cancelling first, and token already cancelled before the future starts

## Imports from / exports to

**Imports:**
- `async_trait::async_trait`
- `std::future::Future`
- `tokio_util::sync::CancellationToken`

**Exports:**
- `CancelErr`, `OrCancelExt`
