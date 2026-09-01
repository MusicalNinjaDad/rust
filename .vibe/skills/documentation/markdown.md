# Markdown Support in Rustdoc

This guide covers the markdown features supported by rustdoc, including standard CommonMark and rustdoc-specific extensions. Based on official rustdoc guidance.

## Overview

Rustdoc uses the [CommonMark Markdown specification](https://commonmark.org/) as its foundation. It supports all standard CommonMark features plus several useful extensions.

**Resources:**
- [CommonMark quick reference](https://commonmark.org/help/)
- [Current CommonMark spec](https://spec.commonmark.org/current/)

## Standard Markdown Features

### Headers

Use `#` for different header levels:

```markdown
# Level 1 - Crate or module title
## Level 2 - Major sections
### Level 3 - Subsections
#### Level 4 - Minor sections
```

**Example:**
```rust
/// # My Struct
///
/// ## Usage
///
/// ### Basic usage
/// 
/// Some text here.
pub struct MyStruct;
```

### Paragraphs

Regular text forms paragraphs:

```markdown
This is a paragraph.

This is another paragraph.
```

**Note:** In documentation comments, blank lines are preserved, so you need to use `///` on each line or leave blank comment lines.

### Text Formatting

**Bold:**
```markdown
**bold text** or __bold text__
```

**Italic:**
```markdown
*italic text* or _italic text_
```

**Strikethrough** (rustdoc extension):
```markdown
~~strikethrough text~~ or ~strikethrough text~
```

**Inline code:**
```markdown
`code` or `` `backticks` ``
```

### Lists

**Bulleted lists:**
```markdown
- Item 1
- Item 2
  - Nested item
- Item 3
```

**Numbered lists:**
```markdown
1. First item
2. Second item
3. Third item
```

**In documentation comments:**
```rust
/// - First item
/// - Second item
///   - Nested item
/// - Third item
```

### Links

**Regular markdown links:**
```markdown
[link text](https://example.com)
```

**Title attribute:**
```markdown
[link text](https://example.com "title")
```

**Reference-style links:**
```markdown
[link text][my-link]

[my-link]: https://example.com
```

### Images

```markdown
![alt text](image.png)
```

**With title:**
```markdown
![alt text](image.png "title")
```

**Note:** Images in rustdoc may not display in all contexts (e.g., when viewing documentation in a terminal or IDE).

### Code Blocks

**Inline code:**
```markdown
Use the `println!` macro.
```

**Fenced code blocks:**
```markdown
```
let x = 5;
```
```

**With language:**
```markdown
```rust
let x = 5;
```
```

**Indented code blocks:**
```markdown
    let x = 5;
    let y = 10;
```

### Blockquotes

```markdown
> This is a blockquote.
> 
> It can span multiple paragraphs.
```

### Horizontal Rules

```markdown
---

Or

***

Or

___
```

### HTML

You can include raw HTML in markdown:

```markdown
<div class="custom">Custom HTML</div>
```

## Rustdoc-Specific Markdown Extensions

### Strikethrough

Text may be rendered with a horizontal line through the center:

```markdown
An example of ~~strikethrough text~~. You can also use ~single tildes~.
```

**Rendered:**
> An example of ~~strikethrough text~~. You can also use ~single tildes~.

This follows the [GitHub Strikethrough extension](https://github.github.com/gfm/#strikethrough-extension-).

### Footnotes

Create numbered footnotes in your documentation:

```markdown
This is an example of a footnote[^note].

[^note]: This text is the contents of the footnote, which will be rendered towards the bottom.
```

**Rendered:**
> This is an example of a footnote[^note].
> 
> [^note]: This text is the contents of the footnote, which will be rendered towards the bottom.

**Key points:**
- Footnote labels start with `[^` followed by the footnote identifier
- Footnote definitions use `[^identifier]:` followed by the text
- Footnotes are automatically numbered based on the order they appear

**Multiple footnotes:**
```markdown
First footnote[^first] and second[^second].

[^first]: First footnote content.
[^second]: Second footnote content.
```

### Tables

Create tables using pipe syntax:

```markdown
| Header1 | Header2 |
|---------|---------|
| Cell 1  | Cell 2  |
| Cell 3  | Cell 4  |
```

**Rendered:**

| Header1 | Header2 |
|---------|---------|
| Cell 1  | Cell 2  |
| Cell 3  | Cell 4  |

**Alignment:**
```markdown
| Left | Center | Right |
|:-----|:------:|------:|
| L    | C      | R     |
```

This follows the [GitHub Tables extension](https://github.github.com/gfm/#tables-extension-).

**Formatting tips:**
- The header row must start and end with `|`
- The separator row must contain at least one `-` between each pair of `|`
- Cells don't need to be aligned vertically
- Outer pipes are optional but recommended for consistency

**Example in rustdoc:**
```rust
/// | Parameter | Description | Default |
/// |-----------|-------------|---------|
/// | `timeout` | Connection timeout in seconds | 30 |
/// | `retries` | Number of retry attempts | 3 |
```

### Task Lists

Create checklists or task lists:

```markdown
- [x] Complete task
- [ ] Incomplete task
- [x] Another complete task
```

**Rendered:**
- [x] Complete task
- [ ] Incomplete task
- [x] Another complete task

This follows the [GitHub Task List extension](https://github.github.com/gfm/#task-list-items-extension-).

**Use cases in documentation:**
- Feature checklists
- Migration guides
- Compatibility matrices

### Smart Punctuation

Some ASCII punctuation sequences are automatically converted to Unicode characters:

| ASCII | Unicode | Description |
|-------|---------|-------------|
| `--`  | –       | En dash |
| `---` | —       | Em dash |
| `...` | …       | Ellipsis |
| `"`  | " or "  | Smart quotes (context-dependent) |
| `'`  | ' or '  | Smart quotes (context-dependent) |

**Example:**
```markdown
This is a range from 1--10...

And some "quoted text".
```

**Rendered:**
> This is a range from 1-10...
> 
> And some "quoted text".

**Note:** You can disable smart punctuation by escaping:
```markdown
This is a range from 1\--10\.\.\.
```

## Rustdoc-Specific HTML Features

### Warning Blocks

Create styled warning or note blocks:

```markdown
/// documentation
///
/// <div class="warning">A big warning!</div>
///
/// more documentation
```

**With markdown inside:**
```markdown
/// documentation
///
/// <div class="warning">
///
/// Go to [this link](https://rust-lang.org)!
///
/// </div>
///
/// more documentation
```

**Important:** To have markdown processed inside HTML tags, you must have a blank line between the opening tag and your markdown content.

### Custom CSS Classes

You can add custom CSS classes to elements:

```markdown
<div class="my-custom-class">Content</div>
```

This is useful when combined with custom CSS (via `--extend-css` or `--theme` flags).

## Code-Specific Formatting

### Code Highlighting

Rustdoc automatically applies syntax highlighting to Rust code:

```rust
/// ```
/// // This code will be syntax highlighted
/// let x: i32 = 5;
/// let y = &x;
/// ```
```

**Specifying language:**
```rust
/// ```rust
/// // Explicit Rust code
/// ```

/// ```text
/// // Plain text, no highlighting
/// ```
```

### Preserving Whitespace

In code blocks, leading and trailing whitespace is preserved.

### Escaping Backticks in Code

To include a backtick in code, use multiple backticks:

```markdown
`` `backtick` ``
```

**Rendered:** ` `backtick` `

## Special Considerations for Documentation Comments

### Comment Syntax

In Rust, documentation comments use `///` or `//!`:

```rust
/// This is a doc comment for the following item
/// 
/// It can span multiple lines
/// 
/// Each line starts with ///
pub fn my_function() {}

//! This is a module-level or crate-level doc comment
//! 
//! It documents the module or crate itself
```

### Blank Lines

In documentation comments, blank lines are significant:

```rust
/// First paragraph.
///
/// Second paragraph.
///
/// Third paragraph.
```

This creates three separate paragraphs in the rendered documentation.

**Without blank lines:**
```rust
/// First paragraph.
/// Second paragraph.
/// Third paragraph.
```

This creates one continuous paragraph.

### Indentation

Indentation in documentation comments is handled specially:

```rust
/// This is the first line.
///     This is indented code.
/// This is back to normal text.
```

**Rendered:**
> This is the first line.
>     This is indented code.
> This is back to normal text.

**Note:** For code blocks, prefer using fenced code blocks with triple backticks.

### Nesting

You can nest most markdown elements:

```rust
/// - List item with **bold** text
/// - List item with `code`
/// - List item with [a link](https://example.com)
///   - Nested list item
```

## Examples in Action

### Comprehensive Documentation Example

```rust
/// # My Struct
///
/// This struct represents a **concept** in the system.
/// It provides functionality for:
/// - Feature A
/// - Feature B
/// - Feature C
///
/// ## Usage
///
/// ```
/// use my_crate::MyStruct;
///
/// let s = MyStruct::new();
/// s.do_something();
/// ```
///
/// ## Configuration
///
/// | Option | Description | Default |
/// |--------|-------------|---------|
/// | `timeout` | Timeout in seconds | 30 |
/// | `retries` | Number of retries | 3 |
///
/// ## Warnings
///
/// <div class="warning">
///
/// Don't do this thing or bad things will happen[^1].
///
/// </div>
///
/// [^1]: This is a footnote explaining the warning.
///
/// ~~~~This text is strikethrough~~~~
///
/// ---
///
/// See also: [OtherStruct], [some_function]
pub struct MyStruct;
```

### Module Documentation Example

```rust
//! # My Module
//!
//! This module provides utilities for working with data.
//!
//! ## Features
//!
//! - [x] Feature 1 (implemented)
//! - [ ] Feature 2 (planned)
//! - [x] Feature 3 (implemented)
//!
//! ## Example
//!
//! ```
//! use my_crate::my_module;
//!
//! let result = my_module::process("input");
//! ```
pub mod my_module {}
```

## Tips and Best Practices

### 1. Use Headers Wisely
- Use `#` for the main title
- Use `##` for major sections
- Use `###` for subsections
- Don't skip header levels (e.g., don't go from `##` to `####`)

### 2. Keep Paragraphs Short
- Short paragraphs are easier to read
- Aim for 2-4 sentences per paragraph
- Use lists for enumerations

### 3. Use Code Blocks Effectively
- Use fenced code blocks (` ``` `) for Rust code
- Use `rust` language specifier for explicit Rust code
- Use `#` to hide boilerplate
- Keep examples minimal but complete

### 4. Use Tables for Structured Data
- Tables are great for configuration options
- Tables are great for function parameters
- Tables are great for comparison charts

### 5. Use Lists for Enumerations
- Use bulleted lists for unordered items
- Use numbered lists for ordered steps
- Use task lists for checklists

### 6. Use Links Generously
- Link to related items in your crate
- Link to standard library types
- Link to external documentation when relevant

### 7. Use Formatting for Emphasis
- Use **bold** for important concepts
- Use *italic* for emphasis
- Use `code` for code-related text

### 8. Use HTML Sparingly
- Prefer markdown over HTML when possible
- Use HTML only when markdown doesn't support what you need
- Use `<div class="warning">` for important notes

### 9. Test Your Markdown
- View the generated documentation with `cargo doc --open`
- Ensure all formatting renders as expected
- Check that links work

### 10. Be Consistent
- Use consistent formatting throughout your documentation
- Follow the same style as the rest of your crate
- Follow the same style as the Rust ecosystem

## Common Issues and Solutions

### Problem: Code block not syntax highlighted

**Solution:**
```rust
/// ```rust
/// // Explicit language
/// ```
```

Or ensure the code is valid Rust (rustdoc auto-detects Rust code).

### Problem: Lists not rendering correctly

**Solution:**
```rust
/// - Item 1
///
/// - Item 2
///
/// - Item 3
```

Make sure there are blank lines between list items in documentation comments.

### Problem: Markdown not working in HTML tags

**Solution:**
```rust
/// <div class="warning">
///
/// This is **bold** markdown inside HTML.
///
/// </div>
```

Add blank lines between HTML tags and markdown content.

### Problem: Smart punctuation causing issues

**Solution:**
```rust
/// Use 1\-2 for a range (escapes the en dash)
/// Use 1..3 for ellipsis
```

Or just accept the automatic conversion.

### Problem: Tables not rendering correctly

**Solution:**
```rust
/// | Header 1 | Header 2 |
/// |----------|----------|
/// | Cell 1   | Cell 2   |
```

Make sure you have the separator row with dashes.

## Summary

Rustdoc supports all standard CommonMark markdown plus useful extensions:

- **Standard**: Headers, paragraphs, formatting, lists, links, images, code blocks, blockquotes, horizontal rules, HTML
- **Extensions**: Strikethrough, footnotes, tables, task lists, smart punctuation
- **Rustdoc-specific**: Warning blocks, custom CSS classes

Use these features to create rich, readable, and informative documentation for your Rust crates.
