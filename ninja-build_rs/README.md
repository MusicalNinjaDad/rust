# ninja-build_rs

Designed to help create good build scripts, with a focus on ease of use for you,
valuable output in `cargo build -vv` & no annoying surprises for anyone downstream.

## Prelude

```rust
use ninja_build_rs::prelude::*;
```

provides:

- A [`Result`] alias & [`BuildError`] type that gives meaningful output from `main() -> Result<()>`.
- [`get_var()`] & [`split_var()`] which automatically register `cargo::rerun-if-env-changed`
  and include the variable name in any errors.
- [`emit_unstable_feature()`](nightly::Nightly::emit_unstable_feature),
  [`cargo_allowed_features`](nightly::cargo_allowed_features) &
  enum [`UnstableFeature`](nightly::UnstableFeature) to provide a safe way to identify the
  availability of nightly features & handle the future stabilisation process without additional
  effort on your part. All while respecting any `allow-feature` whitelists.
