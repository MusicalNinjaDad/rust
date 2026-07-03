//! Designed to help create good build scripts, with a focus on ease of use for you,
//! valuable output in `cargo build -vv` & no annoying surprises for anyone downstream.
//!
//! ## Prelude
//!
//! ```rust
//! use ninja_build_rs::prelude::*;
//! ```
//!
//! provides:
//!
//! - A [`Result`] alias & [`BuildError`] type that gives meaningful output from `main() -> Result<()>`.
//! - [`get_var()`] & [`split_var()`] which automatically register `cargo::rerun-if-env-changed`
//!   and include the variable name in any errors.
//! - [`emit_unstable_feature()`](nightly::Nightly::emit_unstable_feature),
//!   [`cargo_allowed_features`](nightly::cargo_allowed_features) &
//!   enum [`UnstableFeature`](nightly::UnstableFeature) to provide a safe way to identify the
//!   availability of nightly features & handle the future stabilisation process without additional
//!   effort on your part. All while respecting any `allow-feature` whitelists.
use std::{collections::HashSet, env::VarError, ffi::OsString};

/// Recommended prelude: `use ninja-build_rs::prelude::*`
///
/// - A [`Result`] alias & [`BuildError`] type that gives meaningful output from `main() -> Result<()>`.
/// - [`get_var()`] & [`split_var()`] which automatically register `cargo::rerun-if-env-changed`
///   and include the variable name in any errors.
/// - [`emit_unstable_feature()`](nightly::Nightly::emit_unstable_feature),
///   [`cargo_allowed_features`](nightly::cargo_allowed_features) &
///   enum [`UnstableFeature`](nightly::UnstableFeature) to provide a safe way to identify the
///   availability of nightly features & handle the future stabilisation process without additional
///   effort on your part. All while respecting any `allow-feature` whitelists.
pub mod prelude {
    pub use crate::nightly::{AutoCfg, Nightly, UnstableFeature::*, cargo_allowed_features};
    pub use crate::{Result, get_var, split_var};
}

pub mod nightly;

/// Attempt to get an environment variable.
///
/// - Emits `cargo::rerun-if-env-changed=key` to ensure changes trigger a rebuild.
/// - If not found the error returned will include the key name in the debug representation.
pub fn get_var(key: &str) -> Result<String> {
    println!("cargo::rerun-if-env-changed={key}");
    std::env::var(key).map_err(|err| BuildError::from_var_error(key, err))
}

/// Attempt to get an environment variable and split the values using the OS path separator.
///
/// - Emits `cargo::rerun-if-env-changed=key` to ensure changes trigger a rebuild.
/// - If not found the error returned will include the key name in the debug representation.
pub fn split_var(key: &str) -> Result<HashSet<String>> {
    Ok(std::env::split_paths(&get_var(key)?)
        .map(|p| p.to_string_lossy().to_string())
        .collect())
}

/// Result type wrapping [BuildError]. Using `main() -> Result<()>` in `build.rs` will
/// provide useful information in the debug representation sent to stderr on failure.
pub type Result<T> = std::result::Result<T, BuildError>;

#[derive(Debug)]
/// An error designed to have nice debug representations for common errors encountered
/// in build.rs
pub enum BuildError {
    /// If an environment variable was requested but not set
    ///
    /// outputs `VarNotSet("KEY")` to stderr
    VarNotSet(OsString),
    /// If an environment variable contains non-unicode characters
    ///
    /// outputs `VarInvalid("KEY", "contents")` to stderr
    VarInvalid(OsString, OsString),
    /// An IO Error occurred
    ///
    /// outputs `IOError(error details)` to stderr
    IOError(std::io::Error),
    /// An error when creating or using [AutoCfg]
    ///
    /// outputs `AutoCfgError(error details)`
    AutoCfgError(autocfg::Error),
    /// Catch-all for any other error
    ///
    /// outputs `Other(some text)` to stderr
    Other(String),
}

impl BuildError {
    /// Create a `BuildError` from a `VarError` for a given key.
    /// You probably won't need this often and can use [get_var] for most cases.
    pub fn from_var_error(key: &str, err: VarError) -> BuildError {
        match err {
            VarError::NotPresent => BuildError::VarNotSet(key.into()),
            VarError::NotUnicode(contents) => BuildError::VarInvalid(key.into(), contents),
        }
    }
}

impl From<autocfg::Error> for BuildError {
    fn from(e: autocfg::Error) -> Self {
        BuildError::AutoCfgError(e)
    }
}

impl From<std::io::Error> for BuildError {
    fn from(e: std::io::Error) -> Self {
        BuildError::IOError(e)
    }
}

/// Generate your own with `Err("some text")`
impl From<&str> for BuildError {
    fn from(msg: &str) -> Self {
        msg.to_string().into()
    }
}

/// Generate your own with `Err(String)`
impl From<String> for BuildError {
    fn from(msg: String) -> Self {
        BuildError::Other(msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn err_from_str() {
        let err = BuildError::from("oops");
        let msg = r#"Other("oops")"#;
        assert!(format!("{err:?}").contains(msg));
    }

    #[test]
    fn missing_env_var() {
        let random_key = "019de8d0-bb66-769d-9d4d-fec48aebdd49";
        let err = get_var(random_key);
        dbg!(&err);
        assert!(err.is_err());
        assert!(format!("{err:?}").contains(random_key));
    }
}
