# codex-rs/cli/src/desktop_app/

Desktop app installation and launch logic (macOS only).

## What this folder does

Handles finding, downloading, installing, and opening the Codex desktop application. On macOS, it searches standard application directories for `Codex.app`, and if not found, downloads and installs the DMG from a provided URL.

## Where it plugs in

- Called from `app_cmd.rs` in the parent `src/` directory when `codex app` is invoked
- macOS only (guarded by `#[cfg(target_os = "macos")]`)

## Key files

| File | Role |
|------|------|
| `mod.rs` | `run_app_open_or_install` -- public entry point that delegates to platform-specific implementation |
| `mac.rs` | `run_mac_app_open_or_install` -- searches `/Applications/Codex.app` and `~/Applications/Codex.app`; downloads DMG installer if not found; opens the app with `open -a` passing the workspace path |
