# Documentation Tests (Doctests)

This guide covers rustdoc's documentation test feature, which executes your documentation examples as tests. Based on official rustdoc guidance.

## Overview

`rustdoc` supports executing documentation examples as tests to ensure they remain valid and working. This provides:

- **Automated testing** of examples
- **Confidence** that documentation is accurate
- **Feedback** when examples become outdated

### Basic Usage

Doctests are run as part of the `cargo-stage` skill.

```bash
cargo stage --strict --json
```

## How Doctests Work

### Example Detection

Rustdoc extracts code blocks from documentation comments and runs them as tests:

```rust
/// # Examples
///
/// ```
/// let x = 5;
/// ```
# fn f() {}
```

The code inside the triple backticks is extracted and executed.

### Language Detection

By default, if no language is specified for a code block, rustdoc assumes it's Rust code:

```rust
/// ```
/// // This is treated as Rust code
/// let x = 5;
/// ```
```

This is equivalent to:

```rust
/// ```rust
/// let x = 5;
/// ```
```

## Pre-processing Examples

Rustdoc processes examples before running them to make them more convenient to write:

### Pre-processing Algorithm

1. **Insert common allow attributes**: `unused_variables`, `unused_assignments`, `unused_mut`, `unused_attributes`, `dead_code`
   - Small examples often trigger these lints

2. **Add attributes from `doc(test(attr(...)))`**:
   ```rust
   #![doc(test(attr(allow(unused)))]
   ```

3. **Preserve crate attributes**: Leading `#![foo]` attributes are kept intact

4. **Auto-inject crate import**: If the example doesn't contain `extern crate` and `#![doc(test(no_crate_inject))]` is not specified, `extern crate <mycrate>;` is inserted
   - Note: Does **not** include `#[macro_use]`

5. **Wrap in main function**: If the example doesn't contain `fn main`, the code is wrapped in `fn main() { your_code }`

### Example Transformation

**Input:**
```rust
/// ```
/// let x = 5;
/// println!("{}", x);
/// ```
```

**Processed:**
```rust
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_attributes)]
#![allow(dead_code)]
extern crate my_crate;

fn main() {
    let x = 5;
    println!("{}", x);
}
```

## Hiding Portions of Examples

### The `#` Prefix

Lines starting with `# ` are hidden from the rendered output but still compiled and executed:

```rust
/// ```
/// /// Some documentation.
/// # fn foo() {} // this function will be hidden
/// println!("Hello, World!");
/// ```
# fn f() {}
```

**Rendered output:**
```rust
/// Some documentation.
# fn foo() {}
println!("Hello, World!");
```

### Use Cases for Hidden Code

1. **Setup code**: Creating types or values needed for the example
2. **Boilerplate**: Main function, imports, error handling
3. **Teardown**: Cleanup code
4. **Assertions**: Verifying behavior without cluttering the example

### Example: Multi-step Documentation

```rust
First, we set `x` to five:

```
let x = 5;
# let y = 6;
# println!("{}", x + y);
```

Next, we set `y` to six:

```
# let x = 5;
let y = 6;
# println!("{}", x + y);
```

Finally, we print the sum:

```
# let x = 5;
# let y = 6;
println!("{}", x + y);
```
```

Each code block contains the complete program, but only shows the relevant part.

### Escaping `#` Characters

To include a literal `#` at the start of a line (e.g., in a string), use `##`:

```rust
/// ```
/// let s = "foo
/// ## bar # baz";
/// ```
```

**Rendered:**
```rust
let s = "foo
# bar # baz";
```

### Macro Example

For macros that match on `#`:

```rust
/// ```
/// macro_rules! ignore { (##tag) => {}; }
/// ignore! {
///     ###tag
/// }
/// ```
# fn f() {}
```

The first `#` is used as an escape, so `###tag` becomes `##tag` which matches the macro pattern.

## Using `?` in Doc Tests

### The Problem

The `?` operator requires a function that returns `Result` or `Option`:

```rust
/// ```
/// use std::io;
/// let mut input = String::new();
/// io::stdin().read_line(&mut input)?;
/// ```
# fn f() {}
```

This fails because the generated `main` function doesn't return `Result`.

### Solution 1: Explicit Main Function

```rust
/// ```
/// use std::io;
///
/// fn main() -> io::Result<()> {
///     let mut input = String::new();
///     io::stdin().read_line(&mut input)?;
///     Ok(())
/// }
/// ```
# fn f() {}
```

### Solution 2: Hide the Wrapper

```rust
/// ```
/// use std::io;
/// # fn main() -> io::Result<()> {
/// let mut input = String::new();
/// io::stdin().read_line(&mut input)?;
/// # Ok(())
/// # }
/// ```
# fn f() {}
```

### Solution 3: Implicit Result (Rust 1.34.0+)

```rust
/// ```
/// use std::io;
/// let mut input = String::new();
/// io::stdin().read_line(&mut input)?;
/// # Ok::<(), io::Error>(())
/// ```
# fn f() {}
```

**Important**: The `Ok::<(), io::Error>(())` must be written as a single token (no whitespace between `Ok` and `<`) for rustdoc to recognize it as an implicit return. 

**Hint**: When using `?` in doctests which implicitly return `Result`, you need to add a hidden `Result::<_, E>::Ok(())` or `Ok::<(), E>(())` at the end to satisfy the return type. See also `Type::Ok(())` for a more concise form when the type is available in scope.

## Code Block Attributes

Code blocks can have attributes that control how rustdoc handles them. These follow the code fence (opening backticks) on the same line.

### Attribute Syntax

```rust
/// ```rust,ignore
/// // code here
/// ```

/// ```ignore,should_panic
/// // code here
/// ```
```

Multiple attributes can be separated by commas, spaces, or tabs. Comments in parentheses are allowed.

### Available Attributes

| Attribute | Description |
|-----------|-------------|
| `rust` | Explicitly mark as Rust code (default if no other attributes) |
| `ignore` | Don't compile or run this example |
| `should_panic` | Expect this example to panic |
| `no_run` | Compile but don't run |
| `compile_fail` | Expect compilation to fail |
| `edition2015`, `edition2018`, `edition2021`, `edition2024` | Use specific Rust edition |
| `standalone_crate` | Don't merge this doctest with others |
| `ignore-<target>` | Ignore for specific targets (e.g., `ignore-x86_64`) |
| `custom` | Don't treat as Rust code |
| `test_harness` | Run test functions instead of main |

### `ignore` Attribute

Prevents rustdoc from compiling or running the code:

```rust
/// ```ignore
/// fn foo() {
/// ```

**When to use:**
- Incomplete code snippets
- Pseudocode
- Code that requires external dependencies not available during testing

**Best practice:** Include a reason in parentheses:

```rust
/// ```ignore (needs extra dependency)
/// use dependency::functionality;
/// functionality();
/// ```
```

**Note:** This is rarely what you want. Prefer using `#` to hide code or using `no_run` instead.

### `should_panic` Attribute

The example should compile but panic during execution:

```rust
/// ```should_panic
/// assert!(false);
/// ```
# fn foo() {}
```

**With a reason:**
```rust
/// ```should_panic(expected = "assertion failed")
/// assert!(false);
/// ```
```

**Note:** The reason syntax may not be supported in all rustdoc versions.

### `no_run` Attribute

Compile the code but don't execute it:

```rust
/// ```no_run
/// loop {
///     println!("Hello, world");
/// }
/// ```
# fn foo() {}
```

**When to use:**
- Examples that require network access
- Examples that would run forever
- Examples that demonstrate undefined behavior
- Examples that have side effects (file I/O, etc.)

### `compile_fail` Attribute

The example should fail to compile:

```rust
/// ```compile_fail
/// let x = 5;
/// x += 2; // shouldn't compile!
/// ```
# fn foo() {}
```

**Important warning:** Code that fails to compile with the current Rust release may work in a future release as new features are added!

### Edition Attributes

Use a specific Rust edition for the example:

```rust
/// ```edition2018
/// let result: Result<i32, ParseIntError> = try {
///     "1".parse::<i32>()?
///         + "2".parse::<i32>()?
///         + "3".parse::<i32>()?
/// };
/// ```
# fn foo() {}
```

**Available:** `edition2015`, `edition2018`, `edition2021`, `edition2024`

### `standalone_crate` Attribute

Prevents doctest merging (Rust 1.70+ / 2024 edition):

```rust
//! ```standalone_crate
//! let location = std::panic::Location::caller();
//! assert_eq!(location.line(), 4);
//! ```
```

**Why it's needed:**

By default, compatible doctests are merged into one file before compilation for performance. This is much faster but can cause problems when doctests depend on line numbers or other context.

**Example use cases:**
- Tests that use `Location::caller()`
- Tests that depend on being in a specific file
- Tests that would interfere with each other when merged

### Ignore Targets: `ignore-<target>`

Ignore doctests for specific targets:

```rust
/// ```ignore-x86_64
/// assert!(2 == 2);
/// ```
```

**Multiple targets:**
```rust
/// ```ignore-x86_64,ignore-windows
/// assert!(2 == 2);
/// ```
```

**Backward compatibility:**
```rust
/// ```ignore,ignore-x86_64
/// assert!(2 == 2);
/// ```
```

In older rustdoc versions, this will be ignored on all targets. Starting with version 1.88.0, `ignore-x86_64` overrides `ignore`.

### `custom` Attribute

Mark the code block as non-Rust (for syntax highlighting):

```rust
/// ```custom,{class=language-c}
/// int main(void) { return 0; }
/// ```
```

**Alternative syntax:**
```rust
/// ```custom,{.language-c}
/// int main(void) { return 0; }
/// ```
```

**With quotes:**
```rust
/// ```"not rust" {."hello everyone"}
/// int main(void) { return 0; }
/// ```
```

### `test_harness` Attribute

Run test functions instead of the main function:

```rust
//! ```test_harness
//! #[test]
//! #[should_panic]
//! fn abc() { assert!(false); }
//!
//! #[test]
//! fn xyz() { assert!(true); }
//! ```
```

## Documenting Macros

Macros require special handling because they need `#[macro_use]` and a `main` function:

```rust
/// Panic with a given message unless an expression evaluates to true.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate foo;
/// # fn main() {
/// panic_unless!(1 + 1 == 2, "Math is broken.");
/// # }
/// ```
///
/// ```should_panic
/// # #[macro_use] extern crate foo;
/// # fn main() {
/// panic_unless!(true == false, "I'm broken.");
/// # }
/// ```
#[macro_export]
macro_rules! panic_unless {
    ($condition:expr, $($rest:expr),+) => ({ if ! $condition { panic!($($rest),+); } });
}
# fn main() {}
```

**Key points:**
- Add `# #[macro_use] extern crate foo;` for macro availability
- Add `# fn main() {` and `# }` for compilation
- Hide these with `#` prefix

## Testing README Files

You can include and test your README as part of your doctests:

```rust
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
```

**What this does:**
- Includes the README content as documentation on a hidden struct
- The `#[cfg(doctest)]` ensures the struct only exists during doctest collection
- Doctests in the README will be executed
- The struct doesn't appear in public documentation

## `#[cfg(doctest)]` for Doctest-Only Items

Use `#[cfg(doctest)]` to create items that only exist during doctest collection:

```rust
/// We have a struct here. Remember it doesn't accept negative numbers!
pub struct MyStruct(pub usize);

/// ```compile_fail
/// let x = my_crate::MyStruct(-5);
/// ```
#[cfg(doctest)]
pub struct MyStructOnlyTakesUsize;
```

**Use cases:**
- Testing invalid code that shouldn't compile
- Adding doctests that need private items
- Including README content (as shown above)

**Important:** Doctests only link against public items of your crate. To test private items, write regular unit tests.

## Syntax Reference

### Fenced Code Blocks

Rustdoc uses the [CommonMark specification](https://spec.commonmark.org/0.29/#fenced-code-blocks) for fenced code blocks:

```rust
/// ```
/// code here
/// ```
```

**With language:**
```rust
/// ```rust
/// code here
/// ```
```

**With attributes:**
```rust
/// ```rust,ignore
/// code here
/// ```
```

### Indented Code Blocks

Rustdoc also accepts indented code blocks (4+ spaces):

```rust
///     let foo = "foo";
///     assert_eq!(foo, "foo");
```

**Limitations:**
- Cannot use attributes like `ignore` or `should_panic`
- Less idiomatic for Rust documentation
- Prefer fenced code blocks

## Controlling Compilation and Run Directories

By default, `rustdoc --test` compiles and runs doctests from the same working directory.

### Compilation Directory
- Used for compiler diagnostics
- Affects the `file!()` macro
- Affects rustdoc test runner output

### Run Directory
- Affects file-system operations in doctests
- Affects `std::fs::read_to_string` and similar functions

### Changing Directories

Use the `--test-run-directory` flag:

```bash
rustdoc --test --test-run-directory /path/to/run/dir src/lib.rs
```

**When this is useful:**
- In workspaces, where compiler invocations should be relative to the workspace directory
- But doctest examples should run relative to the crate directory

## Best Practices

### 1. Always Test Your Examples
- Run `cargo test --doc` before committing
- Add to CI pipeline
- Ensure all examples compile

### 2. Make Examples Realistic
- Show real usage patterns
- Include proper error handling
- Demonstrate best practices

### 3. Keep Examples Minimal
- Focus on the feature being demonstrated
- Hide boilerplate with `#`
- Use `ignore` sparingly

### 4. Document Error Handling
- Show how to handle errors properly
- Use `?` with proper type annotations
- Avoid `unwrap()` when possible

### 5. Test Edge Cases
- Include examples that show edge cases
- Use `should_panic` for error conditions
- Use `compile_fail` for invalid usage

### 6. Use Appropriate Attributes
- `no_run` for examples that shouldn't execute
- `should_panic` for examples that demonstrate panics
- `compile_fail` for examples showing what not to do
- `ignore` only when necessary

### 7. Keep Doctests Fast
- Avoid slow operations in doctests
- Use `no_run` for network operations
- Consider `standalone_crate` only when necessary (it's slower)

### 8. Document All Public Items
- Every public function, struct, enum, trait, etc. should have examples
- Examples should be in the documentation, not just in tests

## Common Problems and Solutions

### Problem: Example uses `?` but doesn't compile

**Solution:**
```rust
/// ```
/// # fn main() -> Result<(), ErrorType> {
/// let result = my_function()?;
/// # Ok(())
/// # }
/// ```
```

Or:
```rust
/// ```
/// let result = my_function()?;
/// # Ok::<(), ErrorType>(())
/// ```
```

**Hint**: For doctests using `?`, add a hidden `Result::<_, E>::Ok(())`, `Ok::<(), E>(())`, or `Type::Ok(())` (when the type is in scope) at the end to provide the implicit return value.

### Problem: Example needs a trait import

**Solution:**
```rust
/// ```
/// use std::fmt::Debug;
/// # fn main() {
/// let value = 42;
/// println!("{:?}", value);
/// # }
/// ```
```

### Problem: Example needs macro

**Solution:**
```rust
/// ```
/// # #[macro_use] extern crate my_crate;
/// # fn main() {
/// my_macro!();
/// # }
/// ```
```

### Problem: Example would loop forever

**Solution:**
```rust
/// ```no_run
/// loop {
///     println!("This would run forever");
/// }
/// ```
```

### Problem: Example requires network access

**Solution:**
```rust
/// ```no_run
/// let response = reqwest::blocking::get("https://example.com");
/// ```
```

### Problem: Example demonstrates invalid code

**Solution:**
```rust
/// ```compile_fail
/// let x = 5;
/// x += 2; // Can't add to immutable variable
/// ```
```

## Performance Considerations

### Doctest Compilation Time

Doctest compilation can be slow because each example is compiled separately. Rust 1.70+ (2024 edition) merges compatible doctests for better performance.

**Example of time savings:**

- sysinfo crate: 27 seconds compile time vs 4 seconds runtime
- Rust core library: 775 seconds compile time vs 15 seconds runtime

### Improving Performance

- Use `no_run` for examples that don't need to execute
- Use `ignore` for incomplete examples
- Use `standalone_crate` sparingly (it prevents merging)
- Ensure examples are minimal but complete

## Integration with CI

Add doctests to your CI pipeline:

```yaml
# GitHub Actions example
- name: Run doctests
  run: cargo test --doc

# For verbose output
- name: Run doctests with output
  run: cargo test --doc -- --show-output

# For nightly-specific features
- name: Run doctests on nightly
  run: cargo +nightly test --doc
```

## Summary

Doctests are a powerful feature that ensures your documentation examples remain valid. Key takeaways:

1. All code blocks are treated as Rust code by default
2. Examples are pre-processed (attributes added, wrapped in main)
3. Use `#` to hide boilerplate code
4. Use attributes to control test behavior
5. Always run `cargo test --doc` to verify examples
6. Use `#[cfg(doctest)]` for doctest-only items
7. Document all public items with runnable examples
