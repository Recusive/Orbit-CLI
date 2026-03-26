# Hooks Expansion — Full System Design (Merged Plan)

## Context

Orbit Code has a hooks system with 3 events (`SessionStart`, `UserPromptSubmit`, `Stop`) and 1 hook type (`command`). Claude Code has 21 events and 4 hook types (`command`, `http`, `prompt`, `agent`). This plan expands the existing infrastructure — no ground-up rebuild — to reach parity incrementally.

**This plan merges:** `docs/Hooks/` (the detailed 4-phase implementation plan with exact wire formats) with the Track 2 hooks expansion plan (permission modes integration, system design, data flows). Where the two plans conflicted, this document resolves to the Claude Code-compatible wire format.

**Companion plan:** `permission-modes-decouple-sandbox-from-approval.md` (Track 1) — decouples sandbox from approval, adds Default/Accept/Bypass modes. Hooks expansion builds on this by threading `permission_mode` into hook input JSON.

**Reference:** Claude Code hooks reference at `docs/Hooks/hooks-reference.md` (local copy)

**Detailed phase implementations:** `docs/Hooks/01-phase1-agentic-loop.md` through `04-phase4-remaining-events.md` — these contain exact Rust struct definitions, serde annotations, and test code for every event. This overview references them.

**Last verified against codebase:** 2026-03-25

---

## Architecture — What We're Expanding

The existing hooks infrastructure is well-architected. We reuse everything:

```
GENERIC INFRASTRUCTURE (unchanged across all phases)
┌──────────────────────────────────────────────────────────────────┐
│ command_runner.rs — spawns process, pipes JSON stdin, reads      │
│                    stdout/stderr, enforces timeout. Event-agnostic│
│                                                                  │
│ dispatcher.rs    — select_handlers() filters by event + matcher  │
│                   execute_handlers() runs matched handlers in    │
│                   parallel via join_all, takes a parse fn pointer│
│                                                                  │
│ discovery.rs     — walks ConfigLayerStack, loads hooks.json      │
│                   per config layer, builds Vec<ConfiguredHandler> │
│                                                                  │
│ output_parser.rs — shared JSON parsing (universal fields,        │
│                   continue/stopReason/systemMessage/suppress)    │
└──────────────────────────────────────────────────────────────────┘

PER-EVENT MODULES (the pattern we copy for every new event)
┌──────────────────────────────────────────────────────────────────┐
│ events/<event_name>.rs — each ~200-400 lines:                    │
│   *Request struct     (typed input from core)                    │
│   *Outcome struct     (what core acts on)                        │
│   *HandlerData struct (intermediate per-handler result)          │
│   preview() fn        (generates Vec<HookRunSummary>)            │
│   run() async fn      (orchestrates dispatch + parsing)          │
│   parse_completed() fn (maps CommandRunResult → HandlerData)     │
└──────────────────────────────────────────────────────────────────┘
```

---

## System Design — How Hooks Integrate with the Permission System

### The Two Permission Layers

```
┌─────────────────────────────────────────────────────────────────┐
│  LAYER 1: Permission Mode (Track 1 — built-in approval system)   │
│                                                                  │
│  ModeKind::Default → ask for all actionable tools                │
│  ModeKind::Accept  → auto-approve file ops, ask for shell        │
│  ModeKind::Bypass  → never ask, just execute                     │
│                                                                  │
│  This is the in-process gate. Fast, no subprocess.               │
│  Controlled by: config.toml, --mode flag, /default /accept /bypass│
└─────────────────────────────┬───────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  LAYER 2: Hooks (this plan — extensible interception points)     │
│                                                                  │
│  PreToolUse     → external scripts decide allow/deny/ask         │
│  PermissionRequest → auto-respond to approval dialogs            │
│  PostToolUse    → react to tool results (lint, test, etc.)       │
│                                                                  │
│  This is the extensibility layer. External processes.            │
│  Controlled by: hooks.json, plugins, skill frontmatter           │
└─────────────────────────────────────────────────────────────────┘
```

Both layers receive `permission_mode` so they can make mode-aware decisions. A hook in `hooks.json` can check `"permission_mode": "bypass"` and decide not to interfere.

### Data Flow — Tool Execution with Both Layers

```
Agent calls tool (e.g., Bash "npm test")
     │
     ▼
[1] PreToolUse hooks fire (hooks.json)
     │  matcher filters on tool_name ("Bash")
     │  handler returns: allow / deny / ask / modified_input
     │
     ├── deny  → block tool, feed reason to model, STOP
     ├── allow → skip approval, go to [3]
     ├── ask   → go to [2] (force approval dialog)
     └── none  → go to [2] (default: check permission mode)
     │
     ▼
[2] Permission Mode gate (Track 1 — built-in)
     │  Bypass  → skip, go to [3]
     │  Accept  → is_file_op? skip : NeedsApproval
     │  Default → is_read_only? skip : NeedsApproval
     │
     ├── NeedsApproval →
     │       PermissionRequest hooks fire [2a]
     │       │  handler returns: allow / deny / none
     │       ├── allow → go to [3]
     │       ├── deny  → block, STOP
     │       └── none  → TUI approval overlay (existing oneshot channel)
     │                   User picks: allow / deny / always allow
     │
     └── Skip → go to [3]
     │
     ▼
[3] Tool executes (in sandbox if applicable)
     │
     ▼
[4] PostToolUse hooks fire (hooks.json)
     │  matcher filters on tool_name ("Bash")
     │  handler returns: block+reason / additionalContext / nothing
     │
     ├── block → feed reason back to model ("Linter failed: 3 errors")
     └── none  → continue normally
     │
     ▼
Agent continues
```

---

## Phase Structure

| Phase | Scope | Events | New Infrastructure | Detail File |
|-------|-------|--------|--------------------|-------------|
| **1** | Core agentic loop | PreToolUse, PostToolUse, PostToolUseFailure, PermissionRequest | Tool-name matcher, hookSpecificOutput parsing, updatedInput, updatedPermissions | `docs/Hooks/01-phase1-agentic-loop.md` |
| **2** | Session & agent lifecycle | SessionEnd, SubagentStart, SubagentStop, StopFailure, Notification | SessionEnd timeout, agent event wiring | `docs/Hooks/02-phase2-session-lifecycle.md` |
| **3** | Handler types & infra events | HTTP hooks, async command hooks, ConfigChange, InstructionsLoaded | http_runner.rs, async_command_runner.rs, env var interpolation, HandlerKind refactor | `docs/Hooks/03-phase3-advanced-handlers.md` |
| **4** | Remaining events + LLM hooks | PreCompact, PostCompact, WorktreeCreate/Remove, TeammateIdle, TaskCompleted, Elicitation/ElicitationResult, prompt hooks, agent hooks | prompt_runner.rs, agent_runner.rs, PromptHookModelClient trait, AgentHookSpawner trait | `docs/Hooks/04-phase4-remaining-events.md` |

Each phase is independently shippable. Phase 1 is the highest impact — it enables per-tool interception which is the foundation for custom workflows.

---

## Phase 1 — Core Agentic Loop (Priority)

### Events

| Event | When | Matcher | Can Block? | Decision Control |
|-------|------|---------|-----------|-----------------|
| **PreToolUse** | After model generates tool params, before execution | tool name | Yes (allow/deny/ask) | `hookSpecificOutput.permissionDecision` |
| **PostToolUse** | After tool completes successfully | tool name | Yes (decision:block) | Top-level `decision` + `reason` |
| **PostToolUseFailure** | After tool fails | tool name | No (context only) | `additionalContext` |
| **PermissionRequest** | When approval dialog would show | tool name | Yes (allow/deny) | `hookSpecificOutput.decision.behavior` |

### Protocol Changes

**File:** `protocol/src/protocol.rs:1343`

```rust
pub enum HookEventName {
    SessionStart,
    UserPromptSubmit,
    PreToolUse,           // Phase 1
    PostToolUse,          // Phase 1
    PostToolUseFailure,   // Phase 1
    PermissionRequest,    // Phase 1
    Stop,
}
```

**File:** `protocol/src/protocol.rs:1366` — add `ToolCall` scope:

```rust
pub enum HookScope {
    Thread,
    Turn,
    ToolCall,  // NEW: for PreToolUse, PostToolUse, PostToolUseFailure, PermissionRequest
}
```

### Wire Formats (Claude Code-compatible)

**PreToolUse Input** (JSON on stdin):
```json
{
  "session_id": "abc123",
  "turn_id": "turn-1",
  "transcript_path": "/path/to/transcript.jsonl",
  "cwd": "/Users/project",
  "hook_event_name": "PreToolUse",
  "permission_mode": "default",
  "tool_name": "Bash",
  "tool_input": { "command": "npm test" },
  "tool_use_id": "toolu_01ABC..."
}
```

**PreToolUse Output** (JSON on stdout — uses `hookSpecificOutput`, NOT top-level decision):
```json
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "allow",
    "permissionDecisionReason": "Approved by project hook",
    "updatedInput": { "command": "npm test --ci" },
    "additionalContext": "Running in CI mode"
  }
}
```

`permissionDecision`: `"allow"` | `"deny"` | `"ask"`. Deprecated aliases: `"approve"` → `"allow"`, `"block"` → `"deny"`.

**PermissionRequest Output** (nested `hookSpecificOutput.decision`):
```json
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionRequest",
    "decision": {
      "behavior": "allow",
      "updatedInput": { "command": "npm run lint" },
      "updatedPermissions": [
        {
          "type": "addRules",
          "rules": [{ "toolName": "Bash", "ruleContent": "npm run lint" }],
          "behavior": "allow",
          "destination": "localSettings"
        }
      ]
    }
  }
}
```

**PostToolUse Output** (top-level `decision`):
```json
{
  "decision": "block",
  "reason": "Linter found 3 errors — fix before continuing",
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "Errors in: src/main.rs:42, src/lib.rs:15"
  }
}
```

**PostToolUseFailure Output** (context only, cannot block):
```json
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUseFailure",
    "additionalContext": "Command failed. This test is known to be flaky."
  }
}
```

### Core Integration Points

| Hook | Where in Core | Integration |
|------|--------------|-------------|
| PreToolUse | `core/src/tools/orchestrator.rs` — BEFORE existing approval check in `ToolOrchestrator::run()` | Call `hooks.run_pre_tool_use()`, act on decision before entering approval flow |
| PostToolUse | `core/src/tools/registry.rs` — AFTER tool execution (replaces legacy `AfterToolUse` at lines 466-538) | Call `hooks.run_post_tool_use()`, inject block reason if returned |
| PostToolUseFailure | `core/src/tools/registry.rs` — AFTER tool execution fails | Call `hooks.run_post_tool_use_failure()`, inject additional context |
| PermissionRequest | `core/src/codex.rs:request_command_approval()` — BEFORE creating oneshot channel (line 2843) | Call `hooks.run_permission_request()`, auto-approve/deny or fall through to TUI |

### Engine Wiring (hooks crate)

**`engine/config.rs`** — add 4 fields to `HookEvents`:
```rust
#[serde(rename = "PreToolUse", default)]
pub pre_tool_use: Vec<MatcherGroup>,
#[serde(rename = "PostToolUse", default)]
pub post_tool_use: Vec<MatcherGroup>,
#[serde(rename = "PostToolUseFailure", default)]
pub post_tool_use_failure: Vec<MatcherGroup>,
#[serde(rename = "PermissionRequest", default)]
pub permission_request: Vec<MatcherGroup>,
```

**`engine/discovery.rs`** — add 4 discovery loops (copy existing pattern).

**`engine/dispatcher.rs`** — tool-name matcher for all 4 events:
```rust
HookEventName::PreToolUse
| HookEventName::PostToolUse
| HookEventName::PostToolUseFailure
| HookEventName::PermissionRequest => match (&handler.matcher, matcher_input) {
    (Some(matcher), Some(input)) => regex::Regex::new(matcher)
        .map(|regex| regex.is_match(input))
        .unwrap_or(false),
    (None, _) => true,
    _ => false,
},
```

Scope mapping: all 4 → `HookScope::ToolCall`.

**`engine/mod.rs`** — add 8 methods (run + preview per event).

**`registry.rs`** — add 8 public API methods.

**`events/`** — 4 new event modules (~200-400 lines each), following `stop.rs` pattern exactly:
- `events/pre_tool_use.rs`
- `events/post_tool_use.rs`
- `events/post_tool_use_failure.rs`
- `events/permission_request.rs`

**`schema.rs`** — 8 new wire type structs (Input + Output per event) + fixtures.

### Event Module Pattern (copy for each)

```rust
// events/pre_tool_use.rs — follows stop.rs exactly

pub struct PreToolUseRequest {
    pub session_id: ThreadId,
    pub turn_id: String,
    pub cwd: PathBuf,
    pub transcript_path: Option<PathBuf>,
    pub permission_mode: String,
    pub tool_name: String,
    pub tool_input: serde_json::Value,
    pub tool_use_id: String,
    // Cross-cutting: present when hook fires inside a subagent
    pub agent_id: Option<String>,
    pub agent_type: Option<String>,
}

pub struct PreToolUseOutcome {
    pub hook_events: Vec<HookCompletedEvent>,
    pub should_stop: bool,
    pub stop_reason: Option<String>,
    pub decision: Option<PreToolUseDecision>,
    pub decision_reason: Option<String>,
    pub modified_input: Option<serde_json::Value>,
    pub additional_context: Option<String>,
}

pub enum PreToolUseDecision {
    Allow,
    Deny,
    Ask,
}

struct PreToolUseHandlerData {
    should_stop: bool,
    stop_reason: Option<String>,
    decision: Option<PreToolUseDecision>,
    decision_reason: Option<String>,
    modified_input: Option<serde_json::Value>,
    additional_context: Option<String>,
}

pub(crate) fn preview(...) -> Vec<HookRunSummary> { ... }
pub(crate) async fn run(...) -> PreToolUseOutcome { ... }
fn parse_completed(...) -> ParsedHandler<PreToolUseHandlerData> { ... }
```

**Aggregation rules:**
- PreToolUse: most restrictive wins (`Deny > Ask > Allow`). Last `modified_input` wins.
- PostToolUse: any `block` wins. Multiple `additional_context` concatenated.
- PermissionRequest: `Deny > Allow`. Last `updatedInput` wins.

### Deprecate Legacy AfterToolUse

**File:** `hooks/src/types.rs`

`HookEvent::AfterToolUse` and the closure-based `HookFn` dispatch are the old system. `after_tool_use: Vec::new()` in `registry.rs:57` means no handlers ever load for it. PostToolUse replaces it. Mark as deprecated, then remove in a followup.

---

## Phases 2-4 — Summary

Phase 2-4 details are in `docs/Hooks/02-phase2-session-lifecycle.md`, `03-phase3-advanced-handlers.md`, and `04-phase4-remaining-events.md`. Key highlights:

### Phase 2 — Session Lifecycle (5 events)

SessionEnd, SubagentStart, SubagentStop, StopFailure, Notification. SubagentStop reuses Stop's decision control. SessionEnd has a 1.5s default timeout. StopFailure output/exit code are ignored.

### Phase 3 — Advanced Handlers + Infra Events (2 handler types, 2 events)

- **HTTP hooks**: `type: "http"`, POST to URL, env var interpolation in headers via `allowedEnvVars`, non-2xx is non-blocking error. Requires new `http_runner.rs`.
- **Async command hooks**: `"async": true`, run in background, deliver results on next turn. Requires new `async_command_runner.rs`.
- **HandlerKind refactor**: `ConfiguredHandler.command: String` → `ConfiguredHandler.kind: HandlerKind` enum with Command/Http variants. This is the prerequisite for Phase 4's Prompt/Agent types.
- **ConfigChange event**: fires when settings files change. `policy_settings` changes cannot be blocked.
- **InstructionsLoaded event**: fires when CLAUDE.md/rules loaded. No decision control.

### Phase 4 — Remaining Events + LLM Hooks (8 events, 2 handler types)

- **Prompt hooks**: `type: "prompt"`, single LLM call via `PromptHookModelClient` trait (injected into HooksConfig). Returns `{ok: bool, reason: string}`. Default model: haiku. Default timeout: 30s.
- **Agent hooks**: `type: "agent"`, multi-turn subagent via `AgentHookSpawner` trait (injected into HooksConfig). Up to 50 turns with Read/Grep/Glob tools. Default timeout: 60s.
- **Circular dependency solution**: `PromptHookModelClient` and `AgentHookSpawner` are traits defined in `hooks` crate, implemented in `core` crate, injected via `HooksConfig`. This avoids `hooks` → `core` dependency.
- **Remaining events**: PreCompact, PostCompact, WorktreeCreate, WorktreeRemove, TeammateIdle, TaskCompleted, Elicitation, ElicitationResult.

---

## Cross-Cutting Concerns (All Phases)

These span multiple phases and are NOT optional.

### 1. `agent_id` / `agent_type` on ALL event inputs (Phase 1+)

Per Claude Code spec, when a hook fires inside a subagent or when started with `--agent`, every event input gets:
```rust
#[serde(default, skip_serializing_if = "Option::is_none")]
pub agent_id: Option<String>,
#[serde(default, skip_serializing_if = "Option::is_none")]
pub agent_type: Option<String>,
```

Every `*Request` struct and every `*CommandInput` wire type must include these from Phase 1 onwards.

### 2. `permission_mode` on tool event inputs (Phase 1+)

All tool events (PreToolUse, PostToolUse, PostToolUseFailure, PermissionRequest) include `permission_mode` in their JSON input. This field reflects the current `ModeKind` from Track 1:
- `"default"` | `"accept"` | `"bypass"` | `"plan"`

Before Track 1 ships, the existing `hook_permission_mode()` function in `core/src/hook_runtime.rs` maps `AskForApproval::Never` → `"bypassPermissions"` and everything else → `"default"`. After Track 1, this maps from `ModeKind` directly.

### 3. Hook source labels on "ask" prompts (Phase 1)

When a PreToolUse hook returns `permissionDecision: "ask"`, the TUI permission prompt shows a label: `[User]`, `[Project]`, `[Plugin]`, `[Local]`. Derived from `ConfiguredHandler.source_path` → config layer origin.

### 4. `allowManagedHooksOnly` policy (Phase 3)

Enterprise setting. When `true`, only managed-layer hooks execute. User/project/local/plugin hooks are blocked. Implemented in `discovery.rs` as a post-filter.

### 5. Plugin hooks discovery (Phase 3)

Plugins define hooks in `hooks/hooks.json`. If plugins are config layers in `ConfigLayerStack`, this works automatically. Env vars `CLAUDE_PLUGIN_ROOT` and `CLAUDE_PLUGIN_DATA` set on child processes.

### 6. `CLAUDE_CODE_REMOTE` env var (Phase 3)

Set to `"true"` in remote web environments. Set on hook child processes in `command_runner.rs`.

---

## Conflict Resolutions

Three conflicts existed between the older `docs/Hooks/` plan and the Track 2 plan. Resolved here:

| Conflict | Resolution | Reason |
|----------|-----------|--------|
| `HookScope::ToolCall` vs `HookScope::Turn` | **Use `ToolCall`** (new variant) | Matches Claude Code semantics — tool hooks are scoped to a single tool invocation, not the entire turn |
| PermissionRequest output: nested `hookSpecificOutput.decision` vs flat `decision` | **Use nested** (`hookSpecificOutput.decision.behavior` with `updatedInput`, `updatedPermissions`, `message`, `interrupt`) | Matches Claude Code wire format exactly, enables richer control |
| PostToolUseFailure: Phase 1 vs future | **Include in Phase 1** alongside PostToolUse | Same integration point (registry.rs), minimal incremental cost, and Claude Code has both |

---

## File Impact Summary (Phase 1)

| File | Change | Lines |
|------|--------|-------|
| `protocol/src/protocol.rs` | Add 4 variants to `HookEventName`, add `ToolCall` to `HookScope` | ~10 |
| `hooks/src/events/pre_tool_use.rs` | **New** — event module | ~350 |
| `hooks/src/events/post_tool_use.rs` | **New** — event module | ~250 |
| `hooks/src/events/post_tool_use_failure.rs` | **New** — event module | ~200 |
| `hooks/src/events/permission_request.rs` | **New** — event module | ~350 |
| `hooks/src/events/mod.rs` | Register 4 new modules | ~4 |
| `hooks/src/schema.rs` | Wire types + fixtures for 4 events | ~200 |
| `hooks/src/engine/config.rs` | Add 4 fields to `HookEvents` | ~12 |
| `hooks/src/engine/discovery.rs` | Add 4 discovery loops | ~60 |
| `hooks/src/engine/dispatcher.rs` | Tool-name matcher + `ToolCall` scope | ~25 |
| `hooks/src/engine/mod.rs` | Add 8 methods (run + preview per event) | ~50 |
| `hooks/src/registry.rs` | Add 8 public API methods | ~40 |
| `core/src/hook_runtime.rs` | Add 4 integration functions | ~80 |
| `core/src/tools/orchestrator.rs` | Wire PreToolUse before approval | ~40 |
| `core/src/tools/registry.rs` | Wire PostToolUse + PostToolUseFailure, deprecate AfterToolUse | ~60 |
| `core/src/codex.rs` | Wire PermissionRequest before oneshot | ~40 |
| `core/tests/suite/hooks_tool_events.rs` | **New** — integration tests | ~300 |
| **Total (Phase 1)** | | **~2070** |

### Full project totals (all phases)

| Phase | Events | New Files | Approx Lines |
|-------|--------|-----------|-------------|
| Phase 1 | 4 (PreToolUse, PostToolUse, PostToolUseFailure, PermissionRequest) | 5 event modules + tests | ~2070 |
| Phase 2 | 5 (SessionEnd, SubagentStart/Stop, StopFailure, Notification) | 5 event modules + tests | ~1800 |
| Phase 3 | 2 events + 2 handler types (HTTP, async) | http_runner, async_runner, ConfigChange, InstructionsLoaded | ~2200 |
| Phase 4 | 8 events + 2 handler types (prompt, agent) | prompt_runner, agent_runner, 8 event modules | ~3000 |
| **Total** | **21 events, 4 handler types** | | **~9000** |

---

## Verification

After Phase 1:
```bash
just fmt
just fix -p orbit-code-protocol
just fix -p orbit-code-hooks
just fix -p orbit-code-core
just write-hooks-schema
cargo test -p orbit-code-hooks
cargo test -p orbit-code-core
```

End-to-end:
```bash
# Test PreToolUse hook blocks rm commands
echo '{"hooks":{"PreToolUse":[{"matcher":"Bash","hooks":[{"type":"command","command":"bash -c '\''read INPUT; CMD=$(echo $INPUT | jq -r .tool_input.command); if echo $CMD | grep -q \"rm -rf\"; then echo \"{\\\"hookSpecificOutput\\\":{\\\"hookEventName\\\":\\\"PreToolUse\\\",\\\"permissionDecision\\\":\\\"deny\\\",\\\"permissionDecisionReason\\\":\\\"Blocked by hook\\\"}}\"; else exit 0; fi'\''"}]}]}}' > ~/.orbit/hooks.json
just codex
# Ask agent to run "rm -rf /tmp/test" — should be blocked by hook
```

---

## Implementation Order

```
1. Ship Track 1 (Permission Modes) — ~380 lines, independent
   Decouples sandbox from approval. Adds Default/Accept/Bypass.

2. Ship Phase 1 (Agentic Loop Hooks) — ~2070 lines
   PreToolUse, PostToolUse, PostToolUseFailure, PermissionRequest.
   Enables: per-tool interception, linting, auto-approval scripts.

3. Ship Phase 2 (Session Lifecycle) — ~1800 lines
   SessionEnd, SubagentStart/Stop, StopFailure, Notification.
   Enables: cleanup scripts, agent monitoring, error alerting.

4. Ship Phase 3 (Advanced Handlers) — ~2200 lines
   HTTP hooks, async hooks, ConfigChange, InstructionsLoaded.
   Enables: webhook integrations, background tasks, config auditing.

5. Ship Phase 4 (LLM Hooks + Remaining) — ~3000 lines
   Prompt/agent hook types + 8 remaining events.
   Enables: Superpowers-like LLM verification, worktree control, MCP elicitation.
```

Track 1 and Phase 1 can be developed in parallel by different people. Phases 2-4 are sequential (each builds on infrastructure from the previous).
