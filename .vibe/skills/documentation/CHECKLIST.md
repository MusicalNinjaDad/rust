# Rust Documentation Checklist

This checklist is based on official Rust rustdoc guidance. Use it to ensure comprehensive, high-quality documentation for your Rust crate.

## Project Setup

- [ ] Added `#![warn(missing_docs)]` to `lib.rs` or `main.rs`
- [ ] Considered upgrading to `#![deny(missing_docs)]` for strict enforcement
- [ ] All public items (structs, enums, functions, methods, modules, traits, macros) are documented
- [ ] Crate-level documentation exists with `//!` comments in `lib.rs`

## Crate-Level Documentation

### Introduction
- [ ] First line is a clear, concise sentence describing the crate's purpose
- [ ] Describes where the crate fits in the Rust ecosystem
- [ ] Explains the main use case(s) for the crate

### Content
- [ ] Includes a real-world usage example
- [ ] Explains key concepts and architecture
- [ ] Lists main features and capabilities
- [ ] Provides links to additional resources (guides, tutorials)
- [ ] Mentions compatibility and platform support if relevant

### Examples Quality
- [ ] Example is complete enough to be copied and used directly
- [ ] Example demonstrates the library's core value
- [ ] Example avoids unnecessary shortcuts that would confuse users

## Item Documentation Structure

For **every** public item, verify it follows this structure:

### 1. Summary Line (Required)
- [ ] First line is a single, clear sentence
- [ ] Explains **what** the item is/does (not how it works internally)
- [ ] Avoids technical jargon when possible
- [ ] Fits in one line (for search and module overviews)

### 2. Detailed Explanation (As Needed)
- [ ] Explains **how** to use the item
- [ ] Describes **when** to use it (and when not to)
- [ ] Covers behavior and semantics
- [ ] Documents any non-obvious behavior

### 3. Code Example (Required)
- [ ] At least one runnable code example is provided
- [ ] Example demonstrates the most common use case
- [ ] Example compiles successfully as a doctest
- [ ] Example is minimal but complete
- [ ] Consider adding multiple examples for different use cases

### 4. Advanced Sections (As Needed)
- [ ] **Panics**: Documents when and why the function might panic
- [ ] **Errors**: Documents possible error conditions and their meanings
- [ ] **Safety**: For unsafe code, documents safety requirements (see Rust API Guidelines)
- [ ] **Performance**: Notes about performance characteristics
- [ ] **Examples**: Additional examples for edge cases or advanced usage

## Examples Best Practices

### General
- [ ] Error handling is shown (not using `unwrap()` when avoidable)
- [ ] Hidden setup code uses `# ` prefix (not shown in output)
- [ ] Examples avoid `?` operator unless properly handled (see doctests.md)
- [ ] Uses `fn main()` wrapper when necessary for compilation

### Handling Error Types
- [ ] When using `?`, the error type is specified with `Ok::<(), ErrorType>(())`
- [ ] Or uses a proper `main` function with `Result` return type

### Partial Examples
- [ ] For multi-step explanations, each code block contains the complete program
- [ ] Irrelevant lines are hidden with `# ` prefix
- [ ] All code blocks for the same example can compile independently

## Intra-Doc Links

### Basic Linking
- [ ] Uses `[
`ItemName`]` syntax for linking to Rust items
- [ ] Backticks are optional: both `[`Item`]` and `[Item](Item)` work
- [ ] Links to items in scope (current module, parent modules, crate root)

### Generic Types
- [ ] Links to generic types use full syntax: `[`Vec<T>`]` or `[`HashMap<K, V>`]`
- [ ] Type parameters are preserved in links

### Paths
- [ ] Uses paths when needed for disambiguation: `[`super::Item`]`, `[`crate::module::Item`]`
- [ ] Uses `self` and `Self` appropriately

### Disambiguation
- [ ] Uses namespace prefixes when items have the same name: `[`Foo`](struct@Foo)`, `[`Foo`](fn@Foo)`
- [ ] Available prefixes: `struct`, `enum`, `trait`, `union`, `mod`, `module`, `const`, `constant`, `fn`, `function`, `field`, `variant`, `method`, `derive`, `type`, `value`, `macro`, `tyalias`, `typealias`, `prim`, `primitive`
- [ ] Uses `()` suffix for function disambiguation: `[`foo()`]`
- [ ] Uses `!` suffix for macro disambiguation: `[`macro!()`]`

### URL Fragments
- [ ] Uses fragment specifiers for specific sections: `[text](#section)`
- [ ] Links to specific items in other crates: `[`std::fmt#formatting-parameters`]`

## What to Document

### Public API
- [ ] All `pub` items have documentation
- [ ] All `pub` fields in structs/enums have documentation
- [ ] All `pub` methods in traits have documentation
- [ ] All `pub` associated functions have documentation

### Private Items
- [ ] Private items are **not** documented (rustdoc doesn't show them by default)
- [ ] Exception: items used in doctests may need `#[cfg(doctest)]` for testing

### Re-exports
- [ ] Re-exported items inherit documentation from original
- [ ] Additional documentation on re-exports uses the re-export's scope for links

## What to Exclude

### Implementation Details
- [ ] Internal helpers marked with `#[doc(hidden)]`
- [ ] Internal macros that could be misused are hidden
- [ ] Internal error types that shouldn't be used directly are hidden
- [ ] Implementation-only traits are hidden

### Module Documentation
- [ ] Module-level docs (`//!`) explain the module's purpose
- [ ] Module docs describe the organization and key types

## Documentation Tests (Doctests)

### Setup
- [ ] All examples in documentation can compile as doctests
- [ ] `cargo test --doc` passes successfully
- [ ] Doctests run without panicking

### Attributes
- [ ] Uses `ignore` for incomplete or pseudocode examples
- [ ] Uses `should_panic` for examples that should panic
- [ ] Uses `no_run` for examples that shouldn't be executed (network calls, infinite loops)
- [ ] Uses `compile_fail` for examples that should fail to compile
- [ ] Uses `edition2018`, `edition2021`, etc. for edition-specific examples
- [ ] Uses `standalone_crate` for doctests that must run independently

### Hiding Code
- [ ] Uses `# ` prefix to hide setup/teardown code from output
- [ ] Hidden code is still compiled and run
- [ ] Uses `##` to escape literal `#` characters in strings

### Advanced Patterns
- [ ] Uses `#[cfg(doctest)]` for items only needed for doctesting
- [ ] README doctests included via `#[doc = include_str!("../README.md")]` with `#[cfg(doctest)]`
- [ ] Consider using `test_harness` for multiple test functions in one example

## Markdown Features

### Standard Markdown
- [ ] Headers (`#`, `##`, etc.) used appropriately
- [ ] Lists (bulleted, numbered) used for enumerations
- [ ] Code blocks use triple backticks with optional language spec
- [ ] Inline code uses backticks
- [ ] Links and images use standard markdown syntax

### Rustdoc Extensions
- [ ] Strikethrough: `~~text~~` or `~text~`
- [ ] Footnotes: `[^note]` and `[^note]: content`
- [ ] Tables: pipe syntax with headers and separators
- [ ] Task lists: `- [x]` and `- [ ]`
- [ ] Smart punctuation: `--` to –, `...` to …, etc.
- [ ] Warning blocks: `<div class="warning">...</div>`

## Customization

### CSS and Themes
- [ ] Considered custom CSS for consistent branding
- [ ] Uses `--extend-css` or `--theme` flags for rustdoc
- [ ] Theme is accessible and readable

## Verification

### Automated Checks
- [ ] `cargo doc` produces no warnings
- [ ] `cargo test --doc` passes all doctests
- [ ] `cargo test --doc -- --show-output` shows no unexpected warnings
- [ ] All public items appear in generated documentation

### Manual Review
- [ ] Documentation renders correctly in browser
- [ ] All links resolve to the correct targets
- [ ] Examples are useful and educational
- [ ] Tone is consistent and appropriate for audience
- [ ] No typos or grammatical errors

## Maintenance

- [ ] Documentation updated when API changes
- [ ] Examples updated to reflect API changes
- [ ] New public items are documented before merge
- [ ] Documentation review is part of PR process
