---
name: build-safely
description: Load this skill when you need to enable a new unstable feature using build-safely and generate appropriate cfg flags in build.rs
user-invocable: true
---

# Build-Safely Skill

This skill helps you properly gate unstable Rust features using the `build-safely` crate in `build.rs` files.

## When to Use

Load this skill when:
- You need to enable a new unstable Rust feature
- You need to add `#![cfg_attr(unstable_FEATURENAME, feature(FEATURENAME))]` attributes
- You need to emit unstable feature checks in `build.rs`
- You need to create a feature request for a feature not in the `UnstableFeature` enum

## Quick Reference

### 1. Check if the feature exists in `UnstableFeature` enum

The following features have dedicated variants in `build_safely::nightly::UnstableFeature`:

| Variant | CFG Flag | Has CFG Flag |
|---------|----------|---------------|
| `adt_const_params` | `unstable_adt_const_params` | `has_adt_const_params` |
| `assert_matches` | `unstable_assert_matches` | `has_assert_matches` |
| `bool_to_result` | `unstable_bool_to_result` | `has_bool_to_result` |
| `can_vector` | `unstable_can_vector` | `has_can_vector` |
| `doc_notable_trait` | `unstable_doc_notable_trait` | `has_doc_notable_trait` |
| `iterator_try_collect` | `unstable_iterator_try_collect` | `has_iterator_try_collect` |
| `never_type` | `unstable_never_type` | `has_never_type` |
| `proc_macro_diagnostic` | `unstable_proc_macro_diagnostic` | `has_proc_macro_diagnostic` |
| `strip_circumfix` | `unstable_strip_circumfix` | `has_strip_circumfix` |
| `try_trait_v2` | `unstable_try_trait_v2` | `has_try_trait_v2` |
| `try_trait_v2_residual` | `unstable_try_trait_v2_residual` | `has_try_trait_v2_residual` |
| `unsized_const_params` | `unstable_unsized_const_params` | `has_unsized_const_params` |
| `write_all_vectored` | `unstable_write_all_vectored` | `has_write_all_vectored` |

### 2. If the feature is in the enum

Use the dedicated variant:

```rust
use build_safely::prelude::*;

fn main() -> Result<()> {
    let ac = AutoCfg::new()?;
    let allowed_features = cargo_allowed_features()?;
    
    // For a feature with a dedicated variant
    ac.emit_unstable_feature(iterator_try_collect, &allowed_features);
    
    Ok(())
}
```

Then in your library code:

```rust
#![cfg_attr(unstable_iterator_try_collect, feature(iterator_try_collect))]
```

Or to conditionally compile code:

```rust
#[cfg(has_iterator_try_collect)]
use some_unstable_function;
```

### 3. If the feature is NOT in the enum

Use `OtherFeature` and create a feature request:

```rust
use build_safely::prelude::*;
use build_safely::nightly::OtherFeature;

fn main() -> Result<()> {
    let ac = AutoCfg::new()?;
    let allowed_features = cargo_allowed_features()?;
    
    // For a feature without a dedicated variant
    ac.emit_unstable_feature(OtherFeature("exact_div".to_string()), &allowed_features);
    
    Ok(())
}
```

Then in your library code:

```rust
#![cfg_attr(unstable_exact_div, feature(exact_div))]
```

**IMPORTANT**: You MUST create a feature request markdown file for features using `OtherFeature`.

## Creating Feature Requests

When a feature is not in the `UnstableFeature` enum, you must:

1. Create a markdown file in `.vibe/feature-requests/` named `{feature_name}.md`
2. Include the following information:

```markdown
# Feature Request: {feature_name}

## Rust Feature
- Feature name: `{feature_name}`
- Tracking issue: [link to rust-lang/rust issue]

## Usage in Codebase
- Location: Where the feature is used (reponame, filepath, linenumber) in the main codebase (not in build.rs)
- Example: the exact statement in the main codebase (not in build.rs) which uses the feature
Usually one such example and location is sufficient

## Request
Please add a dedicated `UnstableFeature::{feature_name}` variant to support:
- `#![cfg_attr(unstable_{feature_name}, feature({feature_name}))]`
- `#[cfg(has_{feature_name})]`
```

3. After creating the feature request, the user can review it and submit it as an issue to the [build_safely repository](https://github.com/MusicalNinjaDad/build_safely)

## Verification Checklist

After adding a new unstable feature:

- [ ] Added `emit_unstable_feature` call in `build.rs`
- [ ] Used dedicated variant if available, otherwise `OtherFeature`
- [ ] Created feature request markdown file if using `OtherFeature`
- [ ] Added `#![cfg_attr(unstable_FEATURENAME, feature(FEATURENAME))]` to library crate root
- [ ] Used `#[cfg(has_FEATURENAME)]` for conditional compilation where needed
- [ ] Verified the build succeeds with nightly Rust

## Documentation

For more details, see:
- [build-safely crate documentation](https://docs.rs/build_safely/latest/build_safely/)
- [build-safely repository](https://github.com/MusicalNinjaDad/build_safely)
