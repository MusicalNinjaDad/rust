---
name: documentation
description: Load this skill when you need to write Rust documentation, create doc comments, or work with rustdoc features including documentation tests and intra-doc links.
---

# Rust Documentation Skill

This skill provides comprehensive guidance on writing Rust documentation following official rustdoc best practices. Use this when creating, reviewing, or improving documentation in Rust projects.

## Quick Reference Checklist

See [CHECKLIST.md](./CHECKLIST.md) for a complete, actionable checklist.

### Before You Start
- [ ] Add `#![warn(missing_docs)]` to `lib.rs` or `main.rs`
- [ ] Plan crate-level documentation with `//!` comments
- [ ] Identify public API surface that needs documentation

### For Every Public Item
- [ ] First line: short, clear description (one sentence)
- [ ] Detailed explanation (as needed)
- [ ] At least one runnable code example
- [ ] Advanced explanations (edge cases, panics, errors)

### Documentation Quality
- [ ] All examples compile and run as doctests
- [ ] Use intra-doc links for Rust types/items
- [ ] Hide implementation details with `#[doc(hidden)]`
- [ ] Use markdown extensions appropriately

## Detailed Guides

| Topic | File | Description |
|-------|------|-------------|
| **Basics** | [basics.md](./basics.md) | How to write good documentation, structure, and crate-level docs |
| **Content Guidelines** | [content-guidelines.md](./content-guidelines.md) | What to include/exclude, lints, and customization |
| **Linking** | [linking.md](./linking.md) | Intra-doc links, disambiguation, and scoping rules |
| **Doc Tests** | [doctests.md](./doctests.md) | Writing testable examples, attributes, and advanced patterns |
| **Markdown** | [markdown.md](./markdown.md) | Supported markdown features and rustdoc extensions |

## Common Patterns

### Basic Item Documentation
```rust
/// Short sentence explaining what this does.
///
/// More detailed explanation covering edge cases,
/// behavior, and usage guidelines.
///
/// # Examples
///
/// ```
/// // Example code that demonstrates usage
/// let result = my_function(42);
/// assert_eq!(result, 42);
/// ```
///
/// # Panics
///
/// This function panics if...
///
/// # Errors
///
/// Returns an error if...
pub fn my_function(input: i32) -> Result<i32, Error> {
    // implementation
}
```

### Module-Level Documentation
```rust
//! This module provides functionality for X.
//!
//! It includes support for:
//! - Feature A
//! - Feature B
//!
//! # Examples
//!
//! ```
//! use my_crate::this_module;
//! // example usage
//! ```
```

### Hiding Implementation Details
```rust
#[doc(hidden)]
pub fn internal_helper() { ... }

#[doc(hidden)]
pub use internal::some_type;
```

### Intra-Doc Links
```rust
/// See also [`MyStruct`], [`my_function`], or [`super::parent_item`].
/// 
/// For generic types: [`HashMap<K, V>`] or [`Vec<T>`].
/// 
/// With disambiguation: [`Foo`](struct@Foo) vs [`Foo`](fn@Foo).
```

## Essential Commands

```bash
# Check documentation coverage
cargo doc --no-deps --open

# Run documentation tests
cargo test --doc

# Run with output (to see warnings)
cargo test --doc -- --show-output

# Enable strict documentation requirements
# In lib.rs or main.rs:
#![deny(missing_docs)]
```

## When to Load This Skill

- Writing new crate or module documentation
- Adding documentation to public API items
- Creating or fixing documentation tests (doctests)
- Linking to other items in documentation
- Customizing rustdoc output appearance
- Reviewing documentation quality
