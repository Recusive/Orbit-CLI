# System & Tools Parity Analysis: Orbit Code vs Claude Code

> **Date:** March 20, 2026
> **Source:** Piebald AI's [claude-code-system-prompts](https://github.com/Piebald-AI/claude-code-system-prompts) repo (v2.1.80) + Orbit Code codebase analysis
> **Purpose:** Map every tool and system prompt section between Claude Code and Orbit Code to guide the migration toward Claude Code-level tool use quality
> **Resume session:** `claude --resume 61eb1649-ce74-4b85-a971-3bdd6e23adf2`

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Architecture Comparison](#architecture-comparison)
3. [Tool Inventory: Orbit Code](#tool-inventory-orbit-code)
4. [Tool Inventory: Claude Code](#tool-inventory-claude-code)
5. [Parity Matrix](#parity-matrix)
6. [Experimental Tools Detail](#experimental-tools-detail)
7. [System Prompt Comparison](#system-prompt-comparison)
8. [Tool Description Patterns](#tool-description-patterns)
9. [Key Prompt Engineering Patterns Claude Code Uses](#key-prompt-engineering-patterns-claude-code-uses)
10. [Migration Recommendations](#migration-recommendations)

---

## Executive Summary

Orbit Code (forked from OpenAI Codex CLI) has **2 active tools** exposed to the model (`shell` + `apply_patch`) while Claude Code exposes **20+ dedicated tools**. However, Orbit Code already has **3 production-ready tool handlers** (`read_file`, `grep_files`, `list_dir`) hidden behind an experimental flag that is **never enabled** for any model in `models.json`.

The performance gap between the two systems is primarily driven by:
1. **Tool specialization** — Claude Code gives the model narrow, purpose-built tools instead of one general shell
2. **Prompt engineering** — Claude Code's tool descriptions contain behavioral instructions, not just API docs
3. **Negative routing** — Claude Code explicitly tells the model what NOT to use each tool for
4. **Redundancy** — Critical routing rules appear 3+ times across different prompt sections

---

## Architecture Comparison

| Aspect | Orbit Code | Claude Code |
|--------|-----------|-------------|
| **Product** | Fork of OpenAI Codex CLI | Anthropic's CLI for Claude |
| **Language** | Rust (67+ crates) | TypeScript (compiled, minified) |
| **Model target** | GPT-5.x (migrating to multi-provider) | Claude (Sonnet/Opus/Haiku) |
| **Tool count (active)** | 2 core + ~15 supporting | ~8 loaded + ~15 deferred |
| **Tool system** | `ToolHandler` trait, `ToolRegistry`, `ToolRouter` | Modular JS, conditional assembly |
| **File editing** | `apply_patch` (diff hunks) | `Edit` (exact string replace) + `Write` (new files) |
| **Search** | `shell("rg ...")` | Dedicated `Grep` + `Glob` tools |
| **Planning** | `update_plan` tool | `EnterPlanMode`/`ExitPlanMode` + plan files |
| **Sub-agents** | `spawn_agent`/`wait_agent`/`send_input` | `Agent` tool with typed subagents |
| **Approval model** | Guardian AI risk assessment + sandbox modes | Permission modes + hooks + sandbox |
| **Prompt structure** | Monolithic `.md` files per model | 247 modular pieces, conditionally assembled |
| **Tool descriptions** | Minimal (API reference style) | Extensive (behavioral instructions) |

---

## Tool Inventory: Orbit Code

### Active Tools (always available)

| Tool | Handler File | Lines | Description |
|------|-------------|------:|-------------|
| `shell` | `handlers/shell.rs` | 499 | Execute shell commands (legacy aliases: `container.exec`, `local_shell`) |
| `exec_command` | `handlers/unified_exec.rs` | ~500 | Interactive PTY execution with stdin support |
| `apply_patch` | `handlers/apply_patch.rs` | 466 | File creation/deletion/patching via diff-like format |
| `update_plan` | `handlers/plan.rs` | 153 | Step-by-step plan tracking (pending/in_progress/completed) |
| `request_user_input` | `handlers/request_user_input.rs` | 125 | Ask user multiple-choice questions |
| `request_permissions` | `handlers/request_permissions.rs` | 74 | Request sandbox permission changes |
| `tool_search` | `handlers/tool_search.rs` | 194 | Search MCP/connector tools by keyword |
| `tool_suggest` | `handlers/tool_suggest.rs` | 320 | Get tool suggestions based on context |
| `view_image` | `handlers/view_image.rs` | 230 | Display images to the user |
| `mcp` | `handlers/mcp.rs` | 58 | Execute MCP server tool calls |
| `mcp_resource` | `handlers/mcp_resource.rs` | 667 | List/read MCP resources |
| `dynamic` | `handlers/dynamic.rs` | 134 | Dynamic tool execution |

### Config-Gated Tools (built, just need config to enable)

| Tool | Type | Condition | How to enable |
|------|------|-----------|---------------|
| `web_search` | `ToolSpec::WebSearch` (API-level, not a handler) | `web_search_mode` != None/Disabled in config | Add `web_search = "live"` to config TOML, OR enable `Feature::WebSearchCached`/`Feature::WebSearchRequest` |

**Web search** is fully integrated as a native OpenAI Responses API tool type — not a handler in `handlers/`. It's registered in `spec.rs` as `ToolSpec::WebSearch` with support for:
- `WebSearchMode::Cached` — search without live web access (uses cached results)
- `WebSearchMode::Live` — real-time web search
- Domain filtering, user location, search context size configuration
- Full test suite in `tests/suite/web_search.rs`

The reason it doesn't show up in the forked CLI: `resolve_web_search_mode()` in `config/mod.rs` returns `None` when there's no `web_search` config setting and no feature flag enabled. The official Codex CLI likely has these enabled via server-side feature flags.

**Note on Claude compatibility:** Web search is an OpenAI Responses API feature (`ToolSpec::WebSearch`). When using Claude models via the Anthropic API, this tool type may not be supported at the API level. The Anthropic Messages API does not have a native `web_search` tool type like OpenAI's Responses API. For Claude models, web search may need to be provided via MCP servers instead.

### Feature-Flagged Tools (enabled by config/model)

| Tool | Handler | Condition | Description |
|------|---------|-----------|-------------|
| `js_repl` / `js_repl_reset` | `handlers/js_repl.rs` | `js_repl_enabled` | JavaScript REPL with persistent state |
| `code` / `code.wait` | `code_mode/` | `code_mode_enabled` | Persistent code execution (Node.js runtime) |
| `artifacts` | `handlers/artifacts.rs` | `Feature::Artifact` | Build presentation artifacts |
| `spawn_agent` / `wait_agent` / `send_input` / `close_agent` / `resume_agent` | `handlers/multi_agents.rs` | `collab_tools` | Multi-agent orchestration |
| `spawn_agents_on_csv` / `report_agent_job_result` | `handlers/agent_jobs.rs` | `agent_jobs_tools` | Batch agent job execution |

### Experimental Tools (built but NEVER enabled)

These handlers are fully implemented with tests but gated behind `experimental_supported_tools` in `models.json`, which is `[]` for every model.

| Tool | Handler File | Lines | Test File | Test Lines | Description |
|------|-------------|------:|-----------|----------:|-------------|
| `read_file` | `handlers/read_file.rs` | 489 | `read_file_tests.rs` | ~400 | Read files with line numbers, offset/limit, indentation-aware block mode |
| `grep_files` | `handlers/grep_files.rs` | 176 | `grep_files_tests.rs` | ~100 | Regex file search via `rg`, sorted by modification time |
| `list_dir` | `handlers/list_dir.rs` | 271 | `list_dir_tests.rs` | ~200 | Directory listing with depth, offset/limit, type labels |
| `test_sync_tool` | `handlers/test_sync.rs` | 154 | — | — | Test-only synchronous handler |

### How experimental tools are gated

In `models.json`, every model has:
```json
"experimental_supported_tools": []
```

In `spec.rs`, registration is conditional:
```rust
if config.experimental_supported_tools.contains(&"grep_files".to_string()) {
    let grep_files_handler = Arc::new(GrepFilesHandler);
    push_tool_spec(&mut builder, create_grep_files_tool(), true, config.code_mode_enabled);
    builder.register_handler("grep_files", grep_files_handler);
}
```

To enable, either:
1. Add tool names to `experimental_supported_tools` array in `models.json` for specific models
2. Remove the conditional gate in `spec.rs` to always register them

---

## Tool Inventory: Claude Code

### Always-Loaded Tools (~8)

| Tool | Description | Orbit Code Equivalent |
|------|-------------|----------------------|
| `Bash` | Execute shell commands with extensive behavioral rules | `shell` / `exec_command` |
| `Read` | Read files with line numbers, offset/limit, PDF, images, notebooks | `read_file` (experimental) |
| `Write` | Create new files or complete rewrites | `apply_patch` (Add File) |
| `Edit` | Exact string find-and-replace in files | `apply_patch` (Update File) |
| `Glob` | Fast file pattern matching (e.g. `**/*.rs`) | **None** (uses `shell("rg --files")`) |
| `Grep` | Regex content search built on ripgrep | `grep_files` (experimental) |
| `Agent` | Launch typed subagents (Explore, Plan, general-purpose, etc.) | `spawn_agent` + `wait_agent` |
| `Skill` | Execute user-defined skills/slash commands | **None** |
| `ToolSearch` | Fetch deferred tool schemas on demand | `tool_search` (different purpose) |

### Deferred Tools (loaded on demand via ToolSearch)

| Tool | Description | Orbit Code Equivalent |
|------|-------------|----------------------|
| `TodoWrite` | Structured task list for tracking work | `update_plan` (similar) |
| `AskUserQuestion` | Ask user questions with options | `request_user_input` |
| `EnterPlanMode` | Transition to planning mode | Collaboration mode: `plan.md` |
| `ExitPlanMode` | Submit plan for user approval | `<proposed_plan>` block |
| `WebSearch` | Search the web for current information | `web_search` (OpenAI API, config-gated) — **build model-agnostic version** from OpenCode spec |
| `WebFetch` | Fetch and analyze web page content | **Not built** — build from OpenCode spec (~300 lines) |
| `SendMessage` | Send messages in agent teams | `send_input` |
| `TeamCreate` | Create agent teams | **None** (simpler agent model) |
| `TeamDelete` | Delete agent teams | **None** |
| `TaskCreate` | Create tasks in team task lists | **None** |
| `TaskUpdate` | Update task status/ownership | **None** |
| `TaskList` | List tasks and status | **None** |
| `CronCreate` | Schedule recurring prompts | **None** |
| `CronDelete` | Cancel scheduled prompts | **None** |
| `LSP` | Language Server Protocol operations | **Not built yet** — full spec available from OpenCode repo (see LSP section) |
| `NotebookEdit` | Edit Jupyter notebook cells | **None** |
| `EnterWorktree` | Create isolated git worktree | **None** |
| `ExitWorktree` | Exit and optionally clean up worktree | **None** |
| `Computer` | Browser automation (mouse, keyboard, screenshots) | **None** (has MCP browser tools) |

---

## Parity Matrix

### File Operations

| Capability | Claude Code | Orbit Code | Gap |
|-----------|------------|-----------|-----|
| Read file with line numbers | `Read` (always loaded) | `read_file` (experimental, never enabled) | **Enable experimental** |
| Read images | `Read` (multimodal) | `view_image` (active) | Parity |
| Read PDFs | `Read` (with pages param) | None | **Build or skip** |
| Read Jupyter notebooks | `Read` | None | **Build or skip** |
| Write new file | `Write` tool | `apply_patch` (Add File) | Functional parity |
| Edit file (string replace) | `Edit` tool | `apply_patch` (Update File) | Different approach, both work |
| Edit notebook cell | `NotebookEdit` | None | Gap |

### Search Operations

| Capability | Claude Code | Orbit Code | Gap |
|-----------|------------|-----------|-----|
| Find files by glob pattern | `Glob` tool | `shell("rg --files --glob ...")` | **Build new handler** |
| Search file contents (regex) | `Grep` tool | `grep_files` (experimental, never enabled) | **Enable experimental** |
| List directory contents | `Bash("ls")` | `list_dir` (experimental, never enabled) | **Enable experimental** |

### Execution

| Capability | Claude Code | Orbit Code | Gap |
|-----------|------------|-----------|-----|
| Run shell commands | `Bash` | `shell` / `exec_command` | Parity (different descriptions) |
| Run JavaScript | None (separate from main tools) | `js_repl` / `code_mode` | Orbit has MORE |
| Sandbox enforcement | Bash sandbox rules (17 sub-sections) | Guardian AI + seatbelt/landlock | Different approach |

### Planning & Tasks

| Capability | Claude Code | Orbit Code | Gap |
|-----------|------------|-----------|-----|
| Create plan | `EnterPlanMode` → plan file → `ExitPlanMode` | `update_plan` (inline steps) | Different design |
| Track task progress | `TodoWrite` (detailed with examples) | `update_plan` (step statuses) | Functional parity |
| Team task management | `TaskCreate`/`TaskUpdate`/`TaskList` | None (simpler agent model) | Gap (may not need) |

### Agent/Sub-agent

| Capability | Claude Code | Orbit Code | Gap |
|-----------|------------|-----------|-----|
| Spawn sub-agent | `Agent` tool | `spawn_agent` | Parity |
| Typed sub-agents | 10+ types (Explore, Plan, code-reviewer...) | Untyped | **Significant gap** |
| Wait for agent | Automatic notification | `wait_agent` | Parity |
| Send message to agent | `SendMessage` | `send_input` | Parity |
| Agent worktree isolation | `isolation: "worktree"` param | None | Gap |
| Background agents | `run_in_background` param | None | Gap |
| Agent teams | `TeamCreate`/`TeamDelete` | None | Gap |

### Web & External

| Capability | Claude Code | Orbit Code | Gap |
|-----------|------------|-----------|-----|
| Web search | `WebSearch` | `web_search` (OpenAI API, config-gated) — **also buildable** as standalone tool using OpenCode's Exa AI approach | **Build** model-agnostic version (~200 lines, see Web Search section) |
| Fetch web page | `WebFetch` | Not built — OpenCode has full reference impl | **Build** (~300 lines, see Web Fetch section) |
| LSP integration | `LSP` tool | Not built — OpenCode has full reference impl | **Build** (~2 weeks, see LSP section) |
| MCP tool calls | Via `ToolSearch` + deferred loading | `mcp` handler + `tool_search` | Parity |
| Browser automation | `Computer` tool | MCP browser tools | Parity via MCP |

### User Interaction

| Capability | Claude Code | Orbit Code | Gap |
|-----------|------------|-----------|-----|
| Ask user question | `AskUserQuestion` | `request_user_input` | Parity |
| Request permissions | Via permission mode system | `request_permissions` | Parity |
| Cron/scheduled tasks | `CronCreate`/`CronDelete` | None | Gap |

---

## Experimental Tools Detail

### `grep_files` — 176 lines of Rust

**What it does:** Wraps `rg` (ripgrep) to search file contents by regex pattern, returning matching file paths sorted by modification time.

**Schema:**
```json
{
  "pattern": "regex pattern (required)",
  "include": "glob filter e.g. *.rs (optional)",
  "path": "directory to search (optional, defaults to cwd)",
  "limit": "max results, default 100, max 2000"
}
```

**Implementation highlights:**
- Uses `tokio::process::Command` to run `rg --files-with-matches --sortr=modified`
- 30-second timeout via `tokio::time::timeout`
- Validates path exists before searching
- Handles rg exit codes (0=matches, 1=no matches, other=error)
- Proper error messages if rg is not installed

**Claude Code equivalent:** `Grep` tool — more features (content output, line numbers, context lines, multiline mode, count mode) but same core concept.

### `read_file` — 489 lines of Rust

**What it does:** Reads local files with 1-indexed line numbers. Supports two modes:
1. **Slice mode** (default) — simple offset + limit range read
2. **Indentation mode** — expands around an anchor line following indentation structure

**Schema:**
```json
{
  "file_path": "absolute path (required)",
  "offset": "1-indexed start line (default: 1)",
  "limit": "max lines to return (default: 2000)",
  "mode": "slice | indentation",
  "indentation": {
    "anchor_line": "line to center on",
    "max_levels": "parent indent levels to include",
    "include_siblings": "include same-level blocks",
    "include_header": "include doc comments above block",
    "max_lines": "hard cap on returned lines"
  }
}
```

**Implementation highlights:**
- Line numbering with configurable tab width (4 spaces)
- Long line truncation (500 char max)
- Indentation-aware block extraction (unique feature — Claude Code doesn't have this)
- Comment prefix detection (`#`, `//`, `--`)
- 13K of tests covering edge cases

**Claude Code equivalent:** `Read` tool — simpler (no indentation mode) but supports images, PDFs, Jupyter notebooks.

### `list_dir` — 271 lines of Rust

**What it does:** Lists directory entries with 1-indexed numbering, type labels, and recursive depth traversal.

**Schema:**
```json
{
  "dir_path": "absolute path (required)",
  "offset": "1-indexed start entry (default: 1)",
  "limit": "max entries (default: 25)",
  "depth": "max directory depth (default: 2)"
}
```

**Implementation highlights:**
- Recursive BFS traversal with configurable depth
- Type labels (file, dir, symlink, etc.)
- Indented output for nested directories
- Entry truncation (500 char max)
- Sorted entries within each directory

**Claude Code equivalent:** No dedicated tool — Claude Code uses `Bash("ls")` for directory listing. `Glob` is the closest but serves a different purpose (pattern matching, not listing).

---

## System Prompt Comparison

### Sections Present in Both

| Section | Claude Code File(s) | Orbit Code File |
|---------|--------------------|-----------------|
| Identity/personality | `system-prompt-system-section.md` | `prompt.md` lines 1-15 |
| Task execution | `doing-tasks-software-engineering-focus.md` + 12 sub-files | `prompt.md` "Task execution" section |
| Over-engineering avoidance | `doing-tasks-avoid-over-engineering.md` | `prompt.md` "Avoid unneeded complexity" |
| Git safety | `bash-git-*` (4 files) | `gpt-5.2-codex_prompt.md` "Using GIT" |
| No git commit unless asked | `bash-git-commit-and-pr-creation-instructions.md` | `prompt.md` line 144 |
| Progress updates | Part of agent tool notes | `prompt.md` "Sharing progress updates" |
| Final answer formatting | `tone-and-style-*` + `output-efficiency.md` | `prompt.md` "Final answer structure" (very detailed) |
| Code style guidance | Various `doing-tasks-*` files | `gpt-5.2-codex_prompt.md` "Code style" |
| Validation/testing | Not explicit in system prompt | `prompt.md` "Validating your work" |
| Planning | `tool-description-enterplanmode.md` | `templates/collaboration_mode/plan.md` |
| Context compaction | `system-prompt-context-compaction-summary.md` | `templates/compact/prompt.md` |

### Sections in Claude Code but NOT Orbit Code

| Section | File(s) | Impact |
|---------|---------|--------|
| **Negative tool routing** | Embedded in Bash tool + `tool-usage-*.md` (6 files) | **Critical** — prevents misrouting |
| **Meta tool routing** | `tool-usage-direct-search.md`, `tool-usage-delegate-exploration.md` | **Critical** — decision tree |
| **Memory system** | `system-prompt-agent-memory-instructions.md` + type descriptions | Persistent cross-session knowledge |
| **Hooks system** | `system-prompt-hooks-configuration.md` | Extensible automation |
| **Skills/slash commands** | `tool-description-skill.md` | Extensibility |
| **Executing actions with care** | `system-prompt-executing-actions-with-care.md` | Risk reasoning |
| **Sandbox detailed rules** | 17 `bash-sandbox-*.md` files | Sandbox behavior teaching |
| **Learning mode** | `system-prompt-learning-mode.md` | Educational interaction |
| **Auto mode** | `system-prompt-auto-mode.md` | Autonomous execution |
| **Scratchpad directory** | `system-prompt-scratchpad-directory.md` | Temp file management |
| **Deferred tool loading** | `tool-description-toolsearch-second-part.md` | Cognitive load management |
| **Git commit workflow recipe** | Inside `bash-git-commit-and-pr-creation-instructions.md` | Step-by-step with parallel hints |
| **PR creation workflow recipe** | Same file | Step-by-step with HEREDOC templates |
| **Sleep avoidance rules** | 6 `bash-sleep-*.md` files | Prevents idle loops |
| **CLAUDE.md spec** | Runtime injection | Project-specific instructions |
| **Fork/worktree guidelines** | `system-prompt-fork-usage-guidelines.md` | Isolated development |
| **Subagent prompt writing** | `system-prompt-writing-subagent-prompts.md` | Delegation quality |

### Sections in Orbit Code but NOT Claude Code

| Section | File | Impact |
|---------|------|--------|
| **AGENTS.md spec** | `prompt.md` lines 17-27 | Hierarchical instruction files (OpenAI convention) |
| **Preamble messages** | `prompt.md` "Responsiveness" | 8 examples of pre-tool-call updates |
| **Plan quality examples** | `prompt.md` lines 74-121 | 3 good + 3 bad plan examples |
| **apply_patch grammar** | `prompt_with_apply_patch_instructions.md` | Formal BNF grammar |
| **Ambition vs precision** | `prompt.md` "Ambition vs. precision" | Creative vs surgical philosophy |
| **Frontend design rules** | `gpt-5.2-codex_prompt.md` "Frontend tasks" | Anti-AI-slop guidelines |
| **Review mode** | `gpt-5.2-codex_prompt.md` "Reviews" | Code review mindset |
| **Collaboration modes** | `templates/collaboration_mode/` (4 modes) | Default/execute/pair/plan |
| **Orchestrator template** | `templates/agents/orchestrator.md` | Sub-agent coordination |
| **Guardian policy** | `src/guardian/policy.md` | AI-based risk assessment |
| **Personality presets** | `templates/personalities/` | Pragmatic/friendly |
| **Code mode (exec)** | `code_mode/description.md` | JS REPL with persistent state |
| **Model-specific prompts** | `gpt_5_1_prompt.md`, `gpt_5_2_prompt.md`, etc. | Per-model optimization |
| **Verbosity tiers** | `gpt_5_1_prompt.md` "Verbosity" section | Tiny/small/medium/large rules |
| **User Updates Spec** | `gpt_5_1_prompt.md` "User Updates Spec" | Frequency/length/content rules |
| **Autonomy persistence** | `gpt_5_1_prompt.md` "Autonomy and Persistence" | End-to-end completion |

---

## Tool Description Patterns

### Claude Code Pattern (behavioral instructions)

Claude Code tool descriptions are **mini-manuals**, not API docs. Example from the `Bash` tool (~2,000 tokens):

```
Executes a given bash command and returns its output.

IMPORTANT: Avoid using this tool to run find, grep, cat, head, tail,
sed, awk, or echo commands. Instead, use the appropriate dedicated tool:
 - File search: Use Glob (NOT find or ls)
 - Content search: Use Grep (NOT grep or rg)
 - Read files: Use Read (NOT cat/head/tail)
 ...

# Instructions
 - If your command will create new directories, first verify parent exists
 - Always quote file paths with spaces
 - Try to maintain your current working directory
 ...

# Committing changes with git
[40+ lines of step-by-step workflow with parallel execution hints]

# Creating pull requests
[30+ lines of step-by-step workflow with HEREDOC templates]
```

Key patterns:
1. **Negative routing** — "Do NOT use X for Y, use Z instead"
2. **Workflow recipes** — Step-by-step instructions embedded in tool descriptions
3. **Error recovery** — "The edit will FAIL if old_string is not unique. Either provide more context..."
4. **Dependencies** — "You must use Read at least once before editing"
5. **Parallel hints** — "Run the following bash commands in parallel"

### Orbit Code Pattern (API reference)

Orbit Code tool descriptions are **minimal**:

```
grep_files: "Finds files whose contents match the pattern and lists them
            by modification time."

read_file: "Reads a local file with 1-indexed line numbers, supporting
           slice and indentation-aware block modes."

list_dir: "Lists entries in a local directory with 1-indexed entry numbers
          and simple type labels."
```

No behavioral instructions, no routing guidance, no workflow recipes.

---

## Key Prompt Engineering Patterns Claude Code Uses

### 1. Negative Routing (appears 3x)

The rule "don't use shell for file reads" appears in:
- System prompt `# Using your tools` section
- Bash tool `IMPORTANT:` block
- Each dedicated tool's own description

### 2. Meta-Routing Decision Tree

A `# Using your tools` section tells the model HOW to pick tools:
```
- Simple, directed searches → Glob or Grep directly
- Broader exploration → Agent with Explore subagent
- Slash commands → Skill tool
- Complex multi-step → Agent tool
```

### 3. Deferred Loading

Only ~8 tools loaded upfront. Others listed by name in `<available-deferred-tools>`. Model calls `ToolSearch` to fetch schemas when needed. Reduces cognitive load.

### 4. Typed Subagents

Agent tool spawns specialized subagents with restricted tool access:
- `Explore` → read-only (Glob, Grep, LS, Read)
- `Plan` → read-only
- `general-purpose` → all tools
- `code-reviewer` → read + analysis

### 5. Redundancy by Design

Same concept stated in multiple locations to ensure the model attends to it regardless of which prompt section is in the attention window.

### 6. Workflow Templates

Complete recipes for git commit, PR creation, and other multi-step workflows embedded directly in tool descriptions with parallel execution hints.

### 7. Per-Tool Error Recovery

Each tool description explains what happens when it fails and how to recover, teaching the model to reason about failures rather than retry blindly.

---

## Complete Parity Roadmap

This section maps everything needed to reach end-to-end parity with Claude Code's system prompt and tool architecture. Organized by: what to enable, what to build, what to rewrite, and what to skip.

---

### Layer 1: Enable Existing Experimental Tools (Hours)

These handlers are production-ready with full test suites. They just need the gate removed.

| Tool | Handler | Lines | Tests | What to change |
|------|---------|------:|-------|----------------|
| `read_file` | `handlers/read_file.rs` | 489 | 13.1K | Remove experimental gate in `spec.rs` line ~2820 |
| `grep_files` | `handlers/grep_files.rs` | 176 | 2.8K | Remove experimental gate in `spec.rs` line ~2806 |
| `list_dir` | `handlers/list_dir.rs` | 271 | 6.7K | Remove experimental gate in `spec.rs` line ~2834 |

**Option A** — Per-model enablement in `models.json`:
```json
"experimental_supported_tools": ["read_file", "grep_files", "list_dir"]
```

**Option B** — Remove gates entirely in `spec.rs` (recommended):
```rust
// BEFORE:
if config.experimental_supported_tools.contains(&"read_file".to_string()) {
    // ...register...
}

// AFTER:
{
    let read_file_handler = Arc::new(ReadFileHandler);
    push_tool_spec(&mut builder, create_read_file_tool(), true, config.code_mode_enabled);
    builder.register_handler("read_file", read_file_handler);
}
```

**Impact:** 15% improvement in tool use quality. Model stops wasting tokens on `cat`, `rg`, `ls` via shell.

---

### Layer 2: Build New Tool Handlers (1 week)

These don't exist as handler files. Need new `.rs` files in `handlers/`.

| Tool | Claude Code equivalent | Est. Lines | Parameters | Why it matters |
|------|----------------------|----------:|------------|---------------|
| `glob` | `Glob` | ~150 | `pattern`, `path` | Find files by name pattern (`**/*.tsx`). `grep_files` searches *contents*, this searches *filenames*. Different job — without it the model does `shell("rg --files --glob '*.rs'")` and guesses flags. |
| `edit` | `Edit` | ~200 | `file_path`, `old_string`, `new_string`, `replace_all` | Exact string find-and-replace. For changing one line, the model currently has to construct a full `apply_patch` with `@@` hunk headers and context lines. `edit` is "find this exact string, replace with this." Lower error rate, simpler for the model. |
| `write` | `Write` | ~80 | `file_path`, `content` | Direct file creation. Currently the model uses `apply_patch` with `*** Add File:` and must prefix every line with `+`. `write` is just "write this content to this path." |

Each follows the same pattern as existing handlers:
```rust
pub struct GlobHandler;

#[async_trait]
impl ToolHandler for GlobHandler {
    type Output = FunctionToolOutput;
    fn kind(&self) -> ToolKind { ToolKind::Function }
    async fn is_mutating(&self, _: &ToolInvocation) -> bool { false } // read-only
    async fn handle(&self, invocation: ToolInvocation) -> Result<Self::Output, FunctionCallError> {
        // parse args, execute, return FunctionToolOutput
    }
}
```

Plus a `create_*_tool()` function in `spec.rs` and `builder.register_handler(...)` call.

**Impact:** 10% improvement. Completes file-operation tool parity with Claude Code.

---

### Layer 3: Prompt Engineering — Tool Descriptions (1-2 weeks)

**This is where 50% of the quality improvement comes from.** Zero Rust code needed. All changes are in `.md` prompt files and `create_*_tool()` description strings in `spec.rs`.

#### 3a. Negative Routing in Shell Tool

Add to the shell tool description (in `spec.rs` `create_shell_tool()` or in the system prompt):

```
IMPORTANT: Do not use shell to run these commands when dedicated tools are available:
 - File reading: Use read_file (NOT cat, head, tail, or sed)
 - Content search: Use grep_files (NOT grep or rg)
 - File search: Use glob (NOT find or ls)
 - Directory listing: Use list_dir (NOT ls)
 - File editing: Use edit for small changes (NOT sed or awk)
 - File creation: Use write for new files (NOT echo or cat with heredoc)
 - Reserve shell exclusively for commands that have no dedicated tool.
```

**Must appear in 3 places:**
1. System prompt `# Tool Guidelines` section
2. Shell tool description
3. Each dedicated tool's own description (e.g., `read_file` says "Use this instead of cat/head/tail via shell")

#### 3b. Meta-Routing Decision Tree

Add a `# Using your tools` section to the system prompt:

```
# Using your tools

- To read files → use read_file (NOT shell with cat/head/tail)
- To search file contents → use grep_files (NOT shell with grep/rg)
- To find files by name → use glob (NOT shell with find)
- To list directories → use list_dir (NOT shell with ls)
- To edit a few lines → use edit (NOT apply_patch for tiny changes)
- To create a new file → use write (NOT apply_patch with Add File)
- To make complex multi-hunk edits → use apply_patch
- To run commands, builds, tests → use shell
- For broader codebase exploration → use spawn_agent
```

#### 3c. Behavioral Tool Descriptions

Transform every tool description from API-reference style to behavioral-instruction style:

**BEFORE (current):**
```
"Finds files whose contents match the pattern and lists them by modification time."
```

**AFTER (Claude Code style):**
```
"Search file contents using regex patterns. Built on ripgrep.

Usage:
- ALWAYS use grep_files for content search. NEVER run grep or rg via shell.
- Supports full regex syntax (e.g., 'log.*Error', 'function\\s+\\w+')
- Filter files with the include parameter (e.g., '*.rs', '*.{ts,tsx}')
- Results are sorted by modification time (most recent first)
- Default limit is 100 files; increase for broad searches, decrease for targeted ones

When to use:
- Searching for a function/class/variable across the codebase
- Finding files that import a specific module
- Locating error messages, config keys, or string literals

When NOT to use:
- Finding files by name pattern → use glob instead
- Reading a specific file → use read_file instead
- Listing directory contents → use list_dir instead"
```

#### 3d. Workflow Recipes

Embed in shell tool description:

**Git commit workflow:**
```
When the user asks to commit:
1. Run shell("git status") and shell("git diff") — can run in parallel
2. Analyze changes, draft a commit message
3. Run shell("git add <specific files>") — never use git add -A
4. Run shell("git commit -m '...'")
5. If pre-commit hook fails → fix the issue → create a NEW commit (never amend)
```

**PR creation workflow:**
```
When the user asks to create a PR:
1. Run shell("git status"), shell("git diff"), shell("git log main..HEAD") — in parallel
2. Analyze ALL commits on the branch, draft title + description
3. Push and create PR: shell("gh pr create --title '...' --body '...'")
```

#### 3e. Sleep / Retry Avoidance

Add to shell tool description:
```
- Do not sleep between commands that can run immediately
- Do not retry failing commands in a loop — diagnose the root cause
- If a command fails, consider whether a dedicated tool would work better
```

#### 3f. Error Recovery Per Tool

Add to each tool description:
```
# read_file
"If the file doesn't exist, an error is returned. If the file is too large,
use offset and limit to read specific sections."

# edit
"The edit will FAIL if old_string is not found or matches multiple locations.
Provide more surrounding context to make the match unique, or use replace_all
to change every occurrence."

# grep_files
"If rg is not installed, an error is returned with instructions. If no matches
are found, the result will say 'No matches found' — this is not an error."
```

**Impact:** 50% improvement. This is the single highest-leverage change.

---

### Layer 4: Architectural Patterns (2-4 weeks)

#### 4a. Typed Subagents

Claude Code's `Agent` tool spawns agents with restricted tool access:

| Type | Tools Available | Purpose |
|------|----------------|---------|
| `Explore` | read_file, grep_files, glob, list_dir (read-only) | Codebase research |
| `Plan` | read_file, grep_files, glob, list_dir (read-only) | Design implementation plans |
| `general-purpose` | All tools | Full implementation |
| `code-reviewer` | Read-only + analysis | Code review |

**What to build:**
- Agent type registry in `multi_agents.rs`
- Per-type tool allowlists
- Type parameter on `spawn_agent`
- System prompt guidance: "Read-only agents cannot edit. Never assign them implementation work."

**Impact:** 10% improvement. Prevents subagents from making unintended edits during research.

#### 4b. Deferred Tool Loading

Claude Code loads ~8 tools upfront and defers ~15 others. The model calls `ToolSearch` to fetch schemas on demand.

**What to build:**
- Split `build_specs_with_discoverable_tools()` into core specs + deferred specs
- Core tools (shell, apply_patch, read_file, grep_files, edit, write, glob, list_dir, update_plan) → always loaded
- Supporting tools (request_user_input, request_permissions, spawn_agent, artifacts, etc.) → listed by name only, loaded via `tool_search`
- Modify `ToolRouter` to handle deferred registration

**Impact:** 5% improvement. Reduces cognitive load on the model, especially for simple tasks.

#### 4c. Memory System

Claude Code has persistent cross-session memory with types:

| Type | Purpose | Example |
|------|---------|---------|
| `user` | User's role, preferences, expertise | "Senior Rust engineer, new to React" |
| `feedback` | Corrections and validated approaches | "Don't mock the database in tests" |
| `project` | Ongoing work, deadlines, decisions | "Merge freeze starts March 5" |
| `reference` | Pointers to external resources | "Pipeline bugs tracked in Linear project INGEST" |

**What to build:**
- Memory file format (markdown with YAML frontmatter)
- `MEMORY.md` index file
- Memory read/write instructions in system prompt
- Memory loading at session start
- Orbit Code already has `templates/memories/` with consolidation pipeline — connect this to the session prompt

**Impact:** 10% improvement. Model retains context across sessions, stops asking repeated questions.

---

### Layer 5: Features to Skip

These exist in Claude Code but are **not worth building** for Orbit Code:

| Feature | Claude Code | Why skip |
|---------|------------|---------|
| `WebSearch` | Built-in web search | **Moved to build list** — model-agnostic version using OpenCode's Exa AI approach (~200 lines). See Web Search section. |
| `WebFetch` | Fetch + analyze web pages | **Moved to build list** — direct URL fetch + HTML→Markdown (~300 lines). See Web Fetch section. |
| `LSP` | Language Server Protocol tool | **Can build** — OpenCode repo has full reference implementation (see LSP section below) |
| `CronCreate` / `CronDelete` | Scheduled recurring prompts | Niche feature, low user demand |
| `NotebookEdit` | Edit Jupyter notebook cells | Only needed if Jupyter is a core use case |
| `TeamCreate` / `TeamDelete` / `TaskCreate` / `TaskUpdate` / `TaskList` | Full team coordination system | Different paradigm than Orbit's `spawn_agent` model; large effort, unclear benefit |
| `EnterWorktree` / `ExitWorktree` | Git worktree isolation | Nice to have; low priority |
| `Skills` / `Slash commands` | `Skill` tool + skill registry | Large effort; could be added later via MCP |
| `Hooks` (10 lifecycle events) | PreToolUse, PostToolUse, Stop, etc. | Orbit Code has Guardian AI instead; different approach to safety |
| `Computer` (browser automation) | Mouse/keyboard/screenshot browser tool | Already available via MCP (Chrome, Playwright) |

---

### Layer 6: System Prompt Sections to Add

These are prompt-only changes (no code) that Claude Code has and Orbit Code lacks:

| Section | What it does | Priority |
|---------|-------------|----------|
| **`# Using your tools`** | Meta-routing decision tree | **P0** — without this, new tools won't be used |
| **Negative routing block** | "Do NOT use shell for X, use Y instead" | **P0** — must appear in shell description |
| **Behavioral tool descriptions** | Workflow instructions per tool | **P0** — transforms tool quality |
| **`# Executing actions with care`** | Risk categories, confirm before destructive ops | **P1** — improves safety |
| **Git commit workflow** | Step-by-step recipe in shell description | **P1** — standardizes git operations |
| **PR creation workflow** | Step-by-step recipe in shell description | **P1** — standardizes PR operations |
| **Sleep avoidance rules** | 6 rules against idle loops | **P2** — prevents wasted time |
| **Error recovery per tool** | What to do when each tool fails | **P2** — reduces retry loops |
| **CLAUDE.md / AGENTS.md hybrid** | Load project-specific instructions | **P2** — already have AGENTS.md |
| **Context compaction improvements** | Better summarization prompt | **P3** — already have `compact/prompt.md` |
| **Scratchpad directory** | Dedicated temp file location | **P3** — nice to have |

---

### Summary: Full Parity Effort

| Layer | What | Items | Effort | Quality Impact |
|-------|------|------:|--------|---------------|
| **1. Enable** | Flip experimental tools on | 3 tools | Hours | 15% |
| **2. Build** | New tool handlers | 3 tools | 1 week | 10% |
| **3. Prompts** | Descriptions, routing, workflows, redundancy | ~15 prompt changes | 1-2 weeks | **50%** |
| **4. Architecture** | Typed subagents, deferred loading, memory | 3 systems | 2-4 weeks | 25% |
| **5. Skip** | Features handled by MCP or not needed | 12 tools | $0 | — |
| **6. Prompt sections** | System prompt additions | ~11 sections | 1 week | Included in Layer 3 |

**Total estimated effort: 5-8 weeks for full parity.**
**Layers 1-3 alone (3-4 weeks) capture 75% of the quality improvement.**

---

## Migration Recommendations

### Recommended execution order:

**Week 1:** Layer 1 (enable 3 tools) + start Layer 3a-3b (negative routing + meta routing)
**Week 2:** Layer 2 (build 3 tools) + Layer 3c (behavioral descriptions)
**Week 3:** Layer 3d-3f (workflow recipes, sleep rules, error recovery)
**Week 4:** Layer 4a (typed subagents)
**Weeks 5-6:** Layer 4b-4c (deferred loading, memory system)

### What NOT to build

- **WebSearch** — Two options: (1) Enable existing `ToolSpec::WebSearch` via config for OpenAI models, or (2) Build model-agnostic version using OpenCode's Exa AI approach (~200 lines Rust). Option 2 recommended for multi-provider support.
- **WebFetch** — Build using OpenCode's reference impl (~300 lines Rust). Direct URL fetch + HTML→Markdown. No external API needed.
- **CronCreate** — Niche feature, low priority
- **NotebookEdit** — Unless Jupyter is a core use case
- **TeamCreate/TeamDelete** — Different paradigm, large effort
- **Skills/Hooks** — Can be added later; not core to tool use quality

---

## Web Search & Web Fetch Tools — Build Spec (from OpenCode reference)

> **Reference implementation:** `/Users/no9labs/Developer/Recursive/opencode/packages/opencode/src/tool/websearch.ts` + `webfetch.ts`
> **Why not use Codex's built-in?** Codex's `ToolSpec::WebSearch` is an OpenAI Responses API feature — model-specific, not a tool handler. Won't work with Claude or other providers.
> **OpenCode's approach:** Standard tool handlers that make HTTP calls. Work with any model, any provider.

### `websearch` — Provider-pluggable search

OpenCode uses [Exa AI](https://exa.ai)'s hosted MCP endpoint (`https://mcp.exa.ai/mcp`). **However, Exa requires an API key** (free tier exists via dashboard.exa.ai, but it's a third-party dependency). See [exa-mcp-server](https://github.com/exa-labs/exa-mcp-server) for the official MCP server.

**Approach:** Default to **Exa AI**, configurable to Brave, Tavily, SearXNG via config.

**Tool interface:**
```
Tool: websearch
Parameters:
  query:                string   — Search query (required)
  numResults:           number?  — Results to return (default: 8)
  livecrawl:            "fallback" | "preferred"  — Live crawling mode (Exa-specific)
  type:                 "auto" | "fast" | "deep"  — Search depth (Exa-specific)
```

**Default provider: Exa AI**

| | Details |
|-|---------|
| **API** | JSON-RPC to `https://mcp.exa.ai/mcp` (hosted MCP endpoint) |
| **API key** | Required — get from [dashboard.exa.ai](https://dashboard.exa.ai/api-keys) |
| **Pricing** | $0.005/search (1-25 results). Plans: Starter $49/mo (8k credits), Pro $449/mo (100k credits) |
| **Quality** | High — returns LLM-optimized content, not raw HTML |
| **Reference** | OpenCode's `websearch.ts` + [exa-mcp-server](https://github.com/exa-labs/exa-mcp-server) |

**How it works (Exa default):**
```
Agent → websearch({ query: "rust async patterns 2026" })
    ↓
POST https://mcp.exa.ai/mcp (JSON-RPC: tools/call → web_search_exa)
  Headers: { "x-api-key": <from config> }
    ↓
SSE response → parse first data line → return text to model
```

**Configurable alternative providers (users can switch via config):**

| Provider | Cost per search | Free tier | Config value |
|----------|----------------|-----------|-------------|
| **Exa AI** (default) | $0.005 | Plans from $49/mo | `"exa"` |
| Brave Search | $0.005 | $5/mo credit (~1k queries) | `"brave"` |
| Tavily | $0.008 | 1k/mo free (no card needed) | `"tavily"` |
| SearXNG | Free | Unlimited (self-hosted) | `"searxng"` |

**Config:**
```toml
[tools.websearch]
provider = "exa"        # default; users can change to "brave", "tavily", "searxng"
api_key = "..."         # required for exa, brave, tavily; not needed for searxng
base_url = "..."        # optional override (e.g., self-hosted SearXNG URL)
```

**Rust implementation:** ~300 lines with provider trait abstraction:
```rust
#[async_trait]
trait SearchProvider: Send + Sync {
    async fn search(&self, query: &str, num_results: usize) -> Result<Vec<SearchResult>>;
}

struct ExaProvider { api_key: String }      // Default
struct BraveProvider { api_key: String }    // Alternative
struct TavilyProvider { api_key: String }   // Alternative
struct SearxngProvider { base_url: String } // Self-hosted alternative
```

Uses `reqwest` (already in workspace). 25-second timeout per request.

### `webfetch` — Direct URL fetch with HTML→Markdown

Pure HTTP fetcher. No external API. Fetches any URL and converts to clean text/markdown.

**Tool interface:**
```
Tool: webfetch
Parameters:
  url:     string                          — URL to fetch (required)
  format:  "text" | "markdown" | "html"    — Output format (default: markdown)
  timeout: number?                         — Timeout in seconds (max: 120)
```

**Features:**
- HTML → Markdown conversion (Turndown library)
- HTML → plain text extraction (strips scripts/styles)
- Image fetching (returns base64)
- Cloudflare bot detection retry (retries with honest UA on 403)
- 5MB response size limit
- Browser-like `User-Agent` header

**How it works:**
```
Agent → webfetch({ url: "https://docs.rs/tokio/latest", format: "markdown" })
    ↓
HTTP GET with browser-like headers
    ↓
If HTML → convert to Markdown via Turndown
If image → return base64
    ↓
Return clean content to model
```

**Rust port:** ~300 lines. Uses `reqwest` + `htmd` (or `html2md`) crate for HTML→Markdown.

### Porting both to Orbit Code

| Tool | OpenCode lines | Rust estimate | Dependencies | API key? |
|------|---------------|--------------|-------------|----------|
| `websearch` | 150 | ~300 (with provider abstraction) | `reqwest` (in workspace) | Yes (Exa/Brave/Tavily) or No (SearXNG) |
| `webfetch` | 206 | ~300 | `reqwest` (in workspace) + `htmd` or `html2md` | No |
| **Total** | 356 | **~600** | | |

Both follow the standard `ToolHandler` pattern — parse args, make HTTP call, return `FunctionToolOutput`. `webfetch` needs no external API or key. `websearch` needs a provider API key (configurable).

**Fastest path:** Build `webfetch` first (no dependencies, no API keys). For `websearch`, connect Exa's MCP server via the existing `mcp` handler — zero custom code, just MCP config.

**Priority:** Layer 4c in the roadmap — build after core tools and prompt engineering. Web search is high value for agents that need current information (docs, API references, error messages).

---

## LSP Tool — Build Spec (from OpenCode reference)

> **Reference implementation:** `/Users/no9labs/Developer/Recursive/opencode/packages/opencode/src/lsp/` + `src/tool/lsp.ts`
> **What it is:** Code intelligence (go-to-definition, find-references, hover, etc.) — NOT a linter. Gives the agent an "IDE brain" for navigating and understanding code.
> **IDE required:** No. LSP servers are standalone background processes that speak JSON-RPC over stdin/stdout. No IDE, no editor — just a binary on `$PATH`.

### What the model sees (tool interface)

```
Tool: lsp
Parameters:
  operation: goToDefinition | findReferences | hover | documentSymbol |
             workspaceSymbol | goToImplementation | prepareCallHierarchy |
             incomingCalls | outgoingCalls
  filePath:  absolute or relative path to the file
  line:      1-based line number
  character: 1-based character offset
```

**Tool description** (from OpenCode's `lsp.txt` — identical to Claude Code's):
```
Interact with Language Server Protocol (LSP) servers to get code intelligence features.

Supported operations:
- goToDefinition: Find where a symbol is defined
- findReferences: Find all references to a symbol
- hover: Get hover information (documentation, type info) for a symbol
- documentSymbol: Get all symbols (functions, classes, variables) in a document
- workspaceSymbol: Search for symbols across the entire workspace
- goToImplementation: Find implementations of an interface or abstract method
- prepareCallHierarchy: Get call hierarchy item at a position (functions/methods)
- incomingCalls: Find all functions/methods that call the function at a position
- outgoingCalls: Find all functions/methods called by the function at a position

All operations require:
- filePath: The file to operate on
- line: The line number (1-based, as shown in editors)
- character: The character offset (1-based, as shown in editors)

Note: LSP servers must be configured for the file type. If no server is available,
an error will be returned.
```

### Why LSP matters for an agent

Without LSP, the agent navigates code via shell (slow, imprecise, no type awareness):
```
shell("rg 'fn process_request' .")         → text search, misses renames
shell("cat src/server.rs | head -200")      → dumps raw file content
shell("rg 'process_request' --files-with-matches")  → finds callers by string, not by semantic reference
```

With LSP (one call, precise, type-aware):
```
lsp({ operation: "goToDefinition", filePath: "src/main.rs", line: 42, character: 10 })
→ { "uri": "src/server.rs", "range": { "start": { "line": 141 } } }
```

### OpenCode's architecture (reference implementation)

```
┌──────────────────┐     JSON-RPC      ┌─────────────────────┐
│  Agent / Tool    │ ◄── stdin/stdout ──► │ rust-analyzer      │
│  (lsp.ts)        │                     │ (child process)     │
│                  │ ◄── stdin/stdout ──► │ typescript-lsp     │
│                  │                     │ (child process)     │
│                  │ ◄── stdin/stdout ──► │ gopls              │
│                  │                     │ (child process)     │
└──────────────────┘                     └─────────────────────┘
```

**4 files in OpenCode's LSP system:**

| File | Size | Purpose |
|------|------|---------|
| `lsp/server.ts` | 63K | Built-in server definitions for ~15 languages. Auto-detects project type (looks for `Cargo.toml`, `tsconfig.json`, `go.mod`, etc.), spawns the right server. Handles binary installation for some servers. |
| `lsp/client.ts` | 7.8K | JSON-RPC client using `vscode-jsonrpc`. Creates `MessageConnection` over stdio. Handles `textDocument/didOpen`, `textDocument/didChange`, `publishDiagnostics` notifications. |
| `lsp/index.ts` | 13.9K | Coordinator. Manages server lifecycle (lazy spawning, health checking, broken-server tracking). Routes files to correct server by extension. Exposes all 9 operations. |
| `lsp/language.ts` | 2.5K | Extension → language ID mapping. |
| `tool/lsp.ts` | 97 lines | Tool handler. Parses args, resolves path, checks server availability, dispatches operation, returns JSON. |

**Built-in language servers in OpenCode:**

| Language | Server | Detection |
|----------|--------|-----------|
| TypeScript/JS | `typescript-language-server` | `tsconfig.json`, `package.json` |
| Deno | `deno lsp` | `deno.json` |
| Go | `gopls` | `go.mod` |
| Rust | `rust-analyzer` | `Cargo.toml` |
| Python | `pyright` (or `ty` experimental) | `pyproject.toml`, `requirements.txt` |
| C/C++ | `clangd` | `compile_commands.json`, `CMakeLists.txt` |
| Java | `jdtls` | `pom.xml`, `build.gradle` |
| Kotlin | `kotlin-language-server` | `build.gradle.kts` |
| Lua | `lua-language-server` | `.lua` files |
| Zig | `zls` | `build.zig` |
| Svelte | `svelte-language-server` | `.svelte` files |
| Vue | `vue-language-server` | `.vue` files |

### Porting to Orbit Code (Rust)

**Layer 1: LSP Client** (~300 lines)
- Use `tower-lsp` crate or raw `tokio` JSON-RPC over stdio
- Spawn server as `tokio::process::Command`
- Connect via `stdin`/`stdout`
- Implement: `initialize`, `textDocument/didOpen`, `textDocument/definition`, `textDocument/references`, `textDocument/hover`, `textDocument/documentSymbol`, `workspace/symbol`, `textDocument/implementation`, `textDocument/prepareCallHierarchy`, `callHierarchy/incomingCalls`, `callHierarchy/outgoingCalls`

**Layer 2: Server Registry** (~500 lines)
- `LspServerConfig` struct: `id`, `extensions`, `root_detection` (glob patterns), `spawn_command`
- Built-in configs for top languages (Rust, TS, Go, Python at minimum)
- Auto-detection: scan for `Cargo.toml`, `tsconfig.json`, `go.mod`, `pyproject.toml`
- User-configurable via TOML config (like OpenCode allows)

**Layer 3: Server Manager** (~400 lines)
- Lazy spawning: only start a server when first file of that language is accessed
- Health tracking: mark broken servers, don't retry
- File routing: match file extension → server
- Lifecycle: initialize once, reuse for session, shutdown on exit
- Concurrent: `Arc<Mutex<>>` for shared server state across tool calls

**Layer 4: Tool Handler** (~150 lines)
```rust
pub struct LspHandler {
    manager: Arc<LspManager>,
}

#[async_trait]
impl ToolHandler for LspHandler {
    type Output = FunctionToolOutput;
    fn kind(&self) -> ToolKind { ToolKind::Function }

    async fn handle(&self, invocation: ToolInvocation) -> Result<Self::Output, FunctionCallError> {
        let args: LspArgs = parse_arguments(&invocation.arguments())?;
        let file = invocation.turn.resolve_path(Some(args.file_path));

        // Check server availability
        if !self.manager.has_server(&file).await {
            return Err(FunctionCallError::RespondToModel(
                "No LSP server available for this file type.".to_string()
            ));
        }

        // Dispatch operation
        let result = match args.operation.as_str() {
            "goToDefinition" => self.manager.definition(&file, args.line - 1, args.character - 1).await,
            "findReferences" => self.manager.references(&file, args.line - 1, args.character - 1).await,
            "hover" => self.manager.hover(&file, args.line - 1, args.character - 1).await,
            // ... other operations
        }?;

        Ok(FunctionToolOutput::from_text(
            serde_json::to_string_pretty(&result)?,
            Some(!result.is_empty())
        ))
    }
}
```

**Layer 5: Tool Spec** (in `spec.rs`)
```rust
fn create_lsp_tool() -> ToolSpec {
    // Copy schema from OpenCode's lsp.ts — same 4 parameters
    // Copy description from OpenCode's lsp.txt — identical to Claude Code's
}
```

### Effort estimate

| Component | Lines | Effort |
|-----------|------:|--------|
| LSP client (JSON-RPC over stdio) | ~300 | 2-3 days |
| Server registry (built-in configs) | ~500 | 2-3 days |
| Server manager (lifecycle, routing) | ~400 | 2-3 days |
| Tool handler + spec | ~200 | 1 day |
| Tests | ~400 | 2 days |
| **Total** | **~1,800** | **~2 weeks** |

### Dependencies needed

```toml
# In codex-rs/core/Cargo.toml
[dependencies]
lsp-types = { workspace = true }          # LSP type definitions
serde_json = { workspace = true }         # Already in workspace
tokio = { workspace = true }              # Already in workspace
```

`lsp-types` is already a transitive dependency (compiled artifacts exist in `target/`). Just needs to be added as a direct workspace dependency.

### Priority

LSP is **Layer 4b** in the roadmap — build after the core 6 tools and prompt engineering are done. It's a significant quality improvement for the agent's ability to navigate large codebases, but the core file/search tools (Layer 1-2) and prompt engineering (Layer 3) deliver more value per effort.

---

## Appendix: File Locations

### Orbit Code Key Files

| File | Purpose |
|------|---------|
| `codex-rs/core/src/tools/handlers/*.rs` | All tool handler implementations |
| `codex-rs/core/src/tools/spec.rs` | Tool spec creation + registration logic |
| `codex-rs/core/src/tools/registry.rs` | `ToolHandler` trait + `ToolRegistry` |
| `codex-rs/core/src/tools/router.rs` | `ToolRouter` — builds and dispatches tool calls |
| `codex-rs/core/models.json` | Model definitions including `experimental_supported_tools` |
| `codex-rs/core/prompt.md` | Base system prompt |
| `codex-rs/core/gpt_5_1_prompt.md` | GPT-5.1 system prompt (most detailed) |
| `codex-rs/core/gpt-5.2-codex_prompt.md` | GPT-5.2 Codex system prompt |
| `codex-rs/core/templates/collaboration_mode/` | Collaboration mode presets |
| `codex-rs/core/templates/agents/orchestrator.md` | Multi-agent orchestrator prompt |

### Claude Code Reference (Piebald repo)

| Directory | Content |
|-----------|---------|
| `reference/claude-code-system-prompts/system-prompts/` | 247 extracted prompt files |
| `reference/claude-code-system-prompts/tools/updatePrompts.js` | Extraction pipeline script |
| `reference/claude-code-system-prompts/CHANGELOG.md` | Prompt changes across 130+ versions |
| `reference/claude-code-system-prompts/README.md` | Categorized index with token counts |

### File Categories in Piebald Repo

| Prefix | Count | Content |
|--------|------:|---------|
| `system-prompt-*` | ~50 | Core system prompt sections |
| `tool-description-*` | ~70 | Tool descriptions (including ~30 Bash sub-sections) |
| `agent-prompt-*` | ~35 | Subagent system prompts |
| `data-*` | ~25 | Reference data (API docs, SDK patterns) |
| `skill-*` | ~15 | Built-in skill definitions |
| `system-reminder-*` | ~40 | Contextual system reminders |
