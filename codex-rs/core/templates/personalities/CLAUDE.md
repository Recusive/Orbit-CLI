# codex-rs/core/templates/personalities/

Agent personality definitions that control communication style.

## What this folder does

Defines personality templates that shape how the agent communicates with the user. Personalities affect tone, verbosity, and interaction style without changing functional behavior.

## Key files

| File | Purpose |
|------|---------|
| `gpt-5.2-codex_friendly.md` | Friendly personality: warm, encouraging, explains reasoning, uses accessible language |
| `gpt-5.2-codex_pragmatic.md` | Pragmatic personality: concise, direct, focuses on results, minimal commentary |

## Where it plugs into

- Loaded via `include_str!()` in personality configuration
- Selected based on the `personality` config setting
- Injected into the system prompt via `crate::config`
- Can be changed per-turn in the TUI
