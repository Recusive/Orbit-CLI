# codex-rs/file-search/

This file applies to `codex-rs/file-search/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-file-search` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-file-search`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Fuzzy file search engine using `nucleo` for scoring and `ignore` for filesystem walking. Provides both a library API and a standalone CLI binary.

### What this folder does

Implements high-performance fuzzy filename matching across directory trees. Uses the `nucleo` fuzzy matcher (same engine as Helix editor) for relevance scoring and the `ignore` crate (from ripgrep) for gitignore-aware filesystem traversal. Supports both one-shot search (`run()`) and interactive sessions (`create_session()`) with live query updates and streaming results.

### What it plugs into

- **codex-core** -- used as the file search tool for the agent
- Standalone CLI (`codex-file-search`) for manual fuzzy file search
- Interactive sessions can be driven by any UI that implements `SessionReporter`

### Imports from

- `nucleo`: fuzzy matching engine (Nucleo, Matcher, Pattern, Utf32String)
- `ignore`: gitignore-aware file tree walker (WalkBuilder, OverrideBuilder)
- `crossbeam-channel`: multi-producer work signaling between walker and matcher threads
- `clap`: CLI argument parsing
- `serde`, `serde_json`: result serialization

### Exports to

- `run(pattern, roots, options, cancel_flag)` -- one-shot synchronous search returning `FileSearchResults`
- `create_session(dirs, options, reporter, cancel_flag)` -- interactive session returning `FileSearchSession` with `update_query()` for live re-matching
- `FileMatch` (score, path, match_type, root, indices), `MatchType` (File, Directory), `FileSearchResults`, `FileSearchSnapshot`
- `FileSearchOptions` (limit, exclude, threads, compute_indices, respect_gitignore)
- `SessionReporter` trait (on_update, on_complete) for streaming results
- `Reporter` trait (report_match, warn_matches_truncated, warn_no_search_pattern) for one-shot CLI output
- `Cli` -- clap CLI definition
- `run_main(cli, reporter)` -- async CLI entry point
- `file_name_from_path()`, `cmp_by_score_desc_then_path_asc()` -- utilities

### Key files

- `Cargo.toml` -- crate metadata; binary `codex-file-search`, library `codex_file_search`
- `README.md` -- documentation
- `src/lib.rs` -- core implementation: walker thread, matcher thread, session management, one-shot `run()`, interactive `create_session()`
- `src/cli.rs` -- clap CLI definition: pattern, limit, cwd, threads, exclude, json, compute-indices flags
- `src/main.rs` -- binary entry point with stdio reporter (supports plain text, JSON, and highlighted index output)
