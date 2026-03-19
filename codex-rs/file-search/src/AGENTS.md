# codex-rs/file-search/src/

This file applies to `codex-rs/file-search/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-file-search` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-file-search`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source code for the `codex-file-search` crate.

### What this folder does

Implements the fuzzy file search engine with a two-thread architecture: a walker thread that discovers files using the `ignore` crate, and a matcher thread that runs the `nucleo` fuzzy matcher against discovered paths.

### Key files and their roles

- `lib.rs` -- Core library. Defines: `FileMatch` (score, path, match_type, root, indices), `MatchType` (File/Directory), `FileSearchResults`, `FileSearchSnapshot`, `FileSearchOptions`, `SessionReporter` trait, `Reporter` trait. Implements `run()` (one-shot blocking search), `create_session()` (interactive session with live query updates), `walker_worker()` (filesystem traversal respecting gitignore with `require_git(true)`), `matcher_worker()` (nucleo-based fuzzy matching with debounced updates), and utilities (`file_name_from_path`, `cmp_by_score_desc_then_path_asc`, `build_override_matcher`, `get_file_path`). Includes comprehensive tests for scoring, session lifecycle, cancellation, gitignore behavior, and directory matching.
- `cli.rs` -- `Cli` struct with clap definitions: `pattern` (positional), `--limit` (default 64), `--cwd`, `--threads` (default 2), `--exclude`, `--json`, `--compute-indices`.
- `main.rs` -- Binary entry point. Creates a `StdioReporter` that outputs results in plain text, JSON, or with ANSI bold highlighting for matched character indices. Calls `run_main(cli, reporter)`.

### Architecture notes

- Walker and matcher run in separate OS threads (not Tokio tasks) using `crossbeam-channel` for coordination
- The walker uses `ignore::WalkBuilder` with `require_git(true)` to scope gitignore rules to actual git repositories
- The matcher uses `nucleo::Nucleo` with `Config::DEFAULT.match_paths()` for path-aware scoring
- Interactive sessions support query updates without re-walking the filesystem
- Cancellation is cooperative via `Arc<AtomicBool>` checked periodically

### Imports from

- `nucleo`: fuzzy matching
- `ignore`: filesystem walking
- `crossbeam-channel`: thread communication
- `clap`: CLI
