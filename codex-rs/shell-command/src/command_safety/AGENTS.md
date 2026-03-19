# codex-rs/shell-command/src/command_safety/

This file applies to `codex-rs/shell-command/src/command_safety/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-shell-command` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-shell-command`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Command safety classification logic -- determines if commands are safe to auto-approve or dangerous enough to warn about.

### What this folder does

Contains the "known safe" and "known dangerous" command classifiers. The safe classifier maintains an allowlist of read-only commands (ls, cat, grep, git status, etc.) with per-command argument validation. The dangerous classifier flags destructive operations (rm -rf, sudo rm -f). Both classifiers handle `bash -lc "..."` wrappers by parsing the inner script.

### Key files

- `mod.rs` -- re-exports `is_safe_command` and `is_dangerous_command` submodules, plus `windows_safe_commands`.
- `is_safe_command.rs` -- `is_known_safe_command()` checks commands against an allowlist. Handles shell wrappers (`bash -lc`, `zsh -lc`), composite scripts (parsing each subcommand), and per-tool argument validation: `git` (read-only subcommands, config-override rejection), `find` (rejects -exec/-delete), `rg` (rejects --pre/--search-zip), `base64` (rejects -o/--output), `sed -n Np` patterns.
- `is_dangerous_command.rs` -- `command_might_be_dangerous()` flags `rm -f`/`rm -rf` and `sudo` wrappers. Also provides shared helpers: `executable_name_lookup_key()` (normalizes executable names, strips Windows extensions), `find_git_subcommand()` (skips git global options to find the first positional subcommand).
- `windows_safe_commands.rs` -- Windows-specific safe command classification for PowerShell commands (Get-ChildItem, Get-Content, Get-Location, etc.).
- `windows_dangerous_commands.rs` -- Windows-specific dangerous command detection.
- `powershell_parser.ps1` -- PowerShell AST parser script for command classification.

### Imports from

- `crate::bash::parse_shell_lc_plain_commands` for shell script parsing.
- `crate::command_safety::is_dangerous_command::executable_name_lookup_key` and `find_git_subcommand` (shared between safe and dangerous classifiers).

### Exports to

- `is_known_safe_command()` and `command_might_be_dangerous()` are the primary public API, re-exported from `codex-shell-command`.
