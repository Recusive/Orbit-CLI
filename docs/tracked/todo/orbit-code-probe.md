# Plan: `orbit-code-probe` — Developer Diagnostic CLI

## Context

After building features in the Orbit Code workspace, there's no lightweight way to verify backend functions work correctly without launching the full TUI. Developers need a "curl for internal functions" tool that calls backend functions directly and prints results. This enables quick verification loops: build a feature → `cargo run -p orbit-code-probe -- models --filter claude` → see if it works.

## Design Decisions

| Decision | Choice | Why |
|----------|--------|-----|
| Models: bundled mode | Parse `models.json` via `include_str!` → `Vec<ModelInfo>` | Fast, offline, full `ModelInfo` fields. Default. |
| Models: runtime mode | `AuthManager` → `ModelsManager` → `list_models()` then `get_model_info()` per slug | Tests real pipeline. `list_models()` returns `ModelPreset` (loses 20 fields). `get_model_info(slug, &config)` returns full `ModelInfo`. |
| Auth store mode | Config-driven via `config.cli_auth_credentials_store_mode` | Respects file/keyring/auto/ephemeral. Also reports env var presence. |
| Config output | `ConfigLayerStack::effective_config()` → `toml::Value` | `Config` doesn't derive `Serialize`. Layer stack's merged TOML is the right representation. |
| `--env` override | `unsafe { std::env::set_var() }` in sync main, BEFORE tokio runtime | Edition 2024 requires unsafe. Must run before any threads spawn. |
| Crate layout | `[[bin]]` + `[lib]` (exec crate pattern) | Bazel `orbit_code_rust_crate` macro auto-creates `rust_library` from non-main sources. Needs `[lib]` + `src/lib.rs` declaring modules. |
| Output dispatch | `ProbeOutput` enum, NOT `Box<dyn Serialize>` | `serde::Serialize` is not object-safe (generic method). Enum dispatch. |
| Schema check | Cargo-only | Uses `env!("CARGO_MANIFEST_DIR")` for fixture path. Bazel would need a public filegroup for `config.schema.json`. |

## File Structure

```
codex-rs/tools/probe/
├── Cargo.toml
├── BUILD.bazel
└── src/
    ├── main.rs           — sync main → set_var → build tokio runtime → dispatch
    ├── lib.rs            — module declarations (for Bazel rust_library target)
    ├── cli.rs            — ProbeCli, Command enum, per-command arg structs
    ├── output.rs         — ProbeOutput enum + per-variant Serialize/Display
    ├── cmd_home_dir.rs   — probe home-dir [--env K=V]
    ├── cmd_auth.rs       — probe auth [--provider NAME]
    ├── cmd_models.rs     — probe models [--filter STRING] [--runtime]
    ├── cmd_config.rs     — probe config [--cwd PATH]
    ├── cmd_skills.rs     — probe skills
    └── cmd_schema.rs     — probe schema --check
```

## Implementation Steps

### Step 1: Workspace Registration

**Files to modify:**
- `codex-rs/Cargo.toml` — add `"tools/probe"` to `members` array, add `orbit-code-probe = { path = "tools/probe" }` to `[workspace.dependencies]`

**Files to create:**
- `codex-rs/tools/probe/Cargo.toml`
- `codex-rs/tools/probe/BUILD.bazel`

**Cargo.toml contents:**
```toml
[package]
name = "orbit-code-probe"
version.workspace = true
edition.workspace = true
license.workspace = true

[[bin]]
name = "orbit-code-probe"
path = "src/main.rs"

[lib]
name = "orbit_code_probe"
path = "src/lib.rs"

[lints]
workspace = true

[dependencies]
anyhow = { workspace = true }
clap = { workspace = true, features = ["derive"] }
orbit-code-core = { workspace = true }
orbit-code-protocol = { workspace = true }
orbit-code-utils-home-dir = { workspace = true }
serde = { workspace = true, features = ["derive"] }
serde_json = { workspace = true }
toml = { workspace = true }
tokio = { workspace = true, features = ["macros", "rt-multi-thread"] }
```

**BUILD.bazel contents:**
```python
load("//:defs.bzl", "orbit_code_rust_crate")

orbit_code_rust_crate(
    name = "probe",
    crate_name = "orbit_code_probe",
    compile_data = [
        "//codex-rs/core:model_availability_nux_fixtures",
    ],
)
```

Note: Bazel workspace root is the repo root (per `MODULE.bazel`). Some existing BUILD.bazel files use stale `//orbit-code-rs/...` references from a rename — do NOT copy that pattern. Verify `//codex-rs/core:model_availability_nux_fixtures` resolves at build time.

### Step 2: Library Entry Point (`lib.rs`)

Declares all modules. Follows the exec crate pattern where `main.rs` imports from the library:

```rust
//! Developer diagnostic tool for Orbit Code backend functions.

pub mod cli;
pub mod output;
pub mod cmd_home_dir;
pub mod cmd_auth;
pub mod cmd_models;
pub mod cmd_config;
pub mod cmd_skills;
pub mod cmd_schema;
```

### Step 3: CLI Structure (`cli.rs`)

Define clap structs:
- `ProbeCli` — root struct with `--json` global flag + `Command` subcommand
- `Command` enum: `HomeDir(HomeDirArgs)`, `Auth(AuthArgs)`, `Models(ModelsArgs)`, `Config(ConfigArgs)`, `Skills(SkillsArgs)`, `Schema(SchemaArgs)`
- Per-command arg structs:
  - `HomeDirArgs { env: Vec<String> }` — `--env KEY=VALUE`, repeatable
  - `AuthArgs { provider: Option<String> }` — `--provider openai|anthropic`
  - `ModelsArgs { filter: Option<String>, runtime: bool }` — `--filter SUBSTR`, `--runtime` flag
  - `ConfigArgs { cwd: Option<PathBuf> }` — `--cwd PATH`
  - `SkillsArgs {}` — no extra args
  - `SchemaArgs { check: bool }` — `--check` (default true)

### Step 4: Output Types (`output.rs`)

**`ProbeOutput` enum** with `#[serde(untagged)]`:

```rust
#[derive(Serialize)]
#[serde(untagged)]
enum ProbeOutput {
    HomeDir(HomeDirOutput),
    Auth(AuthOutput),
    Models(ModelsOutput),
    Config(ConfigOutput),
    Skills(SkillsOutput),
    Schema(SchemaOutput),
}
```

Per-variant structs all derive `Serialize` and implement `Display`:
- `HomeDirOutput { path: PathBuf }`
- `AuthOutput { providers: Vec<ProviderStatus>, env_keys: Vec<EnvKeyStatus> }` — `ProviderStatus { name, auth_type, source, key_preview }`, `EnvKeyStatus { var_name, is_set }`
- `ModelsOutput { models: Vec<ModelSummary>, total: usize, source: String }` — `ModelSummary` defined below
- `ConfigOutput { toml_value: toml::Value }` — human mode prints TOML, JSON mode converts
- `SkillsOutput { skills: Vec<SkillSummary>, errors: Vec<String> }` — `SkillSummary { name, scope, path, description }`
- `SchemaOutput { matches: bool, fixture_path: PathBuf }`

**`ModelSummary`** — same struct for both bundled and runtime modes. All fields sourced from `ModelInfo`:
```rust
#[derive(Serialize)]
struct ModelSummary {
    slug: String,
    display_name: String,
    context_window: Option<i64>,
    max_output_tokens: Option<i64>,
    default_reasoning_level: Option<ReasoningEffort>,
    thinking_style: ThinkingStyle,
    supported_in_api: bool,
    supports_effort: bool,
    priority: i32,
}
```

Both modes produce `ModelInfo` → `ModelSummary` — no fields lost.

### Step 5: Entry Point (`main.rs`)

**Sync main → set_var → build runtime → dispatch.** `#[tokio::main]` spawns worker threads before the function body runs, making `set_var` inside it UB.

```rust
use orbit_code_probe::cli::Command;
use orbit_code_probe::cli::ProbeCli;
// ... other imports from lib

fn main() -> anyhow::Result<()> {
    let cli = ProbeCli::parse();

    // Apply env overrides BEFORE any threads exist.
    if let Command::HomeDir(ref args) = cli.command {
        for env_pair in &args.env {
            let (key, value) = env_pair.split_once('=')
                .ok_or_else(|| anyhow::anyhow!("--env must be KEY=VALUE, got: {env_pair}"))?;
            // SAFETY: No other threads exist yet — main() is sync and we
            // haven't built the tokio runtime.
            unsafe { std::env::set_var(key, value); }
        }
    }

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async_main(cli))
}

async fn async_main(cli: ProbeCli) -> anyhow::Result<()> {
    let output = match cli.command {
        Command::HomeDir(args) => ProbeOutput::HomeDir(cmd_home_dir::run(&args)?),
        Command::Auth(args) => ProbeOutput::Auth(cmd_auth::run(&args).await?),
        Command::Models(args) => ProbeOutput::Models(cmd_models::run(&args).await?),
        Command::Config(args) => ProbeOutput::Config(cmd_config::run(&args).await?),
        Command::Skills(args) => ProbeOutput::Skills(cmd_skills::run(&args).await?),
        Command::Schema(args) => {
            let result = cmd_schema::run(&args)?;
            let failed = args.check && !result.matches;
            let output = ProbeOutput::Schema(result);
            output.print(cli.json)?;
            if failed { std::process::exit(1); }
            return Ok(());
        }
    };
    output.print(cli.json)
}
```

### Step 6: Command Implementations

**`cmd_home_dir.rs`** — sync
1. Call `orbit_code_utils_home_dir::find_orbit_home()?` (env vars already set in main)
2. Return `HomeDirOutput { path }`

**`cmd_auth.rs`** — async (loads config for store mode)
1. Load config: `ConfigBuilder::default().build().await?`
2. `let store_mode = config.cli_auth_credentials_store_mode;`
3. Load v2 auth: `load_auth_dot_json_v2(&config.orbit_code_home, store_mode)`
4. If IO error (missing `~/.orbit`) → treat as empty, print "no auth configured"
5. If `--provider`: filter HashMap by provider name (case-insensitive)
6. Per-variant redaction of `ProviderAuth` (exhaustive match, no wildcard):
   - `OpenAiApiKey { key }` / `AnthropicApiKey { key }` → `redact_key(&key)` (first 3 + `****` + last 4)
   - `Chatgpt { tokens, last_refresh }` / `ChatgptAuthTokens { .. }` → `"<chatgpt: has_tokens>"` + last_refresh
   - `AnthropicOAuth { expires_at, .. }` → `"<oauth: expires {expires_at}>"`, never show tokens
7. Check env vars: `ORBIT_API_KEY`, `OPENAI_API_KEY`, `ANTHROPIC_API_KEY` — report whether each is set (not their values)
8. Return `AuthOutput { providers, env_keys }`

**`cmd_models.rs`** — async

Both modes produce full `ModelInfo` data — no field loss.

**Default (bundled):**
1. `include_str!("../../../core/models.json")`
2. `serde_json::from_str::<ModelsResponse>(json)?` → `Vec<ModelInfo>`
3. Convert each `ModelInfo` → `ModelSummary`
4. source = `"bundled"`

**`--runtime` flag (real pipeline):**
1. Load config: `ConfigBuilder::default().build().await?`
2. `AuthManager::new(home, /*enable_env=*/false, config.cli_auth_credentials_store_mode)` — `false` matches TUI/app-server behavior. Env var auth (`ORBIT_API_KEY`) is reported separately by `probe auth`.
3. `ModelsManager::new(home, Arc::new(auth_manager), /*catalog=*/None)`
4. `let presets = models_manager.list_models(RefreshStrategy::OnlineIfUncached).await` — triggers refresh, populates internal cache
5. For each preset, call `models_manager.get_model_info(&preset.model, &config).await` → full `ModelInfo`
6. Convert each `ModelInfo` → `ModelSummary`
7. source = `"runtime (online_if_uncached)"`

Note: `list_models()` returns `Vec<ModelPreset>` (loses 20 fields including `context_window`, `thinking_style`, `priority`). But `get_model_info(slug, &config)` returns full `ModelInfo` from the now-populated internal cache. The N async calls are acceptable for a dev tool.

**Both modes then:**
- If `--filter`: case-insensitive substring match on `slug` or `display_name`
- Sort by `priority`
- Return `ModelsOutput { models, total, source }`

**Offline behavior:** If `--runtime` and user is offline with no cache, `list_models(OnlineIfUncached)` falls back to bundled models. `get_model_info()` still works from bundled data. No crash.

**`cmd_config.rs`** — async
1. `ConfigBuilder::default().fallback_cwd(args.cwd.clone()).build().await?`
2. `config.config_layer_stack.effective_config()` → `toml::Value`
3. Return `ConfigOutput { toml_value }`

**`cmd_skills.rs`** — async (config loading is async)
1. `ConfigBuilder::default().build().await?`
2. `PluginsManager::new(config.orbit_code_home.clone())`
3. `SkillsManager::new(home, Arc::new(plugins_manager), config.bundled_skills_enabled())`
   - Side effect: `install_system_skills()` — idempotent, fingerprint-cached, same as TUI startup.
4. `skills_manager.skills_for_config(&config)` → `SkillLoadOutcome`
5. Map `outcome.skills` → `Vec<SkillSummary>` (name, scope, path_to_skills_md, description)
6. Map `outcome.errors` → `Vec<String>` (disabled/failed skill loads visible)
7. Return `SkillsOutput { skills, errors }`

**`cmd_schema.rs`** — sync, Cargo-only
1. `orbit_code_core::config::schema::config_schema_json()?` — returns `Vec<u8>` (pretty JSON bytes)
2. Read fixture: `PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../core/config.schema.json")`
3. Parse generated bytes via `serde_json::from_slice(&bytes)?`
4. Parse fixture as `serde_json::Value`, compare
5. Return `SchemaOutput { matches, fixture_path }`
6. Non-zero exit handled in `main.rs` dispatch (Step 5)

### Step 7: Format, Lint, Docs

- `just fmt`
- `just fix -p orbit-code-probe`
- Update `docs/tracked/orbit-code-roadmap.md` — add probe to appropriate column
- `just bazel-lock-update && just bazel-lock-check`

## Functions & Types Reused (Verified)

| What | Crate | Path | Verified |
|------|-------|------|----------|
| `find_orbit_home()` | `orbit-code-utils-home-dir` | `utils/home-dir/src/lib.rs:17` | pub, sync |
| `load_auth_dot_json_v2()` | `orbit-code-core` | re-exported at `core/src/auth.rs:48` | pub, sync |
| `AuthCredentialsStoreMode` | `orbit-code-core` | re-exported at `core/src/auth.rs:37` | pub |
| `AuthDotJsonV2`, `ProviderAuth`, `ProviderName` | `orbit-code-core` | re-exported at `core/src/auth.rs:39-42` | pub |
| `AuthManager::new(home, enable_env, store_mode)` | `orbit-code-core` | `core/src/auth/manager.rs:107` | pub, sync |
| `ModelsManager::new(home, auth_mgr, catalog)` | `orbit-code-core` | `core/src/models_manager/manager.rs:202` | pub, sync |
| `ModelsManager::list_models(strategy)` | `orbit-code-core` | `core/src/models_manager/manager.rs:278` | pub, async, returns `Vec<ModelPreset>` |
| `ModelsManager::get_model_info(slug, config)` | `orbit-code-core` | `core/src/models_manager/manager.rs:338` | pub, async, returns full `ModelInfo` |
| `RefreshStrategy` | `orbit-code-core` | `core/src/models_manager/manager.rs` | pub |
| `ModelsResponse { models: Vec<ModelInfo> }` | `orbit-code-protocol` | `protocol/src/openai_models.rs:458` | pub |
| `ModelInfo` (35+ fields incl. context_window, thinking_style) | `orbit-code-protocol` | `protocol/src/openai_models.rs:271` | pub |
| `ModelPreset` (15 fields, UI-focused) | `orbit-code-protocol` | `protocol/src/openai_models.rs:146` | pub |
| `ConfigBuilder::default().build()` | `orbit-code-core` | `core/src/config/mod.rs:598-639` | pub, async |
| `ConfigBuilder::fallback_cwd()` | `orbit-code-core` | `core/src/config/mod.rs:634` | pub |
| `Config.config_layer_stack` | `orbit-code-core` | `core/src/config/mod.rs:240` | pub field |
| `Config.cli_auth_credentials_store_mode` | `orbit-code-core` | `core/src/config/mod.rs:383` | pub field |
| `ConfigLayerStack::effective_config()` | `orbit-code-config` | `config/src/state.rs:220` | pub, returns `toml::Value` |
| `PluginsManager::new(PathBuf)` | `orbit-code-core` | `core/src/plugins/manager.rs:468` | pub |
| `SkillsManager::new(home, plugins_mgr, bundled)` | `orbit-code-core` | `core/src/skills/manager.rs:38` | pub |
| `SkillsManager::skills_for_config(&Config)` | `orbit-code-core` | `core/src/skills/manager.rs:59` | pub, sync |
| `Config::bundled_skills_enabled()` | `orbit-code-core` | `core/src/config/mod.rs:2968` | pub |
| `config_schema_json()` | `orbit-code-core` | `core/src/config/schema.rs:83` | pub, sync, returns `Vec<u8>` |

## Verification

```bash
# Build
cargo build -p orbit-code-probe

# Smoke test each command
cargo run -p orbit-code-probe -- home-dir
cargo run -p orbit-code-probe -- home-dir --json
cargo run -p orbit-code-probe -- home-dir --env ORBIT_HOME=/tmp
cargo run -p orbit-code-probe -- auth
cargo run -p orbit-code-probe -- auth --provider anthropic
cargo run -p orbit-code-probe -- auth --json
cargo run -p orbit-code-probe -- models
cargo run -p orbit-code-probe -- models --filter claude
cargo run -p orbit-code-probe -- models --runtime
cargo run -p orbit-code-probe -- models --runtime --filter claude
cargo run -p orbit-code-probe -- models --json
cargo run -p orbit-code-probe -- config
cargo run -p orbit-code-probe -- config --cwd /tmp
cargo run -p orbit-code-probe -- config --json
cargo run -p orbit-code-probe -- skills
cargo run -p orbit-code-probe -- skills --json
cargo run -p orbit-code-probe -- schema --check

# Lint
just fmt
just fix -p orbit-code-probe

# Bazel
just bazel-lock-update && just bazel-lock-check
```

## Risks

| Risk | Mitigation |
|------|-----------|
| `include_str!` path breaks in Bazel | `compile_data` references `//codex-rs/core:model_availability_nux_fixtures`. Verify label resolves — some existing BUILD files use stale `//orbit-code-rs/...` prefix. |
| `std::env::set_var` unsafe in edition 2024 | Runs in sync `main()` before tokio runtime is built — genuinely single-threaded |
| `ConfigBuilder::build()` fails without `~/.orbit` | Handles gracefully with defaults — verified from existing usage |
| Large compile time from `orbit-code-core` dep | Acceptable for dev tool. Already cached in workspace. |
| `SkillsManager::new()` writes to `~/.orbit/skills/.system` | Idempotent, fingerprint-cached. Same thing TUI does on startup. |
| `--env FOO=bar=baz` multi-`=` parsing | Split on first `=` only via `split_once('=')` |
| Auth IO error if `~/.orbit` missing | Catch error, return empty `AuthOutput` with "no auth configured" |
| `schema --check` Cargo-only | Uses `env!("CARGO_MANIFEST_DIR")` relative path. Documented as Cargo-only. Bazel would need public filegroup for `config.schema.json`. |
| Keyring auth not visible with wrong store mode | Uses `config.cli_auth_credentials_store_mode` — respects user's configured backend |
| Runtime models mode hits network | Only with `--runtime` flag. Default bundled is offline. |
| `--runtime` offline with no cache | `list_models(OnlineIfUncached)` falls back to bundled. `get_model_info()` works from bundled data. |
| `ModelPreset` field loss in runtime | `list_models()` gets slugs only. `get_model_info()` per slug returns full `ModelInfo`. Both modes produce identical `ModelSummary`. |
