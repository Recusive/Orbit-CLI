# codex-rs/skills/src/assets/samples/skill-installer/scripts/

This file applies to `codex-rs/skills/src/assets/samples/skill-installer/scripts/` and its descendants. Follow the repo root `AGENTS.md` first, then use the local rules below when you edit this subtree.

## Agent Guidance
- Treat this directory as golden data or generated/static support material. Keep filenames and relative paths stable unless the owning test, renderer, or generator changes first.
- Prefer updating the producer or the corresponding test scenario before editing files here by hand.
- Follow the repo-root Rust rules in `/Users/no9labs/Developer/Recursive/codex/AGENTS.md`: keep modules focused, prefer exhaustive matches, and avoid touching sandbox-env handling unless the task explicitly requires it.
- This subtree belongs to the `codex-skills` crate. Keep public re-exports, module wiring, and tests in sync with any behavior changes here.

## Validate
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && just fmt`
- `cd /Users/no9labs/Developer/Recursive/codex/codex-rs && cargo test -p codex-skills`

## Directory Map
The summary below is based on the existing directory documentation and cross-checked against the files currently present here.

Executable scripts for the skill-installer skill.

### What this folder does

Python scripts for discovering and installing skills from GitHub.

### Key files

- `list-skills.py` -- lists available skills.
- `install-skill-from-github.py` -- installs a skill from a GitHub repository.
- `github_utils.py` -- shared GitHub API utilities.
