# codex-rs/feedback/src/

This file applies to `codex-rs/feedback/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-feedback` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-feedback`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source code for the `codex-feedback` crate.

### What this folder does

Contains the implementation of log ring-buffer capture, metadata tagging, and Sentry-based feedback upload.

### Key files

- `lib.rs` -- Main implementation:
  - **Constants**: `DEFAULT_MAX_BYTES` (4 MB), `SENTRY_DSN`, `UPLOAD_TIMEOUT_SECS` (10), `MAX_FEEDBACK_TAGS` (64)
  - **Types**:
    - `CodexFeedback` -- Thread-safe feedback collector wrapping `FeedbackInner` (ring buffer + tags) in `Arc`
    - `FeedbackInner` -- Holds `Mutex<RingBuffer>` and `Mutex<BTreeMap<String, String>>` for tags
    - `RingBuffer` -- Fixed-capacity byte ring buffer using `VecDeque<u8>`
    - `FeedbackMakeWriter` / `FeedbackWriter` -- `tracing_subscriber` writer that appends to the ring buffer
    - `FeedbackSnapshot` -- Immutable snapshot with bytes, tags, diagnostics, and thread_id
    - `FeedbackMetadataLayer` -- Custom `tracing_subscriber::Layer` that captures key/value tags from events
    - `FeedbackTagsVisitor` -- `tracing::field::Visit` implementation for extracting tag fields
  - **Key methods**:
    - `CodexFeedback::logger_layer()` -- Full-fidelity TRACE-level log capture layer
    - `CodexFeedback::metadata_layer()` -- Tag collection layer filtered to `"feedback_tags"` target
    - `FeedbackSnapshot::upload_feedback()` -- Builds Sentry envelope with event, tags, and attachments; flushes with timeout

- `feedback_diagnostics.rs` -- Connectivity diagnostics:
  - `FeedbackDiagnostics` -- Collects environment-based diagnostics
  - `FeedbackDiagnostic` -- Individual diagnostic with headline and detail lines
  - `collect_from_env()` -- Checks for `OPENAI_BASE_URL` and proxy environment variables (`HTTP_PROXY`, `HTTPS_PROXY`, etc.)
  - `attachment_text()` -- Formats diagnostics as a human-readable attachment string

### Imports from / exports to

**Imports:**
- `codex_protocol::{ThreadId, protocol::SessionSource}`
- `sentry::{Client, protocol::*}` -- Sentry client and protocol types
- `tracing::{Event, Level, field::Visit}` -- Tracing primitives
- `tracing_subscriber::{Layer, filter::Targets, fmt::writer::MakeWriter}`

**Exports:**
- All public types re-exported through `lib.rs`
