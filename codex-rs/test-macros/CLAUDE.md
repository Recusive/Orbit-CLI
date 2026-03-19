# codex-rs/test-macros/

Crate: `codex-test-macros` -- Procedural macros for test infrastructure.

## What this crate does

Provides a proc-macro attribute `#[large_stack_test]` that runs test bodies on a dedicated thread with a 16 MB stack. This is necessary for tests that exercise deeply recursive code paths (e.g., complex parsing, deep AST traversal) that would overflow the default test thread stack.

## Main macro

- `#[large_stack_test]` -- Attribute macro that transforms a test function:
  - **Sync tests**: Wraps the body in `std::thread::Builder::new().stack_size(16MB).spawn()`
  - **Async tests**: Creates a Tokio multi-thread runtime with 2 worker threads, then runs the async body inside the large-stack thread via `runtime.block_on()`
  - Automatically adds `#[test]` attribute if not present
  - Strips `#[tokio::test]` attributes (replaced by the manual runtime construction)
  - Preserves `#[test_case]` and other attributes

## Key behaviors

- Stack size: 16 MB (`LARGE_STACK_TEST_STACK_SIZE_BYTES`)
- Thread is named after the test function for debugging
- Panics from the test thread are properly propagated via `std::panic::resume_unwind`

## What it plugs into

- Used by test functions throughout the workspace that need larger stack sizes
- Applied as `#[codex_test_macros::large_stack_test]` or imported and used as `#[large_stack_test]`

## Imports from / exports to

**Dependencies:**
- `proc-macro2` -- Token stream manipulation
- `quote` -- Code generation
- `syn` -- Rust syntax parsing (full feature)

**Exports:**
- `large_stack_test` -- The procedural macro attribute

## Key files

- `Cargo.toml` -- Crate manifest (`proc-macro = true`)
- `src/lib.rs` -- Complete implementation with macro definition, expansion logic, and attribute filtering
