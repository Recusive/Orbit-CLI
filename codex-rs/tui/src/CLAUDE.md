# codex-rs/tui/src/

Source code root for the `codex-tui` crate.

## What this folder does

Contains all Rust source modules for the TUI: the library root (`lib.rs`), the binary entry point (`main.rs`), the CLI argument parser, the application state machine, the chat widget, streaming pipeline, rendering engine, onboarding flows, and all supporting utilities. This is a flat module layout -- most modules are single `.rs` files declared in `lib.rs`, with complex subsystems split into subdirectories (`app/`, `bottom_pane/`, `chatwidget/`, `streaming/`, `tui/`, etc.).

## Module organization

### Core application

| File | Purpose |
|------|---------|
| `lib.rs` | Library root; module declarations, `run_main()` entry point, config/auth/session bootstrapping |
| `main.rs` | Binary entry point; CLI parsing, dispatch to `run_main()` or app-server mode |
| `cli.rs` | `Cli` struct (clap-derived CLI arguments) |
| `app.rs` | `App` state machine -- the main event loop that drives the entire TUI |
| `app_event.rs` | `AppEvent` enum -- all events the app processes (key input, agent messages, timers, etc.) |
| `app_event_sender.rs` | `AppEventSender` -- typed sender for `AppEvent` channels |
| `app_backtrack.rs` | State management for conversation backtracking/undo |
| `app_server_tui_dispatch.rs` | Logic to detect and dispatch to app-server TUI mode |

### Chat and transcript

| File | Purpose |
|------|---------|
| `chatwidget.rs` | `ChatWidget` -- main chat surface owning history cells, active streaming cell, bottom pane |
| `history_cell.rs` | `HistoryCell` -- individual transcript entries (messages, tool calls, status) |
| `exec_command.rs` | Shell command parsing and display for exec tool calls |
| `diff_render.rs` | Diff rendering for file patches |
| `markdown.rs` | Markdown data types |
| `markdown_render.rs` | Markdown-to-ratatui-Line rendering |
| `markdown_stream.rs` | Incremental markdown streaming collector |

### Input and composer

| File | Purpose |
|------|---------|
| `bottom_pane/` | Interactive footer: composer, popups, approval overlays, selection lists |
| `clipboard_paste.rs` | Paste event handling |
| `clipboard_text.rs` | Clipboard read/write operations |
| `external_editor.rs` | External editor integration ($EDITOR) |

### Rendering

| File | Purpose |
|------|---------|
| `render/` | Rendering engine: syntax highlighting, line utilities, renderable trait |
| `style.rs` | Centralized style constants |
| `color.rs` | Color utilities |
| `shimmer.rs` | Shimmer/loading animation effect for text spans |
| `wrapping.rs` | Text wrapping and adaptive line layout |
| `line_truncation.rs` | Line truncation utilities |
| `live_wrap.rs` | Live wrapping for streaming output |

### Terminal and TUI

| File | Purpose |
|------|---------|
| `tui.rs` | `Tui` wrapper: terminal init/restore, event streams, mode management |
| `tui/` | Terminal subsystem: event stream, frame rate limiter, frame scheduler, job control |
| `custom_terminal.rs` | Custom ratatui terminal with inline scrolling and viewport management |
| `ascii_animation.rs` | Animated spinner using frame data from `frames/` |
| `frames.rs` | Compile-time embedded animation frame constants |

### Session management

| File | Purpose |
|------|---------|
| `resume_picker.rs` | Session resume/fork picker UI |
| `session_log.rs` | High-fidelity session event logging |
| `cwd_prompt.rs` | CWD selection prompt for resume/fork with changed directories |

### Onboarding and configuration

| File | Purpose |
|------|---------|
| `onboarding/` | First-run experience: welcome, login, directory trust |
| `collaboration_modes.rs` | Collaboration mode settings UI |
| `debug_config.rs` | Debug configuration overlay |
| `theme_picker.rs` | Syntax theme picker |
| `model_migration.rs` | Model migration prompts |

### Notifications and status

| File | Purpose |
|------|---------|
| `notifications/` | Desktop notification backends (OSC 9, BEL) |
| `status/` | Status output formatting: account, rate limits, session card |
| `status_indicator_widget.rs` | Animated status dot widget |
| `tooltips.rs` | Tooltip content and display |
| `version.rs` | Version string constants |

## Imports from

- `codex-core`, `codex-protocol`, `codex-state`, `codex-login` and ~30 other workspace crates
- `ratatui`, `crossterm` for terminal UI
- `syntect`, `two-face` for syntax highlighting
- `pulldown-cmark` for markdown parsing
- `tokio` for async runtime

## Exports to

- `codex-tui` binary (via `main.rs`)
- `codex-cli` binary (via `lib.rs::run_main()`)
- `codex-cloud-tasks` and other crates consume `ComposerInput`, `render_markdown_text()`, and other public items
