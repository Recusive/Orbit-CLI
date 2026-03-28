# Automations: Cron-Scheduled Background Agent Runs

> Spec v1 — Engine layer in `codex-rs/`, UI layer in `snowflake-v0`

## Context

The OpenAI Codex desktop app ships an "Automations" feature: users create named prompts with cron schedules that run headless in the background, deposit findings in an inbox, and optionally run in git worktrees for isolation. This feature does not exist in Orbit Code or the Orbit desktop app.

**Goal:** Build the full automation engine in `codex-rs/` (portable Rust) and expose it via app-server v2 so `snowflake-v0` (Tauri + React) can wire the UI. The engine must also be usable from the TUI and `exec` crate for CLI-only users.

**Reference implementation:** OpenAI Codex Automations docs (pasted in conversation, March 2026).

---

## System Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                    snowflake-v0 (Tauri Desktop)                      │
│  ┌────────────────────┐  ┌────────────────────────────────────┐     │
│  │ Automations Panel  │  │  Inbox / Triage Panel              │     │
│  │ - Create/edit form │  │  - Run list (unread/read/archived) │     │
│  │ - Schedule picker  │  │  - Findings viewer                 │     │
│  │ - Enable/disable   │  │  - Worktree diff viewer            │     │
│  └────────┬───────────┘  └──────────────┬─────────────────────┘     │
│           │ Tauri invoke / app-server    │                           │
└───────────┼──────────────────────────────┼──────────────────────────┘
            │                              │
┌───────────▼──────────────────────────────▼──────────────────────────┐
│                     codex-rs (Rust Engine)                           │
│                                                                      │
│  ┌──────────────────┐  ┌──────────────┐  ┌───────────────────────┐  │
│  │ AutomationService│  │   Scheduler  │  │  AutomationExecutor   │  │
│  │ (CRUD + state)   │──│  (cron eval) │──│  (headless Session)   │  │
│  └────────┬─────────┘  └──────────────┘  └───────────┬───────────┘  │
│           │                                           │              │
│  ┌────────▼─────────┐                    ┌───────────▼───────────┐  │
│  │   state crate    │                    │     core / exec       │  │
│  │ (SQLite tables)  │                    │  (Session, sandbox,   │  │
│  │ automations      │                    │   approval, worktree) │  │
│  │ automation_runs  │                    └───────────────────────┘  │
│  └──────────────────┘                                               │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Data Model

### Automation

Represents a saved, recurring prompt configuration.

```rust
pub struct Automation {
    pub id: String,                          // UUID
    pub name: String,                        // User-visible label
    pub prompt: String,                      // The instruction to execute
    pub schedule: String,                    // Cron expression (e.g., "0 9 * * *")
    pub project_path: String,               // Absolute path to project directory
    pub enabled: bool,                       // Whether scheduler should trigger this
    pub sandbox_mode: SandboxMode,           // ReadOnly | WorkspaceWrite | DangerFullAccess
    pub model: Option<String>,              // Override model (None = use default)
    pub reasoning_effort: Option<ReasoningEffort>, // Override reasoning effort
    pub worktree_mode: WorktreeMode,        // Local | Worktree
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_run_at: Option<DateTime<Utc>>,
    pub next_run_at: Option<DateTime<Utc>>,  // Precomputed from schedule + last_run_at
}
```

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorktreeMode {
    /// Run directly in the project directory (can modify working copy).
    Local,
    /// Create a fresh git worktree per run (isolates changes).
    Worktree,
}
```

### AutomationRun

Represents a single execution of an automation.

```rust
pub struct AutomationRun {
    pub id: String,                          // UUID
    pub automation_id: String,               // FK → Automation
    pub status: AutomationRunStatus,
    pub thread_id: Option<String>,           // Session thread (for conversation history)
    pub worktree_path: Option<String>,       // If worktree mode, path to the worktree
    pub worktree_branch: Option<String>,     // Branch name for the worktree
    pub has_findings: bool,                  // Did the agent report anything notable?
    pub findings_summary: Option<String>,    // Short summary for inbox display
    pub read: bool,                          // User has seen this run
    pub archived: bool,                      // User dismissed / archived
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
}
```

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutomationRunStatus {
    Pending,     // Scheduled, not yet started
    Running,     // Agent session active
    Completed,   // Finished successfully
    Failed,      // Finished with error
    Cancelled,   // User or system cancelled
}
```

### Relationship

```
Automation 1 ──── * AutomationRun
    │                    │
    │                    └── thread_id → existing threads table
    │                         (conversation history is the existing
    │                          thread/session infrastructure)
    │
    └── schedule → cron expression evaluated by Scheduler
```

---

## SQLite Migration

**File:** `codex-rs/state/migrations/0021_automations.sql`

Following the pattern from `0014_agent_jobs.sql`:

```sql
CREATE TABLE IF NOT EXISTS automations (
    id              TEXT PRIMARY KEY NOT NULL,
    name            TEXT NOT NULL,
    prompt          TEXT NOT NULL,
    schedule        TEXT NOT NULL,
    project_path    TEXT NOT NULL,
    enabled         INTEGER NOT NULL DEFAULT 1,
    sandbox_mode    TEXT NOT NULL DEFAULT 'workspace-write',
    model           TEXT,
    reasoning_effort TEXT,
    worktree_mode   TEXT NOT NULL DEFAULT 'local',
    created_at      INTEGER NOT NULL,
    updated_at      INTEGER NOT NULL,
    last_run_at     INTEGER,
    next_run_at     INTEGER
);

CREATE TABLE IF NOT EXISTS automation_runs (
    id              TEXT PRIMARY KEY NOT NULL,
    automation_id   TEXT NOT NULL REFERENCES automations(id) ON DELETE CASCADE,
    status          TEXT NOT NULL DEFAULT 'pending',
    thread_id       TEXT,
    worktree_path   TEXT,
    worktree_branch TEXT,
    has_findings    INTEGER NOT NULL DEFAULT 0,
    findings_summary TEXT,
    read            INTEGER NOT NULL DEFAULT 0,
    archived        INTEGER NOT NULL DEFAULT 0,
    started_at      INTEGER NOT NULL,
    completed_at    INTEGER,
    last_error      TEXT
);

CREATE INDEX IF NOT EXISTS idx_automation_runs_automation_id
    ON automation_runs(automation_id);

CREATE INDEX IF NOT EXISTS idx_automation_runs_status
    ON automation_runs(status);

CREATE INDEX IF NOT EXISTS idx_automations_next_run_at
    ON automations(next_run_at)
    WHERE enabled = 1;
```

---

## State Crate Additions

### Model types

**File:** `codex-rs/state/src/model/automation.rs` (new)

Follows the `agent_job.rs` pattern: public domain structs + `(crate)` row types + `TryFrom` conversions.

| Type | Purpose |
|------|---------|
| `Automation` | Domain model |
| `AutomationCreateParams` | Input for creating an automation |
| `AutomationUpdateParams` | Input for updating (partial — all fields optional except id) |
| `AutomationRun` | Domain model |
| `AutomationRunStatus` | Enum with `as_str()` / `parse()` |
| `WorktreeMode` | Enum: `Local` / `Worktree` |
| `AutomationRow` | SQLite row (sqlx::FromRow) |
| `AutomationRunRow` | SQLite row (sqlx::FromRow) |

**File:** `codex-rs/state/src/model/mod.rs` — add `mod automation;` + re-exports.

### Runtime queries

**File:** `codex-rs/state/src/runtime/automations.rs` (new)

| Method | Signature | Purpose |
|--------|-----------|---------|
| `create_automation` | `(&self, params: AutomationCreateParams) -> Result<Automation>` | Insert new automation, compute `next_run_at` |
| `get_automation` | `(&self, id: &str) -> Result<Option<Automation>>` | Fetch by ID |
| `list_automations` | `(&self, project_path: Option<&str>) -> Result<Vec<Automation>>` | List all, optionally filtered by project |
| `update_automation` | `(&self, params: AutomationUpdateParams) -> Result<Automation>` | Partial update |
| `delete_automation` | `(&self, id: &str) -> Result<()>` | Delete automation + cascade runs |
| `set_automation_enabled` | `(&self, id: &str, enabled: bool) -> Result<()>` | Toggle enabled state |
| `get_due_automations` | `(&self, now: DateTime<Utc>) -> Result<Vec<Automation>>` | `WHERE enabled = 1 AND next_run_at <= now` |
| `advance_next_run` | `(&self, id: &str, last_run: DateTime<Utc>) -> Result<()>` | Compute + store next run time from cron |
| `create_automation_run` | `(&self, automation_id: &str, worktree_path: Option<&str>, worktree_branch: Option<&str>) -> Result<AutomationRun>` | Start a run |
| `complete_automation_run` | `(&self, id: &str, has_findings: bool, summary: Option<&str>) -> Result<()>` | Mark completed |
| `fail_automation_run` | `(&self, id: &str, error: &str) -> Result<()>` | Mark failed |
| `list_automation_runs` | `(&self, automation_id: Option<&str>, unread_only: bool, include_archived: bool, limit: u32, cursor: Option<&str>) -> Result<(Vec<AutomationRun>, Option<String>)>` | Paginated inbox query |
| `mark_run_read` | `(&self, id: &str) -> Result<()>` | Mark as read |
| `archive_run` | `(&self, id: &str) -> Result<()>` | Archive a run |
| `unread_run_count` | `(&self) -> Result<u64>` | Badge count for UI |

---

## Scheduler

### Architecture Decision: In-Process Tokio Timer

The scheduler runs as a background `tokio::task` within the app-server or TUI process. It wakes on a fixed interval (60 seconds), queries `get_due_automations()`, and spawns runs.

**Why in-process (not OS-level cron):**
- Codex uses this approach — proven pattern
- No external dependency on launchd/cron/systemd
- Scheduler state (pause/resume) is trivially controllable
- Worktree cleanup and run lifecycle managed in same process

**Why not shorter intervals:** Automations are meant for hourly/daily cadence. 60-second poll is sufficient. Codex uses similar polling.

### Cron Expression Evaluation

**Dependency:** Add `croner` or `cron` crate to workspace `Cargo.toml`.

Evaluation logic:
1. Parse schedule string into cron expression on automation create/update (validate early)
2. On each scheduler tick: `SELECT * FROM automations WHERE enabled = 1 AND next_run_at <= now`
3. After spawning a run: compute next occurrence from cron expression, store in `next_run_at`

```rust
pub struct AutomationScheduler {
    db: Arc<StateRuntime>,
    executor: Arc<AutomationExecutor>,
    cancel: CancellationToken,
}

impl AutomationScheduler {
    pub fn spawn(self) -> JoinHandle<()> {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                tokio::select! {
                    _ = self.cancel.cancelled() => break,
                    _ = interval.tick() => {
                        self.poll_and_run().await;
                    }
                }
            }
        })
    }

    async fn poll_and_run(&self) {
        let now = Utc::now();
        let due = self.db.get_due_automations(now).await;
        for automation in due {
            // Spawn each run as independent task — one failure
            // doesn't block others.
            let executor = self.executor.clone();
            let db = self.db.clone();
            tokio::spawn(async move {
                executor.run(&automation).await;
                db.advance_next_run(&automation.id, now).await;
            });
        }
    }
}
```

### Scheduler Lifecycle

| Event | Behavior |
|-------|----------|
| App-server starts | Scheduler starts automatically |
| App-server shuts down | `CancellationToken` cancels scheduler; in-flight runs get 30s grace |
| Automation disabled | `next_run_at` cleared; scheduler skips it |
| Automation re-enabled | `next_run_at` recomputed from `now + next cron occurrence` |
| Schedule edited | `next_run_at` recomputed |
| Manual "Run Now" | Bypasses scheduler; calls executor directly |

---

## Automation Executor

### Execution Flow

```
AutomationExecutor::run(automation)
    │
    ├── 1. Create AutomationRun record (status: Running)
    │
    ├── 2. If worktree_mode == Worktree:
    │       └── git worktree add <temp_path> -b automation/<run_id>
    │
    ├── 3. Build session config:
    │       ├── cwd = worktree_path or project_path
    │       ├── sandbox_mode = automation.sandbox_mode
    │       ├── approval_policy = AutoApprove (no human in loop)
    │       ├── model = automation.model or default
    │       ├── reasoning_effort = automation.reasoning_effort or default
    │       └── instructions = automation.prompt
    │
    ├── 4. Create headless Session (reuse exec crate path)
    │       ├── Submit Op::UserTurn with prompt
    │       ├── Consume EventMsg stream until TurnCompleted
    │       └── Collect final assistant message as findings
    │
    ├── 5. Determine findings:
    │       ├── If agent produced output → has_findings = true
    │       ├── Extract first ~200 chars as findings_summary
    │       └── If agent said "nothing to report" → has_findings = false
    │
    ├── 6. Complete run record:
    │       ├── status = Completed (or Failed if error)
    │       ├── has_findings, findings_summary
    │       └── thread_id (for viewing full conversation later)
    │
    └── 7. Emit notification:
            └── AutomationRunCompleted { run_id, has_findings }
```

### Headless Session Configuration

The executor builds a `Config` and creates a `Session` (from `core/src/codex.rs`). Key settings:

```rust
let config = Config {
    model: automation.model.clone().unwrap_or_else(|| default_model()),
    sandbox_mode: automation.sandbox_mode,
    approval_mode: ApprovalMode::AutoApprove, // No human in loop
    cwd: effective_cwd.clone(),
    // Skills expansion: if prompt contains $skill-name, resolve it
    ..Config::default()
};
```

### Skills Integration

If the automation prompt contains `$skill-name` references:
1. Before submitting the prompt, scan for `$` prefixed tokens
2. Look up matching skills in the skills registry
3. Expand the skill content inline (or prepend as system context)
4. Submit the expanded prompt to the session

This mirrors Codex's behavior: "You can explicitly trigger a skill as part of an automation by using `$skill-name` inside your automation."

### Worktree Management

**New functions needed in `codex-rs/utils/git/src/operations.rs`:**

```rust
/// Create a new worktree for an automation run.
pub fn create_worktree(
    repo_path: &Path,
    worktree_path: &Path,
    branch_name: &str,
) -> Result<()>

/// Remove a worktree after an automation run is archived.
pub fn remove_worktree(
    repo_path: &Path,
    worktree_path: &Path,
) -> Result<()>

/// List active automation worktrees.
pub fn list_automation_worktrees(
    repo_path: &Path,
) -> Result<Vec<WorktreeInfo>>
```

**Worktree path convention:** `<project_path>/.orbit/worktrees/automation-<run_id>`

**Cleanup policy:**
- Worktree is created on run start
- Worktree persists after run completes (user may want to review the diff)
- Worktree is deleted when the run is **archived**
- Stale worktree cleanup: on scheduler startup, prune worktrees for archived/deleted runs

---

## App-Server V2 API

### New RPC Methods

Register in `app-server-protocol/src/protocol/common.rs` via `client_request_definitions!`:

| Method | Params | Response | Description |
|--------|--------|----------|-------------|
| `automation/create` | `AutomationCreateParams` | `AutomationResponse` | Create a new automation |
| `automation/update` | `AutomationUpdateParams` | `AutomationResponse` | Update an existing automation |
| `automation/delete` | `AutomationDeleteParams` | `()` | Delete an automation |
| `automation/list` | `AutomationListParams` | `AutomationListResponse` | List automations (optional project filter) |
| `automation/get` | `AutomationGetParams` | `AutomationResponse` | Get single automation |
| `automation/setEnabled` | `AutomationSetEnabledParams` | `AutomationResponse` | Toggle enabled |
| `automation/runNow` | `AutomationRunNowParams` | `AutomationRunResponse` | Trigger immediate run |
| `automationRun/list` | `AutomationRunListParams` | `AutomationRunListResponse` | Paginated inbox (cursor pagination) |
| `automationRun/get` | `AutomationRunGetParams` | `AutomationRunResponse` | Get single run with findings |
| `automationRun/markRead` | `AutomationRunMarkReadParams` | `()` | Mark run as read |
| `automationRun/archive` | `AutomationRunArchiveParams` | `()` | Archive run (triggers worktree cleanup) |
| `automationRun/cancel` | `AutomationRunCancelParams` | `()` | Cancel a running automation |
| `automation/unreadCount` | `()` | `AutomationUnreadCountResponse` | Badge count |

### Notifications (server → client)

| Notification | Payload | When |
|---|---|---|
| `automation/runStarted` | `{ automationId, runId }` | A scheduled run begins |
| `automation/runCompleted` | `{ automationId, runId, hasFindings, findingsSummary }` | A run finishes |
| `automation/runFailed` | `{ automationId, runId, error }` | A run errors out |

### Type Definitions

**File:** `codex-rs/app-server-protocol/src/protocol/v2.rs` (append)

```rust
// --- Automation Types ---

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct AutomationCreateParams {
    pub name: String,
    pub prompt: String,
    pub schedule: String,
    pub project_path: String,
    #[ts(optional = nullable)]
    pub sandbox_mode: Option<String>,
    #[ts(optional = nullable)]
    pub model: Option<String>,
    #[ts(optional = nullable)]
    pub reasoning_effort: Option<String>,
    #[ts(optional = nullable)]
    pub worktree_mode: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct AutomationResponse {
    pub id: String,
    pub name: String,
    pub prompt: String,
    pub schedule: String,
    pub project_path: String,
    pub enabled: bool,
    pub sandbox_mode: String,
    #[ts(optional = nullable)]
    pub model: Option<String>,
    #[ts(optional = nullable)]
    pub reasoning_effort: Option<String>,
    pub worktree_mode: String,
    pub created_at: i64,
    pub updated_at: i64,
    #[ts(optional = nullable)]
    pub last_run_at: Option<i64>,
    #[ts(optional = nullable)]
    pub next_run_at: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct AutomationRunListParams {
    #[ts(optional = nullable)]
    pub automation_id: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub unread_only: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub include_archived: bool,
    #[ts(optional = nullable)]
    pub cursor: Option<String>,
    #[ts(optional = nullable)]
    pub limit: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct AutomationRunListResponse {
    pub data: Vec<AutomationRunResponse>,
    #[ts(optional = nullable)]
    pub next_cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct AutomationRunResponse {
    pub id: String,
    pub automation_id: String,
    pub status: String,
    #[ts(optional = nullable)]
    pub thread_id: Option<String>,
    #[ts(optional = nullable)]
    pub worktree_path: Option<String>,
    #[ts(optional = nullable)]
    pub worktree_branch: Option<String>,
    pub has_findings: bool,
    #[ts(optional = nullable)]
    pub findings_summary: Option<String>,
    pub read: bool,
    pub archived: bool,
    pub started_at: i64,
    #[ts(optional = nullable)]
    pub completed_at: Option<i64>,
    #[ts(optional = nullable)]
    pub last_error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct AutomationUnreadCountResponse {
    pub count: u64,
}
```

---

## Security Model

### Approval Policy

Automations run with **no human approval** (unattended). The behavior maps directly to sandbox mode:

| Sandbox Mode | What the agent CAN do | What FAILS |
|---|---|---|
| `ReadOnly` | Read files, run read-only commands, search | Any file write, network access, app control |
| `WorkspaceWrite` | Read + write within project dir | Writes outside workspace, network, app control |
| `DangerFullAccess` | Everything | Nothing fails (highest risk) |

### Guardrails

1. **Default to `WorkspaceWrite`** — balances usefulness with safety
2. **Worktree mode recommended** for `WorkspaceWrite` and `DangerFullAccess` — isolates changes from working copy
3. **Run timeout** — each run has a maximum duration (default: 30 minutes, configurable per automation)
4. **Respect `requirements.toml`** — if admin policy disallows `approval_policy = "never"`, automations fall back to the configured approval policy (which may block unattended execution)
5. **Skills as scope limiters** — encourage users to pair automations with skills that define exactly what the agent should do

### Config Key

Add to `ConfigToml`:

```rust
pub struct AutomationsConfig {
    /// Enable the automations scheduler. Default: true.
    pub enabled: bool,
    /// Maximum concurrent automation runs. Default: 2.
    pub max_concurrent_runs: usize,
    /// Default max runtime per run in seconds. Default: 1800.
    pub default_max_runtime_seconds: u64,
    /// Default sandbox mode for new automations. Default: workspace-write.
    pub default_sandbox_mode: SandboxMode,
}
```

Run `just write-config-schema` after adding.

---

## Crate Placement

| Component | Crate | Rationale |
|-----------|-------|-----------|
| `Automation`, `AutomationRun` models | `state` | Follows `AgentJob` pattern — data models + SQLite persistence |
| `AutomationScheduler` | `core` | Needs `Session` access; scheduler is engine logic |
| `AutomationExecutor` | `core` | Creates and drives headless `Session` |
| `AutomationsConfig` | `config` | Configuration is in the config crate |
| Worktree helpers | `utils/git` | Git operations live here |
| App-server v2 types | `app-server-protocol` | All v2 types are here |
| App-server v2 handlers | `app-server` | RPC method implementations |
| Cron expression parsing | `core` (or new `utils/cron`) | Small utility, could be inline |

---

## TUI Surface (Minimal)

The TUI is secondary to the desktop app for this feature, but basic support:

| Capability | Implementation |
|---|---|
| List automations | New `automation list` subcommand in `cli/` |
| Create automation | `automation create --name "..." --prompt "..." --schedule "0 9 * * *"` |
| Enable/disable | `automation enable <id>` / `automation disable <id>` |
| Run now | `automation run <id>` |
| View inbox | `automation inbox` (list recent runs with findings) |
| Delete | `automation delete <id>` |

These are convenience wrappers that call the same `AutomationService` the app-server uses.

---

## snowflake-v0 Integration (UI Layer)

This section documents what the Tauri desktop app needs. Implementation is in the `snowflake-v0` repo, not `codex-rs`.

### Zustand Stores

| Store | Location | State |
|---|---|---|
| `automation-store.ts` | `apps/agent/src/stores/automations/` | `automations: Automation[]`, `runs: AutomationRun[]`, `unreadCount: number` |

### React Components

| Component | Purpose |
|---|---|
| `AutomationsPanel` | Sidebar panel listing all automations |
| `AutomationCreateDialog` | Form: name, prompt, schedule picker, sandbox, model, worktree toggle |
| `AutomationInbox` | Triage pane: list of runs, filter (all/unread), archive |
| `AutomationRunDetail` | View findings, conversation thread, diff (if worktree) |
| `SchedulePicker` | Cron builder UI (every N hours, daily at time, weekly, custom cron) |

### Tauri Commands (or app-server client calls)

The React frontend calls the app-server v2 methods via the existing `orbit-server` sidecar HTTP API. No new Tauri commands needed if the app-server is the communication path.

If using the Claude backend (agent-bridge): Tauri commands would wrap the same `AutomationService` Rust code, exposed via `src-tauri/src/commands/agent/automations.rs`.

---

## Findings Detection

The hardest UX problem: **how does the system decide if a run has "findings"?**

### Approach: Structured Output + Heuristic Fallback

1. **Primary — Structured output:** Append to the system prompt for automation runs:
   ```
   When you finish, output a JSON block:
   {"has_findings": true|false, "summary": "one-line summary"}
   If there is nothing to report, set has_findings to false.
   ```
   Parse the last assistant message for this JSON block.

2. **Fallback — Heuristic:** If no JSON block found:
   - If the final message contains phrases like "nothing to report", "no issues found", "everything looks good" → `has_findings = false`
   - Otherwise → `has_findings = true`, `summary = first 200 chars of final message`

3. **Future — Model-as-judge:** Use a fast model (Haiku-class) to classify the output as findings/no-findings. Adds latency + cost; defer to v2.

---

## Testing Strategy

### Unit Tests

| Test | Crate | What it verifies |
|------|-------|------------------|
| Cron parsing + next-run computation | `core` | Various cron expressions produce correct next times |
| Automation CRUD | `state` | Create, read, update, delete, list with filters |
| Due automation query | `state` | `get_due_automations` returns correct automations at given time |
| Run lifecycle | `state` | pending → running → completed/failed state transitions |
| Inbox pagination | `state` | Cursor pagination, unread filter, archive filter |
| WorktreeMode serialization | `state` | Round-trip through SQLite |

### Integration Tests

| Test | Crate | What it verifies |
|------|-------|------------------|
| Scheduler poll-and-run | `core` | Due automations trigger executor, `next_run_at` advances |
| Executor headless session | `core` | Automation prompt runs through Session, produces findings |
| Worktree create/cleanup | `utils/git` | Worktree created on run start, removed on archive |
| App-server CRUD | `app-server` | Full round-trip: create → list → update → delete via JSON-RPC |
| App-server run lifecycle | `app-server` | runNow → notifications → markRead → archive |

### Snapshot Tests

| Test | Crate | What it verifies |
|------|-------|------------------|
| CLI `automation list` output | `tui` | Rendered automation list matches snapshot |
| CLI `automation inbox` output | `tui` | Rendered inbox matches snapshot |

---

## Implementation Phases

### Phase 1: Data Model + CRUD (Foundation)
- State crate: migration, models, runtime queries
- App-server v2: type definitions, CRUD methods
- Config: `AutomationsConfig` + schema regeneration
- Tests: unit tests for state + app-server

### Phase 2: Executor (Headless Runs)
- Core: `AutomationExecutor` — create headless session, run prompt, collect findings
- Findings detection (structured output + heuristic)
- Integration with existing Session/exec infrastructure
- Tests: executor integration tests with wiremock

### Phase 3: Scheduler (Cron)
- Core: `AutomationScheduler` — tokio timer, poll-and-run loop
- Cron expression parsing + `next_run_at` computation
- App-server: start scheduler on server boot, stop on shutdown
- Tests: scheduler integration tests

### Phase 4: Worktree Support
- Utils/git: `create_worktree`, `remove_worktree`, `list_automation_worktrees`
- Executor: worktree creation before run, path passthrough
- Archive handler: worktree cleanup
- Stale worktree pruning on startup
- Tests: worktree lifecycle tests

### Phase 5: Notifications + Inbox
- App-server: emit `automation/runStarted`, `runCompleted`, `runFailed`
- App-server: `automationRun/list`, `markRead`, `archive`, `unreadCount`
- Tests: notification delivery, inbox queries

### Phase 6: CLI Subcommands
- CLI: `automation` subcommand group
- TUI: minimal list/inbox views (if warranted)

### Phase 7: snowflake-v0 UI (separate repo)
- Zustand stores, React components, schedule picker
- Wire to app-server v2 API
- Inbox panel in sidebar

---

## Open Questions

| # | Question | Impact | Proposed Default |
|---|----------|--------|------------------|
| 1 | Should automations persist across app restarts? | Scheduler must re-hydrate state | Yes — SQLite is durable, `next_run_at` recomputed on boot |
| 2 | Max concurrent runs? | Resource exhaustion risk | Default 2, configurable |
| 3 | Should "Run Now" bypass the schedule? | UX clarity | Yes — manual trigger is independent of schedule |
| 4 | Should automation history be bounded? | SQLite growth | Keep last 100 runs per automation, auto-archive older |
| 5 | Should the scheduler run in the TUI too? | TUI is typically short-lived | No — scheduler is app-server only; CLI gets `automation run` for manual triggers |
| 6 | Cron library choice? | Dependency | `croner` (lightweight, no-std compatible, well-maintained) |
| 7 | Should skills expansion happen in executor or in prompt? | Architecture | In executor — resolve `$skill-name` before submitting to Session |
| 8 | How to handle overlapping runs? | Same automation triggered while previous still running | Skip — log warning, don't spawn duplicate |

---

## Files Modified / Created

| File | Change |
|------|--------|
| `state/migrations/0021_automations.sql` | **New** — Create tables |
| `state/src/model/automation.rs` | **New** — Domain types |
| `state/src/model/mod.rs` | Add `mod automation` + re-exports |
| `state/src/runtime/automations.rs` | **New** — Query layer |
| `state/src/runtime.rs` | Add `mod automations` |
| `core/src/automations/mod.rs` | **New** — `AutomationScheduler` + `AutomationExecutor` |
| `core/src/lib.rs` | Add `mod automations` |
| `config/src/types.rs` | Add `AutomationsConfig` |
| `config/src/lib.rs` | Wire config section |
| `utils/git/src/operations.rs` | Add worktree create/remove/list functions |
| `app-server-protocol/src/protocol/v2.rs` | Add automation type definitions |
| `app-server-protocol/src/protocol/common.rs` | Register automation RPC methods |
| `app-server/src/orbit_code_message_processor.rs` | Handle automation RPC calls |
| `cli/src/lib.rs` | Add `automation` subcommand |
| `Cargo.toml` (workspace) | Add `croner` dependency |

---

## Edge Cases

| Edge Case | Resolution |
|-----------|------------|
| App killed while run is in-progress | On startup, mark stale `Running` runs as `Failed` with "interrupted" error |
| Project directory deleted/moved | Run fails with clear error; automation remains (user must update path or delete) |
| Git repo with dirty working tree + worktree mode | `git worktree add` works regardless of dirty state — this is fine |
| Git repo with dirty working tree + local mode | Warn in findings if uncommitted changes exist at run start |
| Cron expression fires while previous run still active | Skip the run; log it; `next_run_at` still advances |
| User deletes automation while run is active | Cancel the run (CancellationToken); clean up worktree |
| Network unavailable during run | Session/model call fails; run marked Failed with error |
| Invalid cron expression | Reject at create/update time — validate before storing |
| 100+ runs accumulated | Auto-archive runs older than configured threshold |
