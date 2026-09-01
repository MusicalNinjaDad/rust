# Content Guidelines

This guide covers what to include and exclude in your Rust documentation, based on official rustdoc best practices.

## Enforcing Documentation Coverage

### Lint Levels

Rustdoc provides lints to help ensure comprehensive documentation:

```rust
// In lib.rs or main.rs

// Warn about missing documentation (recommended starting point)
#![warn(missing_docs)]

// Deny missing documentation (recommended for libraries)
#![deny(missing_docs)]
```

**Using the lints:**

1. Start with `#![warn(missing_docs)]` to see what's missing
2. Run `cargo doc` and examine the warnings
3. Add documentation to address the warnings
4. Once fully documented, switch to `#![deny(missing_docs)]`

**Example output:**
```text
Documenting docdemo v0.1.0 (/Users/username/docdemo)
warning: missing documentation for the crate
 --> src/main.rs:1:1
  |
1 | / #![warn(missing_docs)]
2 | |
3 | | fn main() {
4 | |     println!("Hello, world!");
5 | | }
  | |_^
```

**Note**: The lint level is defined by the attribute itself (`warn` vs `deny`), not by separate configuration.

### Additional Lints

See the [Lints chapter](https://doc.rust-lang.org/rustdoc/lints.html) for more rustdoc lints that can help improve documentation quality.

## Examples: Best Practices

### The Power of Examples

Examples are one of the most powerful parts of documentation. They:
- Show real usage
- Demonstrate best practices
- Serve as executable tests (doctests)
- Can be copied and pasted by users

### Handling Complexity

Documentation examples often take shortcuts to remain clear and focused:

**Common shortcuts:**
- Simplifying error handling
- Omitting executor setup for async code
- Using minimal examples instead of production-ready code

**Example: Async code without executor**
```rust
/// Example
/// ```
/// let fortytwo = "42".parse::<u32>()?;
/// println!("{} + 10 = {}", fortytwo, fortytwo+10);
/// ```
```

This won't compile because there's no `main` function and no executor for the `?` operator.

### Making Examples Work

**Solution 1: Add explicit main with error handling**
```rust
/// Example
/// ```
/// fn main() -> Result<(), std::num::ParseIntError> {
/// let fortytwo = "42".parse::<u32>()?;
/// println!("{} + 10 = {}", fortytwo, fortytwo+10);
///     Ok(())
/// }
/// ```
```

**Solution 2: Hide the error handling**
```rust
/// ```
/// # fn main() -> Result<(), std::num::ParseIntError> {
/// let fortytwo = "42".parse::<u32>()?;
/// println!("{} + 10 = {}", fortytwo, fortytwo+10);
/// #     Ok(())
/// # }
/// ```
```

This approach keeps the example clean in the rendered output while ensuring it compiles.

### Error Handling Guidelines

**Avoid:**
- Using `unwrap()` in examples (can panic unexpectedly)
- Using `expect()` with generic messages

**Prefer:**
- Proper error handling with `?` operator
- Explicit error types
- Matching on error variants when educational

**When `unwrap()` is acceptable:**
- In examples where the operation cannot reasonably fail
- When demonstrating the happy path only
- When the failure case is documented elsewhere

## What to Exclude

### Hiding Implementation Details

Some parts of your public interface may appear in rustdoc output by default but shouldn't be part of your public API documentation.

**Use `#[doc(hidden)]` to hide:**

```rust
/// This is a public function but it's an implementation detail.
#[doc(hidden)]
pub fn internal_helper() {
    // implementation
}
```

**When to hide items:**
- Internal macros that make the crate easier to implement but would be misused
- Internal error types that shouldn't be used directly
- Implementation-only traits
- Re-exports that are only for convenience

**Example: Hiding a macro**
```rust
#[doc(hidden)]
macro_rules! internal_macro {
    // implementation
}
```

**Example: Hiding an error type**
```rust
#[doc(hidden)]
pub struct InternalError {
    // implementation
}
```

### What rustdoc Shows by Default

By default, rustdoc shows:
- All public items (structs, enums, functions, methods, modules, traits, etc.)
- Documentation comments
- Implementation details (unless hidden)

rustdoc does **not** show:
- Private items (non-`pub`)
- Items marked with `#[doc(hidden)]`

### API Guidelines Reference

For more on what to show/hide, see the [Rust API Guidelines on documentation](https://rust-lang.github.io/api-guidelines/documentation.html#rustdoc-does-not-show-unhelpful-implementation-details-c-hidden):

> Rustdoc does not show unhelpful implementation details.

Implementation details that are not useful for users of the crate should be hidden.

## Customizing Documentation Output

### Custom CSS

You can pass a custom CSS file to rustdoc to style the documentation:

```bash
rustdoc --extend-css custom.css src/lib.rs
```

This extends the default rustdoc CSS with your custom styles.

**Creating a custom theme:**

```bash
# Create a custom CSS file
cat > custom.css << 'EOF'
/* Your custom styles here */
:root {
    --rust-red: #b7410e;
    --rust-blue: #000000;
    --rust-background: #ffffff;
}

/* Example: Custom font */
body {
    font-family: 'Segoe UI', system-ui, sans-serif;
}
EOF

# Build documentation with custom CSS
rustdoc --extend-css custom.css src/lib.rs
```

### Theme Flag

For more extensive theming, use the `--theme` flag:

```bash
rustdoc --theme awesome.css src/lib.rs
```

This replaces the default theme entirely.

### Dark Theme

Note that rustdoc already includes a dark theme that can be enabled by clicking the gear icon in the upper right corner of the generated documentation. You typically don't need to create your own dark theme unless you want custom branding.

### Example: Ayu Theme

The Rust repository includes an [Ayu theme example](https://github.com/rust-lang/rust/blob/HEAD/src/librustdoc/html/static/css/rustdoc.css#L2384-L2574) that you can use as a starting point for your own themes.

### Customizing with Cargo

In your `Cargo.toml`, you can specify rustdoc options:

```toml
[package]
# ...

[package.metadata.docs.rs]
# Custom rustdoc flags
rustdoc-args = ["--extend-css", "custom.css"]
```

Or use build scripts to pass custom flags.

## Organizing Documentation

### Crate-Level Organization

**Recommended structure for crate-level docs:**

1. **Purpose**: What the crate does (one sentence)
2. **Use cases**: When to use the crate
3. **Getting started**: Basic setup and first example
4. **Features**: Main features and capabilities
5. **Examples**: Real-world usage examples
6. **Architecture**: High-level overview of how it works
7. **Limitations**: What the crate doesn't do
8. **Alternatives**: When to consider other crates

### Module-Level Organization

**Recommended structure for module docs:**

1. **Purpose**: What the module provides
2. **Key types**: Important types in the module
3. **Organization**: How the module is structured
4. **Examples**: Module-specific examples
5. **See also**: Related modules or crates

### Item-Level Organization

**Recommended structure for item docs:**

1. **Summary**: One-sentence description
2. **Detailed description**: How it works, when to use it
3. **Parameters**: For functions, describe each parameter (if not obvious)
4. **Return value**: For functions, describe the return value (if not obvious)
5. **Panics**: Conditions that cause panics
6. **Errors**: Error conditions and their meanings
7. **Safety**: For unsafe code, document safety requirements
8. **Examples**: At least one runnable example
9. **See also**: Links to related items

## Documentation Inheritance

### Trait Implementations

Documentation for trait implementations is automatically generated by rustdoc. Only document trait implementations when there is specific, important information to convey. When you do document a trait implementation, the example should highlight the point made in the notes section.

**General rule**: Do not document standard trait implementations (like `From`, `Add`, `Sub`, `PartialEq`, etc.) unless they have important implementation details, precision notes, or other non-obvious behavior.

**Example: When documentation IS needed (has important notes)**
```rust
impl From<Duration> for Msf {
    /// Converts a [`Duration`] to an [`Msf`].
    ///
    /// # Notes
    ///
    /// - Milliseconds are converted to frames (1000ms = 75 frames)
    /// - The conversion truncates fractional frames
    fn from(duration: Duration) -> Self {
        // implementation
    }
}
```

**Example: When documentation is NOT needed (standard, obvious behavior)**
```rust
// No doc comment needed - this is a standard, obvious conversion
impl From<Msf> for Frame {
    fn from(msf: Msf) -> Self {
        // implementation
    }
}
```

For trait implementations that require custom documentation, include a `# Notes` section explaining the important details, and ensure the example demonstrates the non-obvious behavior.

### Re-exports

When re-exporting items, you can add additional documentation that will be shown alongside the original item's documentation:

```rust
/// Re-exported for convenience.
///
/// This is the same as `std::fs::File` but available
/// at the crate root for easier access.
pub use std::fs::File;
```

**Important**: Links in the re-export's additional documentation are resolved in the scope of the re-export, not the original definition. This allows you to link to items in the new crate.

**Example:**
```rust
/// See also [foo()]
pub use std::process::Command;

pub fn foo() {}
```

Here, the link to `foo()` will resolve correctly even though `Command` is from the standard library.

### Macro Documentation

Macros require special handling for documentation:

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
#[macro_export]
macro_rules! panic_unless {
    ($condition:expr, $($rest:expr),+) => ({ if ! $condition { panic!($($rest),+); } });
}
```

**Key points for macro docs:**
- Need to add `# #[macro_use] extern crate foo;` for the macro to be available
- Need to add `# fn main() {` and `# }` for the example to compile
- Hide these with `#` prefix so they don't appear in the output

## Documentation for Different Audiences

### Beginners
- Explain concepts clearly
- Provide simple examples
- Avoid jargon or explain it
- Include "getting started" guides

### Intermediate Users
- Show common use cases
- Document edge cases
- Provide performance notes
- Include best practices

### Advanced Users
- Document internal behavior when relevant
- Show advanced patterns
- Explain design decisions
- Include architectural details

### Maintainers
- Document implementation notes with `//!` comments
- Use `#[doc(hidden)]` for internal APIs
- Document safety invariants for unsafe code
- Include module-level architecture notes

## Localization and Internationalization

Rustdoc supports Unicode in documentation. However:

- Documentation is typically written in English for the widest audience
- Code examples should use ASCII identifiers for copy-paste compatibility
- Comments in examples can be in any language if the audience expects it

## Accessibility

**For documentation content:**
- Use clear, descriptive language
- Provide alt text for images (in markdown)
- Use semantic markdown (headers, lists, etc.)
- Ensure color contrast in custom themes

**For generated HTML:**
- Rustdoc generates accessible HTML by default
- Screen readers can navigate the documentation
- Keyboard navigation is supported

## Versioning Documentation

**Documenting breaking changes:**
- Add a "Changes" or "Changelog" section to crate docs
- Document migration paths
- Note deprecated items with `#[deprecated]` attribute

**Example:**
```rust
/// This function does X.
///
/// # Deprecated
///
/// This function is deprecated. Use [`new_function`] instead.
#[deprecated(since = "1.0.0", note = "Use new_function instead")]
pub fn old_function() { ... }
```

## Documentation Testing Strategy

1. **Manual review**: Read documentation as a user would
2. **Link checking**: Verify all intra-doc links resolve correctly
3. **Doctest execution**: Run `cargo test --doc`
4. **Cross-reference checking**: Ensure related items are linked
5. **Search testing**: Verify items appear in search results
6. **Render testing**: Check HTML output in multiple browsers

## Documentation Maintenance

**When to update documentation:**
- When adding new public items
- When changing behavior of existing items
- When fixing bugs that change behavior
- When updating dependencies
- Regularly to improve clarity

**Documentation review process:**
- Include documentation in code reviews
- Require documentation for new public APIs
- Review documentation before releases
- Update examples when APIs change

**Automated checks:**
- Add `cargo test --doc` to CI
- Add `cargo doc --no-deps` to CI
- Use `#![deny(missing_docs)]` in libraries
- Consider using tools like `cargo-readme` for README validation
