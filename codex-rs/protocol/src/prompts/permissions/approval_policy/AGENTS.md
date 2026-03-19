# codex-rs/protocol/src/prompts/permissions/approval_policy/

This file applies to `codex-rs/protocol/src/prompts/permissions/approval_policy/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-protocol` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-protocol`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Approval policy prompt templates -- one per `AskForApproval` variant.

### What this folder does

Contains markdown instructions that tell the agent how command approval works under the current policy. Each file corresponds to a different approval mode.

### Key files

- `never.md` -- `AskForApproval::Never`: all commands run without approval; agent must not provide `sandbox_permissions`
- `on_failure.md` -- `AskForApproval::OnFailure`: commands run in sandbox first; failures are escalated to user for approval to retry without sandbox
- `on_request_rule.md` -- `AskForApproval::UnlessTrusted` / rule-based: commands run outside sandbox if approved or matching an allow rule; includes detailed escalation request instructions, `sandbox_permissions: "require_escalated"`, `justification`, and `prefix_rule` guidance with examples and banned patterns
- `on_request_rule_request_permission.md` -- extended variant that also supports `sandbox_permissions: "with_additional_permissions"` for requesting sandboxed extra permissions (network, filesystem read/write) before falling back to full escalation
- `unless_trusted.md` -- `AskForApproval::UnlessTrusted`: most commands escalated for user approval, with a limited allowlist of safe read commands

### What it plugs into

Selected by `codex-core` based on the session's `AskForApproval` configuration. The chosen template is embedded in the system prompt.

### Exports to

Static markdown content consumed during system prompt assembly.
