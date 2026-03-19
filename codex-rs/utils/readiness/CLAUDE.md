# codex-rs/utils/readiness/

Crate `codex-utils-readiness` -- async readiness flag with token-based authorization.

## What this folder does

Provides a thread-safe readiness flag that supports subscription-based authorization. Components can subscribe to receive a token, and any token holder can mark the flag as ready. Other components can asynchronously wait for readiness. The flag becomes ready automatically if no subscribers exist.

## Key types and functions

- `Readiness` trait -- async trait with `is_ready()`, `subscribe()`, `mark_ready(token)`, `wait_ready()`
- `ReadinessFlag` -- concrete implementation using atomics, a Tokio mutex for token tracking, and a `watch` channel for async notification
- `Token` -- opaque subscription token (i32-based)
- `ReadinessError` -- error variants for lock timeout and already-ready flag

## Imports from

- `async-trait` -- async trait support
- `thiserror` -- error derivation
- `time` -- time utilities
- `tokio` -- `Mutex`, `watch` channel, timeout

## Exports to

Used by `codex-core` for coordinating readiness between subsystems (e.g., waiting for sandbox initialization before accepting commands).

## Key files

- `Cargo.toml` -- crate metadata and dependencies
- `src/lib.rs` -- `Readiness` trait, `ReadinessFlag` implementation, `Token`, `ReadinessError`, and comprehensive tests
