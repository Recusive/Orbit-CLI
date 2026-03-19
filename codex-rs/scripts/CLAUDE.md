# codex-rs/scripts/

Helper scripts for the codex-rs workspace.

## What this folder does

Contains setup and automation scripts for building and developing the codex-rs workspace on various platforms.

## Key files

- `setup-windows.ps1` -- PowerShell script that bootstraps a Windows development environment:
  - Installs Visual Studio 2022 Build Tools (MSVC + Windows SDK + ARM64 toolchains)
  - Installs Rust toolchain via `rustup` (pinned to 1.93.0 with clippy, rustfmt, rust-src)
  - Installs Git, ripgrep, just, CMake, LLVM/Clang via `winget`
  - Installs `cargo-insta` for snapshot testing
  - Configures PATH and environment variables (LIBCLANG_PATH, CC, CXX)
  - Enters VS Developer Shell for MSVC linker access
  - Runs `cargo build` to verify the setup (skippable with `-SkipBuild`)
  - Requires Administrator privileges and winget

## What it plugs into

- Run manually by developers setting up Windows build environments
- References `rust-toolchain.toml` for the toolchain version to install
- Builds the workspace defined in `codex-rs/Cargo.toml`

## Imports from / exports to

- No code imports; standalone setup scripts
- Produces a configured build environment capable of compiling the workspace
