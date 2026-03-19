# codex-rs/apply-patch/src/

This file applies to `codex-rs/apply-patch/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-apply-patch` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-apply-patch`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source code for the `codex-apply-patch` crate. Contains the library and binary entry points for parsing and applying the custom patch format.

### What this folder does

Implements all core logic: patch text parsing, hunk application to the filesystem, fuzzy line matching, shell invocation detection (heredoc extraction via tree-sitter), and the standalone CLI binary.

### What it plugs into

- The library is consumed by `codex-core` for verified patch application and diff computation.
- The binary (`apply_patch`) is invoked by `codex-cli` and `codex-exec` as a subprocess.

### Key files

| File | Role |
|------|------|
| `lib.rs` | Library entry point. Defines `apply_patch()`, `apply_hunks()`, `unified_diff_from_chunks()`, error types (`ApplyPatchError`, `IoError`), result types (`ApplyPatchAction`, `ApplyPatchFileChange`, `MaybeApplyPatchVerified`), and the `APPLY_PATCH_TOOL_INSTRUCTIONS` constant. Contains the filesystem application logic (`apply_hunks_to_files`, `derive_new_contents_from_chunks`, `compute_replacements`, `apply_replacements`). |
| `parser.rs` | Patch text parser. Parses the `*** Begin Patch ... *** End Patch` format into `Vec<Hunk>` where each `Hunk` is `AddFile`, `DeleteFile`, or `UpdateFile`. Supports strict and lenient modes (lenient strips heredoc wrappers). Defines `Hunk`, `UpdateFileChunk`, and `ParseError` types. |
| `invocation.rs` | Shell invocation detector. Uses tree-sitter with a Bash grammar to recognize `apply_patch` heredoc invocations, including `cd <path> && apply_patch <<'EOF'` forms. Defines `maybe_parse_apply_patch()` and `maybe_parse_apply_patch_verified()`. Supports Unix shells (bash/zsh/sh), PowerShell, and cmd.exe. |
| `seek_sequence.rs` | Fuzzy line-sequence matcher. `seek_sequence()` locates a pattern of lines within a file with four levels of decreasing strictness: exact, trailing-whitespace-trimmed, fully-trimmed, and Unicode-normalized (maps typographic dashes/quotes/spaces to ASCII equivalents). |
| `standalone_executable.rs` | CLI runtime. Reads the patch from argv[1] or stdin, calls `apply_patch()`, and returns an appropriate exit code. |
| `main.rs` | Binary entry point. Delegates to `standalone_executable::main()`. |

### Imports / exports

- `lib.rs` re-exports `parser::parse_patch`, `parser::Hunk`, `parser::ParseError`, `invocation::maybe_parse_apply_patch_verified`, and `standalone_executable::main`.
- `parser.rs` imports `ApplyPatchArgs` from `lib.rs` (via `crate::`).
- `invocation.rs` imports types from both `parser` and `lib.rs`.
- `seek_sequence.rs` is `pub(crate)` only -- used by `lib.rs` in `compute_replacements()`.
