# Upstream Tracking

> **Last updated:** 2026-03-24

Orbit Code is forked from [OpenAI Codex CLI](https://github.com/openai/codex). This document tracks the fork point, upstream versioning, and sync status.

---

## Git Remote

```
upstream  https://github.com/openai/codex.git
origin    https://github.com/Recusive/Orbit-Code.git
```

---

## Fork Point

| Field | Value |
|-------|-------|
| **Commit** | `01df50cf422b2eb89cb6ad8f845548e8c0d3c60c` |
| **Message** | "Add thread/shellCommand to app server API surface (#14988)" |
| **Date** | 2026-03-18 |
| **Branch** | `upstream/main` |
| **Nearest release** | `0.116.0-alpha.11` (tag: `rust-v0.116.0-alpha.11`) |
| **Relation to release** | Fork is 7 commits **past** the `0.116.0-alpha.11` release cut |

### Commits included past 0.116.0-alpha.11

These 7 commits from `main` were not yet in a tagged release when the fork was taken:

1. `4fd277461` — Add Python SDK thread.run convenience methods (#15088)
2. `903660edb` — Remove stdio transport from exec server (#15119)
3. `20f2a216d` — feat(core, tracing): create turn spans over websockets (#14632)
4. `b14689df3` — Forward session and turn headers to MCP HTTP requests (#15011)
5. `42e932d7b` — [hooks] turn_id extension for Stop & UserPromptSubmit (#15118)
6. `10eb3ec7f` — Simple directory mentions (#14970)
7. `01df50cf4` — Add thread/shellCommand to app server API surface (#14988)

---

## Upstream Versioning Model

OpenAI uses an unusual release workflow:

1. **Development** happens on `main` with `version = "0.0.0"` (placeholder) in `Cargo.toml`
2. **Releases** are cut to the `latest-alpha-cli` branch — a snapshot of `main` plus a single version-bump commit ("Release X.Y.Z-alpha.N")
3. The `latest-alpha-cli` branch is **force-pushed** with each release (no tag accumulation on the branch)
4. Release tags follow the pattern `rust-v0.X.0-alpha.Y`
5. The npm package (`@openai/codex`) uses `0.0.0-dev` on `main`

This means:
- Release tags are **not ancestors of `main`** — they're siblings (same base + 1 version-bump commit)
- To compare against a release, use the tag directly, not `main`
- `main` always has unreleased commits past the latest tag

---

## Current Sync Status

| | Version | Commit | Date |
|--|---------|--------|------|
| **Fork base** | ~0.116.0-alpha.11 + 7 | `01df50cf4` | 2026-03-18 |
| **Next sync target** | 0.117.0-alpha.12 | `48300ac21` | 2026-03-24 |
| **Delta** | 95 commits | | |

### Notable changes in 0.117.0-alpha.12

- **Multi-agent v2** — new sub-agent communication, structured output, path-like agent addressing
- **Plugins TUI** — install/uninstall menu, product-scoped plugins, featured plugins
- **Exec server** — remote exec, filesystem split between local and remote
- **V8 integration** — code mode on v8, V8 Bazel build
- **Auth refactor** — moved auth code into `login` crate
- **Hooks** — non-streaming shell-only PreToolUse support
- **Features crate** — split features into `codex-features` crate
- **Guardian** — follow-up reminders, approval context, denial rationale threading
- **TUI** — `/title` terminal title config, `/compact` follow-up queuing, Shift+Left tmux edit

---

## How to Check for Upstream Updates

```bash
# Fetch latest upstream state
git fetch upstream main latest-alpha-cli

# See current upstream release version
git show upstream/latest-alpha-cli:codex-rs/Cargo.toml | grep '^version'

# Compare fork point to latest release
git log --oneline rust-v0.116.0-alpha.11..upstream/latest-alpha-cli --first-parent

# Count commits behind
git rev-list --count 01df50cf4..upstream/main

# Full diff of what changed upstream since fork
git diff 01df50cf4..upstream/latest-alpha-cli -- codex-rs/
```

---

## Sync History

| Date | Action | From | To |
|------|--------|------|----|
| 2026-03-18 | Initial fork | — | `01df50cf4` (~0.116.0-alpha.11+7) |
