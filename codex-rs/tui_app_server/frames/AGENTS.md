# codex-rs/tui_app_server/frames/

This file applies to `codex-rs/tui_app_server/frames/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Treat this directory as golden data or generated/static support material. Keep filenames and relative paths stable unless the owning test, renderer, or generator changes first.
- Prefer updating the producer or the corresponding test scenario before editing files here by hand.
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-tui-app-server` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.
- Any user-visible TUI change needs matching snapshot coverage. Mirror behavior in the sibling TUI implementation when the same feature exists there.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-tui-app-server`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo insta pending-snapshots -p codex-tui-app-server`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

ASCII art animation frame assets for the TUI loading spinners.

### What this folder does

Contains 10 subdirectories, each holding 36 text-based animation frames for a distinct loading spinner style. These frames are embedded at compile time via `include_str!()` in `src/frames.rs` and cycled at a configurable interval to produce smooth terminal animations during agent processing.

### What it plugs into

- **src/frames.rs**: The `frames_for!` macro includes every `frame_N.txt` from each subdirectory into compile-time constant arrays (`FRAMES_DEFAULT`, `FRAMES_CODEX`, etc.).
- **src/ascii_animation.rs**: Drives frame cycling and renders the current frame into the TUI.
- **onboarding/welcome.rs**: Displays an animation on the welcome screen.

### Variants

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

### File layout

Each subdirectory contains exactly 36 files: `frame_1.txt` through `frame_36.txt`. Every file holds a single ASCII art frame as plain text.
