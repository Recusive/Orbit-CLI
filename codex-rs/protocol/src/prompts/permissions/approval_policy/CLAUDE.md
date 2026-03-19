# codex-rs/protocol/src/prompts/permissions/approval_policy/

Approval policy prompt templates -- one per `AskForApproval` variant.

## What this folder does

Contains markdown instructions that tell the agent how command approval works under the current policy. Each file corresponds to a different approval mode.

## Key files

- `never.md` -- `AskForApproval::Never`: all commands run without approval; agent must not provide `sandbox_permissions`
- `on_failure.md` -- `AskForApproval::OnFailure`: commands run in sandbox first; failures are escalated to user for approval to retry without sandbox
- `on_request_rule.md` -- `AskForApproval::UnlessTrusted` / rule-based: commands run outside sandbox if approved or matching an allow rule; includes detailed escalation request instructions, `sandbox_permissions: "require_escalated"`, `justification`, and `prefix_rule` guidance with examples and banned patterns
- `on_request_rule_request_permission.md` -- extended variant that also supports `sandbox_permissions: "with_additional_permissions"` for requesting sandboxed extra permissions (network, filesystem read/write) before falling back to full escalation
- `unless_trusted.md` -- `AskForApproval::UnlessTrusted`: most commands escalated for user approval, with a limited allowlist of safe read commands

## What it plugs into

Selected by `codex-core` based on the session's `AskForApproval` configuration. The chosen template is embedded in the system prompt.

## Exports to

Static markdown content consumed during system prompt assembly.
