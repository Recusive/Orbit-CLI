# tools/

This file applies to `tools/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Read the files listed below before changing behavior in this subtree; keep neighboring docs and call sites consistent with any structural change.

## Validate
- Run the nearest package or crate tests that exercise this subtree.

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

### Purpose

Developer tooling for the Codex monorepo. Currently contains a single tool for enforcing Rust coding conventions.

### Contents

| Directory | Role |
|-----------|------|
| `argument-comment-lint/` | Dylint-based Rust lint that enforces `/*param*/` argument comments on anonymous literal arguments. See `tools/argument-comment-lint/CLAUDE.md` for details. |

### Relationship to Other Directories

- The lint tool targets the `codex-rs/` Rust workspace
- Referenced by the `justfile` at the repo root (`just argument-comment-lint`)
