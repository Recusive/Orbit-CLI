# codex-rs/process-hardening/src/

Source code for the `codex-process-hardening` crate.

## What this folder does

Contains the single-file implementation of platform-specific process security hardening.

## Key files

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

## Imports from / exports to

**Imports:**
- `libc::{prctl, ptrace, setrlimit, rlimit, RLIMIT_CORE, PR_SET_DUMPABLE, PT_DENY_ATTACH}` (platform-conditional)
- `std::ffi::OsString`, `std::os::unix::ffi::OsStrExt`

**Exports:**
- `pre_main_hardening()`
