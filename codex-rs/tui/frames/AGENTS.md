# codex-rs/tui/frames/

This file applies to `codex-rs/tui/frames/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Treat this directory as golden data or generated/static support material. Keep filenames and relative paths stable unless the owning test, renderer, or generator changes first.
- Prefer updating the producer or the corresponding test scenario before editing files here by hand.
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-tui` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.
- Any user-visible TUI change needs matching snapshot coverage. Mirror behavior in the sibling TUI implementation when the same feature exists there.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-tui`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo insta pending-snapshots -p codex-tui`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

ASCII art animation frame assets for the TUI loading spinners.

### What this folder does

Contains 10 subdirectories, each holding 36 ASCII art text frames (frame_1.txt through frame_36.txt) that are compiled into the `codex-tui` binary at build time via `include_str!()`. These frames drive the animated spinner/loading indicator displayed while the agent is processing.

### What it plugs into

- **src/frames.rs**: Uses the `frames_for!()` macro to embed all frame files at compile time as `[&str; 36]` constant arrays. Each variant (default, codex, openai, blocks, dots, hash, hbars, vbars, shapes, slug) maps to one subdirectory here.
- **src/ascii_animation.rs**: Consumes the frame arrays to drive timed animation rendering in the TUI.

### Subdirectories

| Directory | Description |
|-----------|-------------|
| `default/` | Default animation variant |
| `codex/` | Codex-branded animation |
| `openai/` | OpenAI-branded animation |
| `blocks/` | Block-character animation |
| `dots/` | Dot-based animation |
| `hash/` | Hash-character animation |
| `hbars/` | Horizontal bar animation |
| `vbars/` | Vertical bar animation |
| `shapes/` | Geometric shapes animation |
| `slug/` | Slug-style animation |

### Key details

- Each subdirectory contains exactly 36 frames: `frame_1.txt` through `frame_36.txt`.
- Default frame tick rate is 80ms (`FRAME_TICK_DEFAULT`), yielding a ~2.9 second animation cycle.
- Frames are plain ASCII text, not binary assets.
