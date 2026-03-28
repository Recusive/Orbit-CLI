# Permission Modes — Decouple Sandbox from Approval

## Context

Orbit Code currently couples the sandbox (filesystem access scope) with the approval system (permission dialogs). When a user sets `sandbox_mode = "danger-full-access"` to give the agent access to all paths on the machine, the approval prompts also stop firing — because `render_decision_for_unmatched_command()` in `core/src/exec_policy.rs:582-589` interprets `FileSystemSandboxKind::Unrestricted` as "just run commands without asking."

Four modes:

- **Default** — respects user's sandbox_mode and approval_policy config; preserves existing behavior exactly
- **Accept** — forces DangerFullAccess sandbox; auto-approves file mutations only, prompts for shell commands and other tools (MCP, etc.), forbids dangerous unmatched commands
- **Bypass** — forces DangerFullAccess sandbox; auto-approves non-dangerous commands, forbids dangerous unmatched commands (mirrors `AskForApproval::Never` semantics)
- **Plan** — already exists, no tool execution

**Reference:** Claude Code implements this via `permission_mode` field (`default`, `acceptEdits`, `plan`, `dontAsk`, `bypassPermissions`) passed to hooks. Orbit Code already has a superior approval mechanism (in-process oneshot channels + TUI overlay) that just needs its decision logic corrected.

**Last verified against codebase:** 2026-03-26

---

## Authoritative Behavior Matrix

This is the single source of truth. All prose, pseudocode, and tests must match this table. "Existing logic" means the current `render_decision_for_unmatched_command()` / `assess_patch_safety()` / `default_exec_approval_requirement()` code runs unchanged.

| Mode | Safe read (`ls`) | Non-dangerous shell (`npm test`) | Dangerous shell (`rm -rf`) | Safe apply_patch | Non-safe apply_patch | Other tool (MCP etc.) |
|------|-----------------|--------------------------------|---------------------------|-----------------|--------------------|-----------------------|
| **Default** | Allow | Existing logic | Existing logic (Prompt) | Existing logic (AutoApprove) | Existing logic (AskUser) | Existing logic |
| **Accept** | Allow | Prompt | Forbidden | Skip (auto) | Skip (auto) | Prompt |
| **Bypass** | Allow | Allow | Forbidden | Skip (auto) | Skip (auto) | Skip (auto) |
| **Plan** | Forbidden | Forbidden | Forbidden | Forbidden | Forbidden | Forbidden |

Key invariant: **Dangerous unmatched commands are never auto-approved.** Default prompts (existing behavior). Accept and Bypass return Forbidden — users wanting to allow specific dangerous commands use `.rules` allow entries.

---

## Why ModeKind, Not a Separate Abstraction

The repo already has a permission posture abstraction: `ApprovalPreset` in `codex-rs/utils/approval-presets/src/lib.rs`, which pairs `AskForApproval` + `SandboxPolicy`. There's also the TUI's `/permissions` popup that lets users pick "Read Only" / "Default" / "Full Access" presets.

We intentionally expand `ModeKind` rather than creating a new abstraction because:

1. **ModeKind already controls tool execution.** Plan mode prevents tool calls entirely — that's a permission decision, not a developer-instruction decision. Adding Accept/Bypass extends this existing axis.

2. **Collaboration modes are the primary user-facing UX.** Users switch modes via Shift+Tab and `/collab`. Having two independent mode systems creates contradictory UI state.

3. **ApprovalPresets are presentation-layer, not behavior-layer.** `ApprovalPreset` holds `AskForApproval` + `SandboxPolicy` values for the `/permissions` popup — it doesn't drive behavior. The behavior chain is `ModeKind` → `TurnContext` → tool execution.

4. **Hook `permission_mode` maps from approval state, not a separate concept.** The hook system already derives `permission_mode` from `AskForApproval` — it's a projection.

**Trade-offs accepted:** `ModeKind` variants now appear in `TurnStartedEvent.collaboration_mode_kind`, `collaborationMode/list` API, and developer-instruction preset resolution. Every consumer of `ModeKind` must handle the new variants. The consumers are enumerated in Phase 2 and Phase 7.

---

## Mode Surface: How Modes Are Set

### What exists today

`ModeKind` is **runtime session state**, not config state. There is no `mode` field in `Config` or `ConfigToml`. Current entry points:

- **TUI Shift+Tab / `/collab`** — cycles through `CollaborationModeMask` presets at runtime via `collaboration_modes::next_mask()`
- **App-server `collaborationMode/apply`** — applies a `CollaborationModeMask` to the session
- **`--full-auto`** CLI flag — sets `(WorkspaceWrite, OnRequest)` directly in `tui/src/lib.rs:268` and `tui_app_server/src/lib.rs:594`
- **`--dangerously-bypass-approvals-and-sandbox`** CLI flag — sets `(DangerFullAccess, Never)` directly

### What this plan adds

New modes are set the same way existing modes are — via collaboration mode presets applied at runtime:

1. **TUI Shift+Tab / `/collab`** — existing cycling automatically includes Accept/Bypass when presets and `is_tui_visible()` are updated (Phase 2)
2. **TUI `/accept` and `/bypass`** — direct slash commands (Phase 7)
3. **App-server `collaborationMode/apply`** — existing API, new preset values
4. **No config.toml field** — modes are not persisted. On restart/resume, mode resets to Default.
5. **No `--mode` CLI flag in v1** — defer to a follow-up. Existing `--dangerously-bypass-approvals-and-sandbox` already covers the CLI bypass case.

### Interaction with existing bypass flags

- `--full-auto` sets `(WorkspaceWrite, OnRequest)` — unrelated to ModeKind, continues to work as-is
- `--dangerously-bypass-approvals-and-sandbox` sets `(DangerFullAccess, Never)` — closest to Bypass mode but operates on `approval_policy` + `sandbox_policy` directly. Both can coexist: the flag sets the policy values, ModeKind operates at the tool-execution layer above those values.

---

## Model-Facing Instructions

### Current architecture

Model-facing permission instructions are assembled in `protocol/src/models.rs` via two functions:
- `build_approval_policy_prompt(approval_policy)` — selects from `include_str!` templates in `protocol/src/prompts/permissions/approval_policy/`
- `build_sandbox_policy_prompt(sandbox_policy)` — selects from `protocol/src/prompts/permissions/sandbox_mode/`

These run based on `approval_policy` and `sandbox_policy` values in `TurnContext`, independent of `ModeKind`.

Collaboration mode developer_instructions are a SEPARATE layer, loaded from `core/templates/collaboration_mode/*.md` presets.

### Problem

The existing `AskForApproval` prompt templates do NOT describe Accept/Bypass behavior:
- `OnRequest` template (`on_request_rule.md`) describes sandbox escalation, prefix rules, and `sandbox_permissions` parameters — none of which apply in Accept mode.
- `Never` template (`never.md`) says "commands will be rejected" — the exact opposite of Bypass, where commands auto-approve.

Mapping Accept → OnRequest or Bypass → Never would send the model contradictory instructions.

### Solution

In Accept and Bypass mode, **suppress the approval_policy prompt entirely**. The collaboration mode developer_instructions (from `core/templates/collaboration_mode/accept.md` / `bypass.md`) handle all permission-related guidance instead. The sandbox_policy prompt (`danger_full_access.md`: "No filesystem sandboxing - all commands are permitted") remains accurate and is still included.

This means:
- **Accept mode** → model sees `danger_full_access` sandbox prompt + Accept developer instructions (which describe: file ops auto-approved, shell prompts, no escalation parameters needed)
- **Bypass mode** → model sees `danger_full_access` sandbox prompt + Bypass developer instructions (which describe: all tools auto-approved, proceed autonomously)
- **Default/Plan** → existing approval + sandbox prompts unchanged

The collaboration mode templates in Phase 2.1 (`accept.md`, `bypass.md`) must cover the permission guidance that the suppressed approval prompt would have provided. See Phase 4.5 for the implementation.

---

## Constraint Handling

### During mode switch (TUI / app-server)

When a user switches to Accept or Bypass, the mode forces `DangerFullAccess`. This must pass the `Constrained<SandboxPolicy>` ceiling:

```rust
fn try_switch_mode(mode_kind: ModeKind, permissions: &Permissions) -> Result<(), String> {
    if mode_kind.forces_full_access() {
        if let Err(err) = permissions.sandbox_policy.can_set(&SandboxPolicy::DangerFullAccess) {
            return Err(format!(
                "Cannot switch to {} mode: {}",
                mode_kind.display_name(),
                err,
            ));
        }
    }
    Ok(())
}
```

Use `can_set()` (probe API) — NOT `try_set()` which doesn't exist. The actual sandbox override happens in `codex.rs` when `TurnContext` is built (Phase 4.4).

### Failed mode switch UX

If the switch fails due to requirements ceiling:
- **TUI:** Show a notification: "Cannot switch to Bypass mode: requirements restrict sandbox to read-only"
- **App-server:** Return error response on `collaborationMode/apply`
- The mode stays at its current value (no silent downgrade)

This is cleaner than silent downgrade because:
1. The user sees exactly why their request failed
2. No hidden state mismatch between what user requested and what runs
3. Consistent with how `Constrained::set()` works elsewhere

---

## Permissions Popup + Status Card

### Problem

The `/permissions` popup renders `ApprovalPreset` choices based on `approval_policy` + `sandbox_policy`. The status card shows these values too. In Accept/Bypass mode, ModeKind overrides behavior at the tool-execution layer, so these displays would show stale values.

### Solution

When in Accept or Bypass mode:

1. **Status card** — show the mode name and effective posture, not the raw config values. Files: `tui/src/status/card.rs`, `tui_app_server/src/status/card.rs`.

2. **Permissions popup** — disable with a message: "Permissions are controlled by [Accept/Bypass] collaboration mode. Switch to Default mode to use the permissions popup." This prevents contradictory state between two independent controls.

3. **Mode indicator** — the existing `CollaborationModeIndicator` in `chatwidget.rs` already shows the active mode. It just needs new match arms for Accept/Bypass.

---

## System Design — Data Flow

```
TUI / App-server
     │
     ├── User switches to Accept/Bypass via Shift+Tab or /accept or /bypass
     │   └── can_set() check on Constrained<SandboxPolicy>
     │       ├── fails → show error, stay on current mode
     │       └── passes → apply CollaborationModeMask with mode=Accept/Bypass
     │
     ▼
CollaborationMode.mode = ModeKind::Accept (runtime session state)
     │
     ▼
Turn starts → TurnContext built (core/src/codex.rs)
     │
     ├── Sandbox override: if forces_full_access() → constrained_sandbox_policy.set(DangerFullAccess)
     │
     ├── Model instructions: effective approval/sandbox derived from ModeKind
     │   → build_approval_policy_prompt(effective_approval)
     │   → build_sandbox_policy_prompt(effective_sandbox)
     │   → collaboration mode developer_instructions from preset
     │
     ├── Hook permission_mode: derived from ModeKind (not AskForApproval)
     │
     ▼
Tool call arrives
     │
     ├── exec_policy .rules evaluation (always first)
     │   → if matched: use rule decision
     │   → if unmatched: fall through
     │
     ├── render_decision_for_unmatched_command() with mode_kind
     │   (3 callsites: codex.rs, shell handler, unix_escalation)
     │
     ├── default_exec_approval_requirement() with mode_kind
     │   (2 callsites in orchestrator.rs: initial + retry)
     │
     └── orchestrator mode override (ToolCategory-based)
         (Accept auto-approves FileMutation, Bypass auto-approves all)
```

---

## Phase 1: Protocol Types

### 1.1 Extend `ModeKind` enum

**File:** `protocol/src/config_types.rs:314`

```rust
pub enum ModeKind {
    Plan,
    #[default]
    Default,
    #[serde(alias = "accept-edits", alias = "acceptEdits")]
    Accept,
    #[serde(alias = "bypass-permissions", alias = "bypassPermissions")]
    Bypass,
    #[doc(hidden)] PairProgramming,
    #[doc(hidden)] Execute,
}
```

Update all match arms exhaustively (Rule 33):
- `display_name()` — Accept → "Accept", Bypass → "Bypass"
- `is_tui_visible()` — include Accept/Bypass → true
- `TUI_VISIBLE_COLLABORATION_MODES` — 4-element array

### 1.2 Add helper methods

```rust
impl ModeKind {
    pub const fn forces_full_access(self) -> bool {
        matches!(self, Self::Accept | Self::Bypass)
    }
    pub const fn allows_tool_execution(self) -> bool {
        !matches!(self, Self::Plan)
    }
    pub const fn auto_approves_file_ops(self) -> bool {
        matches!(self, Self::Accept | Self::Bypass)
    }
    pub const fn auto_approves_shell(self) -> bool {
        matches!(self, Self::Bypass)
    }
}
```

### 1.3 Add `ToolCategory` enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolCategory { ReadOnly, FileMutation, Shell, Other }
```

### 1.4 Schema regeneration

```bash
just write-config-schema
just write-app-server-schema
```

---

## Phase 2: Collaboration Mode Presets + Templates

### 2.1 Add template files

**New files:** `core/templates/collaboration_mode/accept.md`, `core/templates/collaboration_mode/bypass.md`

Follow the existing pattern from `default.md`: include `<collaboration_mode>` marker, `{{KNOWN_MODE_NAMES}}` placeholder, and mode-specific behavior guidance.

### 2.2 Update `core/BUILD.bazel`

Currently exports only `default.md` and `plan.md`. Add:
```python
"templates/collaboration_mode/accept.md",
"templates/collaboration_mode/bypass.md",
```

### 2.3 Add presets with `developer_instructions`

**File:** `core/src/models_manager/collaboration_mode_presets.rs`

Presets MUST populate `developer_instructions: Some(Some(...))` — setting `None` breaks `app-server/src/orbit_code_message_processor.rs:595-611` `normalize_turn_start_collaboration_mode()`.

```rust
fn accept_preset() -> CollaborationModeMask {
    CollaborationModeMask {
        name: ModeKind::Accept.display_name().to_string(),
        mode: Some(ModeKind::Accept),
        model: None,
        reasoning_effort: None,
        developer_instructions: Some(Some(accept_mode_instructions())),
    }
}
```

Load via `include_str!("../../templates/collaboration_mode/accept.md")`.

### 2.4 Mirror in `tui_app_server/src/model_catalog.rs`

Add identical presets with identical developer_instructions.

---

## Phase 3: ModeKind Threading

### 3.1 Add `mode_kind` to `ExecApprovalRequest`

**File:** `core/src/exec_policy.rs:196`

```rust
pub(crate) struct ExecApprovalRequest<'a> {
    pub(crate) command: &'a [String],
    pub(crate) mode_kind: ModeKind,  // NEW
    // ... rest unchanged
}
```

### 3.2 Update ALL 3 construction sites

| Site | File | Value |
|------|------|-------|
| 1 | `core/src/codex.rs` | `turn_context.collaboration_mode.mode` |
| 2 | `core/src/unified_exec/process_manager.rs:599` | `context.turn.collaboration_mode.mode` |
| 3 | `core/src/tools/handlers/shell.rs:431` | `turn.collaboration_mode.mode` |

### 3.3 Update `unix_escalation.rs` direct fallback

**File:** `core/src/tools/runtimes/shell/unix_escalation.rs:784-792`

Thread `mode_kind` to the function's parameter list and pass to `render_decision_for_unmatched_command()`. Trace: the caller must receive `mode_kind` from `TurnContext`.

### 3.4 Add `ToolCategory` to `ToolRuntime` trait

**File:** `core/src/tools/sandboxing.rs`

```rust
fn tool_category(&self) -> ToolCategory { ToolCategory::Other }
```

Implement: `shell.rs` → Shell, `unified_exec.rs` → Shell, `apply_patch.rs` → FileMutation.

---

## Phase 4: Decision Logic

### 4.1 Rewrite `render_decision_for_unmatched_command()`

**File:** `core/src/exec_policy.rs:538-617`

Must match the Authoritative Behavior Matrix exactly:

```rust
pub fn render_decision_for_unmatched_command(
    mode_kind: ModeKind,
    approval_policy: AskForApproval,
    sandbox_policy: &SandboxPolicy,
    file_system_sandbox_policy: &FileSystemSandboxPolicy,
    command: &[String],
    sandbox_permissions: SandboxPermissions,
    used_complex_parsing: bool,
) -> Decision {
    if is_known_safe_command(command) && !used_complex_parsing {
        return match mode_kind {
            ModeKind::Plan => Decision::Forbidden,
            _ => Decision::Allow,
        };
    }

    if command_might_be_dangerous(command) {
        return match mode_kind {
            ModeKind::Bypass | ModeKind::Accept | ModeKind::Plan => Decision::Forbidden,
            ModeKind::Default | ModeKind::PairProgramming | ModeKind::Execute => {
                // Preserve existing behavior — delegates to approval_policy
                existing_dangerous_command_logic(approval_policy)
            }
        };
    }

    match mode_kind {
        ModeKind::Bypass => Decision::Allow,
        ModeKind::Accept => Decision::Prompt,
        ModeKind::Default | ModeKind::PairProgramming | ModeKind::Execute => {
            render_decision_for_default_mode(
                approval_policy, sandbox_policy, file_system_sandbox_policy,
                command, sandbox_permissions, used_complex_parsing,
            )
        }
        ModeKind::Plan => Decision::Forbidden,
    }
}
```

Extract `render_decision_for_default_mode()` — exact copy of current lines 550-616. The Windows `runtime_sandbox_provides_safety` check is preserved.

### 4.2 Update `default_exec_approval_requirement()` — BOTH callsites

**File:** `core/src/tools/sandboxing.rs:167`

Add `mode_kind: ModeKind` parameter. Only Bypass short-circuits here — Accept is NOT included because Accept only auto-approves `FileMutation` tools, and this function doesn't know the tool category. The Accept + FileMutation override happens in the orchestrator (Phase 4.3) which has access to `tool.tool_category()`.

```rust
pub(crate) fn default_exec_approval_requirement(
    mode_kind: ModeKind,
    policy: AskForApproval,
    file_system_sandbox_policy: &FileSystemSandboxPolicy,
) -> ExecApprovalRequirement {
    match mode_kind {
        ModeKind::Bypass => {
            return ExecApprovalRequirement::Skip {
                bypass_sandbox: true,
                proposed_execpolicy_amendment: None,
            };
        }
        ModeKind::Accept | ModeKind::Default | ModeKind::Plan
        | ModeKind::PairProgramming | ModeKind::Execute => {
            // Accept falls through — its FileMutation override is in the orchestrator.
            // Default/Plan/deprecated use existing logic.
        }
    }
    // ... existing logic unchanged ...
}
```

**Callsite 1:** `orchestrator.rs:121` (initial approval)
**Callsite 2:** `orchestrator.rs:252` (retry after sandbox denial)

Both receive `turn_ctx` and can access `turn_ctx.collaboration_mode.mode`.

### 4.3 Orchestrator mode override

**File:** `core/src/tools/orchestrator.rs` — after requirement resolution at line ~123

```rust
let requirement = match (turn_ctx.collaboration_mode.mode, tool.tool_category()) {
    (ModeKind::Bypass, _) => ExecApprovalRequirement::Skip {
        bypass_sandbox: true, proposed_execpolicy_amendment: None,
    },
    (ModeKind::Accept, ToolCategory::FileMutation) => ExecApprovalRequirement::Skip {
        bypass_sandbox: true, proposed_execpolicy_amendment: None,
    },
    _ => requirement,
};
```

### 4.4 Sandbox override in session/turn layer

**File:** `core/src/codex.rs` — where `TurnContext` is built from `SessionConfiguration`

`ModeKind` is runtime session state, not config state. The sandbox override must happen in the session/turn layer (codex.rs) where `TurnContext` is assembled, NOT in `config/mod.rs` which runs at config-load time before a session exists.

Apply the override after the `Constrained<SandboxPolicy>` is resolved and before it's stored in `TurnContext`:

```rust
// In TurnContext construction, after sandbox_policy is resolved from config:
let mode_kind = session_configuration.collaboration_mode.mode;
if mode_kind.forces_full_access() {
    if let Err(err) = constrained_sandbox_policy.set(SandboxPolicy::DangerFullAccess) {
        tracing::warn!(
            "mode {mode_kind:?} requires DangerFullAccess but requirements prevent it: {err}"
        );
        // Mode was already validated at switch time via can_set().
        // If we get here, requirements changed between switch and turn start.
        // Fall back to existing sandbox_policy — the mode's approval behavior
        // still applies (via exec_policy + orchestrator overrides), just without
        // the full-access sandbox.
    }
}
```

Uses `Constrained::set()` (the real API). This also correctly feeds the overridden sandbox into `TurnContext.file_system_sandbox_policy`, `TurnContext.network_sandbox_policy`, and the `ToolsConfig` that the orchestrator reads.

**NOT in `config/mod.rs`:** Config loading builds a `SessionConfiguration`, not a `TurnContext`. The `ModeKind` lives on `CollaborationMode` which is session state applied via `CollaborationModeMask`. By the time config/mod.rs runs, no collaboration mode is active yet.

### 4.5 Model-facing instruction override

**File:** `core/src/codex.rs` — where model prompt is assembled

The existing `AskForApproval` templates do NOT describe Accept/Bypass behavior accurately:
- `OnRequest` template (`on_request_rule.md`) describes sandbox escalation, prefix rules, and `sandbox_permissions` parameters — none of which apply in Accept mode where file ops auto-approve.
- `Never` template (`never.md`) says "commands will be rejected" — the opposite of Bypass, where commands auto-approve.

**Solution:** In Accept and Bypass mode, **suppress** the `approval_policy` prompt entirely. The collaboration mode developer_instructions (from `core/templates/collaboration_mode/accept.md` / `bypass.md`) handle all permission-related guidance instead. The `sandbox_policy` prompt (`danger_full_access.md`) is still accurate and should be included.

```rust
let mode_kind = turn_context.collaboration_mode.mode;

// Sandbox prompt: use DangerFullAccess for Accept/Bypass, existing for others
let effective_sandbox = if mode_kind.forces_full_access() {
    SandboxMode::DangerFullAccess
} else {
    turn_context.sandbox_mode()
};

// Approval prompt: suppress for Accept/Bypass (developer_instructions handle it)
let approval_prompt = match mode_kind {
    ModeKind::Accept | ModeKind::Bypass => None,
    ModeKind::Default | ModeKind::Plan
    | ModeKind::PairProgramming | ModeKind::Execute => {
        Some(DeveloperInstructions::approval_text(
            turn_context.approval_policy.value(),
            // ... existing params ...
        ))
    }
};
```

This means the model receives:
- **Accept:** sandbox prompt ("no filesystem sandboxing") + developer instructions ("file ops auto-approved, shell prompts") — no misleading escalation guidance.
- **Bypass:** sandbox prompt ("no filesystem sandboxing") + developer instructions ("all tools auto-approved") — no "commands will be rejected" contradiction.
- **Default/Plan:** existing approval + sandbox prompts unchanged.

The collaboration mode templates (`accept.md`, `bypass.md`) in Phase 2.1 must cover the permission guidance that the suppressed approval prompt would have provided.

---

## Phase 5: Hook Permission Mode

### 5.1 Centralize `hook_permission_mode()`

**File:** `core/src/hook_runtime.rs:267-276`

Replace AskForApproval-based mapping with ModeKind-based:

```rust
pub(crate) fn hook_permission_mode(turn_context: &TurnContext) -> String {
    match turn_context.collaboration_mode.mode {
        ModeKind::Plan => "plan",
        ModeKind::Default | ModeKind::PairProgramming | ModeKind::Execute => "default",
        ModeKind::Accept => "acceptEdits",
        ModeKind::Bypass => "bypassPermissions",
    }
    .to_string()
}
```

### 5.2 Update Stop hook inline mapping

**File:** `core/src/codex.rs:5760-5766`

Replace inline match with call to centralized function:
```rust
let stop_hook_permission_mode = crate::hook_runtime::hook_permission_mode(&turn_context);
```

### 5.3 No hook schema changes needed

`hooks/src/schema.rs:306-314` already enumerates all required values.

---

## Phase 6: Permissions Popup + Status Card

### 6.1 Disable `/permissions` in Accept/Bypass

**Files:** `tui/src/chatwidget.rs`, `tui_app_server/src/chatwidget.rs`

When the user runs `/permissions` while in Accept or Bypass mode:
```rust
if matches!(self.active_collaboration_mode_kind(), ModeKind::Accept | ModeKind::Bypass) {
    self.push_system_notification(
        "Permissions are controlled by the current collaboration mode. \
         Switch to Default mode to use the permissions popup."
    );
    return;
}
```

### 6.2 Status card shows effective posture

**Files:** `tui/src/status/card.rs`, `tui_app_server/src/status/card.rs`

When rendering approval/sandbox in the status card, check the active ModeKind:
- Accept → show "Accept mode (file ops auto-approved, shell prompts)"
- Bypass → show "Bypass mode (all auto-approved, dangerous forbidden)"
- Default/Plan → show existing approval_policy + sandbox_policy rendering

### 6.3 Mode indicator exhaustive match

**Files:** `tui/src/chatwidget.rs`, `tui_app_server/src/chatwidget.rs`

`CollaborationModeIndicator` and `collaboration_mode_indicator()` use exhaustive matches. Add arms for Accept and Bypass.

---

## Phase 7: TUI Integration

### 7.1 Mode cycling

Existing `collaboration_modes::next_mask()` auto-includes new modes via `is_tui_visible()` filter + new presets. No code change needed in `collaboration_modes.rs`.

### 7.2 Slash commands

**File:** `tui/src/slash_command.rs` — add `Accept`, `Bypass` variants
**File:** `tui/src/chatwidget.rs` — route via `mask_for_kind()` → `switch_collaboration_mode()`

Before switching, validate with `can_set()`:
```rust
SlashCommand::Bypass => {
    if let Err(msg) = try_switch_mode(ModeKind::Bypass, &self.config.permissions) {
        self.push_system_notification(&msg);
        return;
    }
    if let Some(mask) = collaboration_modes::mask_for_kind(models_manager, ModeKind::Bypass) {
        self.switch_collaboration_mode(mask);
    }
}
```

### 7.3 Mirror ALL TUI changes in `tui_app_server/`

| tui/ file | tui_app_server/ equivalent | Change |
|---|---|---|
| `src/slash_command.rs` | `src/slash_command.rs` | Add Accept, Bypass variants |
| `src/chatwidget.rs` | `src/chatwidget.rs` | Dispatch, mode indicator, permissions popup gate |
| `src/status/card.rs` | `src/status/card.rs` | Effective posture rendering |
| Snapshots | Snapshots | Accept new snapshots |

---

## Phase 8: Testing

### 8.1 Unit tests — exec_policy

**File:** `core/src/exec_policy_tests.rs` (existing)

Test every cell of the Authoritative Behavior Matrix.

### 8.2 Unit tests — sandboxing

**File:** `core/src/tools/sandboxing_tests.rs` (existing)

Test `default_exec_approval_requirement` with each ModeKind.

### 8.3 Unit tests — unix escalation

Test `render_decision_for_unmatched_command` direct call respects mode_kind.

### 8.4 Integration tests — hooks

**File:** `core/tests/suite/hooks.rs` (existing)

Assert SessionStart, UserPromptSubmit, and Stop hooks receive correct `permission_mode` for each ModeKind.

### 8.5 Integration tests — approval flows

**File:** `core/tests/suite/permission_modes.rs` (new, add to `tests/suite/mod.rs`)

TestCodexBuilder end-to-end for each mode.

### 8.6 TUI tests

**Files:** `tui/src/chatwidget/tests.rs`, `tui_app_server/src/chatwidget/tests.rs`

Mode indicator, slash command dispatch, permissions popup gate.

### 8.7 Snapshot tests

```bash
cargo test -p orbit-code-tui
cargo insta accept -p orbit-code-tui
```

---

## Files Changed (Complete List)

| File | Change |
|------|--------|
| `protocol/src/config_types.rs` | Add Accept, Bypass to ModeKind; ToolCategory enum; helpers |
| `core/templates/collaboration_mode/accept.md` | New template |
| `core/templates/collaboration_mode/bypass.md` | New template |
| `core/BUILD.bazel` | Export accept.md and bypass.md |
| `core/src/models_manager/collaboration_mode_presets.rs` | Add presets with developer_instructions |
| `core/src/exec_policy.rs` | mode_kind in ExecApprovalRequest; rewrite render_decision; extract default_mode |
| `core/src/tools/sandboxing.rs` | mode_kind in default_exec_approval_requirement; tool_category() trait method |
| `core/src/tools/orchestrator.rs` | Mode override; thread mode_kind to both default_exec callsites |
| `core/src/tools/handlers/shell.rs` | mode_kind in ExecApprovalRequest construction |
| `core/src/tools/runtimes/shell/unix_escalation.rs` | Thread mode_kind to direct fallback |
| `core/src/tools/runtimes/shell.rs` | tool_category() → Shell |
| `core/src/tools/runtimes/unified_exec.rs` | tool_category() → Shell |
| `core/src/tools/runtimes/apply_patch.rs` | tool_category() → FileMutation |
| `core/src/codex.rs` | Sandbox override via Constrained::set(); ExecApprovalRequest mode_kind; effective approval/sandbox for prompts; stop hook |
| `core/src/unified_exec/process_manager.rs` | ExecApprovalRequest mode_kind |
| `core/src/hook_runtime.rs` | Rewrite hook_permission_mode from ModeKind |
| `tui/src/slash_command.rs` | Add Accept, Bypass |
| `tui/src/chatwidget.rs` | Dispatch, indicator, permissions popup gate |
| `tui/src/status/card.rs` | Effective posture rendering |
| `tui_app_server/src/model_catalog.rs` | Mirror presets |
| `tui_app_server/src/slash_command.rs` | Mirror slash commands |
| `tui_app_server/src/chatwidget.rs` | Mirror dispatch, indicator, popup gate |
| `tui_app_server/src/status/card.rs` | Mirror status card |
| `core/src/exec_policy_tests.rs` | Behavior matrix tests |
| `core/src/tools/sandboxing_tests.rs` | Mode override tests |
| `core/tests/suite/permission_modes.rs` | Integration tests |
| `core/tests/suite/hooks.rs` | Hook permission_mode assertions |

---

## Edge Cases & Decisions

| Edge Case | Decision |
|---|---|
| No config.toml `mode` field | Modes are runtime session state. No config field in v1. |
| No `--mode` CLI flag | Defer to follow-up. Use `--dangerously-bypass-approvals-and-sandbox` for CLI bypass. |
| Requirements ceiling prevents DangerFullAccess | Mode switch rejected with error message (not silent downgrade). |
| Requirements change between mode switch and turn start | `Constrained::set()` in config phase will fail; fall back to existing sandbox. |
| Mode persistence across resume | Modes do NOT persist. Reset to Default on resume. |
| Mode switch mid-turn | Takes effect on next turn. |
| `.rules` deny vs Bypass | `.rules` evaluated first, denials override. |
| Default semantics for apply_patch | Existing behavior preserved (safe patches auto-approve). |
| `/permissions` in Accept/Bypass | Disabled with message. |
| Status card in Accept/Bypass | Shows mode-derived effective posture. |
| Hook `permission_mode` after failed downgrade | If mode switch was rejected, mode stays at Default, hook emits "default". |
| Accept + non-file-mutation tool (MCP, etc.) | Prompts for approval (see behavior matrix — Accept only auto-approves FileMutation). |
| App-server `collaborationMode/apply` failure | Return JSON-RPC error with `message` explaining the requirements ceiling. Client renders the error message. |
| `--full-auto` + `/accept` in-session | `--full-auto` sets approval/sandbox at startup. `/accept` overrides behavior at the tool-execution layer. Both coexist — ModeKind operates above the config-set values. |
| `--dangerously-bypass-approvals-and-sandbox` + `/bypass` | Functionally equivalent but independent: the flag sets `(DangerFullAccess, Never)`, `/bypass` sets ModeKind. Both result in no prompts + dangerous forbidden. |
| Bypass user-facing copy | "All non-dangerous tools auto-approved. Dangerous unmatched commands are forbidden — use .rules allow entries to permit specific dangerous commands." |

---

## Verification

Per-phase:
```bash
just fmt
just fix -p orbit-code-protocol
just fix -p orbit-code-core
just fix -p orbit-code-tui
just fix -p orbit-code-tui-app-server
cargo test -p orbit-code-protocol
cargo test -p orbit-code-core
cargo test -p orbit-code-tui
cargo test -p orbit-code-app-server-protocol
```

Targeted:
```bash
cargo test -p orbit-code-core -- exec_policy
cargo test -p orbit-code-core -- sandboxing
cargo test -p orbit-code-core -- unix_escalation
cargo test -p orbit-code-core -- suite::hooks
cargo test -p orbit-code-core -- suite::exec_policy
cargo test -p orbit-code-core -- suite::approvals
cargo test -p orbit-code-tui -- chatwidget
cargo test -p orbit-code-tui-app-server -- chatwidget
```

End-to-end:
```bash
just codex   # Default: should prompt for shell commands
# Then /accept → file ops auto-approve, shell prompts
# Then /bypass → all auto-approve, dangerous forbidden
# Then /default → back to normal
```
