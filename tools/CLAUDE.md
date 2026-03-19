# tools/

## Purpose

Developer tooling for the Codex monorepo. Currently contains a single tool for enforcing Rust coding conventions.

## Contents

| Directory | Role |
|-----------|------|
| `argument-comment-lint/` | Dylint-based Rust lint that enforces `/*param*/` argument comments on anonymous literal arguments. See `tools/argument-comment-lint/CLAUDE.md` for details. |

## Relationship to Other Directories

- The lint tool targets the `codex-rs/` Rust workspace
- Referenced by the `justfile` at the repo root (`just argument-comment-lint`)
