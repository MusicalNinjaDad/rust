---
name: rust-api-design
description: Load this skill when designing Rust APIs or reviewing Rust library code for API quality.
---

# Rust API Design

Use this skill when designing Rust APIs or reviewing Rust library code for API quality.

## Checklist

### Naming

See [naming.md](naming.md) for details.

- [ ] Casing conforms to RFC 430 (C-CASE)
- [ ] Ad-hoc conversions follow `as_`, `to_`, `into_` conventions (C-CONV)
- [ ] Getter names follow Rust convention (C-GETTER)
- [ ] Methods on collections that produce iterators follow `iter`, `iter_mut`, `into_iter` (C-ITER)
- [ ] Iterator type names match the methods that produce them (C-ITER-TY)
- [ ] Feature names are free of placeholder words (C-FEATURE)
- [ ] Names use a consistent word order (C-WORD-ORDER)

### Interoperability

See [interoperability.md](interoperability.md) for details.

- [ ] Types eagerly implement common traits (C-COMMON-TRAITS)
- [ ] Conversions use the standard traits `From`, `AsRef`, `AsMut` (C-CONV-TRAITS)
- [ ] Collections implement `FromIterator` and `Extend` (C-COLLECT)
- [ ] Data structures implement Serde's `Serialize`, `Deserialize` (C-SERDE)
- [ ] Types are `Send` and `Sync` where possible (C-SEND-SYNC)
- [ ] Error types are meaningful and well-behaved (C-GOOD-ERR)
- [ ] Binary number types provide `Hex`, `Octal`, `Binary` formatting (C-NUM-FMT)
- [ ] Generic reader/writer functions take `R: Read` and `W: Write` by value (C-RW-VALUE)

### Macros

See [macros.md](macros.md) for details.

- [ ] Input syntax is evocative of the output (C-EVOCATIVE)
- [ ] Macros compose well with attributes (C-MACRO-ATTR)
- [ ] Item macros work anywhere that items are allowed (C-ANYWHERE)
- [ ] Item macros support visibility specifiers (C-MACRO-VIS)
- [ ] Type fragments are flexible (C-MACRO-TY)

### Documentation

See [documentation.md](documentation.md) for details.

- [ ] Crate level docs are thorough and include examples (C-CRATE-DOC)
- [ ] All items have a rustdoc example (C-EXAMPLE)
- [ ] Examples use `?`, not `try!`, not `unwrap` (C-QUESTION-MARK)
- [ ] Function docs include error, panic, and safety considerations (C-FAILURE)
- [ ] Prose contains hyperlinks to relevant things (C-LINK)
- [ ] Cargo.toml includes all common metadata (C-METADATA)
- [ ] Release notes document all significant changes (C-RELNOTES)
- [ ] Rustdoc does not show unhelpful implementation details (C-HIDDEN)

### Predictability

See [predictability.md](predictability.md) for details.

- [ ] Smart pointers do not add inherent methods (C-SMART-PTR)
- [ ] Conversions live on the most specific type involved (C-CONV-SPECIFIC)
- [ ] Functions with a clear receiver are methods (C-METHOD)
- [ ] Functions do not take out-parameters (C-NO-OUT)
- [ ] Operator overloads are unsurprising (C-OVERLOAD)
- [ ] Only smart pointers implement `Deref` and `DerefMut` (C-DEREF)
- [ ] Constructors are static, inherent methods (C-CTOR)

### Flexibility

See [flexibility.md](flexibility.md) for details.

- [ ] Functions expose intermediate results to avoid duplicate work (C-INTERMEDIATE)
- [ ] Caller decides where to copy and place data (C-CALLER-CONTROL)
- [ ] Functions minimize assumptions about parameters by using generics (C-GENERIC)
- [ ] Traits are object-safe if they may be useful as a trait object (C-OBJECT)

### Type safety

See [type-safety.md](type-safety.md) for details.

- [ ] Newtypes provide static distinctions (C-NEWTYPE)
- [ ] Arguments convey meaning through types, not `bool` or `Option` (C-CUSTOM-TYPE)
- [ ] Types for a set of flags are `bitflags`, not enums (C-BITFLAG)
- [ ] Builders enable construction of complex values (C-BUILDER)

### Dependability

See [dependability.md](dependability.md) for details.

- [ ] Functions validate their arguments (C-VALIDATE)
- [ ] Destructors never fail (C-DTOR-FAIL)
- [ ] Destructors that may block have alternatives (C-DTOR-BLOCK)

### Debuggability

See [debuggability.md](debuggability.md) for details.

- [ ] All public types implement `Debug` (C-DEBUG)
- [ ] `Debug` representation is never empty (C-DEBUG-NONEMPTY)

### Future proofing

See [future-proofing.md](future-proofing.md) for details.

- [ ] Sealed traits protect against downstream implementations (C-SEALED)
- [ ] Structs have private fields (C-STRUCT-PRIVATE)
- [ ] Newtypes encapsulate implementation details (C-NEWTYPE-HIDE)
- [ ] Data structures do not duplicate derived trait bounds (C-STRUCT-BOUNDS)

### Necessities

See [necessities.md](necessities.md) for details.

- [ ] Public dependencies of a stable crate are stable (C-STABLE)
- [ ] Crate and its dependencies have a permissive license (C-PERMISSIVE)
