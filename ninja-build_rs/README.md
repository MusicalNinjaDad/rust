# ninja-build_rs

Designed to help create good build scripts, with a focus on ease of use for you,
valuable output in cargo build -vv & no annoying surprises for anyone downstream.

## Usage

```rust
use ninja_build_rs::prelude::*;
// Result uses BuildError to give meaningful messages
fn main() -> Result<()> {
    // get an environment variable and re-run build script if it changes.
    let my_var: String = get_var("MY_VAR")?;
    // get values from an environment variable, separated by the
    // OS path separator and re-run build script if it changes.
    let my_vals: IndexSet<String> = split_var("MY_VALUES")?;
    if my_vals.contains("some_value") {
        unimplemented!("do something")
    }
    // get a new AutoCfg or provide a valuable error
    // rather than panicing.
    let ac = AutoCfg::new()?;
    // check to see if the downstream crate has defined
    // `unstable.allow-features` in `.cargo/config.toml`.
    // It is mandatory to perform this check and pass the
    // result to any calls to `emit_unstable_feature`
    let allowed_features = cargo_allowed_features()?;
    // We want to make use of `assert_matches` if it is available
    ac.emit_unstable_feature(assert_matches, &allowed_features);
    //                       ^^^^^^^^^^^^^^ - enum variant to avoid typos
    Ok(())
}

```

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

## Note to downstream crates

If you (transiently) depend on a crate which uses `ninja-build_rs` and have implemented a
whitelist of `allowed-features`.

Due to limitations in the information provided by cargo:

- This will obtain config.toml files based upon `OUT_DIR`. If this is not under the project
  root, you can override by providing an alternative path via the environment variable
  `NINJA_CARGO_CONFIG_DIR`. See cargo's documentation on config file hierarchical structure
  for more details.
- This will not respect additional entries passed at the command line via
  `cargo --config unstable.allow-features=[...]`
