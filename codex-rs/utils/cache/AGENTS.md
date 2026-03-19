# codex-rs/utils/cache/

This file applies to `codex-rs/utils/cache/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-utils-cache` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-utils-cache`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Crate `codex-utils-cache` -- thread-safe LRU cache and content hashing.

### What this folder does

Provides a Tokio-aware LRU cache (`BlockingLruCache`) that safely degrades to a no-op when no Tokio runtime is present, plus a `sha1_digest` helper for content-based cache keys.

### Key types and functions

- `BlockingLruCache<K, V>` -- LRU cache protected by a Tokio mutex; operations are no-ops outside a runtime
- `sha1_digest(bytes)` -- computes a 20-byte SHA-1 digest for use as content-based cache keys
- `get_or_insert_with`, `get_or_try_insert_with` -- compute-on-miss cache access patterns

### Imports from

- `lru` -- underlying LRU data structure
- `sha1` -- SHA-1 digest computation
- `tokio` -- async mutex and runtime detection

### Exports to

Consumed by `codex-utils-image` for caching processed images, and potentially other crates needing content-addressed caching.

### Key files

- `Cargo.toml` -- crate metadata and dependencies
- `src/lib.rs` -- `BlockingLruCache` implementation, `sha1_digest`, and tests
