# .github/codex/

This file applies to `.github/codex/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Workflow and automation changes should be validated against their callers. Prefer small, explicit changes to job names, permissions, and artifact paths.

## Validate
- No dedicated local build step for this directory; validate by checking the workflows or callers that reference it.

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Configuration directory for the Codex GitHub Action (`openai/codex-action`). Contains model settings and label-triggered prompt templates used by automated CI workflows.

### Purpose

Provides the Codex GitHub Action with its runtime configuration (model selection) and task-specific prompts that are triggered when certain labels are applied to issues or PRs. This enables AI-assisted triage, code review, and issue resolution within the repository's CI pipelines.

### Directory Structure

| Directory | Description |
|-----------|-------------|
| `home/` | Codex home directory config (model selection via `config.toml`) |
| `labels/` | Markdown prompt templates keyed by label name. When a label matching a filename is applied, the corresponding prompt is executed by the Codex Action. |

### Plugs Into

- The `openai/codex-action@main` GitHub Action reads these configuration files.
- Label-triggered prompts are used by workflows like `issue-labeler.yml` and `issue-deduplicator.yml`, and can be triggered manually by applying labels such as `codex-review`, `codex-triage`, or `codex-attempt` to issues/PRs.
- The `config.toml` sets the default model used when the Codex Action runs.
