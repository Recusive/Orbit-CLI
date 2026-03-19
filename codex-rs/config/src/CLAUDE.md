# codex-rs/config/src/

Source code for the `codex-config` crate.

## What this folder does

Contains the implementation of Codex's layered configuration system, including loading, merging, validation, and diagnostics.

## Key files

- `lib.rs` -- Module declarations and public re-exports. Defines `CONFIG_TOML_FILE` constant ("config.toml").

- `state.rs` -- Core configuration state management:
  - `LoaderOverrides` -- Test-friendly overrides for managed config paths and macOS preferences
  - `ConfigLayerEntry` -- A single named TOML configuration layer with version fingerprint and optional raw TOML
  - `ConfigLayerStack` -- Ordered collection of layers; handles merging and conversion to `ConfigRequirements`
  - `ConfigLayerStackOrdering` -- Controls merge order of layers

- `config_requirements.rs` -- Full configuration requirements model:
  - `ConfigRequirementsToml` -- Serde-deserializable TOML structure for all config fields
  - `ConfigRequirements` / `ConfigRequirementsWithSources` -- Resolved requirements with source tracking
  - `McpServerRequirement`, `NetworkConstraints`, `SandboxModeRequirement`, `WebSearchModeRequirement`, etc.
  - `RequirementSource` -- Tracks which layer a requirement came from
  - `Sourced<T>` / `ConstrainedWithSource<T>` -- Values annotated with their origin

- `cloud_requirements.rs` -- Cloud requirements loading:
  - `CloudRequirementsLoader` -- Loads remote/managed configuration requirements
  - `CloudRequirementsLoadError` / `CloudRequirementsLoadErrorCode` -- Error types

- `constraint.rs` -- Constraint enforcement:
  - `Constrained<T>` -- Wraps a value that may be restricted by requirements
  - `ConstraintError` -- Error when a user setting violates a requirement
  - `ConstraintResult<T>` -- Result type alias

- `merge.rs` -- `merge_toml_values()` -- Deep-merges two TOML values (tables merge recursively, other values are overwritten)

- `overrides.rs` -- `build_cli_overrides_layer()` -- Builds a TOML config layer from CLI flags

- `diagnostics.rs` -- Error reporting with TOML source positions:
  - `ConfigError`, `ConfigLoadError` -- Error types
  - `TextRange`, `TextPosition` -- Source location types
  - `format_config_error()`, `format_config_error_with_source()` -- Human-readable error formatting

- `fingerprint.rs` -- `version_for_toml()` -- Computes a SHA-based fingerprint of a TOML value for change detection; `record_origins()` tracks which keys came from which layer

- `requirements_exec_policy.rs` -- Execution policy from requirements:
  - `RequirementsExecPolicy` / `RequirementsExecPolicyToml` -- Policy rules from config requirements
  - `RequirementsExecPolicyPrefixRuleToml` / `RequirementsExecPolicyPatternTokenToml` -- Pattern matching rules
  - `RequirementsExecPolicyDecisionToml` -- Allow/deny decisions

## Imports from / exports to

**Key imports:**
- `codex_app_server_protocol::{ConfigLayer, ConfigLayerMetadata, ConfigLayerSource}`
- `codex_execpolicy` -- Execution policy types
- `codex_protocol` -- Protocol types
- `codex_utils_absolute_path::AbsolutePathBuf`
- `toml::Value`, `toml_edit` -- TOML parsing and editing

**All public types are re-exported through `lib.rs`.**
