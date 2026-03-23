---
name: enforce-conventions
description: Review Orbit Code diffs, branches, pull requests, or specific files for compliance with repo coding conventions. Use when the user asks to enforce conventions, check coding standards, audit a diff or branch, review merge readiness, or verify changes against AGENTS.md and docs/pattern/CODING_CONVENTIONS.md. Also use proactively before opening a PR or after substantial implementation work in this repo.
---

# Enforce Conventions

Review the requested changes against the current repo conventions and report only concrete violations. Treat this as a code review: findings first, ordered by severity, with file and line references plus the shortest practical fix.

## 1. Determine the review scope

- If the user already named the scope, use it.
- If the scope is unclear, ask one concise question and offer:
  - uncommitted changes: `git diff` plus `git diff --cached`
  - branch vs `main`: `git diff main...HEAD`
  - specific PR: `gh pr diff <number>`
  - specific files: `git diff -- <paths>`
- Always collect a size summary with the matching `--stat` command.
- If the diff is larger than roughly 3000 lines, split the review by file or subsystem instead of reasoning about the whole thing at once.

## 2. Load conventions fresh

Always read these sources for the current review:

- repo root `AGENTS.md`
- `docs/pattern/CODING_CONVENTIONS.md`
- `references/checklist.md`

Then look for closer `AGENTS.md` files that govern the changed paths. For each changed file, search upward and read the nearest governing `AGENTS.md` if it differs from the repo root. Use the most specific instructions alongside the root conventions.

If a nearby `CLAUDE.md` exists and disagrees with `AGENTS.md`, call out the conflict explicitly instead of guessing.

## 3. Run mechanical checks first

Use `references/checklist.md` for the exact grep patterns and review prompts. Restrict grep-style checks to changed files whenever possible.

Always check:

- `unwrap()` and `expect()` in non-test Rust library code
- multi-imports in Rust (`use ...::{...}`)
- v2 serde violations: `skip_serializing_if` on `Option<T>`, missing `#[ts(export_to = "v2/")]`, bad optional collections, missing aligned `#[ts(rename)]`
- disallowed ratatui colors or methods
- missing `pretty_assertions::assert_eq`
- print macros in TUI crates
- opaque literal callsites missing `/*param*/` comments
- wildcard match arms on project enums
- new or modified code touching sandbox env vars
- `std::env::set_var` or other process-environment mutation in tests
- new API surface added to v1 instead of v2

## 4. Do structural review

Read the changed code and check it against the relevant convention categories:

- module organization, visibility, and file size limits
- error types and crate-local `Result<T>` aliases
- async patterns, cancellation, and retry behavior
- serde derives, `rename_all`, v2 payload rules, and experimental gating
- API design, especially bare `bool` or ambiguous `Option` params
- documentation requirements for modules and public items
- testing expectations, including snapshot coverage for UI changes
- dependency, schema, and lockfile follow-through
- TUI mirror rule: changes under `tui/` must be mirrored in `tui_app_server/` unless the diff documents why not
- near-duplicate blocks that should be extracted

Treat the repo's explicit warnings as critical:

- sandbox env var handling
- process environment mutation in tests
- missing Bazel data updates for `include_str!`, `include_bytes!`, or `sqlx::migrate!`
- new v1 API surface

## 5. Report findings

Follow the standard Codex review style:

- findings first
- order by severity: critical, moderate, minor
- each finding includes the rule number, `path:line`, what violates the convention, why it matters, and the shortest practical fix
- keep summaries brief and factual
- if there are no findings, say so explicitly and mention residual risk or unverified areas

Use this format when it helps:

```markdown
### [C1] Rule NN — Title
File: `path/to/file.rs:123`
Violation: What is wrong.
Fix: How to correct it.
```

Severity guidance:

- Critical: CI-breakers, denied clippy issues, `unwrap` or `expect` in lib code, missing TUI mirror, API contract breaks, root warning violations
- Moderate: non-blocking convention drift, stale comments, wildcard matches on project enums, avoidable `pub mod`, duplication
- Minor: docs gaps, naming polish, low-risk cleanup

## 6. Offer safe follow-up

After the report, offer to fix the auto-fixable items.

Usually safe to fix directly:

- formatting with `just fmt`
- missing module docs
- `pub mod` to `mod` plus selective `pub use`
- missing `#[ts(export_to = "v2/")]`
- missing `pretty_assertions::assert_eq`
- missing `/*param*/` comments

Ask before larger behavioral or architectural changes:

- error-type refactors
- module extraction
- duplication cleanup
- TUI mirror work with behavioral impact

## 7. Re-verify after fixes

If you fix anything, re-run the relevant mechanical checks and then suggest the smallest necessary verification commands, typically:

```bash
just fmt
just fix -p <crate>
cargo test -p <crate>
```

Add schema or lockfile regeneration only when the diff actually requires it.
