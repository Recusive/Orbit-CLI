# codex-rs/cloud-tasks/src/

This file applies to `codex-rs/cloud-tasks/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-cloud-tasks` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-cloud-tasks`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source directory for the `codex-cloud-tasks` crate.

### What this folder does

Contains the implementation of the Codex Cloud tasks TUI and CLI subcommands for listing, viewing, applying, and creating cloud tasks.

### Key files

| File | Role |
|------|------|
| `lib.rs` | Module declarations, `Cli` re-export, `run_main` entry point, task listing/apply orchestration |
| `cli.rs` | `Cli` clap struct with `Command` enum: `Exec`, `Status`, `List`, `Apply`, `Diff` subcommands |
| `app.rs` | Ratatui app state management and terminal event loop |
| `ui.rs` | Terminal UI rendering (task list, diff view, status display) |
| `new_task.rs` | New task creation: prompt input, environment selection, branch detection, multi-attempt support |
| `env_detect.rs` | Git environment detection helpers |
| `scrollable_diff.rs` | Scrollable unified diff viewer widget |
| `util.rs` | Shared utilities: relative time formatting, error log appending, user agent suffix |
