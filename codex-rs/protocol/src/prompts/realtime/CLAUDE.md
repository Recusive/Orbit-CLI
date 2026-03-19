# codex-rs/protocol/src/prompts/realtime/

Realtime voice conversation prompt templates.

## What this folder does

Contains markdown instructions that frame the start and end of a realtime (voice) conversation session. These instruct the agent to adapt its behavior for transcript-style input from an intermediary.

## Key files

- `realtime_start.md` -- injected when a realtime conversation begins; instructs the agent that it is operating as a backend executor behind an intermediary, user text is transcript-style (possibly unpunctuated with recognition errors), and responses should be concise and action-oriented
- `realtime_end.md` -- injected when a realtime conversation ends; instructs the agent to resume normal chat behavior with typed text (no more transcript assumptions)

## What it plugs into

Injected into the conversation context by `codex-core` when realtime mode is toggled on/off during a session.

## Exports to

Static markdown content consumed during session message construction.
