# codex-rs/shell-command/src/

This file applies to `codex-rs/shell-command/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-shell-command` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-shell-command`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source code for the `codex-shell-command` command parsing and safety library.

### What this folder does

Implements shell command parsing (using tree-sitter-bash) and safety classification for determining whether commands can be auto-approved.

### Key files

- `lib.rs` -- module declarations; re-exports `is_safe_command` and `is_dangerous_command` from the `command_safety` submodule.
- `bash.rs` -- tree-sitter-bash parsing engine. Key functions: `try_parse_shell()` parses bash source into a tree; `try_parse_word_only_commands_sequence()` extracts plain commands from scripts using only safe operators (&&, ||, ;, |); `extract_bash_command()` unwraps `bash -lc "..."` invocations; `parse_shell_lc_single_command_prefix()` handles heredoc scripts.
- `powershell.rs` -- PowerShell command extraction (`extract_powershell_command()`), UTF-8 output prefix injection, and executable discovery (`try_find_pwsh_executable_blocking()`, `try_find_powershell_executable_blocking()`).
- `shell_detect.rs` -- `detect_shell_type()` maps executable paths/names to `ShellType` enum (Zsh, Bash, Sh, PowerShell, Cmd).
- `parse_command.rs` -- additional command parsing utilities.
- `command_safety/` -- safety classification submodule.

### Imports from

- `tree-sitter`, `tree-sitter-bash` for AST-based parsing.
- `codex-utils-absolute-path`, `which`, `shlex`.

### Exports to

- Parent crate re-exports `is_safe_command` and `is_dangerous_command`.
- `bash`, `powershell`, `parse_command`, and `command_safety` modules are public.
