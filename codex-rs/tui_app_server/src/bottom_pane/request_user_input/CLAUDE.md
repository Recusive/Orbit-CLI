# codex-rs/tui_app_server/src/bottom_pane/request_user_input/

Multi-question user-input overlay for the bottom pane.

## What this folder does

Implements the `RequestUserInputOverlay` -- a state machine that presents multi-question forms to the user in response to `request_user_input` events from the agent. Each question can be answered by selecting an option, providing freeform notes, or both. The overlay handles keyboard navigation, option selection, note entry, question sequencing, and final submission.

## What it plugs into

- **../mod.rs**: The parent `BottomPane` pushes a `RequestUserInputOverlay` onto the view stack when an agent requests user input.
- **../../app.rs**: `App` creates the overlay from `ServerRequest::ToolRequestUserInput` events.
- **codex_protocol::request_user_input**: `RequestUserInputEvent` defines the question schema.

## Key files

| File | Role |
|------|------|
| `mod.rs` | `RequestUserInputOverlay` struct and state machine -- question navigation, option highlighting, notes toggle, submission logic. |
| `layout.rs` | Layout computation for the overlay (option rows, notes area, footer, scroll region). |
| `render.rs` | Rendering logic -- draws question text, option list, notes composer, footer hints, and scroll indicators. |

## Imports from

- `crate::bottom_pane` -- `ChatComposer`, `ChatComposerConfig`, `InputResult`, `CancellationEvent`, `BottomPaneView`, `ScrollState`, selection popup helpers.
- `crate::app_event` / `crate::app_event_sender` -- `AppEvent`, `AppEventSender`.
- `crate::history_cell` -- for emitting answered-question history cells.
- `codex_protocol::request_user_input` -- `RequestUserInputEvent`.
- `codex_protocol::user_input` -- `TextElement`.

## Exports to

- **crate::bottom_pane**: `RequestUserInputOverlay` (re-exported in the parent `mod.rs`).
