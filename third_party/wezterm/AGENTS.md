# third_party/wezterm/

This file applies to `third_party/wezterm/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Read the files listed below before changing behavior in this subtree; keep neighboring docs and call sites consistent with any structural change.

## Validate
- Run the nearest package or crate tests that exercise this subtree.

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

### Purpose

Contains the license file for [WezTerm](https://github.com/wez/wezterm), a GPU-accelerated terminal emulator whose code has been adapted for use in the Codex project.

### Key Files

| File | Role |
|------|------|
| `LICENSE` | MIT license for WezTerm (Copyright 2018-Present Wez Furlong) |

### What WezTerm Code Is Used For

Terminal emulation and PTY handling code from WezTerm has been adapted for use in the Codex TUI and exec subsystems. This includes terminal escape sequence processing and pseudo-terminal management.

### Relationship to Other Directories

- `codex-rs/tui/`: The terminal UI uses adapted WezTerm terminal handling code
- `codex-rs/exec/`: The execution subsystem uses PTY handling adapted from WezTerm
- Referenced by the root `NOTICE` file for attribution
