# codex-rs/apply-patch/

Rust crate (`codex-apply-patch`) that parses and applies a custom patch format to the filesystem. It serves as both a library and a standalone CLI binary (`apply_patch`).

## What this folder does

Implements a patch engine for a stripped-down, file-oriented diff format (distinct from standard unified diff). The format supports three operations: Add File, Delete File, and Update File (with optional move/rename). The crate parses patch text into structured hunks, locates matching lines in existing files using fuzzy sequence matching, and applies the changes to the filesystem.

## What it plugs into

- **codex-core** uses this crate to intercept `apply_patch` tool calls from the LLM, verify correctness before execution, and compute unified diffs for display.
- **codex-cli / codex-exec** invoke the standalone `apply_patch` binary for sandboxed patch application.
- The `CODEX_CORE_APPLY_PATCH_ARG1` constant coordinates self-invocation from the `codex-arg0` dispatcher.

## Imports from / exports to

**External dependencies:** `anyhow`, `similar` (for unified diff generation), `thiserror`, `tree-sitter`, `tree-sitter-bash` (for parsing shell heredoc invocations).

**Exports (public API):**
- `apply_patch()` / `apply_hunks()` -- apply a patch string or parsed hunks to the filesystem
- `parse_patch()` -- parse patch text into `Vec<Hunk>`
- `maybe_parse_apply_patch_verified()` -- determine if an argv corresponds to an `apply_patch` invocation and produce verified `ApplyPatchAction` with file changes
- `unified_diff_from_chunks()` -- compute a standard unified diff from parsed update chunks
- `ApplyPatchAction`, `ApplyPatchFileChange`, `ApplyPatchError`, `Hunk`, `ParseError` -- core types
- `APPLY_PATCH_TOOL_INSTRUCTIONS` -- embedded markdown instructions for LLM tool usage
- `CODEX_CORE_APPLY_PATCH_ARG1` -- sentinel flag for self-invocation
- `main()` -- standalone binary entry point

## Key files

| File | Role |
|------|------|
| `Cargo.toml` | Crate manifest; defines both the library (`codex_apply_patch`) and binary (`apply_patch`) targets |
| `src/lib.rs` | Library root; patch application logic, hunk-to-filesystem application, unified diff generation, error types |
| `src/parser.rs` | Patch text parser; implements the Lark-like grammar for `*** Begin Patch` format with strict and lenient modes |
| `src/invocation.rs` | Argv parsing; detects `apply_patch` invocations from shell commands using tree-sitter Bash, extracts heredocs, resolves working directories |
| `src/seek_sequence.rs` | Fuzzy line-sequence matcher; finds old lines in source files with decreasing strictness (exact, rstrip, trim, Unicode normalization) |
| `src/standalone_executable.rs` | CLI entry point; reads patch from argv or stdin, invokes `apply_patch()`, handles exit codes |
| `src/main.rs` | Binary entry point; delegates to `standalone_executable::main()` |
| `apply_patch_tool_instructions.md` | LLM-facing documentation for the patch format, embedded at compile time via `include_str!` |
| `BUILD.bazel` | Bazel build rule |
| `tests/` | Integration tests (CLI tests, scenario-based filesystem tests) |
