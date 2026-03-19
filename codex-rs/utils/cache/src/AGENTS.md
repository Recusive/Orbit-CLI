# codex-rs/utils/cache/src/

This file applies to `codex-rs/utils/cache/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-utils-cache` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-utils-cache`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source directory for the `codex-utils-cache` crate.

### Key files

- `lib.rs` -- single-file implementation containing:
  - `BlockingLruCache<K, V>` -- wraps `lru::LruCache` in a `Tokio::Mutex`; all operations gracefully degrade to no-ops when no Tokio runtime is available (via `tokio::runtime::Handle::try_current`)
  - Methods: `new`, `try_with_capacity`, `get`, `insert`, `remove`, `clear`, `get_or_insert_with`, `get_or_try_insert_with`, `with_mut`, `blocking_lock`
  - `sha1_digest(bytes: &[u8]) -> [u8; 20]` -- SHA-1 hash helper
  - `lock_if_runtime` -- internal helper using `tokio::task::block_in_place` for safe blocking lock acquisition
  - Tests verifying store/retrieve, LRU eviction, and disabled behavior without a runtime
