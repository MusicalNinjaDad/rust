//! Extensions to the amazing [autocfg] designed to help with ergonomically and safely handling
//! experimental features in nightly.

use std::{
    path::Path,
    process::{Command, Output},
};

use autocfg::AutoCfg;

use crate::{BuildError, Result, get_var};
use probes::{has, make_probe};

/// Known features with `unstable_...` & `has_...`
#[allow(non_camel_case_types, reason = "shadowing feature naming")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnstableFeature {
    /// Also offers `assert_matches_location=root/module` to identify whether to
    /// `use std::assert_matches`(root) or `use std::assert_matches::assert_matches` (module)
    assert_matches,
    iterator_try_collect,
    never_type,
    proc_macro_diagnostic,
    try_trait_v2,
    try_trait_v2_residual,
    /// only provides `unstable_...` - please raise a PR to add a custom probe for `has_...`
    Other(&'static str),
}

impl From<&'static str> for UnstableFeature {
    fn from(feature: &'static str) -> Self {
        match feature {
            "assert_matches" => Self::assert_matches,
            "iterator_try_collect" => Self::iterator_try_collect,
            "never_type" => Self::never_type,
            "proc_macro_diagnostic" => Self::proc_macro_diagnostic,
            "try_trait_v2" => Self::try_trait_v2,
            "try_trait_v2_residual" => Self::try_trait_v2_residual,
            _ => Self::Other(feature),
        }
    }
}

mod probes {
    use autocfg::AutoCfg;

    pub fn make_probe(feature: &str, allowed: bool, probe: &str) -> String {
        let mut _probe = String::with_capacity(256);
        if allowed {
            _probe.push('\n');
            _probe.push_str("#![allow(stable_features)]");
            _probe.push('\n');

            _probe.push_str("#![feature(");
            _probe.push_str(feature);
            _probe.push_str(")]");
            _probe.push('\n');
        };
        _probe.push_str("#![allow(unused)]");
        _probe.push('\n');
        _probe.push_str(probe);
        _probe
    }

    pub fn has(ac: &AutoCfg, feature: &str, allowed: bool, probe: &str) {
        let cfg = format!("has_{feature}");
        autocfg::emit_possibility(&cfg);
        let code = make_probe(feature, allowed, probe);
        if ac.probe_raw(&code).is_ok() {
            autocfg::emit(&cfg);
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

pub trait Nightly {
    /// Offers at least 2 cfg flags for all [known features](UnstableFeature)
    ///
    /// You must pass a set of [AllowedFeatures], created by calling [cargo_allowed_features]
    ///
    /// ## cfg `unstable_...`
    /// - To be used at top-level crate via `#![cfg_attr(unstable_foo, feature(foo))]`
    ///
    /// ## Cfg-gating `has_...`
    /// Do **not** rely on `cfg(not(unstable_foo))` to suggest that `feature(foo)` is stable! There are 3
    /// reasons that `cfg(unstable_foo)` could be `false`:
    ///   1. The build is using `stable`/`beta`
    ///   2. The feature has been stabilised
    ///   3. The compiler is from before the feature was implemented
    ///
    /// All [known features](UnstableFeature) have a `has_...` cfg for this purpose.
    ///
    /// If you need to test that a feature is available in order to cfg-gate your code and it is not
    /// on the list of [known features](UnstableFeature), please raise a PR with a suggested probe.
    fn emit_unstable_feature(&self, feature: &'static str, allowed_features: &AllowedFeatures);
}

fn default_unstable_cfg(ac: &AutoCfg, feature: &'static str, allowed: bool) {
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

impl Nightly for AutoCfg {
    fn emit_unstable_feature(&self, feature: &'static str, allowed_features: &AllowedFeatures) {
        dbg!(&feature);
        let ac = self;
        let allowed = allowed_features.includes(feature);
        match UnstableFeature::from(feature) {
            UnstableFeature::assert_matches => {
                default_unstable_cfg(self, feature, allowed);
                has(ac, feature, allowed, probes::assert_matches::AVAILABLE);
                autocfg::emit_possibility("assert_matches_location, values(\"root\", \"module\")");
                if self
                    .probe_raw(&make_probe(feature, allowed, probes::assert_matches::ROOT))
                    .is_ok()
                {
                    autocfg::emit("assert_matches_location=\"root\"")
                } else if allowed && self.probe_raw(probes::assert_matches::MODULE).is_ok() {
                    //    ^^^^^^^ assert_matches was stabilised in root
                    autocfg::emit("assert_matches_location=\"module\"");
                }
            }
            UnstableFeature::iterator_try_collect => {
                default_unstable_cfg(self, feature, allowed);
                has(
                    ac,
                    feature,
                    allowed,
                    probes::iterator_try_collect::AVAILABLE,
                );
            }
            UnstableFeature::never_type => {
                default_unstable_cfg(self, feature, allowed);
                has(ac, feature, allowed, probes::never_type::AVAILABLE);
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
                    feature,
                    allowed,
                    probes::proc_macro_diagnostic::AVAILABLE,
                );
            }
            UnstableFeature::try_trait_v2 => {
                default_unstable_cfg(self, feature, allowed);
                has(ac, feature, allowed, probes::try_trait_v2::AVAILABLE);
            }
            UnstableFeature::try_trait_v2_residual => {
                default_unstable_cfg(self, feature, allowed);
                has(
                    ac,
                    feature,
                    allowed,
                    probes::try_trait_v2_residual::AVAILABLE,
                );
            }
            UnstableFeature::Other(feature) => default_unstable_cfg(self, feature, allowed),
        }
    }
}

pub fn cargo_unstable() -> Result<bool> {
    Ok(Command::new(get_var("CARGO")?)
        .args([
            "-Zunstable-options",
            "--config",
            "unstable.allow-features=[\"unstable-options\"]",
            "help",
        ])
        .status()
        .map_err(|err| BuildError::Other(err.to_string()))?
        .success())
}

fn cargo_config<P: AsRef<Path>>(
    current_dir: &Option<P>,
    added_unstable_options: bool,
) -> Result<Output> {
    let mut cargo = Command::new(get_var("CARGO")?);
    if let Some(dir) = &current_dir {
        cargo.current_dir(dir);
    }
    cargo.arg("-Zunstable-options");
    if added_unstable_options {
        cargo.args(["--config", "unstable.allow-features=[\"unstable-options\"]"]);
    }
    cargo.args(["config", "get"]);
    cargo
        .output()
        .map_err(|err| BuildError::Other(err.to_string()))
}

/// Identify which experimental features are allowed for this build.
///
/// This works fine on any channel and respects whitelists (`unstable.allowed-features`) in all
/// relevant cargo config.toml files.
///
/// ## Note
/// - This will not respect additional entries passed at the command line via
///   `cargo --config unstable.allow-features=[...]`
pub fn cargo_allowed_features() -> Result<AllowedFeatures> {
    _cargo_allowed_features(Option::<std::path::PathBuf>::None)
}

fn _cargo_allowed_features<P: AsRef<Path>>(current_dir: Option<P>) -> Result<AllowedFeatures> {
    if !cargo_unstable()? {
        // cargo won't accept `-Z` - so we're on a not-unstable toolchain
        return Ok(AllowedFeatures(_AllowedFeatures::None));
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

    let allowed = String::from_utf8_lossy(&output.stdout);
    let allowed = match allowed
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
                .map(ToString::to_string)
                .collect();
            if features.is_empty() {
                AllowedFeatures(_AllowedFeatures::None)
            } else {
                AllowedFeatures(_AllowedFeatures::Some(features))
            }
        }
    };
    Ok(allowed)
}

/// The set of allowed experimental features for the current build. The only way to create this
/// is via a call to [cargo_allowed_features] - this is deliberate, to ensure that people who have
/// decided to restrict the experimental features they use to a whitelist are respected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllowedFeatures(_AllowedFeatures);

impl AllowedFeatures {
    /// Not public as this doesn't consider any restrictions made via `RUSTFLAGS`, those
    /// features will be disabled for all calls to rustc when running probes.
    fn includes(&self, feature: &str) -> bool {
        match &self.0 {
            _AllowedFeatures::None => false,
            _AllowedFeatures::All => true,
            _AllowedFeatures::Some(features) => features.iter().find(|f| *f == feature).is_some(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum _AllowedFeatures {
    None,
    All,
    Some(Vec<String>),
}

#[cfg(test)]
mod tests {
    use std::{
        assert_matches,
        fs::{self, File},
        io::Write,
    };

    use tempfile::TempDir;

    use super::*;

    #[test]
    fn no_config_toml() {
        let tmp = TempDir::new().expect("tempdir");
        let allowed = _cargo_allowed_features(Some(&tmp));
        if cargo_unstable().expect("cargo_unstable") {
            assert_matches!(allowed, Ok(AllowedFeatures(_AllowedFeatures::All)));
            assert!(allowed.unwrap().includes("try_trait_v2"));
        } else {
            assert_matches!(allowed, Ok(AllowedFeatures(_AllowedFeatures::None)));
            assert!(!allowed.unwrap().includes("try_trait_v2"));
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
                if features == &vec!["try_trait_v2", "unstable-options"]
            );
            assert!(allowed.includes("try_trait_v2"));
            assert!(allowed.includes("unstable-options"));
        } else {
            assert_matches!(allowed, AllowedFeatures(_AllowedFeatures::None));
            assert!(!allowed.includes("try_trait_v2"));
            assert!(!allowed.includes("unstable-options"));
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
                if features == &vec!["try_trait_v2"]
            );
            assert!(allowed.includes("try_trait_v2"));
        } else {
            assert_matches!(allowed, AllowedFeatures(_AllowedFeatures::None));
            assert!(!allowed.includes("try_trait_v2"));
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

        assert_eq!(probes::make_probe("assert_matches", true, probe), expected);
    }
}
