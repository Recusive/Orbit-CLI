# codex-rs/execpolicy-legacy/src/

This file applies to `codex-rs/execpolicy-legacy/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-execpolicy-legacy` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-execpolicy-legacy`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source code for the `codex-execpolicy-legacy` crate.

### What this folder does

Implements the legacy argument-aware exec policy engine. This engine understands per-program argument semantics -- it can parse flags, options, positional arguments, and special syntax like sed commands to determine whether a proposed shell command is safe.

### Key files and their roles

- `lib.rs` -- Module declarations and public re-exports. Embeds `default.policy` via `include_str!` and provides `get_default_policy()`.
- `main.rs` -- Binary entry point. Supports `check` (execv-style args) and `check-json` (JSON input with program/args) subcommands. Outputs JSON categorized as safe, match, forbidden, or unverified. Uses exit codes: 0 (ok), 12 (matched but writes files), 13 (might be safe), 14 (forbidden).
- `policy.rs` -- `Policy` struct with `check(&ExecCall)` method that returns `MatchedExec`.
- `policy_parser.rs` -- `PolicyParser`: Starlark evaluator for the legacy policy DSL with custom built-in functions for defining program specs.
- `program.rs` -- `ProgramSpec` (program argument schema definition), `MatchedExec` (Match or Forbidden result), `Forbidden` cause type, example validation types.
- `valid_exec.rs` -- `ValidExec`: a validated execution result containing matched arguments, flags, and options. Has `might_write_files()` to determine if the command could be destructive.
- `execv_checker.rs` -- `ExecvChecker`: core matching logic that checks an `ExecCall` against a `ProgramSpec`.
- `exec_call.rs` -- `ExecCall`: input type with `program` and `args` fields.
- `opt.rs` -- `Opt`: option definition with short/long names, argument type, and metadata.
- `arg_type.rs` -- `ArgType` enum: classifies arguments (file, directory, string, etc.) for write-safety analysis.
- `arg_matcher.rs` -- `ArgMatcher`: pattern-based argument validator using regex.
- `arg_resolver.rs` -- `PositionalArg` and argument resolution logic for mapping command tokens to program spec slots.
- `sed_command.rs` -- `parse_sed_command()`: specialized parser for sed command syntax to evaluate sed-specific policy rules.
- `error.rs` -- `Error` type for policy check failures.
- `default.policy` -- Built-in Starlark policy defining safe commands (ls, cat, head, grep, find, etc.) with detailed argument specs.

### Imports from

- `starlark`: Starlark interpreter
- `allocative`: memory-aware Starlark types
- `multimap`: rule indexing
- `regex-lite`: argument pattern matching
