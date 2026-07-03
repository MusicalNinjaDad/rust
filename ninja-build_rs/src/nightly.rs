#![expect(clippy::test_attr_in_doctest)]
//! Checking for experimental or stabilised features is prone to subtle errors which create issues
//! for downstream users and verbose when done properly. This provides extensions to the amazing
//! [autocfg::AutoCfg] (re-exported via our prelude to make your life easier) to safely identify the
//! availability of nightly features & handle the future stabilisation process without additional
//! effort on your part. All while respecting any `allow-feature` whitelists.
//!
//! For a list of known features with dedicated probes see [UnstableFeature]
//!
//! # Usage
//!
//! ## In `build.rs`
//!
//! ```rust, no_run
//! use ninja_build_rs::prelude::*;
//!
//! fn main() -> Result<()> {
//!     // get a new AutoCfg or provide a valuable error
//!     // rather than panicing
//!     let ac = AutoCfg::new()?;
//!
//!     // check to see if the downstream crate has defined
//!     // `unstable.allow-features` in `.cargo/config.toml`.
//!     // It is mandatory to perform this check and pass the
//!     // result to any calls to `emit_unstable_feature`
//!     let allowed_features = cargo_allowed_features()?;
//!
//!     // We want to make use of `assert_matches` if it is available
//!     ac.emit_unstable_feature(assert_matches, &allowed_features);
//!     //                       ^^^^^^^^^^^^^^ - enum variant to avoid typos
//!
//!     Ok(())
//! }
//! ```
//!
//! ## In `lib.rs` / `main.rs`
//!
//! ```rust
//! // only enable unstable feature if it is available and has not yet been stabilised
//! #![cfg_attr(unstable_assert_matches, feature(assert_matches))]
//!
//! #[cfg(test)]
//! // Do these tests if `assert_matches` is available
//! #[cfg(has_assert_matches)]
//! mod tests {
//!     // `assert_matches` was moved in early 2026 before stabilisation
//!     #[cfg(assert_matches_location = "root")]
//!     use std::assert_matches;
//!
//!     // in earlier nightly compilers `assert_matches` was in a separate module
//!     #[cfg(assert_matches_location = "module")]
//!     use std::assert_matches::assert_matches;
//!
//!     #[test]
//!     fn has() {
//!         assert_matches!(Some(5), Some(n) if n == 5);
//!     }
//! }
//!
//! #[cfg(test)]
//! // Do these tests if `assert_matches` is not available
//! #[cfg(not(has_assert_matches))]
//! mod tests {
//!     #[test]
//!     fn has_not() {
//!         assert_eq!(Some(5), Some(5));
//!     }
//! }
//! ```
//!
//! # Note to downstream crates
//!
//! If you (transiently) depend on a crate which uses `ninja-build_rs` and have implemented a
//! whitelist of `allowed-features`.
//!
//! Due to limitations in the information provided by cargo:
//!
//! - This will obtain config.toml files based upon `OUT_DIR`. If this is not under the project
//!   root, you can override by providing an alternative path via the environment variable
//!   `NINJA_CARGO_CONFIG_DIR`. See cargo's documentation on config file hierarchical structure
//!   for more details.
//! - This will not respect additional entries passed at the command line via
//!   `cargo --config unstable.allow-features=[...]`

use std::{
    fmt::Debug,
    path::Path,
    process::{Command, Output},
};

/// re-exported from autocfg
///
pub use autocfg::AutoCfg;
use derive_more::Display;

use crate::{BuildError, Result, get_var};
use probes::{has, make_probe, unstable};

/// Known features with `unstable_...` & dedicated probes for `has_...`.
///
/// If the feature you want is not in this list you can use `Other` to get `unstable_...`
/// but please also raise a PR (or open an issue) to add a custom probe for `has_...`.
#[allow(non_camel_case_types, reason = "shadowing feature naming")]
#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum UnstableFeature {
    /// ## Provides cfg flags:
    /// - `#![cfg_attr(unstable_assert_matches, feature(assert_matches))]`
    /// - `#[cfg(has_assert_matches)]`
    /// - ```rust, ignore
    ///   #[cfg(assert_matches_location = "root")]
    ///   use std::assert_matches;
    ///   ```
    /// - ```rust, ignore
    ///   #[cfg(assert_matches_location = "module")]
    ///   use std::assert_matches::assert_matches;
    ///   ```
    assert_matches,
    /// ## Provides cfg flags:
    /// - `#![cfg_attr(unstable_iterator_try_collect, feature(iterator_try_collect))]`
    /// - `#[cfg(has_iterator_try_collect)]`
    iterator_try_collect,
    /// ## Provides cfg flags:
    /// - `#![cfg_attr(unstable_never_type, feature(never_type))]`
    /// - `#[cfg(has_never_type)]`
    never_type,
    /// ## Provides cfg flags:
    /// - `#![cfg_attr(unstable_proc_macro_diagnostic, feature(proc_macro_diagnostic))]`
    /// - `#[cfg(has_proc_macro_diagnostic)]`
    proc_macro_diagnostic,
    /// ## Provides cfg flags:
    /// - `#![cfg_attr(unstable_try_trait_v2, feature(try_trait_v2))]`
    /// - `#[cfg(has_try_trait_v2)]`
    try_trait_v2,
    /// ## Provides cfg flags:
    /// - `#![cfg_attr(unstable_try_trait_v2_residual, feature(try_trait_v2_residual))]`
    /// - `#[cfg(has_try_trait_v2_residual)]`
    try_trait_v2_residual,
    /// only provides `unstable_...` - please raise a PR to add a custom probe for `has_...`
    OtherFeature(String),
}

impl UnstableFeature {
    // This is not pub or trait From to avoid risk of typos
    fn from(feature: &str) -> Self {
        match feature {
            "assert_matches" => Self::assert_matches,
            "iterator_try_collect" => Self::iterator_try_collect,
            "never_type" => Self::never_type,
            "proc_macro_diagnostic" => Self::proc_macro_diagnostic,
            "try_trait_v2" => Self::try_trait_v2,
            "try_trait_v2_residual" => Self::try_trait_v2_residual,
            _ => Self::OtherFeature(feature.to_string()),
        }
    }
}

mod probes {
    use super::{AutoCfg, UnstableFeature};

    /// Prefix with:
    /// - #![allow(stable_features)] (only if allowed)
    /// - #![feature()] (only if allowed)
    /// - #![allow(unused)] (always)
    pub fn make_probe(feature: &UnstableFeature, allowed: bool, probe: &str) -> String {
        let mut _probe = String::with_capacity(256);
        if allowed {
            _probe.push('\n');
            _probe.push_str("#![allow(stable_features)]");
            _probe.push('\n');

            _probe.push_str("#![feature(");
            _probe.push_str(&feature.to_string());
            _probe.push_str(")]");
            _probe.push('\n');
        };
        _probe.push_str("#![allow(unused)]");
        _probe.push('\n');
        _probe.push_str(probe);
        _probe
    }

    /// Register `#[cfg(has_feature)]` & set based on the probe
    pub fn has(ac: &AutoCfg, feature: &UnstableFeature, allowed: bool, probe: &str) {
        let cfg = format!("has_{feature}");
        autocfg::emit_possibility(&cfg);
        let code = make_probe(feature, allowed, probe);
        if ac.probe_raw(&code).is_ok() {
            autocfg::emit(&cfg);
        }
    }

    /// Register `#[cfg(has_feature)]` & run a default probe
    pub fn unstable(ac: &AutoCfg, feature: &UnstableFeature, allowed: bool) {
        let cfg = format!("unstable_{feature}");
        autocfg::emit_possibility(&cfg);

        if allowed {
            let code = format!(
                r#"
#![deny(stable_features)]
#![feature({feature})]
#![allow(unused)]
"#
            );

            if ac.probe_raw(&code).is_ok() {
                autocfg::emit(&cfg);
            }
        }
    }

    pub mod assert_matches {
        pub const AVAILABLE: &str = r#"
use std::assert_matches;
"#;
        pub const ROOT: &str = r#"
use std::assert_matches;

fn main() {
    assert_matches!(Some(4), Some(_));
}
"#;
        // was stabilised in root - so no need to remove feature from this probe
        pub const MODULE: &str = r#"
#![allow(stable_features)]
#![feature(assert_matches)]
use std::assert_matches::assert_matches;

fn main() {
    assert_matches!(Some(4), Some(_));
}
"#;
    }

    pub mod iterator_try_collect {
        // vec! not array: https://internals.rust-lang.org/t/code-compiles-on-playground-but-fails-when-passed-via-stdin-to-rustc/24393
        pub const AVAILABLE: &str = r#"
fn try_collect() {
    let _: Option<Vec<_>> = std::iter::Iterator::try_collect(&mut vec![Some(1)].into_iter());
}
"#;
    }

    pub mod never_type {
        pub const AVAILABLE: &str = r#"
type Bang = !;
"#;
    }

    pub mod proc_macro_diagnostic {
        /// Special probe as feature only available in proc_macro context
        pub const UNSTABLE: &str = r#"
#![deny(stable_features)]
#![feature(proc_macro_diagnostic)]
#![allow(unused)]
extern crate proc_macro;
"#;
        pub const AVAILABLE: &str = r#"
extern crate proc_macro;
use proc_macro::Diagnostic;      
"#;
    }

    pub mod try_trait_v2 {
        pub const AVAILABLE: &str = r#"
use std::ops::Try;
"#;
    }

    pub mod try_trait_v2_residual {
        pub const AVAILABLE: &str = r#"
use std::ops::Residual;
"#;
    }
}

/// Adds [`AutoCfg::emit_unstable_feature`](Nightly::emit_unstable_feature)
pub trait Nightly {
    /// Offers at least 2 cfg flags for all [known features](UnstableFeature)
    ///
    /// # Feature enablement: `cfg(unstable_...)`
    ///
    /// - To be used at top-level crate via `#![cfg_attr(unstable_foo, feature(foo))]`
    ///
    /// # Cfg-gating: `cfg(has_...)`
    ///
    /// - **Do not rely on `#[cfg(not(unstable_foo))]` to suggest that `feature(foo)` is stable!**
    /// - There are 3 reasons that `#[cfg(unstable_foo)]` could be `false`:
    ///     1. The build is using `stable`/`beta` or the feature is not on the `allow-features` whitelist
    ///     2. The feature has been stabilised
    ///     3. The compiler is from before the feature was implemented
    /// - All [known features](UnstableFeature) have a `#[cfg(has_...)]` for this purpose.
    ///
    /// # Note
    ///
    /// - You must pass a set of [AllowedFeatures], created by calling [cargo_allowed_features]
    /// - If you need to test that a feature is available in order to cfg-gate your code and it is not
    ///   on the list of [known features](UnstableFeature), please raise a PR with a suggested probe.
    fn emit_unstable_feature(&self, feature: UnstableFeature, allowed_features: &AllowedFeatures);
}

impl Nightly for AutoCfg {
    fn emit_unstable_feature(&self, feature: UnstableFeature, allowed_features: &AllowedFeatures) {
        // show in `cargo build -vv`
        dbg!(&feature);

        let ac = self;
        let allowed = allowed_features.includes(&feature);
        match feature {
            UnstableFeature::assert_matches => {
                unstable(self, &feature, allowed);
                has(ac, &feature, allowed, probes::assert_matches::AVAILABLE);
                autocfg::emit_possibility("assert_matches_location, values(\"root\", \"module\")");
                if self
                    .probe_raw(&make_probe(&feature, allowed, probes::assert_matches::ROOT))
                    .is_ok()
                {
                    autocfg::emit("assert_matches_location=\"root\"")
                } else if allowed && self.probe_raw(probes::assert_matches::MODULE).is_ok() {
                    //    ^^^^^^^ assert_matches was stabilised in root
                    autocfg::emit("assert_matches_location=\"module\"");
                }
            }
            UnstableFeature::iterator_try_collect => {
                unstable(self, &feature, allowed);
                has(
                    ac,
                    &feature,
                    allowed,
                    probes::iterator_try_collect::AVAILABLE,
                );
            }
            UnstableFeature::never_type => {
                unstable(self, &feature, allowed);
                has(ac, &feature, allowed, probes::never_type::AVAILABLE);
            }
            UnstableFeature::proc_macro_diagnostic => {
                autocfg::emit_possibility("unstable_proc_macro_diagnostic");
                if allowed
                    && self
                        .probe_raw(probes::proc_macro_diagnostic::UNSTABLE)
                        .is_ok()
                {
                    autocfg::emit("unstable_proc_macro_diagnostic");
                }
                has(
                    ac,
                    &feature,
                    allowed,
                    probes::proc_macro_diagnostic::AVAILABLE,
                );
            }
            UnstableFeature::try_trait_v2 => {
                unstable(self, &feature, allowed);
                has(ac, &feature, allowed, probes::try_trait_v2::AVAILABLE);
            }
            UnstableFeature::try_trait_v2_residual => {
                unstable(self, &feature, allowed);
                has(
                    ac,
                    &feature,
                    allowed,
                    probes::try_trait_v2_residual::AVAILABLE,
                );
            }
            UnstableFeature::OtherFeature(_) => unstable(self, &feature, allowed),
        }
    }
}

/// Check whether cargo will accept unstable flags. You probably never need to run this
/// yourself and should prefer to simply call [`cargo_allowed_features`].
pub fn cargo_unstable() -> Result<bool> {
    Ok(Command::new(get_var("CARGO")?)
        .args([
            "-Zunstable-options",
            "--config",
            "unstable.allow-features=[\"unstable-options\"]",
            "help",
        ])
        .output()
        .map_err(|err| BuildError::Other(err.to_string()))?
        .status
        .success())
}

fn cargo_config<P: AsRef<Path>>(
    current_dir: &Option<P>,
    added_unstable_options: bool,
) -> Result<Output> {
    let mut cargo_config_get = Command::new(get_var("CARGO")?);
    if let Some(dir) = &current_dir {
        cargo_config_get.current_dir(dir);
    }
    cargo_config_get.arg("-Zunstable-options");
    if added_unstable_options {
        cargo_config_get.args(["--config", "unstable.allow-features=[\"unstable-options\"]"]);
    }
    cargo_config_get.args(["config", "get"]);

    // show in `cargo build -vv`
    dbg!(&cargo_config_get);

    cargo_config_get
        .output()
        .map_err(|err| BuildError::Other(err.to_string()))
}

/// Identify which experimental features are allowed for this build.
///
/// This works fine on any channel and respects whitelists (`unstable.allowed-features`) in all
/// relevant cargo config.toml files.
///
/// ## Note to downstream crates
///
/// Due to limitations in the information provided by cargo:
///
/// - This will obtain config.toml files based upon `OUT_DIR`. If this is not under the project
///   root, you can override by providing an alternative path via the environment variable
///   `NINJA_CARGO_CONFIG_DIR`. See cargo's documentation on config file hierarchical structure
///   for more details.
/// - This will not respect additional entries passed at the command line via
///   `cargo --config unstable.allow-features=[...]`
pub fn cargo_allowed_features() -> Result<AllowedFeatures> {
    println!("cargo::rerun-if-env-changed=NINJA_CARGO_CONFIG_DIR");
    let cwd = std::env::var("NINJA_CARGO_CONFIG_DIR")
        .or_else(|_| std::env::var("OUT_DIR"))
        .ok();
    _cargo_allowed_features(cwd)
}

fn _cargo_allowed_features<P: AsRef<Path>>(current_dir: Option<P>) -> Result<AllowedFeatures> {
    if !cargo_unstable()? {
        // show in `cargo build -vv`
        dbg!("cargo won't accept `-Z` - so we're on a not-unstable toolchain");

        let allowed_features = AllowedFeatures(_AllowedFeatures::None);

        // show in `cargo build -vv`
        dbg!(&allowed_features);
        return Ok(allowed_features);
    }

    let mut added_unstable_options = false;
    let mut output = cargo_config(&current_dir, added_unstable_options)?;

    if !output.status.success() {
        // Maybe there is a restricted list which doesn't include unstable-options
        added_unstable_options = true;
        output = cargo_config(&current_dir, added_unstable_options)?;

        if !output.status.success() {
            // Nope something else went wrong!
            return Err(BuildError::Other(format!(
                "cargo config failed with error {code}: {stderr}",
                code = output.status,
                stderr = String::from_utf8_lossy(&output.stderr)
            )));
        }
    };

    let cargo_config = String::from_utf8_lossy(&output.stdout);

    let allowed_features = match cargo_config
        .lines()
        .find(|line| line.starts_with("unstable.allow-features"))
    {
        None => AllowedFeatures(_AllowedFeatures::All),
        Some(features) => {
            // default output format is toml
            let features: Vec<_> = features
                .strip_prefix("unstable.allow-features = [")
                .ok_or_else(|| {
                    BuildError::Other(format!(
                        "invalid cargo config output: {}",
                        String::from_utf8_lossy(&output.stdout)
                    ))
                })?
                .strip_suffix("]")
                .ok_or_else(|| {
                    BuildError::Other(format!(
                        "invalid cargo config output: {}",
                        String::from_utf8_lossy(&output.stdout)
                    ))
                })?
                .replace("\"", "")
                .split(",")
                .map(str::trim)
                .filter(|feature| !added_unstable_options || *feature != "unstable-options")
                .map(UnstableFeature::from)
                .collect();
            if features.is_empty() {
                AllowedFeatures(_AllowedFeatures::None)
            } else {
                AllowedFeatures(_AllowedFeatures::Some(features))
            }
        }
    };

    // show in `cargo build -vv`
    dbg!(&allowed_features);

    Ok(allowed_features)
}

/// The set of allowed experimental features for the current build. The only way to create this
/// is via a call to [cargo_allowed_features] - this is deliberate, to ensure that people who have
/// decided to restrict the experimental features they use to a whitelist are respected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllowedFeatures(_AllowedFeatures);

impl AllowedFeatures {
    /// Not public as this doesn't consider any restrictions made via `RUSTFLAGS`, those
    /// features will be disabled for all calls to rustc when running probes.
    fn includes(&self, feature: &UnstableFeature) -> bool {
        match &self.0 {
            _AllowedFeatures::None => false,
            _AllowedFeatures::All => true,
            _AllowedFeatures::Some(features) => features.iter().find(|f| *f == feature).is_some(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// private to make it impossible to manually construct an [AllowedFeatures] from outside this crate
enum _AllowedFeatures {
    None,
    All,
    Some(Vec<UnstableFeature>),
}

#[cfg(test)]
mod tests {
    use std::{
        assert_matches,
        fs::{self, File},
        io::Write,
    };

    use tempfile::TempDir;

    use super::UnstableFeature::*;
    use super::*;

    #[test]
    fn no_config_toml() {
        let tmp = TempDir::new().expect("tempdir");
        let allowed = _cargo_allowed_features(Some(&tmp));
        if cargo_unstable().expect("cargo_unstable") {
            assert_matches!(allowed, Ok(AllowedFeatures(_AllowedFeatures::All)));
            assert!(allowed.unwrap().includes(&try_trait_v2));
        } else {
            assert_matches!(allowed, Ok(AllowedFeatures(_AllowedFeatures::None)));
            assert!(!allowed.unwrap().includes(&try_trait_v2));
        }
    }

    #[test]
    fn allowed_features() {
        let tmp = TempDir::new().expect("tempdir");
        let config_location = tmp.path().join(".cargo");
        fs::create_dir(&config_location).expect(".cargo created");
        dbg!(&config_location);
        let mut config =
            File::create_new(config_location.join("config.toml")).expect("create config.toml");
        writeln!(
            config,
            "unstable.allow-features = [\"try_trait_v2\", \"unstable-options\"]"
        )
        .expect("added to config");

        let allowed = _cargo_allowed_features(Some(&tmp)).unwrap();
        if cargo_unstable().expect("cargo_unstable") {
            assert_matches!(
                allowed,
                AllowedFeatures(_AllowedFeatures::Some(ref features))
                if features == &vec![try_trait_v2, OtherFeature("unstable-options".to_string())]
            );
            assert!(allowed.includes(&try_trait_v2));
            assert!(allowed.includes(&OtherFeature("unstable-options".to_string())));
        } else {
            assert_matches!(allowed, AllowedFeatures(_AllowedFeatures::None));
            assert!(!allowed.includes(&try_trait_v2));
            assert!(!allowed.includes(&OtherFeature("unstable-options".to_string())));
        }
    }

    #[test]
    fn allowed_features_no_unstable_options() {
        let tmp = TempDir::new().expect("tempdir");
        let config_location = tmp.path().join(".cargo");
        fs::create_dir(&config_location).expect(".cargo created");
        dbg!(&config_location);
        let mut config =
            File::create_new(config_location.join("config.toml")).expect("create config.toml");
        writeln!(config, "unstable.allow-features = [\"try_trait_v2\"]").expect("added to config");

        let allowed = _cargo_allowed_features(Some(&tmp)).unwrap();
        if cargo_unstable().expect("cargo_unstable") {
            assert_matches!(
                allowed,
                AllowedFeatures(_AllowedFeatures::Some(ref features))
                if features == &vec![try_trait_v2]
            );
            assert!(allowed.includes(&try_trait_v2));
        } else {
            assert_matches!(allowed, AllowedFeatures(_AllowedFeatures::None));
            assert!(!allowed.includes(&try_trait_v2));
        }
    }

    #[test]
    fn all_forbidden() {
        let tmp = TempDir::new().expect("tempdir");
        let config_location = tmp.path().join(".cargo");
        fs::create_dir(&config_location).expect(".cargo created");
        dbg!(&config_location);
        let mut config =
            File::create_new(config_location.join("config.toml")).expect("create config.toml");
        writeln!(config, "unstable.allow-features = []").expect("added to config");

        let allowed = _cargo_allowed_features(Some(&tmp));
        assert_matches!(allowed, Ok(AllowedFeatures(_AllowedFeatures::None)));
    }

    #[test]
    fn make_assert_matches_probe() {
        let expected = r#"
#![allow(stable_features)]
#![feature(assert_matches)]
#![allow(unused)]

use std::assert_matches;
"#;
        let probe = r#"
use std::assert_matches;
"#;

        assert_eq!(probes::make_probe(&assert_matches, true, probe), expected);
    }

    #[test]
    fn unstable_feature_display() {
        assert_eq!(
            "foo",
            format!("{}", UnstableFeature::OtherFeature("foo".to_string()))
        );
        assert_eq!("try_trait_v2", format!("{}", UnstableFeature::try_trait_v2))
    }
}
