# .github/ISSUE_TEMPLATE/

GitHub issue form templates that provide structured bug reports and feature requests through GitHub's issue forms UI.

## Purpose

Defines six YAML-based issue forms that guide users through reporting problems or requesting features for different Codex products. Each template creates a distinct form with required fields, dropdowns, and text areas.

## Key Files

| File | Form Name | Labels Applied | Description |
|------|-----------|----------------|-------------|
| `1-codex-app.yml` | Codex App Bug | `app` | Bug reports for the Codex desktop application. Collects version, subscription plan, platform, and reproduction steps. |
| `2-extension.yml` | IDE Extension Bug | `extension` | Bug reports for VS Code / IDE extensions. Additionally collects which IDE is in use. |
| `3-cli.yml` | CLI Bug | `bug`, `needs triage` | Bug reports for the Codex CLI. Collects CLI version (via `codex --version`), model name, and terminal emulator info. |
| `4-bug-report.yml` | Other Bug | `bug` | General bug reports for Codex Web, integrations, or other components not covered above. |
| `5-feature-request.yml` | Feature Request | `enhancement` | Feature proposals. Asks which variant (App, Extension, CLI, Web) the request targets. |
| `6-docs-issue.yml` | Documentation Issue | `docs` | Reports missing, incorrect, or confusing documentation. Includes a dropdown for issue type. |

## Plugs Into

- GitHub's native issue forms system. When a user clicks "New Issue" on the repository, GitHub renders these YAML files as structured web forms.
- The `issue-labeler.yml` workflow further enriches labels on opened issues using the Codex GitHub Action.
- The `issue-deduplicator.yml` workflow checks newly opened issues for potential duplicates.

## Imports / Exports

- No code imports. These are declarative YAML consumed directly by GitHub.
- Labels referenced here (`app`, `extension`, `bug`, `needs triage`, `enhancement`, `docs`) must exist in the repository's label set for auto-labeling to work.
