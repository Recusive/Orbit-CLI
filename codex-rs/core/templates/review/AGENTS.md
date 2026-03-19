# codex-rs/core/templates/review/

This file applies to `codex-rs/core/templates/review/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-core` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-core`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Code review output format templates and history message templates.

### What this folder does

Defines the XML and markdown templates used for the code review workflow, including output format specifications and conversation history messages.

### Key files

| File | Purpose |
|------|---------|
| `exit_success.xml` | XML template for successful review completion output format |
| `exit_interrupted.xml` | XML template for interrupted review output format |
| `history_message_completed.md` | Markdown template for completed review messages in conversation history |
| `history_message_interrupted.md` | Markdown template for interrupted review messages in conversation history |

### Where it plugs into

- Loaded via `include_str!()` in `crate::review_prompts` and `crate::review_format`
- Used by `crate::tasks::review::ReviewTask` during code review execution
- Output templates define the structured format the review agent must use
