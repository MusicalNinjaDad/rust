# Intra-Doc Linking

This guide covers how to create links to Rust items within documentation comments, based on official rustdoc guidance.

## Intra-Doc Links Overview

Rustdoc supports **intra-doc links** - links that automatically resolve to other Rust documentation pages using the item's path. These are different from regular markdown links.

### Basic Syntax

Rustdoc provides several equivalent ways to link to items:

```rust
/// This struct is not [Bar]
pub struct Foo1;

/// This struct is also not [bar](Bar)
pub struct Foo2;

/// This struct is also not [bar][b]
///
/// [b]: Bar
pub struct Foo3;

/// This struct is also not [`Bar`]
pub struct Foo4;

/// This struct *is* [`Bar`]!
pub struct Bar;
```

**Key points:**
- All of the above link to the `Bar` type
- Backticks around the link are stripped, so `[`Option`]` links to `Option`
- Unlike regular markdown, `[bar][Bar]` syntax works without a reference link definition

## Valid Link Targets

### Items in Scope

You can link to anything that's in scope from the current module:

```rust
mod inner {
    pub struct MyType;
    
    /// Link to [MyType] in the same module
    pub fn my_function() {}
}
```

### Paths

Use paths to link to items in other modules:

```rust
mod parent {
    mod child {
        pub struct InnerType;
    }
    
    /// Link to [child::InnerType] or [self::child::InnerType]
    pub fn outer_function() {}
}

/// Link to [parent::child::InnerType]
pub fn top_level() {}
```

**Available path components:**
- `Self` - the current type
- `self` - the current module
- `super` - the parent module
- `crate` - the crate root

### Special Paths

```rust
/// Link to [super::ParentType]
pub struct ChildType;

impl ChildType {
    /// Link to [Self] for the current type
    pub fn method(&self) {}
}

/// Link to [crate::module::Type] from anywhere in the crate
pub struct TopLevel;
```

### Generic Types

You can link to generic types. The type parameters are preserved in the link:

```rust
use std::collections::HashMap;

/// This struct wraps a [HashMap<K, V>].
/// 
/// For vectors: [Vec<T>].
/// For options: [Option<T>].
pub struct Wrapper<T>(T);

/// This is a version of [Receiver<T>] with support for [std::future].
///
/// You can obtain a [std::future::Future] by calling [Self::recv()].
pub struct AsyncReceiver<T> {
    sender: std::sync::mpsc::Receiver<T>
}

impl<T> AsyncReceiver<T> {
    pub async fn recv() -> T {
        unimplemented!()
    }
}
```

**Note**: Fully-qualified syntax like `<Vec as IntoIterator>::into_iter()` is [not yet supported](https://github.com/rust-lang/rust/issues/74563).

### Primitive Types

You can link to all primitive types listed in the [standard library documentation](https://doc.rust-lang.org/stable/std/index.html#primitives):

```rust
/// Works with primitive types like:
/// - [i32]
/// - [u64]
/// - [f64]
/// - [bool]
/// - [str]
/// - [char]
/// - [&str]
pub fn handle_primitives() {}
```

## URL Fragment Specifiers

You can use URL fragment specifiers (the part after `#`) just like regular links:

```rust
/// This is a special implementation of [positional parameters].
///
/// [positional parameters]: std::fmt#formatting-parameters
pub struct MySpecialFormatter;
```

This creates a link to the `std::fmt` module with the fragment `#formatting-parameters`.

**Common fragment targets:**
- Module sections: `#examples`, `#panics`, `#errors`
- Specific headings in documentation
- External documentation pages

## Namespaces and Disambiguation

### The Problem

Rust has three namespaces:
1. **Type namespace**: structs, enums, traits, type aliases
2. **Value namespace**: functions, constants, statics
3. **Macro namespace**: macros

Items can have the same name in different namespaces:

```rust
struct Foo {};        // Type namespace
fn Foo() {};          // Value namespace
macro_rules! Foo {};  // Macro namespace
```

When this happens, rustdoc will warn about ambiguity and suggest using a disambiguator.

### Disambiguation Prefixes

Use namespace prefixes to disambiguate:

```rust
/// See also: [`Foo`](struct@Foo) - the struct
/// 
/// This is different from [`Foo`](fn@Foo) - the function
struct Bar;

/// This is different from [`Foo`](struct@Foo)
fn Foo() {}

/// This is different from [`Foo`](fn@Foo)
struct Foo;
```

**Available prefixes:**
- `struct` - for structs
- `enum` - for enums
- `trait` - for traits
- `union` - for unions
- `mod` or `module` - for modules
- `const` or `constant` - for constants
- `fn` or `function` - for functions
- `field` - for struct fields
- `variant` - for enum variants
- `method` - for methods
- `derive` - for derive macros
- `type` or `tyalias` or `typealias` - for type aliases
- `value` - for values (functions, constants, etc.)
- `macro` - for macros
- `prim` or `primitive` - for primitive types

**Note**: The prefix is stripped when displayed. `[struct@Foo]` will render as `Foo` in the documentation.

### Function and Macro Disambiguation

You can also use suffixes for disambiguation:

```rust
/// This is different from [`foo!()`].
fn foo() {}

/// This is different from [`foo()`]
macro_rules! foo {
  () => {}
}
```

For macros, the `!` can be followed by `()`, `{}`, or `[]`:

```rust
/// See [`my_macro!()`], [`my_macro!{}`], or [`my_macro![]`]
```

### Automatic Disambiguation

There's one case where disambiguation is performed automatically: when an intra-doc link resolves to both a trait and a derive proc-macro at the same time. In this case, rustdoc will always generate a link to the trait and won't emit a "missing disambiguation" warning.

**Example:**
```rust
/// Link to [Clone] - this will link to the trait, not the derive macro
pub struct MyType;
```

If you want to link to the proc-macro instead, use the `macro@` disambiguator:

```rust
/// Link to [`Clone`](macro@Clone) - this will link to the derive macro
pub struct MyType;
```

## Link Resolution Rules

### Scope Resolution

Links are resolved in the scope of the module where the **item is defined**, not where the documentation comment appears.

```rust
mod inner {
    /// Link to [f()]
    pub struct S;
    
    pub fn f() {}
}

pub use inner::S; // The link to `f` in S's docs will still resolve correctly
```

### Re-export Resolution

When you re-export an item, you can add additional documentation to it. Links in that additional documentation are resolved in the scope of the re-export, not the original definition.

```rust
/// See also [foo()]
pub use std::process::Command;

pub fn foo() {}
```

Here, the link to `foo()` resolves to the local `foo()` function, not something in `std::process`.

### Link Warnings

If a link cannot be resolved, rustdoc will emit a warning. However:

- No warning is given for links from other crates (they can't be resolved at documentation time)
- No warning is given if the link doesn't look "sufficiently like" an intra-doc link
- Links containing `/` or `[]` characters are ignored (treated as regular markdown)

**Example of ignored links:**
```rust
/// This is not a link: [example.com/path]
/// This is not a link: [item[0]]
/// This is a regular markdown link: [text](https://example.com)
```

### Re-exported Item Links

When re-exporting an item, the original item's documentation is shown, but links in the additional documentation on the re-export use the re-export's scope:

```rust
mod original {
    /// Original documentation for [Helper]
    pub struct Helper;
    
    pub fn helper_function() {}
}

/// Additional docs for re-export.
/// See also [another_function()]
/// And [Helper] from original
pub use original::Helper;

fn another_function() {}
```

Here, `[another_function()]` links to the local function, and `[Helper]` links to the re-exported type.

## Macro Rules Scoping

Because of how `macro_rules!` macros are scoped in Rust, the intra-doc links of a `macro_rules!` macro are resolved **relative to the crate root**, not the module where the macro is defined.

```rust
mod my_module {
    /// Link to [crate::OtherType] - resolved from crate root, not my_module
    macro_rules! my_macro {
        () => {}
    }
}

struct OtherType;
```

This is [a known limitation](https://github.com/rust-lang/rust/issues/72243).

## Handling Unresolvable Links

In some cases, an intra-doc link cannot be generated (e.g., when the item is behind a `cfg` flag). The behavior depends on the link syntax:

```rust
/// Links that cannot be resolved:
/// 1. [a] - displayed as `[a]`
/// 2. [b][c] - displayed as `[b][c]`
/// 3. [d](e) - replaced with a link to `e`
/// 4. [f] - replaced with a link to `g`
///
/// [f]: g
```

**Summary:**
- Syntax 1 and 2: Link text is displayed as-is
- Syntax 3 and 4: Link is created to the target, even if it doesn't resolve

## Best Practices

### When to Use Intra-Doc Links

✅ **Use intra-doc links for:**
- Rust types, traits, functions, modules
- Items in the standard library
- Items in your dependencies
- Items in the same crate
- Generic type parameters

❌ **Don't use intra-doc links for:**
- External documentation (use regular markdown links)
- Non-Rust resources (use regular markdown links)
- Items that might not exist in all versions

### Link Style Guide

**Prefer:**
```rust
/// See also [MyType], [my_function], and [MODULE_CONSTANT]
```

**Instead of:**
```rust
/// See also <code>MyType</code>, <code>my_function</code>, and <code>MODULE_CONSTANT</code>
```

**For standard library types:**
```rust
/// Returns a [Result] with [Option] data
/// 
/// Related: [std::io::Error], [std::fmt::Display]
```

### Organizing Links

**In a list:**
```rust
/// Related items:
/// - [FunctionA]
/// - [FunctionB]
/// - [TypeC]
```

**In a sentence:**
```rust
/// This implements [TraitA] and [TraitB] for [MyType].
```

**With descriptions:**
```rust
/// See also:
/// - [FunctionA] for X
/// - [FunctionB] for Y
/// - [TypeC] for Z
```

## Common Patterns

### Linking to Methods

```rust
/// See [Vec::push] for adding elements.
/// 
/// Or use [Vec::with_capacity] for pre-allocation.
```

### Linking to Associated Items

```rust
/// This uses [MyStruct::new] to create instances.
/// 
/// And [MyStruct::from_str] for parsing.
```

### Linking to Module Contents

```rust
/// Items in [std::fs] for filesystem operations.
/// 
/// And [std::io] for I/O operations.
```

### Linking to Generic Types in Other Crates

```rust
/// Works with [serde::Serialize] and [serde::Deserialize].
/// 
/// And [tokio::task::JoinHandle<T>].
```

## Troubleshooting

### "Link not resolved" Warnings

If you see warnings about links not being resolved:

1. **Check scope**: Is the item visible from the module where the documentation is defined?
2. **Check name**: Is the name spelled correctly?
3. **Check namespace**: Do you need a disambiguator prefix?
4. **Check path**: Do you need to use a full path like `crate::module::Item`?

### Links Not Working in Generated Docs

If links appear as text instead of actual links in the generated HTML:

1. **Check syntax**: Are you using the correct intra-doc link syntax?
2. **Check for cfg**: Is the target item conditionally compiled and not available?
3. **Check for hidden**: Is the target item marked as `#[doc(hidden)]`?

### Links to Other Crates

Links to items in other crates (dependencies) will work if:
- The crate is available when rustdoc runs
- The item exists in that version of the crate
- The link uses the correct path

**Note**: rustdoc won't warn about unresolvable links to other crates, so test your documentation by building it with `cargo doc --no-deps --open`.

## Advanced Examples

### Complex Generic Links

```rust
/// Implements [FromIterator<T>] for [MyCollection<T>].
/// 
/// Also implements [IntoIterator<Item = T>].
/// 
/// And [AsRef<[T]>].
```

### Fully Qualified Paths

```rust
/// See also:
/// - [std::collections::HashMap<K, V>]
/// - [std::sync::Arc<T>]
/// - [std::pin::Pin<P>]
```

### Disambiguation in Practice

```rust
mod types {
    pub struct Error;
}

mod functions {
    pub fn Error() -> types::Error {
        types::Error
    }
}

/// Returns a [Error](struct@Error) type.
/// 
/// Not to be confused with [Error](fn@Error) function.
pub fn get_error() -> types::Error {
    functions::Error()
}
```

### Macro Disambiguation

```rust
/// This is a regular function: [foo]().
/// 
/// This is a macro: [foo!]().
/// 
/// Use [vec!] for creating vectors.
/// 
/// Use [println!] for printing.
```
