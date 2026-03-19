# codex-rs/tui/src/streaming/

This file applies to `codex-rs/tui/src/streaming/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-tui` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.
- Any user-visible TUI change needs matching snapshot coverage. Mirror behavior in the sibling TUI implementation when the same feature exists there.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-tui`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo insta pending-snapshots -p codex-tui`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Streaming primitives for the TUI transcript pipeline.

### What this folder does

Manages the incremental rendering of streaming agent output. Owns newline-gated markdown collection, a FIFO queue of committed render lines, and the adaptive chunking policy that controls how fast queued lines are drained to the display. The key invariant is queue ordering -- all drains pop from the front, and arrival timestamps enable policy decisions based on queue age.

### What it plugs into

- **../chatwidget.rs**: `ChatWidget` owns `StreamController` and `PlanStreamController` instances to manage active streaming of message and plan content.
- **../app.rs**: `App` drives commit ticks via `run_commit_tick()` during the event loop.
- Uses `MarkdownStreamCollector` from `../markdown_stream.rs` for incremental markdown parsing.

### Key files

| File | Role |
|------|------|
| `mod.rs` | `StreamState` -- holds in-flight markdown stream state and a FIFO queue of committed render lines (`QueuedLine` with enqueue timestamps). Provides `new()`, `clear()`, `step()` (drain one line), `commit_pending()` (flush collector output to queue), and queue introspection methods. |
| `controller.rs` | `StreamController` -- higher-level controller that manages newline-gated streaming, header emission, and commit animation. Provides `push()` (accept a delta), `commit_tick()` (drain queued lines per the chunking policy), `finish()` (finalize stream). Also defines `PlanStreamController` for plan/proposed-action streams with distinct styling. |
| `chunking.rs` | `AdaptiveChunkingPolicy` -- two-gear adaptive drain policy. In `Smooth` mode, drains one line per tick (baseline typing effect). In `CatchUp` mode, drains all queued lines immediately when backlog pressure rises. Uses hysteresis with hold timers to prevent rapid gear-flapping. Key types: `QueueSnapshot`, `DrainPlan`, `ChunkingDecision`, `ChunkingMode`. |
| `commit_tick.rs` | `run_commit_tick()` -- orchestrates commit-tick drains across streaming controllers. Bridges chunking policy with concrete controllers. Computes queue pressure, selects a drain plan, applies it, and returns emitted `HistoryCell`s. Key types: `CommitTickScope`, `CommitTickOutput`. |

### Architecture

```
Agent delta -> StreamController.push() -> MarkdownStreamCollector
  -> commit_pending() -> QueuedLine FIFO
  -> commit_tick() -> AdaptiveChunkingPolicy.decide() -> DrainPlan
  -> step()/drain_all() -> HistoryCell emission
```

### Design notes

- Smooth mode provides a "typing" animation effect at ~120 FPS baseline tick rate.
- CatchUp mode ensures display lag converges when the agent produces output faster than the baseline drain rate.
- The policy is source-agnostic: it depends only on queue depth and queue age, not on the identity of the stream source.
