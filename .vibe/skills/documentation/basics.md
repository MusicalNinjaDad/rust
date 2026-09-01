# Documentation Basics

This guide covers the fundamentals of writing good Rust documentation based on official rustdoc guidance.

## Principles of Good Documentation

Good documentation balances two opposing goals:
1. **Expertise**: You need deep understanding of the subject
2. **Novice perspective**: You need to write for people learning it

Documentation should:
- Be **clear** - easy to understand
- Be **complete** - cover all important aspects
- Follow the rule: **more documentation is better**
- Document **every public item** - if it's public, it should be documented

## Getting Started with Crate Documentation

### Crate-Level Documentation

The first lines in `lib.rs` (or `main.rs` for binaries) use `//!` for crate-level documentation. These lines compose the front-page of your documentation.

```rust
//! Fast and easy queue abstraction.
//!
//! Provides an abstraction over a queue. When the abstraction is used
//! there are these advantages:
//! - Fast
//! - [Easy]
//!
//! [Easy]: http://thatwaseasy.example.com
```

**Best practices for crate-level docs:**

1. **First line**: A sentence without highly technical details, but with a good description of where this crate fits within the Rust ecosystem. Users should know whether this crate meets their use case after reading this line.

2. **Include an example**: Show a real-world usage example that users can copy and paste to get started. Stick to the library's role without taking shortcuts.

3. **Progressive disclosure**: Start incrementally with:
   - Introduction
   - Example
   - Features
   Then evolve to comprehensive reference documentation.

4. **Learn from great examples**:
   - [`hashbrown`](https://docs.rs/hashbrown/): Summarizes role, provides technical links, explains why to use it
   - [`futures`](https://docs.rs/futures/): Uses inline comments to explain complexities
   - [`backtrace`](https://docs.rs/backtrace/): Walks through setup, Cargo.toml changes, quick example
   - [`regex`](https://docs.rs/regex/): Comprehensive reference with requirements, edge cases, examples

### Module-Level Documentation

Use `//!` for module-level documentation. This appears at the top of the module's documentation page.

```rust
//! This module provides functionality for handling HTTP requests.
//!
//! It includes:
//! - Request building and sending
//! - Response handling
//! - Error management
```

## Documenting Components

All public API items should follow this basic structure:

```text
[short sentence explaining what it is]

[more detailed explanation]

[at least one code example that users can copy/paste to try it]

[even more advanced explanations if necessary]
```

**Why examples are crucial:**
- Help users understand what the item is
- Show how it's used
- Reveal its purpose
- Serve as executable documentation (doctests)

### Example: Function Documentation

From the standard library's `std::env::args()`:

```rust
/// Returns the arguments which this program was started with (normally passed
/// via the command line).
///
/// The first element is traditionally the path of the executable, but it can be
/// set to arbitrary text, and may not even exist. This means this property should
/// not be relied upon for security purposes.
///
/// On Unix systems shell usually expands unquoted arguments with glob patterns
/// (such as `*` and `?`). On Windows this is not done, and such arguments are
/// passed as-is.
///
/// # Panics
///
/// The returned iterator will panic during iteration if any argument to the
/// process is not valid unicode. If this is not desired,
/// use the [`args_os`] function instead.
///
/// # Examples
///
/// ```
/// use std::env;
///
/// // Prints each argument on a separate line
/// for argument in env::args() {
///     println!("{argument}");
/// }
/// ```
///
/// [`args_os`]: ./fn.args_os.html
```

**Key observations:**

1. **Summary line**: "Returns the arguments which this program was started with..." - clear and concise
2. **Detailed explanation**: Covers behavior across platforms
3. **Panics section**: Documents edge cases that cause panics
4. **Examples section**: Provides runnable code
5. **Link**: Uses intra-doc link to related function

**Important**: Everything before the first empty line is reused in searches and module overviews. Keep the summary to one line for best results.

### What Not to Document

Don't document what the type system already makes clear:
- Don't explicitly write parameter and return types (rustdoc adds hyperlinks automatically)
- Avoid redundant information that's clear from the signature

**Bad:**
```rust
/// This function takes a String parameter and returns an i32.
pub fn parse(s: String) -> i32 { ... }
```

**Good:**
```rust
/// Parses a string into an integer.
///
/// # Examples
///
/// ```
/// let result = parse("42".to_string());
/// assert_eq!(result, 42);
/// ```
pub fn parse(s: String) -> i32 { ... }
```

## Common Documentation Sections

### Standard Sections

| Section | When to Use | Content |
|---------|-------------|---------|
| **Panics** | When function can panic | Explain conditions that cause panics |
| **Errors** | When function returns Result | Explain error conditions and their meanings |
| **Safety** | For unsafe code | Document safety requirements and invariants |
| **Examples** | Always | At least one runnable example |
| **Performance** | For performance-critical code | Note performance characteristics |
| **Note** | For important caveats | Additional information users should know |

### Section Ordering

Typical order for function documentation:
1. Summary line
2. Detailed explanation
3. Panics (if applicable)
4. Errors (if applicable)
5. Safety (if applicable)
6. Examples
7. Additional sections (Performance, Note, etc.)

### Section Headers

Use ATX-style headers (with `#`) for sections:

```rust
/// # Panics
///
/// This function panics if...
///
/// # Examples
///
/// ```
/// // example code
/// ```
```

## Documentation for Different Item Types

### Structs

```rust
/// Represents a 2D point in Cartesian coordinates.
///
/// This struct is commonly used for geometric calculations
/// and graphics programming.
///
/// # Examples
///
/// ```
/// let point = Point { x: 10.0, y: 20.0 };
/// println!("Point at ({}, {})", point.x, point.y);
/// ```
pub struct Point {
    /// The x-coordinate of the point.
    pub x: f64,
    /// The y-coordinate of the point.
    pub y: f64,
}
```

### Enums

```rust
/// Represents different types of HTTP methods.
///
/// This enum covers standard HTTP methods as defined in
/// RFC 7231 and RFC 5789.
///
/// # Examples
///
/// ```
/// use http::Method;
///
/// let method = Method::Get;
/// assert_eq!(method.as_str(), "GET");
/// ```
pub enum Method {
    /// The HTTP GET method.
    Get,
    /// The HTTP POST method.
    Post,
    /// The HTTP PUT method.
    Put,
    /// The HTTP DELETE method.
    Delete,
    /// The HTTP PATCH method.
    Patch,
}
```

### Traits

```rust
/// A trait for objects that can be serialized to a string.
///
/// Implementors of this trait can be converted to a string
/// representation for storage or transmission.
///
/// # Examples
///
/// ```
/// struct MyData {
///     value: i32,
/// }
///
/// impl ToString for MyData {
///     fn to_string(&self) -> String {
///         format!("MyData({})", self.value)
///     }
/// }
///
/// let data = MyData { value: 42 };
/// assert_eq!(data.to_string(), "MyData(42)");
/// ```
pub trait ToString {
    /// Convert the object to a string representation.
    fn to_string(&self) -> String;
}
```

### Modules

```rust
//! Provides utilities for working with strings.
//!
//! This module contains functions for common string operations
//! such as trimming, padding, and case conversion.
//!
//! # Examples
//!
//! ```
//! use my_crate::string_utils;
//!
//! let s = "  hello  ";
//! let trimmed = string_utils::trim(s);
//! assert_eq!(trimmed, "hello");
//! ```
pub mod string_utils {
    // module contents
}
```

### Macros

```rust
/// Creates a new vector with the given elements.
///
/// This macro provides a convenient syntax for creating vectors
/// with any number of elements.
///
/// # Examples
///
/// ```
/// let v = vec![1, 2, 3];
/// assert_eq!(v.len(), 3);
/// ```
#[macro_export]
macro_rules! vec {
    // implementation
}
```

## Tips for Writing Better Documentation

1. **Write for your audience**: Consider who will use your crate and what they need to know.

2. **Be specific**: Avoid vague descriptions like "This is useful" or "This does something".

3. **Show, don't tell**: Use examples to demonstrate usage rather than just describing it.

4. **Document the "why"**: Explain when to use something and when not to.

5. **Keep it up to date**: Update documentation when you change the code.

6. **Review documentation**: Treat documentation reviews as seriously as code reviews.

7. **Test your examples**: All examples should work as doctests.

8. **Use links**: Link to related items, standard library types, and external resources.

9. **Be consistent**: Use consistent style and structure across your documentation.

10. **Start simple**: Begin with basic documentation and expand it over time.
