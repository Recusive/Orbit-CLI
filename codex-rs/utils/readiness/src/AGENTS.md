# codex-rs/utils/readiness/src/

This file applies to `codex-rs/utils/readiness/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-utils-readiness` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-utils-readiness`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source directory for the `codex-utils-readiness` crate.

### Key files

- `lib.rs` -- single-file implementation containing:
  - `Readiness` async trait -- defines `is_ready()`, `subscribe()`, `mark_ready(token)`, `wait_ready()`
  - `ReadinessFlag` struct -- fields: `ready` (AtomicBool), `next_id` (AtomicI32), `tokens` (Mutex<HashSet<Token>>), `tx` (watch::Sender)
  - Key semantics:
    - `is_ready()` returns true if already ready, or if no subscribers exist (auto-ready)
    - `subscribe()` returns a `Token` or `FlagAlreadyReady` error; double-checks under lock
    - `mark_ready(token)` validates token exists in set, sets atomic flag, clears all tokens, broadcasts
    - `wait_ready()` subscribes to watch channel and awaits until true
    - Token 0 is reserved and never authorized
    - Lock acquisition has a 1-second timeout to prevent deadlocks
  - `errors` module -- `ReadinessError::TokenLockFailed` and `ReadinessError::FlagAlreadyReady`
  - Tests for roundtrip subscribe/mark, rejection of unknown tokens, async wait unblocking, auto-ready without subscribers, lock contention, and token uniqueness
