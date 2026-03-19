# codex-rs/protocol/src/prompts/base_instructions/

Base system prompt instructions for the Codex CLI agent.

## What this folder does

Contains the core system prompt that defines the Codex agent's personality, capabilities, behavior guidelines, and formatting rules. This is included in every session regardless of configuration.

## Key files

- `default.md` -- the full base instruction set covering:
  - Agent identity and capabilities (coding agent in Codex CLI)
  - Personality (concise, direct, friendly)
  - AGENTS.md spec (how to respect repo-level instructions)
  - Responsiveness guidelines (preamble messages before tool calls)
  - Planning guidelines (`update_plan` tool usage)
  - Task execution rules (work autonomously, use `apply_patch`, coding best practices)
  - Validation workflow (testing, formatting, iterating)
  - Ambition vs. precision guidelines
  - Progress update cadence
  - Final answer formatting (headers, bullets, monospace, file references, tone)
  - Shell command guidelines (prefer `rg`)

## What it plugs into

Included at compile time into the system prompt assembly logic in `codex-core`. Forms the foundation of every Codex session's developer message.

## Exports to

Content is embedded as a static string and combined with permission/sandbox instructions and user-specific context to form the complete system prompt.
