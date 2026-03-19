# codex-rs/shell-command/src/

Source code for the `codex-shell-command` command parsing and safety library.

## What this folder does

Implements shell command parsing (using tree-sitter-bash) and safety classification for determining whether commands can be auto-approved.

## Key files

- `lib.rs` -- module declarations; re-exports `is_safe_command` and `is_dangerous_command` from the `command_safety` submodule.
- `bash.rs` -- tree-sitter-bash parsing engine. Key functions: `try_parse_shell()` parses bash source into a tree; `try_parse_word_only_commands_sequence()` extracts plain commands from scripts using only safe operators (&&, ||, ;, |); `extract_bash_command()` unwraps `bash -lc "..."` invocations; `parse_shell_lc_single_command_prefix()` handles heredoc scripts.
- `powershell.rs` -- PowerShell command extraction (`extract_powershell_command()`), UTF-8 output prefix injection, and executable discovery (`try_find_pwsh_executable_blocking()`, `try_find_powershell_executable_blocking()`).
- `shell_detect.rs` -- `detect_shell_type()` maps executable paths/names to `ShellType` enum (Zsh, Bash, Sh, PowerShell, Cmd).
- `parse_command.rs` -- additional command parsing utilities.
- `command_safety/` -- safety classification submodule.

## Imports from

- `tree-sitter`, `tree-sitter-bash` for AST-based parsing.
- `codex-utils-absolute-path`, `which`, `shlex`.

## Exports to

- Parent crate re-exports `is_safe_command` and `is_dangerous_command`.
- `bash`, `powershell`, `parse_command`, and `command_safety` modules are public.
