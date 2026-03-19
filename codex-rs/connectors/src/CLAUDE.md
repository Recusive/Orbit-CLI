# codex-rs/connectors/src/

Source directory for the `codex-connectors` crate.

## What this folder does

Contains the single-file implementation of connector listing, caching, and directory API response types.

## Key files

| File | Role |
|------|------|
| `lib.rs` | `AllConnectorsCacheKey` -- cache key type; `CachedAllConnectors` -- cache entry with expiry; `ALL_CONNECTORS_CACHE` -- global `LazyLock<Mutex<Option<...>>>` cache; `DirectoryListResponse` -- serde type for `/backend-api/aip/p/directory/...` responses; `list_all_connectors_with_options` -- fetches connectors via a generic async callback, caches results with 1-hour TTL, handles workspace vs personal accounts with different directory paths; `cached_all_connectors` -- returns cached data if unexpired and matching key |
