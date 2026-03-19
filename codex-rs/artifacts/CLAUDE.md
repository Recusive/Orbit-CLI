# codex-rs/artifacts/

Runtime and process-management helpers for Codex artifact generation.

## What this folder does

This crate handles locating, validating, downloading, and executing the pinned artifact runtime -- a JavaScript-based build tool (`@oai/artifact-tool`) used for artifact generation. It wraps the `codex-package-manager` crate to manage cached runtime installations under `~/.codex/packages/artifacts/`.

## Where it plugs in

- Consumed by higher-level Codex crates that need to build or render artifacts (e.g., `codex-core`)
- Uses `codex-package-manager` for download, extraction, and cache management of runtime packages
- Downloads release assets from GitHub releases (`https://github.com/openai/codex/releases/download/`)

## Imports from

- `codex-package-manager` -- `ManagedPackage`, `PackageManager`, `PackageManagerConfig`, `PackagePlatform`, `PackageReleaseArchive`
- `reqwest` -- HTTP client for downloading runtime assets
- `serde` / `serde_json` -- manifest deserialization
- `tempfile` -- staging directory for build scripts
- `tokio` -- async filesystem, process spawning, timeouts
- `url` -- URL construction for release assets
- `which` -- locating `node` and `electron` on the system PATH

## Exports to

Public API re-exported from `lib.rs`:

- `ArtifactsClient` -- executes artifact build requests against a resolved runtime
- `ArtifactBuildRequest` / `ArtifactCommandOutput` -- request/response types for builds
- `ArtifactRuntimeManager` / `ArtifactRuntimeManagerConfig` -- resolve or install a runtime
- `ArtifactRuntimeReleaseLocator` -- describes where a release can be downloaded
- `InstalledArtifactRuntime` -- a validated, on-disk runtime installation
- `JsRuntime` / `JsRuntimeKind` -- discovered JavaScript executable metadata
- `ReleaseManifest` -- release metadata for a runtime version
- `load_cached_runtime` -- loads a previously installed runtime without downloading
- `is_js_runtime_available` / `can_manage_artifact_runtime` -- capability checks

## Key files

| File | Role |
|------|------|
| `Cargo.toml` | Crate manifest; depends on `codex-package-manager`, `reqwest`, `tokio`, etc. |
| `src/lib.rs` | Module declarations and public re-exports |
| `src/client.rs` | `ArtifactsClient` -- spawns JS build commands with timeout, captures stdout/stderr |
| `src/runtime/mod.rs` | Runtime submodule re-exports |
| `src/runtime/manager.rs` | `ArtifactRuntimeManager` -- package-manager-backed installer and resolver |
| `src/runtime/installed.rs` | `InstalledArtifactRuntime` -- loads and validates extracted runtimes from disk |
| `src/runtime/js_runtime.rs` | JS runtime discovery (Node, Electron, Codex desktop app bundles) |
| `src/runtime/manifest.rs` | `ReleaseManifest` serde type for release metadata JSON |
| `src/runtime/error.rs` | `ArtifactRuntimeError` enum |
| `src/tests.rs` | Integration tests (non-Windows only) |
