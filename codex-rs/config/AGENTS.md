# codex-rs/config/

This file applies to `codex-rs/config/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-config` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-config`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Crate: `codex-config` -- Configuration loading, merging, and validation for the Codex CLI.

### What this crate does

Manages the layered configuration system for Codex. Configuration comes from multiple sources (user config.toml, project config, managed/cloud requirements, CLI overrides) and this crate merges them into a single effective configuration. It also validates constraints, computes fingerprints for change detection, and provides error diagnostics with source positions.

### Main types and modules

- `ConfigLayerStack` / `ConfigLayerEntry` -- Ordered stack of TOML configuration layers with metadata and merge logic
- `ConfigRequirements` / `ConfigRequirementsToml` -- Parsed configuration requirements (sandbox mode, network constraints, MCP servers, exec policy, etc.)
- `CloudRequirementsLoader` -- Loads configuration requirements from cloud/remote sources
- `Constrained<T>` / `ConstraintError` -- Wrapper types for values that may be constrained by requirements
- `RequirementsExecPolicy` -- Execution policy rules from requirements (prefix patterns, allow/deny decisions)
- `LoaderOverrides` -- Test-friendly overrides for configuration inputs

### Key features

- **Layered merge**: Multiple TOML config layers are merged with later layers taking precedence via `merge_toml_values`
- **Fingerprinting**: `version_for_toml` computes SHA-based fingerprints for change detection
- **Constraint system**: Requirements can constrain config values; `Constrained<T>` tracks whether a value is user-set or requirements-enforced
- **Error diagnostics**: Rich error reporting with TOML source positions (`TextRange`, `TextPosition`)
- **CLI override builder**: `build_cli_overrides_layer` creates a config layer from command-line flags

### What it plugs into

- Used by `codex-core` to load the effective configuration at session start
- Used by `codex-app-server` to expose config layers to IDE clients
- Referenced by `codex-cli` for command-line config override handling

### Imports from / exports to

**Dependencies:**
- `codex-app-server-protocol` -- `ConfigLayer`, `ConfigLayerMetadata`, `ConfigLayerSource` types
- `codex-execpolicy` -- Execution policy types
- `codex-protocol` -- Protocol types
- `codex-utils-absolute-path` -- `AbsolutePathBuf`
- `futures`, `serde`, `serde_json`, `toml`, `toml_edit`, `tracing`, `sha2`, `thiserror`, `tokio`

**Exports:**
- All public types listed above are re-exported from `lib.rs`

### Key files

- `Cargo.toml` -- Crate manifest
- `src/lib.rs` -- Module declarations and public re-exports
- `src/state.rs` -- `ConfigLayerStack`, `ConfigLayerEntry`, `LoaderOverrides`
- `src/config_requirements.rs` -- `ConfigRequirements`, `ConfigRequirementsToml`, and all requirement types
- `src/cloud_requirements.rs` -- Cloud/remote requirements loading
- `src/constraint.rs` -- `Constrained<T>`, `ConstraintError`
- `src/merge.rs` -- TOML value merge logic
- `src/overrides.rs` -- CLI override layer builder
- `src/diagnostics.rs` -- Error types with TOML source position info
- `src/fingerprint.rs` -- SHA fingerprinting for change detection
- `src/requirements_exec_policy.rs` -- Execution policy from requirements
