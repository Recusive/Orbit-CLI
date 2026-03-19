# .codex/skills/test-tui/ — TUI Testing Skill

## What This Folder Does

Provides a lightweight skill that guides the Codex agent through interactive testing of the Codex TUI (terminal user interface). Unlike the `babysit-pr` skill, this skill has no scripts or references — it is purely instructional.

## Key Files

| File | Role |
|------|------|
| `SKILL.md` | Skill definition with YAML front matter (`name: test-tui`) and concise instructions for launching and testing the TUI interactively. |

## What It Plugs Into

- **Codex agent**: Activated when the user asks to test the TUI. The agent follows the instructions in `SKILL.md`.
- **`just codex`**: The skill directs the agent to use the `just codex` task runner target (defined in the root `justfile`, which sets working directory to `codex-rs/`).
- **Codex TUI** (`codex-rs/tui/`): The Rust TUI binary being tested.

## Instructions Summary

The skill tells the agent to:

1. Start the TUI interactively using `just codex -c ...`.
2. Always set `RUST_LOG="trace"` for verbose logging.
3. Pass `-c log_dir=<some_temp_dir>` to write logs to a specific directory for debugging.
4. When sending test messages programmatically, send the text first, then send Enter as a separate write (not in one burst) — this matters for terminal input handling.

## Relationship to Other Components

- **`codex-rs/tui/`**: The Rust crate containing the TUI implementation being tested.
- **Root `justfile`**: Defines the `just codex` target that builds and runs the TUI binary.
- **No external dependencies**: This skill requires only the built TUI binary and a terminal.
