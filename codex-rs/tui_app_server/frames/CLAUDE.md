# codex-rs/tui_app_server/frames/

ASCII art animation frame assets for the TUI loading spinners.

## What this folder does

Contains 10 subdirectories, each holding 36 text-based animation frames for a distinct loading spinner style. These frames are embedded at compile time via `include_str!()` in `src/frames.rs` and cycled at a configurable interval to produce smooth terminal animations during agent processing.

## What it plugs into

- **src/frames.rs**: The `frames_for!` macro includes every `frame_N.txt` from each subdirectory into compile-time constant arrays (`FRAMES_DEFAULT`, `FRAMES_CODEX`, etc.).
- **src/ascii_animation.rs**: Drives frame cycling and renders the current frame into the TUI.
- **onboarding/welcome.rs**: Displays an animation on the welcome screen.

## Variants

| Subdirectory | Description |
|-------------|-------------|
| `default/` | Default animation style |
| `codex/` | Codex-branded animation |
| `openai/` | OpenAI-branded animation |
| `blocks/` | Block-character animation |
| `dots/` | Dot-pattern animation |
| `hash/` | Hash-character animation |
| `hbars/` | Horizontal-bar animation |
| `vbars/` | Vertical-bar animation |
| `shapes/` | Geometric shape animation |
| `slug/` | Slug-style animation |

## File layout

Each subdirectory contains exactly 36 files: `frame_1.txt` through `frame_36.txt`. Every file holds a single ASCII art frame as plain text.
