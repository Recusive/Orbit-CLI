# codex-rs/shell-command/

Command parsing and safety classification library for shell commands.

## What this folder does

Provides utilities for parsing shell commands (bash, zsh, PowerShell) and classifying them as "known safe" (auto-approvable without user confirmation) or "known dangerous" (requiring a warning). Uses tree-sitter-bash for robust shell script parsing.

## What it plugs into

- Used by `codex-core` to determine whether a shell command proposed by the AI agent can be auto-approved or needs user confirmation.
- The safety classification drives the approval UX in both TUI and headless modes.

## Imports from

- `codex-protocol` -- protocol types.
- `codex-utils-absolute-path` -- path normalization.
- `tree-sitter`, `tree-sitter-bash` -- shell script parsing.
- `shlex` -- shell word splitting.
- `regex`, `which`, `url`, `base64`.

## Exports to

- `is_safe_command` / `is_known_safe_command` -- returns true for commands that are provably read-only.
- `is_dangerous_command` / `command_might_be_dangerous` -- returns true for commands that may cause destructive side effects.
- `bash` module -- shell parsing helpers.
- `powershell` module -- PowerShell command extraction and executable discovery.
- `parse_command` module -- additional command parsing utilities.

## Key files

- `Cargo.toml` -- crate manifest.
- `src/lib.rs` -- module declarations and re-exports.
- `src/bash.rs` -- tree-sitter-based bash parsing: `try_parse_shell()`, `try_parse_word_only_commands_sequence()`, `parse_shell_lc_plain_commands()`, `extract_bash_command()`.
- `src/powershell.rs` -- PowerShell command extraction and executable discovery.
- `src/shell_detect.rs` -- detects shell type (Zsh, Bash, Sh, PowerShell, Cmd) from executable path.
- `src/command_safety/` -- safety classification logic.
