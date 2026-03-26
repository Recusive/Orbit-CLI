# Plan: Fix 50 Pre-Existing Test Failures

## Context

A full test run (`cargo nextest run --no-fail-fast -P slow -j 4`) completed with 7168 passed, **50 failed**, 1 timed out, 30 skipped. These failures are all pre-existing — not caused by the recent `.codex` → `.orbit` migration. They fall into 9 root-cause groups spanning binary resolution, schema violations, rename artifacts, scope classification bugs, and environment-dependent flakiness.

**Goal:** Fix every failure that has a deterministic code-level root cause. Document environment-dependent failures separately.

---

## Tier 1 — Mechanical Fixes (33 tests)

### Step 1: Binary Name Resolution (27 tests)

**Root cause:** Binary names in `Cargo.toml` were renamed from `codex-*` to `orbit-code-*`, but test files still reference the old names via `cargo_bin()` and `env!("CARGO_BIN_EXE_*")`.

**Files to modify (8 locations):**

| File | Line | Old | New |
|------|------|-----|-----|
| `codex-rs/stdio-to-uds/tests/stdio_to_uds.rs` | ~69 | `"codex-stdio-to-uds"` | `"orbit-code-stdio-to-uds"` |
| `codex-rs/exec/tests/suite/apply_patch.rs` | ~26 | `"codex-exec"` | `"orbit-code-exec"` |
| `codex-rs/exec/tests/suite/sandbox.rs` | ~46 | `"codex-exec"` | `"orbit-code-linux-sandbox"` |
| `codex-rs/exec-server/tests/common/exec_server.rs` | ~41 | `"codex-exec-server"` | `"orbit-code-exec-server"` |
| `codex-rs/core/tests/common/lib.rs` | ~158, ~487, ~503 | `"codex-linux-sandbox"` | `"orbit-code-linux-sandbox"` |
| `codex-rs/core/tests/common/test_orbit_code_exec.rs` | ~15 | `"codex-exec"` | `"orbit-code-exec"` |
| `codex-rs/linux-sandbox/tests/suite/managed_proxy.rs` | * | `env!("CARGO_BIN_EXE_codex-linux-sandbox")` | `env!("CARGO_BIN_EXE_orbit_code_linux_sandbox")` |
| `codex-rs/linux-sandbox/tests/suite/landlock.rs` | * | `env!("CARGO_BIN_EXE_codex-linux-sandbox")` | `env!("CARGO_BIN_EXE_orbit_code_linux_sandbox")` |

Also check `codex-rs/core/tests/suite/live_cli.rs:33` — references `"codex-rs"` (should be `"orbit-code"`). This test is `#[ignore]` but fix it for correctness.

**Verify:** `cargo test -p orbit-code-exec` + `cargo test -p orbit-code-exec-server` + `cargo test -p orbit-code-stdio-to-uds`

### Step 2: App-Server Protocol Schema (3 tests)

**Root cause:** `AccountUpdatedNotification` in `v2.rs:3583-3588` has `#[ts(optional = nullable)]` on the `provider` field. Convention says this annotation is only for `*Params` types.

**Fix:**
1. In `codex-rs/app-server-protocol/src/protocol/v2.rs:3586` — remove `#[ts(optional = nullable)]` from `provider` field
2. Run `just write-app-server-schema` to regenerate fixtures
3. Run `cargo test -p orbit-code-app-server-protocol` to verify all 3 tests pass

### Step 3: Rate Limit Test Expectations (3 tests)

**Root cause:** During the rename, test limit_ids were changed from `codex_*` to `orbit_code_*`, but the test HTTP headers still use `x-codex-*` prefix. The `normalize_limit_id()` function converts `_` to `-`, so `orbit_code_secondary` becomes prefix `x-orbit-code-secondary`, which doesn't match `x-codex-secondary-*` headers.

**Files to modify:** `codex-rs/codex-api/src/rate_limits.rs` (test section, ~lines 289-350)

| Test | Fix |
|------|-----|
| `parse_rate_limit_for_limit_reads_secondary_headers` | Change `Some("orbit_code_secondary")` → `Some("codex_secondary")` in both the function call (line ~306) and assertion (line ~307) |
| `parse_rate_limit_for_limit_prefers_limit_name_header` | Change `Some("orbit_code_bengalfox")` → `Some("codex_bengalfox")` in both the function call (line ~329) and assertion (line ~330) |
| `parse_all_rate_limits_reads_all_limit_families` | Change assertion `Some("orbit_code_secondary")` → `Some("codex_secondary")` (line ~349) |

**Verify:** `cargo test -p orbit-code-api`

---

## Tier 2 — Bug Fixes (up to 8 tests)

### Step 4: Skills Scope Classification (1 test)

**Root cause:** In `codex-rs/core/src/skills/loader.rs`, `discover_skills_under_root()` walks the User root `$ORBIT_HOME/skills/` recursively. This traverses into `$ORBIT_HOME/skills/.system/`, discovering embedded system skills with `SkillScope::User`. Later, the System root walk (`$ORBIT_HOME/skills/.system/`) finds the same skills with `SkillScope::System`. Since User root is processed first, `find()` returns the User-scoped entry.

**Fix:** In `discover_skills_under_root()`, when `scope != SkillScope::System`, skip subdirectories named `.system` during the directory walk (around line 480-496 where directories are enqueued). This prevents the User walk from entering the system skills cache.

**File:** `codex-rs/core/src/skills/loader.rs` (~line 480-496, add check before enqueueing `.system` directory)

**Verify:** `cargo test -p orbit-code-core -- suite::skills`

### Step 5: Session File Test (1 test)

**Root cause:** `integration_creates_and_checks_session_file` spawns `orbit-code exec` as a subprocess. If binary resolution is broken (Group 1), this test also fails. **Fix Group 1 first, then re-run this test.** If it still fails, investigate the session file directory structure logic.

**File:** `codex-rs/core/tests/suite/cli_stream.rs:334-518`
**Verify:** `cargo test -p orbit-code-core -- suite::cli_stream`

### Step 6: Login E2E JWT (1 test)

**Root cause:** The test's mock issuer handles all POST `/oauth/token` requests identically. The login server makes TWO requests: (1) code exchange, (2) `obtain_api_key()` for API key exchange. If `obtain_api_key()` fails (returns `Err`, `.ok()` = `None`), then `openai_api_key` is `None` in the persisted `auth.json`, and `json["OPENAI_API_KEY"]` is `Null`.

**Investigation needed:** Read `obtain_api_key` error path. Check if `build_reqwest_client_with_custom_ca()` fails connecting to plain HTTP mock servers. The function is at `codex-rs/login/src/server.rs:1060-1092`.

**File:** `codex-rs/login/tests/suite/login_server_e2e.rs:82-157`
**Verify:** `cargo test -p orbit-code-login -- suite::login_server_e2e`

### Step 7: Cloud Requirements Tests (3 tests)

**Root cause:** These test the error path when cloud requirements endpoint returns 401 + refresh token revocation. Need to verify mock server setup, token refresh error handling, and expected error payloads.

**Files:**
- `codex-rs/app-server/tests/suite/v2/thread_start.rs:336-415`
- `codex-rs/app-server/tests/suite/v2/thread_resume.rs:1423-1512`
- `codex-rs/app-server/tests/suite/v2/thread_fork.rs:226-319`

**Verify:** `cargo test -p orbit-code-app-server -- suite::v2::thread_start::thread_start_surfaces_cloud`

### Step 8: App-Server Misc (2 tests)

**Root cause TBD.** Need to read test code, run individually, and check error output.

**Files:**
- `codex-rs/app-server/tests/suite/v2/app_list.rs:698-799`
- `codex-rs/app-server/tests/suite/v2/command_exec.rs:708-788`

**Verify:** `cargo test -p orbit-code-app-server -- suite::v2::app_list` and `suite::v2::command_exec`

### Step 9: Models Cache TTL (1 test)

**Root cause TBD.** The test validates ETag-triggered cache refresh. Need to investigate why the cache TTL isn't renewed.

**File:** `codex-rs/core/tests/suite/models_cache_ttl.rs:44-136`
**Verify:** `cargo test -p orbit-code-core -- suite::models_cache_ttl`

### Step 10: Apply Patch Scenarios (1 test)

**Root cause TBD.** Snapshot mismatch in one or more scenario directories. Need to run the test, identify which scenario fails, and check the expected vs actual output.

**File:** `codex-rs/apply-patch/tests/suite/scenarios.rs:11-62`
**Verify:** `cargo test -p orbit-code-apply-patch`

---

## Tier 3 — Environment-Dependent / Flaky (4-6 tests)

These tests have root causes tied to system load, macOS sandbox overhead, or tight timeouts. Document but may not be fixable in code alone.

### OTEL HTTP Loopback (4 tests)

**Root cause:** Custom TCP polling with 1-second channel timeout is too tight under system load. Tests bind to localhost, emit traces/metrics, then assert the mock server received them within tight deadlines.

**File:** `codex-rs/otel/tests/suite/otlp_http_loopback.rs`
**Options:** Increase timeouts (3s→5s for channel recv), or mark as `#[ignore]` with a comment explaining flakiness.

### Unified Exec Seatbelt (1 test, 86s runtime)

**Root cause:** Python startup under macOS Seatbelt sandbox is slow. The 1.5s yield_time may not be enough.

**File:** `codex-rs/core/tests/suite/unified_exec.rs:2665+`
**Option:** Increase yield_time, or skip on macOS CI.

### Approval Matrix Timeout (1 test, TIMEOUT at 600s)

**Root cause:** The matrix test iterates 20+ scenarios, each building a full test harness with MockServer. Total runtime exceeds the 600s nextest timeout.

**File:** `codex-rs/core/tests/suite/approvals.rs:1627+`
**Options:** Increase timeout in nextest config, or split the matrix into smaller test functions.

---

## Execution Order

1. **Step 1** (binary names) — highest impact, 27 tests
2. **Step 2** (protocol schema) — 3 tests
3. **Step 3** (rate limits) — 3 tests
4. **Step 4** (skills scope) — 1 test
5. **Steps 5-10** (investigate and fix remaining) — up to 8 tests
6. **Tier 3** (document/triage environment failures) — 4-6 tests

After each step, run the affected crate's tests to verify. After all steps, run the full suite: `cargo nextest run --no-fail-fast -P slow -j 4`

---

## Verification

1. After Tier 1 fixes: `cargo test -p orbit-code-exec && cargo test -p orbit-code-exec-server && cargo test -p orbit-code-stdio-to-uds && cargo test -p orbit-code-app-server-protocol && cargo test -p orbit-code-api`
2. After Tier 2 fixes: `cargo test -p orbit-code-core && cargo test -p orbit-code-login && cargo test -p orbit-code-app-server && cargo test -p orbit-code-apply-patch`
3. Full suite: `cargo nextest run --no-fail-fast -P slow -j 4`
4. TUI verification (if any visible changes): `scripts/e2e-capture.sh`
