# tools/argument-comment-lint/ui/

## Purpose

Test fixtures for the argument-comment-lint Dylint library. These are "UI tests" in the Rust compiler testing convention -- each `.rs` file is compiled with the lint enabled, and its compiler output is compared against a corresponding `.stderr` file.

## Key Files

| File | Role |
|------|------|
| `comment_matches.rs` | Positive test: correct `/*param*/` comments that should pass without warnings |
| `comment_matches_multiline.rs` | Positive test: correct comments across multiline call sites |
| `comment_mismatch.rs` | Negative test: `/*param*/` comment that does not match the parameter name |
| `comment_mismatch.stderr` | Expected compiler output for the mismatch test |
| `uncommented_literal.rs` | Negative test: literal arguments without `/*param*/` comments |
| `uncommented_literal.stderr` | Expected compiler output for the uncommented literal test |
| `allow_string_literals.rs` | Positive test: string literals are exempt from the lint |
| `allow_char_literals.rs` | Positive test: char literals are exempt from the lint |
| `ignore_external_methods.rs` | Positive test: methods from external crates are not checked |

## How Tests Work

The `dylint_testing` framework compiles each `.rs` file with the lint crate loaded, captures compiler diagnostics, and diffs them against the `.stderr` snapshots. Tests are run via `cargo test` in the parent directory.

## Relationship to Other Directories

- Tested by the `#[test]` functions in `../src/lib.rs` (which use `dylint_testing::ui_test_example`)
