# codex-rs/utils/sleep-inhibitor/src/

Source directory for the `codex-utils-sleep-inhibitor` crate.

## Key files

- `lib.rs` -- public API:
  - `SleepInhibitor` struct with fields: `enabled`, `turn_running`, `platform` (platform-specific impl)
  - `new(enabled: bool)` -- constructor
  - `set_turn_running(bool)` -- acquires/releases sleep prevention based on enabled + turn state
  - `is_turn_running()` -- getter
  - Conditional compilation selects the platform backend via `use ... as imp`
  - Tests for toggle behavior, disabled mode, idempotent calls, and multiple toggles
- `macos.rs` -- macOS implementation:
  - `MacSleepAssertion` -- creates `IOPMAssertionCreateWithName` with `PreventUserIdleSystemSleep`; releases on `Drop`
  - Wraps IOKit FFI from `iokit_bindings.rs` via `core_foundation::string::CFString`
- `iokit_bindings.rs` -- generated IOKit FFI bindings (included by macos.rs)
- `linux_inhibitor.rs` -- Linux implementation using subprocess inhibitors
- `windows_inhibitor.rs` -- Windows implementation using `PowerCreateRequest`/`PowerSetRequest`
- `dummy.rs` -- no-op implementation for unsupported platforms
