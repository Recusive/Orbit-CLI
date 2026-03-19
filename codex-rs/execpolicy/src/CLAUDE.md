# codex-rs/execpolicy/src/

Source code for the `codex-execpolicy` crate.

## What this folder does

Implements the Starlark-based exec policy engine: parsing `.codexpolicy` files, building an in-memory policy, and evaluating commands against prefix rules and network rules.

## Key files and their roles

- `lib.rs` -- Module declarations and public re-exports.
- `policy.rs` -- `Policy` struct: stores rules indexed by program name (`MultiMap<String, RuleRef>`), network rules, and host executable paths. Core methods: `check()`, `check_with_options()`, `check_multiple()`, `matches_for_command()`, `add_prefix_rule()`, `add_network_rule()`, `merge_overlay()`, `compiled_network_domains()`, `get_allowed_prefixes()`. Also defines `Evaluation` (decision + matched rules).
- `decision.rs` -- `Decision` enum with `Allow < Prompt < Forbidden` ordering. Used throughout for expressing rule outcomes.
- `rule.rs` -- Core rule types: `Rule` trait (program, matches, as_any), `PrefixRule` (pattern + decision + justification), `PrefixPattern` (first token + rest tokens with alt support), `PatternToken` (Single or Alts), `NetworkRule` (host, protocol, decision, justification), `RuleMatch` (PrefixRuleMatch or HeuristicsRuleMatch). Includes validation helpers for match/not-match examples.
- `parser.rs` -- `PolicyParser`: uses the Starlark interpreter to evaluate `.codexpolicy` files. Registers custom Starlark functions: `prefix_rule()` (with pattern, decision, match, not_match, justification), `network_rule()` (host, protocol, decision, justification), `host_executable()` (name, paths). Builds a `Policy` via `PolicyBuilder`.
- `execpolicycheck.rs` -- `ExecPolicyCheckCommand`: CLI entry point for `codex-execpolicy check`. Loads policy files, evaluates a command, and outputs JSON with matched rules and decision.
- `amend.rs` -- `blocking_append_allow_prefix_rule()` and `blocking_append_network_rule()`: helpers that append new rules to existing `.codexpolicy` files on disk.
- `error.rs` -- `Error` enum with variants for parsing, pattern, decision, rule, and example validation errors. Includes `ErrorLocation`, `TextPosition`, `TextRange` for source location tracking.
- `executable_name.rs` -- `executable_lookup_key()` and `executable_path_lookup_key()`: extract basename from executable paths for rule matching.
- `main.rs` -- Binary entry point: parses CLI and dispatches to `ExecPolicyCheckCommand::run()`.
