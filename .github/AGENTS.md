# .github/

This file applies to `.github/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- This subtree belongs to the `codex-monorepo` package. Keep `package.json` entry points, exports, and scripts aligned with source changes.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex && pnpm format`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

GitHub configuration root for the Codex repository. Contains CI/CD workflows, issue templates, composite actions for code signing, automation scripts, and Codex GitHub Action configuration.

### Purpose

Orchestrates all GitHub-level automation: continuous integration, release pipelines, issue triage, code signing, dependency updates, and contributor management.

### Directory Structure

| Path | Description |
|------|-------------|
| `ISSUE_TEMPLATE/` | GitHub issue form templates (bug reports, feature requests, docs) |
| `actions/` | Reusable composite actions for platform-specific code signing |
| `codex/` | Configuration for the Codex GitHub Action (model config, label-based prompts) |
| `scripts/` | Shell scripts used by CI workflows (musl build toolchain setup) |
| `workflows/` | GitHub Actions workflow definitions (CI, releases, issue automation) |

### Key Files

- **`blob-size-allowlist.txt`** -- Paths exempted from the 512 KB blob size policy enforced by `blob-size-policy.yml`. Entries are matched exactly relative to the repo root.
- **`dependabot.yaml`** -- Dependabot configuration tracking six ecosystems: bun, cargo, devcontainers, docker, github-actions, and rust-toolchain.
- **`dotslash-config.json`** -- DotSlash release artifact mapping for the `codex`, `codex-responses-api-proxy`, `codex-command-runner`, and `codex-windows-sandbox-setup` binaries across all supported platforms (macOS, Linux, Windows; x86_64 and aarch64).
- **`pull_request_template.md`** -- Default PR template directing contributors to `docs/contributing.md`.
- **`codex-cli-splash.png`** -- Splash image asset (~838 KB, allowlisted in blob-size-policy).

### Imports / Dependencies

- Workflows reference composite actions from `actions/` via `./.github/actions/<name>`.
- Workflows reference scripts from `scripts/` via `$GITHUB_WORKSPACE/.github/scripts/`.
- Workflows reference Bazel config from `workflows/ci.bazelrc`.
- `dotslash-config.json` is consumed by `facebook/dotslash-publish-release` during the release workflow.
- `blob-size-allowlist.txt` is consumed by `scripts/check_blob_size.py` via the `blob-size-policy.yml` workflow.

### Plugs Into

- GitHub's native issue template system (ISSUE_TEMPLATE/)
- GitHub Actions runtime (workflows/)
- Dependabot automated dependency updates
- DotSlash binary distribution system
