# codex-rs/tui_app_server/frames/shapes/

ASCII art frames for the "shapes" loading animation variant.

## What this folder does

Contains 36 text files (`frame_1.txt` through `frame_36.txt`), each holding one frame of the "shapes" ASCII art animation. These are embedded at compile time by `src/frames.rs` via `include_str!()`.

## What it plugs into

- **src/frames.rs**: The `frames_for!("shapes")` macro invocation produces the `FRAMES_SHAPES` constant array.
- **src/ascii_animation.rs**: Cycles through these frames at runtime to render the loading animation in the TUI.

## Key files

All 36 files follow the pattern `frame_N.txt` where N ranges from 1 to 36. Each file is a single plain-text ASCII art frame.
