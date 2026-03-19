# codex-rs/ansi-escape/src/

Source code for the `codex-ansi-escape` crate.

## What this folder does

Contains the single-file implementation of ANSI escape sequence parsing for TUI rendering.

## Key files

- `lib.rs` -- Complete crate implementation:
  - `expand_tabs(s)` -- Internal helper that replaces tab characters with 4 spaces to avoid visual artifacts in transcript/gutter views
  - `ansi_escape(s: &str) -> Text<'static>` -- Converts ANSI-escaped strings to Ratatui `Text` using `ansi-to-tui`'s `IntoText` trait; panics on parse or UTF-8 errors (considered programmer errors)
  - `ansi_escape_line(s: &str) -> Line<'static>` -- Wraps `ansi_escape` and extracts only the first line; logs a warning if multiple lines are present

## Imports from / exports to

**Imports:**
- `ansi_to_tui::{Error, IntoText}` -- Core conversion trait
- `ratatui::text::{Line, Text}` -- Output types
- `tracing` -- Warning/error logging

**Exports:**
- `ansi_escape` and `ansi_escape_line` are the public API
