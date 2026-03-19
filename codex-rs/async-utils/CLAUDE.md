# codex-rs/async-utils/

Crate: `codex-async-utils` -- Async utility extensions for Tokio-based code.

## What this crate does

Provides ergonomic extensions for working with async futures and cancellation tokens. The primary feature is the `OrCancelExt` trait, which allows any `Future` to be raced against a `CancellationToken`.

## Main types and functions

- `CancelErr` -- Error type returned when a future is cancelled (unit variant `Cancelled`)
- `OrCancelExt` trait -- Extension trait for all `Future + Send` types:
  - `.or_cancel(token: &CancellationToken) -> Result<Output, CancelErr>` -- Races the future against the token; returns `Ok(output)` if the future completes first, or `Err(CancelErr::Cancelled)` if the token fires first

## What it plugs into

- Used throughout the workspace wherever async operations need to be cancellable (agent turns, network requests, etc.)

## Imports from / exports to

**Dependencies:**
- `async-trait` -- For async trait definitions
- `tokio` -- Runtime (macros, rt, time)
- `tokio-util` -- Provides `CancellationToken`

**Exports:**
- `CancelErr` and `OrCancelExt` are the public API

## Key files

- `Cargo.toml` -- Crate manifest
- `src/lib.rs` -- Single-file implementation with the trait, blanket impl, and tests
