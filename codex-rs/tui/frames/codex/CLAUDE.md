# codex-rs/tui/frames/codex/

Codex-branded ASCII animation frames.

## What this folder does

Contains 36 ASCII art text frames (`frame_1.txt` through `frame_36.txt`) for the "codex" animation variant of the TUI loading spinner. This variant uses Codex branding in its animation style.

## What it plugs into

- **src/frames.rs**: Embedded at compile time via `include_str!()` into the `FRAMES_CODEX` constant array.
- Referenced through `ALL_VARIANTS` in `src/frames.rs` for variant selection.

## Key files

- `frame_1.txt` through `frame_36.txt` -- sequential animation frames, each a multi-line ASCII art string.
