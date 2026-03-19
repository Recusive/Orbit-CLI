# codex-rs/connectors/src/

This file applies to `codex-rs/connectors/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-connectors` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-connectors`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source directory for the `codex-connectors` crate.

### What this folder does

Contains the single-file implementation of connector listing, caching, and directory API response types.

### Key files

| File | Role |
|------|------|
| `lib.rs` | `AllConnectorsCacheKey` -- cache key type; `CachedAllConnectors` -- cache entry with expiry; `ALL_CONNECTORS_CACHE` -- global `LazyLock<Mutex<Option<...>>>` cache; `DirectoryListResponse` -- serde type for `/backend-api/aip/p/directory/...` responses; `list_all_connectors_with_options` -- fetches connectors via a generic async callback, caches results with 1-hour TTL, handles workspace vs personal accounts with different directory paths; `cached_all_connectors` -- returns cached data if unexpired and matching key |
