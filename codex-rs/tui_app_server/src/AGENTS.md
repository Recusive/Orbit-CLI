# codex-rs/tui_app_server/src/

This file applies to `codex-rs/tui_app_server/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-tui-app-server` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.
- Any user-visible TUI change needs matching snapshot coverage. Mirror behavior in the sibling TUI implementation when the same feature exists there.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-tui-app-server`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo insta pending-snapshots -p codex-tui-app-server`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source root for the `codex-tui-app-server` crate.

### What this folder does

Contains all Rust source code for the app-server-backed TUI. The module tree is declared in `lib.rs` and the binary entry point lives in `main.rs`. The code is organized into submodules covering the application lifecycle, UI widgets, streaming pipeline, onboarding, status display, and terminal management.

### What it plugs into

- **lib.rs**: Root module that re-exports public API (`run_main`, `Cli`, `AppExitInfo`, `ComposerInput`, etc.) and orchestrates startup (config loading, app-server connection, onboarding, session selection, ratatui event loop).
- **main.rs**: Binary entry point that parses CLI arguments and delegates to `run_main()`.
- **Cargo.toml** (parent): References `src/main.rs` and `src/lib.rs` as the binary and library targets.

### Key files

| File | Role |
|------|------|
| `lib.rs` | Library root; module declarations, app-server lifecycle, onboarding orchestration, main entry point. |
| `main.rs` | Binary entry; parses CLI via clap, calls `run_main()`. |
| `cli.rs` | `Cli` struct with clap argument definitions. |
| `app.rs` | `App` state machine (partial; continued in `app/` submodule). |
| `app_server_session.rs` | `AppServerSession` wrapper with typed JSON-RPC methods for all app-server operations. |
| `tui.rs` | Terminal init/restore, raw mode, alternate screen, `Tui` wrapper (partial; continued in `tui/` submodule). |
| `chatwidget.rs` | `ChatWidget` main chat surface (partial; continued in `chatwidget/` submodule). |
| `frames.rs` | Compile-time frame embedding via `include_str!` macros. |
| `app_event.rs` | `AppEvent` enum -- all events the TUI can process. |
| `app_event_sender.rs` | `AppEventSender` -- typed channel for dispatching `AppEvent`s. |
| `app_command.rs` | `AppCommand` enum -- user-initiated commands from the UI. |
| `app_backtrack.rs` | `BacktrackState` for undo/rollback of agent turns. |
| `history_cell.rs` | `HistoryCell` types for the chat transcript (messages, exec calls, plans, etc.). |
| `markdown.rs` / `markdown_render.rs` / `markdown_stream.rs` | Markdown parsing, rendering to ratatui spans, and streaming collection. |
| `diff_render.rs` | Renders file diffs in the chat transcript. |
| `style.rs` | TUI color and style constants. |
| `color.rs` | Color utilities. |
| `ui_consts.rs` | Layout constants (padding, margins, sizes). |
| `version.rs` | `CODEX_CLI_VERSION` constant. |
| `resume_picker.rs` | Session resume/fork picker UI. |
| `selection_list.rs` | Generic selection list widget. |
| `session_log.rs` | High-fidelity session event logging. |
| `slash_command.rs` | Slash command parsing and dispatch. |
| `file_search.rs` | File search manager for the file search popup. |
| `mention_codec.rs` | Encode/decode @-mentions in user input. |
| `model_catalog.rs` | Available model listing. |
| `model_migration.rs` | Model upgrade migration prompts. |
| `multi_agents.rs` | Multi-agent thread picker and navigation helpers. |
| `voice.rs` | Voice capture and transcription (feature-gated). |
| `audio_device.rs` | Audio device enumeration (feature-gated). |
| `clipboard_paste.rs` / `clipboard_text.rs` | Clipboard integration. |
| `wrapping.rs` | Text wrapping utilities. |
| `line_truncation.rs` | Line truncation for display. |
| `live_wrap.rs` | Live word-wrap for streaming text. |
| `shimmer.rs` | Shimmer animation effect. |
| `pager_overlay.rs` | Full-screen pager overlay for long content. |
| `tooltips.rs` | Tooltip rendering. |
| `key_hint.rs` | Keyboard shortcut hint rendering. |
| `cwd_prompt.rs` | Working directory selection prompt for resume/fork. |
| `external_editor.rs` | Launch external editor for input. |
| `exec_command.rs` | Shell command formatting. |
| `get_git_diff.rs` | Git diff retrieval. |
| `additional_dirs.rs` | Additional writable directory validation. |
| `collaboration_modes.rs` | Collaboration mode presets. |
| `skills_helpers.rs` | Skill metadata helpers. |
| `oss_selection.rs` | OSS provider selection UI. |
| `local_chatgpt_auth.rs` | Local ChatGPT auth token loading. |
| `debug_config.rs` | Debug configuration display. |
| `terminal_palette.rs` | Terminal color palette detection. |
| `text_formatting.rs` | Text formatting utilities. |
| `theme_picker.rs` | Syntax theme picker. |
| `update_prompt.rs` | CLI update prompt. |
| `updates.rs` | Update checking logic. |
| `test_backend.rs` | Test-only ratatui backend for snapshot tests. |
