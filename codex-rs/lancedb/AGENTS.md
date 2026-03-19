# codex-rs/lancedb/

This file applies to `codex-rs/lancedb/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.

## Validate
- Run the nearest package or crate tests that exercise this subtree.

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Placeholder directory for LanceDB integration.

### What this folder does

This is currently an empty directory. It is reserved for LanceDB-related code or assets (e.g., native library binaries, configuration, or a future crate for vector database integration).

### Status

Empty -- no source files, no Cargo.toml, no build artifacts present.

### Context

LanceDB is a vector database that could be used for semantic search, embeddings storage, or file search features. The `codex-package-manager` crate provides infrastructure for downloading and caching native library packages, which may be used to manage LanceDB native dependencies when this directory is populated.

### What it plugs into

- Not yet integrated into the workspace
- May eventually relate to `codex-file-search` or embedding-based features in `codex-core`
