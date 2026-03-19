# codex-rs/tui/frames/openai/

OpenAI-branded ASCII animation frames.

## What this folder does

Contains 36 ASCII art text frames (`frame_1.txt` through `frame_36.txt`) for the "openai" animation variant of the TUI loading spinner. This variant uses OpenAI branding in its animation style.

## What it plugs into

- **src/frames.rs**: Embedded at compile time via `include_str!()` into the `FRAMES_OPENAI` constant array.
- Referenced through `ALL_VARIANTS` in `src/frames.rs` for variant selection.

## Key files

- `frame_1.txt` through `frame_36.txt` -- sequential animation frames, each a multi-line ASCII art string.
