# codex-rs/core/src/tools/runtimes/shell/

Shell command execution runtime with zsh-fork backend support.

## What this folder does

Implements the shell command execution runtime, including a specialized zsh-fork backend for faster command execution on supported platforms.

### Zsh-fork backend (`zsh_fork_backend.rs`)
On supported Unix platforms, shell commands can be executed through an executable-level escalation server that forks zsh processes. This avoids the overhead of full process spawning for each command. The backend:
- Detects when a shell command matches the `zsh -c/-lc` pattern
- Prepares the execution through the escalation server
- Falls back to the normal shell runtime when the platform or command shape doesn't match

### Unix escalation (`unix_escalation.rs`)
Handles privilege escalation for shell commands on Unix platforms, managing the lifecycle of escalation server connections.

## Key files

| File | Purpose |
|------|---------|
| `zsh_fork_backend.rs` | `maybe_run_shell_command()`, `maybe_prepare_unified_exec()` -- zsh-fork shell execution |
| `unix_escalation.rs` | Unix-specific escalation server integration |
| `unix_escalation_tests.rs` | Tests for Unix escalation |

## Imports from

- `crate::tools::runtimes` -- `ShellRequest`, `UnifiedExecRequest`
- `crate::tools::sandboxing` -- `SandboxAttempt`, `ToolCtx`, `ToolError`
- `crate::sandboxing` -- `ExecRequest`
- `crate::unified_exec` -- `SpawnLifecycleHandle`

## Exports to

- `crate::tools::runtimes::shell` (parent `shell.rs`) -- used as an alternative backend for shell command execution
- `crate::tools::runtimes::unified_exec` -- zsh-fork preparation for unified exec processes
