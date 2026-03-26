# Permission Modes — Decouple Sandbox from Approval

## Context

Orbit Code currently couples the sandbox (filesystem access scope) with the approval system (permission dialogs). When a user sets `sandbox_mode = "danger-full-access"` to give the agent access to all paths on the machine, the approval prompts also stop firing — because `render_decision_for_unmatched_command()` in `core/src/exec_policy.rs:582-589` interprets `FileSystemSandboxKind::Unrestricted` as "just run commands without asking."

The user wants full system access (no path restrictions) with a separate permission mode controlling whether and when the agent asks before acting. Four modes:

- **Default** — reads free, every actionable tool (shell, write, patch, edit) asks for permission
- **Accept** — all file operations (write, edit, apply_patch) auto-approve anywhere, only shell commands ask
- **Bypass** — nothing asks, everything executes immediately
- **Plan** — already exists, no tool execution

**Reference:** Claude Code implements this via `permission_mode` field (`default`, `acceptEdits`, `auto`, `dontAsk`, `bypassPermissions`) passed to hooks. Orbit Code already has a superior approval mechanism (in-process oneshot channels + TUI overlay) that just needs its decision logic corrected.

**Last verified against codebase:** 2026-03-25

---

## System Design — Complete Data Flow

### Current Architecture (the bug)

```
User sets: sandbox_mode = "danger-full-access"
     │
     ▼
SandboxPolicy::DangerFullAccess
     │
     ▼
FileSystemSandboxPolicy::unrestricted()
     │
     ▼
FileSystemSandboxKind::Unrestricted
     │
     ▼
render_decision_for_unmatched_command() [core/src/exec_policy.rs:582]
     │
     ├── AskForApproval::OnRequest + Unrestricted
     │   → "The user has indicated we should just run commands"
     │   → Decision::Allow                    ← APPROVAL SILENTLY SKIPPED
     │
     └── Result: full access AND no permission gates
         Both concerns are coupled in one match arm
```

### Target Architecture (decoupled)

```
config.toml:
  mode = "default"          # or "accept", "bypass"
     │
     ▼
ModeKind enum (protocol/src/config_types.rs)
  ├── Default   ─┐
  ├── Accept    ─┤── All force SandboxPolicy::DangerFullAccess
  ├── Bypass    ─┘   (full system access, no path restrictions)
  └── Plan      ─── No tool execution (existing behavior)
     │
     ▼
Tool call arrives (shell, apply_patch, file_write, etc.)
     │
     ▼
┌─────────────────────────────────────────────────────────────┐
│ NEW: Permission mode check (Layer 0 — before everything)     │
│                                                              │
│ match mode_kind {                                            │
│   Bypass  → ExecApprovalRequirement::Skip (always)           │
│   Accept  → is_file_operation(tool)?                         │
│              yes → Skip (auto-approve)                       │
│              no  → NeedsApproval                             │
│   Default → is_read_only_operation(tool)?                    │
│              yes → Skip                                      │
│              no  → NeedsApproval                             │
│   Plan    → unreachable (no tools in plan mode)              │
│ }                                                            │
└──────────────────────┬──────────────────────────────────────┘
                       │ if NeedsApproval
                       ▼
              Existing oneshot channel mechanism
              TUI approval overlay (unchanged)
              User picks: allow / deny / always allow
```

### What Changes vs What Stays

```
STAYS UNCHANGED:
  ✓ oneshot channel blocking mechanism (core/src/codex.rs:2843-2913)
  ✓ TUI approval overlay (tui/src/bottom_pane/approval_overlay.rs)
  ✓ ReviewDecision enum (Approved, ApprovedForSession, Denied, Abort)
  ✓ ApprovalStore session cache (core/src/tools/sandboxing.rs)
  ✓ ToolOrchestrator::run() flow (core/src/tools/orchestrator.rs)
  ✓ Starlark .rules files (still work as refinement layer)
  ✓ Hooks system (SessionStart, UserPromptSubmit, Stop)

CHANGES:
  ✗ ModeKind enum — add Accept, Bypass variants
  ✗ SandboxPolicy resolution — force DangerFullAccess for non-Plan modes
  ✗ render_decision_for_unmatched_command() — check ModeKind before sandbox
  ✗ create_exec_approval_requirement_for_command() — thread ModeKind through
  ✗ Config TOML — surface mode field
  ✗ TUI — add /accept, /bypass commands, display current mode
  ✗ CLI — add --mode flag
  ✗ System prompt templates — per-mode instructions
```

---

## Phase 1: Protocol Types

### 1.1 Extend `ModeKind` enum

**File:** `protocol/src/config_types.rs:314`

Current:
```rust
pub enum ModeKind {
    Plan,
    #[default]
    Default,
    #[doc(hidden)] PairProgramming,  // deprecated alias → Default
    #[doc(hidden)] Execute,          // deprecated alias → Default
}
```

Add two new variants:
```rust
pub enum ModeKind {
    Plan,
    #[default]
    Default,
    /// Auto-approve all file operations (write, edit, apply_patch) anywhere.
    /// Shell commands still require approval.
    #[serde(alias = "accept-edits", alias = "acceptEdits")]
    Accept,
    /// No approval prompts. All tools execute immediately.
    /// Full system access, no restrictions.
    #[serde(alias = "bypass-permissions", alias = "bypassPermissions")]
    Bypass,
    #[doc(hidden)] PairProgramming,
    #[doc(hidden)] Execute,
}
```

Update `TUI_VISIBLE_COLLABORATION_MODES` to include all 4:
```rust
pub const TUI_VISIBLE_COLLABORATION_MODES: [ModeKind; 4] = [
    ModeKind::Default,
    ModeKind::Plan,
    ModeKind::Accept,
    ModeKind::Bypass,
];
```

Update `display_name()`:
```rust
Self::Accept => "Accept",
Self::Bypass => "Bypass",
```

Update `is_tui_visible()`:
```rust
matches!(self, Self::Plan | Self::Default | Self::Accept | Self::Bypass)
```

### 1.2 Add helper methods to `ModeKind`

```rust
impl ModeKind {
    /// Whether this mode forces full system access (no sandbox path restrictions).
    pub const fn forces_full_access(self) -> bool {
        matches!(self, Self::Default | Self::Accept | Self::Bypass)
    }

    /// Whether this mode allows tool execution at all.
    pub const fn allows_tool_execution(self) -> bool {
        !matches!(self, Self::Plan)
    }

    /// Whether this mode auto-approves file operations (write, edit, patch).
    pub const fn auto_approves_file_ops(self) -> bool {
        matches!(self, Self::Accept | Self::Bypass)
    }

    /// Whether this mode auto-approves shell commands.
    pub const fn auto_approves_shell(self) -> bool {
        matches!(self, Self::Bypass)
    }
}
```

### 1.3 Add system prompt templates

**New files:**
- `protocol/src/prompts/permissions/mode/default.md`
- `protocol/src/prompts/permissions/mode/accept.md`
- `protocol/src/prompts/permissions/mode/bypass.md`

Content example for `default.md`:
```markdown
You have full system access — you can read and write files anywhere on this
machine. However, every actionable operation (shell commands, file writes,
patches) requires explicit user approval before execution. Read-only operations
proceed automatically.
```

Wire via `include_str!` in `protocol.rs` alongside existing prompt templates.

**Bazel:** Update `protocol/BUILD.bazel` `compile_data` with the new template paths.

### 1.4 Run schema regeneration

```bash
just write-config-schema     # ModeKind changed
just write-app-server-schema # Protocol types changed
```

---

## Phase 2: Sandbox Override

### 2.1 Force `DangerFullAccess` for non-Plan modes

**File:** `core/src/config/mod.rs` — where `SandboxPolicy` is resolved from `SandboxMode`

Currently `SandboxMode` → `SandboxPolicy` mapping happens around lines 1781-1798. Add an override:

```rust
// After resolving sandbox_policy from config:
if mode_kind.forces_full_access() {
    sandbox_policy = SandboxPolicy::DangerFullAccess;
}
```

This means:
- `Default` mode → `DangerFullAccess` (full access + approval gates)
- `Accept` mode → `DangerFullAccess` (full access + file ops auto-approved)
- `Bypass` mode → `DangerFullAccess` (full access + no gates)
- `Plan` mode → whatever the config says (but no tools run anyway)

### 2.2 Thread `ModeKind` into `Op::UserTurn`

**File:** `protocol/src/protocol.rs` — `Op::UserTurn` struct

The `ModeKind` needs to reach the exec policy decision function. It's already carried in `Op::UserTurn` via the `CollaborationMode` field. Verify that `mode_kind` is accessible in the tool execution path.

Trace: `Op::UserTurn.collaboration_mode.mode` → `TurnContext` → tool handlers → `ExecPolicyManager`. If not already threaded, add `mode_kind: ModeKind` to `ExecApprovalRequest` (the struct passed to `create_exec_approval_requirement_for_command()`).

---

## Phase 3: Decision Logic

### 3.1 Modify `render_decision_for_unmatched_command()`

**File:** `core/src/exec_policy.rs:538-617`

This is the core fix. Add `mode_kind: ModeKind` parameter and check it FIRST:

```rust
pub fn render_decision_for_unmatched_command(
    mode_kind: ModeKind,           // NEW parameter
    approval_policy: AskForApproval,
    sandbox_policy: &SandboxPolicy,
    file_system_sandbox_policy: &FileSystemSandboxPolicy,
    command: &[String],
    sandbox_permissions: SandboxPermissions,
    used_complex_parsing: bool,
) -> Decision {
    // NEW: Mode-based routing takes priority
    match mode_kind {
        ModeKind::Bypass => return Decision::Allow,
        ModeKind::Plan => {
            // Should never reach here — plan mode doesn't execute tools
            return Decision::Forbidden;
        }
        // Default and Accept fall through to refined logic below
        _ => {}
    }

    // Dangerous commands ALWAYS prompt (even in Accept mode)
    if command_might_be_dangerous(command) {
        return match mode_kind {
            ModeKind::Bypass => Decision::Allow,  // already handled above
            _ => Decision::Prompt,
        };
    }

    // Known safe READ commands are free in all non-Bypass modes
    if is_known_safe_command(command) && !used_complex_parsing {
        return Decision::Allow;
    }

    // Mode-specific behavior for remaining commands
    match mode_kind {
        ModeKind::Default => {
            // Everything that isn't a known-safe read needs approval
            Decision::Prompt
        }
        ModeKind::Accept => {
            // Shell commands need approval in Accept mode
            // (File ops are handled separately in the orchestrator)
            Decision::Prompt
        }
        _ => {
            // Fallback to existing logic for any other mode
            // (preserves backward compatibility)
            /* ... existing match on approval_policy ... */
        }
    }
}
```

### 3.2 Handle file operations in Accept mode

**File:** `core/src/tools/orchestrator.rs:100-347`

For non-shell tools (apply_patch, file_write, file_edit), the approval path is different. Add a mode check in `ToolOrchestrator::run()`:

```rust
// In ToolOrchestrator::run(), before the approval gate:
if let Some(requirement) = tool.exec_approval_requirement(&req) {
    // NEW: Accept mode auto-approves file operations
    let requirement = if mode_kind.auto_approves_file_ops()
        && tool.is_file_operation()
    {
        ExecApprovalRequirement::Skip {
            bypass_sandbox: true,
            proposed_execpolicy_amendment: None,
        }
    } else {
        requirement
    };
    // ... existing approval logic ...
}
```

This requires adding `is_file_operation()` to the tool trait or checking the tool name.

### 3.3 Update `create_exec_approval_requirement_for_command()` signature

**File:** `core/src/exec_policy.rs:226-310`

Thread `mode_kind` through to `render_decision_for_unmatched_command()`:

```rust
pub fn create_exec_approval_requirement_for_command(
    &self,
    request: ExecApprovalRequest,  // Add mode_kind to this struct
) -> ExecApprovalRequirement {
    // ... existing command parsing ...

    let exec_policy_fallback = |cmd: &[String]| {
        render_decision_for_unmatched_command(
            request.mode_kind,     // NEW: pass through
            approval_policy,
            sandbox_policy,
            file_system_sandbox_policy,
            cmd,
            sandbox_permissions,
            used_complex_parsing,
        )
    };
    // ... rest unchanged ...
}
```

---

## Phase 4: Config & CLI

### 4.1 TOML config field

**File:** `core/src/config/mod.rs`

The `mode` field already exists in `CollaborationMode` → `ModeKind`. Verify it's exposed in the TOML schema. If not, add to the TOML config layer:

```toml
# ~/.orbit/config.toml
mode = "default"    # or "accept", "bypass", "plan"
```

### 4.2 CLI flag

**File:** `cli/` — argument parsing

Add `--mode` flag:

```rust
#[clap(long, value_enum, default_value = "default")]
mode: ModeKind,
```

### 4.3 Regenerate schemas

```bash
just write-config-schema
just write-app-server-schema
```

---

## Phase 5: TUI Integration

### 5.1 Mode switching — Shift+Tab keybinding

**File:** `tui/src/app.rs` — key event handling

Add `Shift+Tab` to cycle through modes: Default → Accept → Bypass → Plan → Default.

```rust
// In handle_key_event():
KeyEvent { code: KeyCode::BackTab, .. } => {
    // BackTab = Shift+Tab
    let next = match self.current_mode_kind() {
        ModeKind::Default => ModeKind::Accept,
        ModeKind::Accept => ModeKind::Bypass,
        ModeKind::Bypass => ModeKind::Plan,
        ModeKind::Plan => ModeKind::Default,
        _ => ModeKind::Default,
    };
    self.switch_mode(next);
}
```

### 5.2 Mode switching — slash commands

**File:** `tui/src/app.rs` — slash command handling

Also add slash commands for direct mode selection:

```rust
"/accept" => switch_mode(ModeKind::Accept),
"/bypass" => switch_mode(ModeKind::Bypass),
"/default" => switch_mode(ModeKind::Default),
```

### 5.3 Display current mode in footer

**File:** `tui/src/bottom_pane/footer.rs` (or equivalent status display)

Show the current mode in the TUI footer so the user always knows what permission level they're at:

```
[Default] model: claude-opus-4-6 | context: 200K
[Bypass]  model: claude-opus-4-6 | context: 200K
```

### 5.4 DO NOT modify the approval dialog

The existing approval overlay (allow / deny / chat about it) is correct as-is. This plan only changes WHEN the dialog appears, not WHAT it looks like. Do not add options, change labels, or modify the overlay component.

### 5.5 Mirror in `tui_app_server/`

Per convention 54, all TUI changes must be mirrored in `tui_app_server/`.

---

## Phase 6: Testing

### 6.1 Unit tests for decision logic

**File:** `core/src/exec_policy_tests.rs` (new sibling test file)

Test matrix — each mode × command type:

| Mode | Safe read (`ls`) | Normal shell (`npm test`) | Dangerous (`rm -rf`) | File op (write) |
|------|-----------------|--------------------------|---------------------|----------------|
| Default | Allow | Prompt | Prompt | Prompt |
| Accept | Allow | Prompt | Prompt | Skip (auto) |
| Bypass | Allow | Allow | Allow | Allow |
| Plan | Forbidden | Forbidden | Forbidden | Forbidden |

### 6.2 Integration tests

**File:** `core/tests/suite/permission_modes.rs`

Use `TestCodexBuilder` to test each mode end-to-end:
- Default mode: submit a shell command → verify `ExecApprovalRequest` event emitted
- Accept mode: submit a file write → verify no approval needed; submit shell → verify approval needed
- Bypass mode: submit anything → verify no approval events

### 6.3 Snapshot tests

**File:** `tui/` — insta snapshots

Add snapshots for the mode indicator in the footer and the mode switching behavior.

```bash
cargo test -p orbit-code-tui
cargo insta accept -p orbit-code-tui
```

---

## Files Changed (Complete List)

| File | Change | Lines |
|------|--------|-------|
| `protocol/src/config_types.rs` | Add `Accept`, `Bypass` to `ModeKind`, helper methods | ~40 |
| `protocol/src/protocol.rs` | Add `mode_kind` to prompt template loading, possibly to `ExecApprovalRequest` wire type | ~10 |
| `protocol/src/prompts/permissions/mode/*.md` | New system prompt templates per mode | ~30 |
| `core/src/exec_policy.rs` | Add `mode_kind` param, rewrite `render_decision_for_unmatched_command()` | ~50 |
| `core/src/tools/orchestrator.rs` | Accept mode auto-approve for file ops | ~15 |
| `core/src/config/mod.rs` | Force `DangerFullAccess` for non-Plan modes | ~10 |
| `cli/src/` | Add `--mode` CLI flag | ~5 |
| `tui/src/app.rs` | Add `/accept`, `/bypass`, `/default` commands | ~20 |
| `tui/src/bottom_pane/footer.rs` | Display current mode in status line | ~15 |
| `tui_app_server/` | Mirror TUI changes | ~35 |
| `core/tests/suite/permission_modes.rs` | Integration tests | ~150 |
| **Total** | | **~380** |

---

## Verification

After each phase:
```bash
just fmt
just fix -p orbit-code-protocol
just fix -p orbit-code-core
just fix -p orbit-code-tui
cargo test -p orbit-code-protocol
cargo test -p orbit-code-core
cargo test -p orbit-code-tui
```

End-to-end:
```bash
# Test each mode
just codex -- --mode default   # should prompt for shell commands
just codex -- --mode accept    # should auto-approve file ops
just codex -- --mode bypass    # should never prompt
```
