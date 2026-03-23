# Convention Enforcement Checklist

Quick-reference for each rule with the exact grep patterns or structural checks needed. Rules are numbered to match the repo root `AGENTS.md` and `docs/pattern/CODING_CONVENTIONS.md`.

---

## Table of Contents

1. [Rust Module Organization (Rules 1–6)](#rust-module-organization)
2. [Error Handling (Rules 7–11)](#error-handling)
3. [Async Patterns (Rules 12–17)](#async-patterns)
4. [Serde & Serialization (Rules 18–21)](#serde--serialization)
5. [Traits & Visibility (Rules 22–25)](#traits--visibility)
6. [Imports & Naming (Rules 26–28)](#imports--naming)
7. [API Design (Rules 29–33)](#api-design)
8. [Documentation (Rules 34–37)](#documentation)
9. [Testing (Rules 38–45)](#testing)
10. [Clippy & Lints (Rules 46–49)](#clippy--lints)
11. [TUI (Rules 50–55)](#tui)
12. [Config & Dependencies (Rules 56–60)](#config--dependencies)
13. [TypeScript (Rules 61–67)](#typescript)
14. [Build & Workflow (Rules 70–75)](#build--workflow)
15. [Critical Warnings](#critical-warnings)

---

## Rust Module Organization

### Rule 1 — Private modules with selective re-exports
**Check:** For any new `pub mod foo;` in changed files, verify the module's types are NOT also individually re-exported via `pub use foo::Type;`. If they are, the module should be `mod foo;` (private).
**Grep:** `pub mod` in lib.rs or mod.rs files
**Cross-ref:** Look for matching `pub use ...::foo::` lines in the same file

### Rule 2 — pub mod for major subsystems only
**Check:** `pub mod` is acceptable for protocol crates and major subsystems (auth, config, exec, tools). For implementation details or focused helper modules, it should be `mod`.
**Context:** Compare against existing patterns in the same lib.rs

### Rule 3 — File vs subdirectory
**Check:** A new subdirectory with mod.rs should have 3+ sub-modules. A single focused module should be a single .rs file.
**Grep:** Count files in new subdirectories

### Rule 4 — all.rs test aggregator
**Check:** New integration tests must go in `tests/suite/`, aggregated through `tests/all.rs`. Never create a new top-level `tests/foo.rs` file.
**Grep:** New files matching `tests/*.rs` that aren't `all.rs` or `common/`

### Rule 5 — Module size limits
**Check:** Files receiving large additions. Target <500 LoC, split at ~800 LoC (excluding tests).
**Method:** `wc -l` on changed files, subtract test lines

### Rule 6 — Move tests with extracted code
**Check:** When code is extracted to a new module, related tests should move too.
**Method:** Check if extracted functions had tests that stayed behind

---

## Error Handling

### Rule 7 — thiserror derives
**Check:** New error types must use `#[derive(Debug, Error)]` with `#[error(...)]` on every variant.
**Grep:** `enum.*Error` or `struct.*Error` in new code — verify derives

### Rule 8 — Per-crate Result type alias
**Check:** If a new error type is defined, look for `pub type Result<T> = std::result::Result<T, ErrorType>;`
**Grep:** New error types without accompanying Result alias

### Rule 9 — #[from] for direct wrapping
**Check:** Error variants wrapping another error should use `#[from]` unless custom logic is needed.
**Grep:** Manual `From` impls where `#[from]` would suffice

### Rule 10 — Domain-specific error methods
**Check:** Error types should have query methods (is_retryable, etc.) with exhaustive matches.
**Grep:** Wildcard `_ =>` in error type impl blocks

### Rule 11 — No unwrap/expect in library code
**CRITICAL check.**
**Grep pattern for violations:**
```
# In changed .rs files, excluding test files:
unwrap()
expect(
```
**Exclude:** Files matching `**/tests/**`, `**/*_tests.rs`, `**/tests.rs`, `#[cfg(test)]` blocks
**Note:** `unwrap_or`, `unwrap_or_default`, `unwrap_or_else` are FINE — only bare `unwrap()` and `expect()` are violations.

---

## Async Patterns

### Rule 12 — Tokio only
**Check:** No other async runtimes (async-std, smol, etc.)
**Grep:** `async-std`, `smol` in new Cargo.toml entries

### Rule 13 — Channel selection
**Check:** broadcast for fan-out, oneshot for request/response, async_channel for MPMC
**Method:** Review channel usage in new async code

### Rule 14 — Shared state
**Check:** `tokio::sync::Mutex` (not `std::sync::Mutex`) for async code. Wrapped in `Arc<>`.
**Grep:** `std::sync::Mutex` in async contexts

### Rule 15 — CancellationToken for shutdown
**Check:** New background tasks should use CancellationToken with child tokens
**Method:** Review new `tokio::spawn` calls for shutdown handling

### Rule 16 — JoinSet for parallel tasks
**Check:** Parallel task spawning should use JoinSet, not manual Vec<JoinHandle>
**Grep:** `Vec<JoinHandle` in new code

### Rule 17 — Retry with backoff
**Check:** New retry logic should use exponential backoff with ±10% jitter
**Method:** Review retry loops for proper backoff calculation

---

## Serde & Serialization

### Rule 18 — Derive sets by type category
**Check derived traits on new types:**

| Category | Required Derives |
|----------|-----------------|
| Config | `Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, JsonSchema` |
| Protocol | Above + `TS` |
| V2 app-server | Above + `TS` + `ExperimentalApi` where needed |

**Method:** Check derives on new structs/enums in the appropriate crate

### Rule 19 — rename_all by context
| Context | rename_all |
|---------|-----------|
| Config TOML | `kebab-case` |
| Protocol | `snake_case` |
| App-server v2 | `camelCase` |

**Exception:** Config RPC payloads use `snake_case` to mirror TOML keys.
**Grep:** `rename_all` in new types — verify it matches the context

### Rule 20 — No skip_serializing_if on v2 Option fields
**CRITICAL for v2 protocol types.**
**Grep:** `skip_serializing_if` in v2.rs or v2 protocol files where the field is `Option<T>`
**Correct pattern:** `#[ts(optional = nullable)]` instead

### Rule 21 — ts(export_to = "v2/") on v2 types
**Grep:** New struct/enum in v2 protocol files without `#[ts(export_to = "v2/")]`
**Also:** If `#[serde(rename)]` exists, verify matching `#[ts(rename)]`

---

## Traits & Visibility

### Rule 22 — Send + Sync bounds
**Check:** Traits used behind `Arc<dyn Trait>` must have `Send + Sync` bounds
**Grep:** `Arc<dyn` without `Send + Sync` on the trait definition

### Rule 23 — #[async_trait]
**Check:** Traits with async methods must use `#[async_trait]`
**Grep:** `async fn` in trait definitions without `#[async_trait]` above

### Rule 24 — Default trait implementations
**Check:** Optional trait methods should have default (no-op) implementations
**Method:** Review new trait definitions

### Rule 25 — Visibility levels
**Check:** `pub(crate)` for crate-internal, `pub(super)` for parent-module-only
**Grep:** Bare `pub` on types that should be `pub(crate)`

---

## Imports & Naming

### Rule 26 — One import per use statement
**Grep:** `use .*::\{` (curly brace multi-imports)
**Fix:** Run `just fmt`

### Rule 27 — Package naming
**Check:** Package names use hyphens (`orbit-code-*`), library names use underscores (`orbit_code_*`)
**Grep:** New Cargo.toml entries

### Rule 28 — App-server naming
**Check:** `*Params` for requests, `*Response` for responses, `*Notification` for notifications
**Grep:** New structs in app-server protocol that don't follow the naming pattern

---

## API Design

### Rule 29 — No bool/ambiguous Option parameters
**Grep:** Function signatures with `bool` or `Option<()>` parameters where the callsite would read `foo(true)` or `bar(None)`
**Also grep:** `Result<_, bool>` — using bool as an error type
**Fix:** Use enums, named methods, or newtypes

### Rule 30 — /*param*/ comments for opaque literals
**Check:** Callsites passing `true`, `false`, `None`, or magic numbers should have `/*param_name*/` comments
**Exception:** String/char literals are exempt unless the comment adds clarity
**Grep:** Function calls with bare `true`, `false`, `None` without preceding `/**/`

### Rule 31 — clap patterns
**Check:** CLI structs use `#[derive(Debug, Parser)]` with `#[clap(flatten)]` for shared groups
**Method:** Review new CLI argument additions

### Rule 32 — String IDs at boundaries
**Check:** Prefer `String` over UUID types at API boundaries. Timestamps as `i64` Unix seconds named `*_at`.
**Grep:** New `Uuid` types in public API signatures

### Rule 33 — Exhaustive matches, no wildcards
**CRITICAL for project-defined enums.**
**Grep:** `_ =>` in match arms
**Cross-ref:** Check if the matched type is a project enum (not std/external). Project enums should list all variants explicitly so the compiler catches additions.

---

## Documentation

### Rule 34 — Module-level docs
**Check:** Every new module must have `//!` docs at the top
**Grep:** New `.rs` files without `//!` in the first 5 lines

### Rule 35 — Public item docs
**Check:** New `pub` items must have `///` docs. Use `[`TypeName`]` syntax for cross-references.
**Exception:** Trivial getters and self-evident enum variants

### Rule 36 — Inline comments
**Check:** Comments should explain WHY, not WHAT. Flag comments that restate the code.
**Method:** Read comments in changed code for restating patterns

### Rule 37 — Update docs/ folder
**Check:** API changes should update relevant docs (especially `app-server/README.md`)
**Method:** Check if API surface changed without docs/ changes

---

## Testing

### Rule 38 — pretty_assertions::assert_eq!
**Grep:** Test modules with `assert_eq!` but without `use pretty_assertions::assert_eq;`

### Rule 39 — insta snapshot tests for UI changes
**Check:** TUI-affecting changes need insta snapshot coverage
**Method:** Check if new TUI render code has corresponding snapshot tests

### Rule 40 — wiremock response helpers
**Check:** New HTTP mocking should use `wiremock::MockServer` and `core_test_support::responses`
**Method:** Review new HTTP tests for helper usage

### Rule 41 — TestCodexBuilder fluent setup
**Check:** New integration tests should prefer `TestCodexBuilder`
**Method:** Review new integration test setup code

### Rule 42 — wait_for_event helper
**Check:** Async event assertions should prefer `wait_for_event(codex, predicate)`
**Grep:** `wait_for_event_with_timeout` in new tests

### Rule 43 — cargo_bin and find_resource
**Check:** Use `codex_utils_cargo_bin::cargo_bin()` and `find_resource!`
**Grep:** `env!(\"CARGO_MANIFEST_DIR\")` or hardcoded binary paths in new tests

### Rule 44 — ctor in tests/common/lib.rs
**Check:** Process-startup initialization should use `#[ctor]` in `tests/common/lib.rs`
**Method:** Review new test initialization patterns

### Rule 45 — Avoid boilerplate experimental marker tests
**Check:** Do not add tests that only assert experimental field markers in `common.rs`
**Method:** Review new protocol schema tests for redundant marker assertions

---

## Clippy & Lints

### Rule 46 — Denied clippy lints
**Check:** New code must not violate workspace-denied lints.
**Watch for:** `expect_used`, `unwrap_used`, `uninlined_format_args`, `redundant_closure_for_method_calls`, `collapsible_if`, `manual_*`, `needless_*`
**Method:** Review the diff and recommend `just fix -p <crate>` when appropriate

### Rule 47 — Disallowed ratatui methods
**Grep:** `Color::Rgb|Color::Indexed|\\.white\\(|\\.black\\(|\\.yellow\\(`

### Rule 48 — Error size threshold
**Check:** Large error variants should be boxed if they exceed 256 bytes
**Method:** Review new error enums with large payload fields

### Rule 49 — No print macros in TUI crates
**Grep:** `print!|println!|eprint!|eprintln!`
**Also check:** `#![deny(clippy::print_stdout, clippy::print_stderr)]` at top of TUI crate `lib.rs`

---

## TUI

### Rule 50 — Stylize helpers
**Check:** Prefer `"text".red()`, `.dim()`, `.bold()`, `.cyan()` over manual styling for static styles
**Method:** Review new TUI text rendering code

### Rule 51 — Concise span and line construction
**Check:** Prefer `"text".into()` for simple spans and `vec![...].into()` for obvious lines
**Method:** Review new TUI formatting code against file-local patterns

### Rule 52 — Approved color palette
**Check:** Headers bold, secondary dim, selection cyan, success green, errors red, branding magenta
**Grep:** Any new blue/yellow/white/black/Rgb/Indexed usage

### Rule 53 — Text wrapping utilities
**Check:** Use `adaptive_wrap_lines()`, `word_wrap_lines()`, `textwrap::wrap()`, and `prefix_lines`
**Method:** Review new wrapping code for custom ad hoc implementations

### Rule 54 — Mirror tui changes in tui_app_server
**CRITICAL check.**
**Method:** For each changed file under `tui/src/`, compare the counterpart under `tui_app_server/src/`
**Examples:**
- `tui/src/chatwidget.rs` ↔ `tui_app_server/src/chatwidget.rs`
- `tui/src/onboarding/` ↔ `tui_app_server/src/onboarding/`
- `tui/src/status/` ↔ `tui_app_server/src/status/`
**Flag:** Missing mirrored states, enums, render methods, handlers, widget fields, or tests

### Rule 55 — Preserve file-local style
**Check:** Do not refactor between equivalent forms without a readability or functional gain
**Method:** Flag churn-only conversions like `Span::styled` ↔ `set_style`, `Line::from` ↔ `.into()`

---

## Config & Dependencies

### Rule 56 — JsonSchema for config
**Check:** Config type changes should keep `JsonSchema` derives and regenerate schema output
**Method:** If `ConfigToml` changed, verify the schema artifact changed too

### Rule 57 — App-server schema regeneration
**Check:** App-server protocol shape changes should regenerate schemas and validate the crate
**Method:** If v2 protocol types changed, look for schema output updates

### Rule 58 — Workspace dependencies
**Check:** New dependencies belong in root `[workspace.dependencies]`
**Method:** Review `Cargo.toml` changes for duplicated per-crate dependency declarations

### Rule 59 — Bazel lock updates
**Check:** Dependency changes should update and check `MODULE.bazel.lock`
**Method:** Look for lockfile changes whenever deps changed

### Rule 60 — Standard dev-dependencies
**Check:** Prefer `pretty_assertions`, `tempfile`, `wiremock`, and `insta` as the standard test helpers
**Method:** Review new test crates for ad hoc alternatives

---

## TypeScript

### Rule 61 — ESM first
**Check:** `package.json` keeps `"type": "module"` and files use `import` / `export`

### Rule 62 — node: built-in imports
**Grep:** Imports of built-ins without the `node:` prefix

### Rule 63 — Strict compiler settings
**Check:** `strict: true` and `noUncheckedIndexedAccess: true` stay enabled

### Rule 64 — Type-only exports
**Check:** Use `export type` for type-only re-exports

### Rule 65 — Unused parameter naming
**Check:** Prefix intentionally unused parameters with `_`

### Rule 66 — Tooling expectations
**Check:** TS builds use tsup and tests use Jest with ts-jest where applicable

### Rule 67 — Prettier and ESLint
**Check:** Formatting and lint config should stay aligned with project defaults

---

## Build & Workflow

### Rule 70 — just fmt after Rust changes
**Check:** Recommend `just fmt` for Rust diffs and note formatting drift when present

### Rule 71 — just fix scoped by crate
**Check:** Recommend `just fix -p <crate>` for larger Rust changes instead of workspace-wide `just fix`

### Rule 72 — Test the changed crate first
**Check:** Recommend the smallest relevant test command before `just test`

### Rule 73 — Do not re-run tests after just fix or just fmt
**Check:** If you apply those commands, do not suggest redundant reruns in the same fix pass

### Rule 74 — New API surface belongs in v2
**CRITICAL check.**
**Method:** Flag any new API surface added to v1

### Rule 75 — Experimental API gating
**Check:** Experimental methods and fields use `#[experimental(...)]`, `ExperimentalApi`, and `inspect_params: true` when needed

---

## Critical Warnings

### Sandbox env vars
**CRITICAL check.**
**Flag immediately:** Any diff touching `CODEX_SANDBOX_NETWORK_DISABLED_ENV_VAR`, `CODEX_SANDBOX_ENV_VAR`, `CODEX_SANDBOX_NETWORK_DISABLED`, or `CODEX_SANDBOX`

### Process environment mutation in tests
**CRITICAL check.**
**Grep:** `std::env::set_var`, `std::env::remove_var`, or wrappers doing the same in test code

### Bazel data for embedded files or migrations
**Check:** If the diff adds `include_str!`, `include_bytes!`, or `sqlx::migrate!`, verify the crate's `BUILD.bazel` adds the needed data declarations

### Review reminders
- Restrict mechanical checks to changed files whenever possible.
- Use file-local and subtree-local `AGENTS.md` instructions in addition to the repo root rules.
- A missing TUI mirror or v1 API addition is a merge blocker even if the code compiles.
