# .github/codex/labels/

This file applies to `.github/codex/labels/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Workflow and automation changes should be validated against their callers. Prefer small, explicit changes to job names, permissions, and artifact paths.

## Validate
- No dedicated local build step for this directory; validate by checking the workflows or callers that reference it.

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Prompt templates for the Codex GitHub Action, triggered by applying specific labels to issues or pull requests.

### Purpose

Each Markdown file in this directory is a prompt template that defines what the Codex Action should do when the corresponding label is applied. The filename (minus `.md`) maps to the label name. Templates can include placeholder variables like `{CODEX_ACTION_ISSUE_TITLE}`, `{CODEX_ACTION_ISSUE_BODY}`, and `{CODEX_ACTION_GITHUB_EVENT_PATH}` that are substituted at runtime.

### Key Files

| File | Label | Purpose |
|------|-------|---------|
| `codex-attempt.md` | `codex-attempt` | Instructs Codex to attempt solving the reported issue: create a branch, commit a fix, and open a pull request. Receives the issue title and body as context. |
| `codex-review.md` | `codex-review` | Instructs Codex to review a PR and produce a concise Markdown summary with a few bullet points and a friendly review. Uses the GitHub event JSON to determine base/head refs. |
| `codex-rust-review.md` | `codex-rust-review` | Extended PR review prompt with Rust-specific review guidelines. Covers general principles (motivation, single-concern PRs), code organization (crate placement, file size), test assertions (deep vs. piecemeal), and tactical Rust advice (no `unsafe`, idiomatic patterns, sorted `Cargo.toml` deps, newtype pattern). |
| `codex-triage.md` | `codex-triage` | Instructs Codex to troubleshoot whether a reported issue is valid and provide a concise, respectful summary comment. |

### Template Variables

- `{CODEX_ACTION_ISSUE_TITLE}` -- Title of the issue that triggered the workflow.
- `{CODEX_ACTION_ISSUE_BODY}` -- Body text of the issue.
- `{CODEX_ACTION_GITHUB_EVENT_PATH}` -- Path to the JSON file containing the full GitHub webhook event payload (contains base/head refs for PRs).

### Plugs Into

- The `openai/codex-action@main` GitHub Action. When a label matching a filename is applied to an issue or PR, the action reads the corresponding template, substitutes variables, and executes the prompt using the model configured in `../home/config.toml`.
- Repository maintainers trigger these by manually applying labels (e.g., `codex-attempt`, `codex-triage`) to issues or PRs.
