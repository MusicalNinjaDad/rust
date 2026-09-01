# Debuggability



## All public types implement `Debug` (C-DEBUG)

If there are exceptions, they are rare.



## `Debug` representation is never empty (C-DEBUG-NONEMPTY)

Even for conceptually empty values, the `Debug` representation should never be
empty.

```rust
let empty_str = "";
assert_eq!(format!("{:?}", empty_str), "\"\"");

let empty_vec = Vec::<bool>::new();
assert_eq!(format!("{:?}", empty_vec), "[]");
```
