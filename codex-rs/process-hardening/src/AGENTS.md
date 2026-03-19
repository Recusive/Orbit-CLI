# codex-rs/process-hardening/src/

This file applies to `codex-rs/process-hardening/src/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-process-hardening` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-process-hardening`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Source code for the `codex-process-hardening` crate.

### What this folder does

Contains the single-file implementation of platform-specific process security hardening.

### Key files

- `lib.rs` -- Complete crate implementation:
  - **Constants**: Platform-specific exit codes for hardening failures:
    - `PRCTL_FAILED_EXIT_CODE` (5) -- Linux `prctl` failure
    - `PTRACE_DENY_ATTACH_FAILED_EXIT_CODE` (6) -- macOS `ptrace` failure
    - `SET_RLIMIT_CORE_FAILED_EXIT_CODE` (7) -- Core dump limit failure
  - **Public function**: `pre_main_hardening()` -- Entry point that dispatches to platform-specific functions
  - **Platform functions** (all `pub(crate)`):
    - `pre_main_hardening_linux()` -- prctl + core dump + LD_* cleanup
    - `pre_main_hardening_macos()` -- ptrace deny + core dump + DYLD_* cleanup
    - `pre_main_hardening_bsd()` -- core dump + LD_* cleanup
    - `pre_main_hardening_windows()` -- placeholder
  - **Helpers**:
    - `set_core_file_size_limit_to_zero()` -- Sets RLIMIT_CORE to 0 via `libc::setrlimit`
    - `env_keys_with_prefix(vars, prefix)` -- Filters environment variable keys by byte prefix; handles non-UTF-8 keys
  - **Tests**: Verify correct filtering of env vars including non-UTF-8 edge cases

### Imports from / exports to

**Imports:**
- `libc::{prctl, ptrace, setrlimit, rlimit, RLIMIT_CORE, PR_SET_DUMPABLE, PT_DENY_ATTACH}` (platform-conditional)
- `std::ffi::OsString`, `std::os::unix::ffi::OsStrExt`

**Exports:**
- `pre_main_hardening()`
