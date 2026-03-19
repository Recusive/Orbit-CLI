# codex-rs/skills/src/assets/

Static assets embedded into the `codex-skills` binary at compile time.

## What this folder does

Contains the `samples/` directory with built-in skill packages that are embedded via `include_dir!()` and extracted to `CODEX_HOME/skills/.system` on startup.

## Key subdirectories

- `samples/` -- individual skill packages (openai-docs, skill-creator, skill-installer).
