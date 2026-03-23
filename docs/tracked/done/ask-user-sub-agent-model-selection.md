# Plan: Ask User for Sub-Agent Model/Reasoning Selection Before Spawning

## Context

When the main agent spawns sub-agents via `spawn_agent`, it autonomously picks the model and reasoning level — the user has no input. Since `request_user_input` is now ungated (available in all collaboration modes, per `ungate-request-user-input.md`) and multiple providers are supported (OpenAI + Anthropic/Claude), the agent should ask the user which model/reasoning to use before spawning.

A "Pick what's best" option preserves the current autonomous behavior as an escape hatch. If the user already specified model and/or reasoning in their message, the agent respects that directly without asking.

This is a **prompt-only change** — modifications to the `spawn_agent` tool description and a narrow carve-out in `default.md`. No new Rust code paths, no new tools, no handler/schema/wire-protocol changes. `request_user_input` remains unavailable to sub-agents.

## Key Changes

### 1. `spawn_agent` tool description — `codex-rs/core/src/tools/spec.rs` (lines 1107–1108)

Insert a new `### Choosing the model and reasoning level` section between `{available_models_description}` (line 1107) and `### When to delegate vs. do the subtask yourself` (line 1108).

**New section to insert:**

```
### Choosing the model and reasoning level
Before calling `spawn_agent`, you must determine which model and reasoning effort to use. If you do not have access to the `request_user_input` tool (i.e., you are a sub-agent), skip the asking steps below and choose autonomously. Otherwise, follow these rules in order:

1. **User fully specified:** If the user's message explicitly names both a model and reasoning level (e.g., "use gpt-5.4 with xhigh"), respect that directly. Do not ask.
2. **User partially specified:** If the user named a model but not reasoning (or vice versa), ask only for the missing field. Use a single `request_user_input` call for the unresolved field:
   - If model is known but reasoning is missing: pick the 2 most relevant reasoning efforts supported by that model as options, plus "Pick what's best (Recommended)."
   - If reasoning is known but model is missing: pick the 2 most relevant models that support that reasoning level as options, plus "Pick what's best (Recommended)."
3. **Only one viable option:** If there is exactly one available model and it supports only one reasoning effort, use it directly. Do not ask.
4. **Otherwise, ask sequentially:** When both model and reasoning are unresolved, ask in two steps:
   - **Step 1 — Model:** Call `request_user_input` with a single question:
     - **id:** `sub_agent_model`
     - **header:** `Agent model`
     - **question:** "Which model should the sub-agent use for [brief task description]?"
     - **options (2-3):** Pick the 2-3 most relevant models for the task from the available models list above. Use the exact model slug as the label (e.g., `gpt-5.4`, `claude-sonnet-4-20250514`) and describe its strengths in the description field. Put "Pick what's best (Recommended)" first with description "Let me choose the best model for this task." The client adds a freeform "Other" option automatically.
   - **Step 2 — Reasoning:** If the user picked a specific model (not "Pick what's best"), call `request_user_input` again:
     - **id:** `sub_agent_reasoning`
     - **header:** `Reasoning`
     - **question:** "Which reasoning level for [model name]?"
     - **options (2-3):** Pick the 2 most relevant reasoning efforts supported by the chosen model, plus "Pick what's best (Recommended)" first. Models may support more than 3 efforts — curate the best fits for the task, do not list all.
   - If the user picked "Pick what's best" for model, skip the reasoning question — use autonomous selection for both.
5. **Batch spawns:** If you are about to spawn multiple sub-agents in one round, ask once before the first spawn. Apply the user's chosen model and reasoning level to all agents in that batch.
6. **Compatibility:** If the user's explicit choice is invalid or incompatible with available presets (e.g., a reasoning level not supported by the chosen model), ask for clarification via `request_user_input` instead of guessing or passing invalid arguments.
7. **Slug mapping:** When calling `spawn_agent`, always use the exact model slug (shown in backticks in the models list above, e.g., `gpt-5.4`) for the `model` parameter and the exact effort name (e.g., `high`, `xhigh`) for the `reasoning_effort` parameter. If the user's answer from `request_user_input` uses a display name or freeform text, map it back to the correct slug/effort value from the available models list.

After receiving the user's answer, call `spawn_agent` with the chosen `model` and `reasoning_effort` parameters (or omit them if the user picked "Pick what's best").
```

### 2. Default mode reinforcement — `codex-rs/core/templates/collaboration_mode/default.md`

Default mode already instructs the agent to use `request_user_input` when it needs to ask questions. Add a brief reinforcement that model/reasoning selection for sub-agents is one of those cases where asking is required.

**Append after the existing paragraph (line 7):**
```
When spawning sub-agents and the model/reasoning choice is not already fixed by user input or a single viable preset, use `request_user_input` to ask the user for model and reasoning level selection before calling `spawn_agent`. See the `spawn_agent` tool description for the detailed flow.
```

## Design Decisions

| Decision | Choice | Why |
|----------|--------|-----|
| Placement | `spec.rs` + `default.md` reinforcement | Tool description covers the mechanics; `default.md` reinforces that this is a required-ask scenario |
| Question flow | Sequential (model → reasoning) | Avoids combinatorial explosion; reasoning options depend on model choice |
| Partial spec handling | Ask only for missing field | Respects what the user already told you |
| Option count | 2-3 per question (curated) | Stays within `request_user_input` 2-3 option contract; models with 4+ reasoning levels get curated to the 2 most relevant |
| "Pick what's best" | First option with "(Recommended)" | Preserves current autonomous behavior; short-circuits the reasoning question |
| Batch handling | Ask once per batch | Avoids repeated popups when spawning multiple agents |
| Invalid combos | Ask for clarification | Never guess or pass invalid arguments |
| Sub-agent scoping | Escape clause in prompt preamble | Sub-agents get `spawn_agent` but not `request_user_input`; preamble tells them to skip asking and choose autonomously |
| Slug mapping | Explicit instruction to use exact slugs | `find_spawn_agent_model_name()` matches against slugs, not display names; `request_user_input` returns human-friendly labels that need mapping |

## What Stays Unchanged

- `request_user_input` handler — no code changes
- `spawn_agent` handler — no code changes
- `spawn_agents_on_csv` — no model/reasoning overrides, not part of this change
- SubAgent exclusion for `request_user_input` — stays in place (only main agent asks)
- All other sections of the `spawn_agent` description — untouched
- Plan mode template — no changes needed (Plan mode already encourages asking questions via `request_user_input`)

## Test Plan

### Automated
- Update existing e2e test in `codex-rs/core/tests/suite/spawn_agent_description.rs` to assert the `spawn_agent` description contains the new "Choosing the model and reasoning level" section, including the sub-agent escape clause and slug mapping rule
- Update existing app-server test in `codex-rs/app-server/tests/suite/v2/turn_start.rs` to assert Default-mode collaboration instructions contain the spawn-specific reinforcement text
- Add a spec test in `codex-rs/core/src/tools/spec_tests.rs` asserting the `spawn_agent` description contains:
  - The ask-before-pick rule text
  - The explicit-user-choice exception
  - The "Pick what's best" escape hatch
  - The batch spawn instruction
- Add a formatter test for `spawn_agent_models_description()` covering mixed GPT/Claude presets with visible vs hidden models

### Manual TUI Verification
```bash
# 1. Build and check for snapshot failures
cargo test -p orbit-code-core

# 2. Accept any snapshot updates if needed
cargo insta accept -p orbit-code-core

# 3. Format
just fmt

# 4. Clippy
just fix -p orbit-code-core

# 5. Manual TUI testing — run `just codex` and test these scenarios:
#    a. "spawn a sub-agent to fix this bug" → agent asks for model, then reasoning
#    b. "spawn a sub-agent using gpt-5.4 xhigh" → agent skips asking, spawns directly
#    c. "spawn a sub-agent with high reasoning" → agent asks for model only (reasoning already given)
#    d. "spawn 3 sub-agents for these tasks" → agent asks once, applies to all
#    e. Only one model configured → agent skips asking
#    f. User picks "Pick what's best" for model → reasoning question is skipped
```

## Assumptions

- `spawn_agents_on_csv` does not expose per-worker model/reasoning overrides and is excluded from this change.
- The fix is prompt-level only; no runtime enforcement beyond existing validation in the spawn handler.
- `request_user_input`'s "Other" freeform (always enabled via `is_other: true` in the handler) covers any model/reasoning the user wants that isn't in the top picks.
- Only Plan and Default are registered as built-in / TUI-visible collaboration presets today (`TUI_VISIBLE_COLLABORATION_MODES` in `protocol/src/config_types.rs:336`). The `ModeKind` enum has hidden `Execute` and `PairProgramming` variants but they are not user-facing. Default mode is where sub-agents are spawned, and it already supports `request_user_input`. Plan mode already encourages asking questions, so no reinforcement is needed there.
