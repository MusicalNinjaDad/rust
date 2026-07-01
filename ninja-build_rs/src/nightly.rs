//! Extensions to the amazing [autocfg] designed to help with ergonomically and safely handling
//! experimental features in nightly.

use std::{fmt::Display, path::Path, process::Command};

use autocfg::AutoCfg;

use crate::{BuildError, Result, get_var};

/// Location of assert_matches!() macro. Stabilisation was reverted at last minute
/// on 2026-04-10, leaving the macro in the new planned location.
///
/// See [AutoCfg::assert_matches_location] for more details
#[deprecated(since = "0.1.1", note = "handled by `emit_unstable_feature`")]
pub enum AssertMatchesLocation {
    /// Macro is at `std::assert_matches`
    Root,
    /// Macro is at `std::assert_matches::assert_matches`
    Module,
}

#[expect(deprecated)]
impl Display for AssertMatchesLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssertMatchesLocation::Root => write!(f, "assert_matches_in_root"),
            AssertMatchesLocation::Module => write!(f, "assert_matches_in_module"),
        }
    }
}

#[expect(deprecated)]
impl AssertMatchesLocation {
    /// See [AutoCfg::assert_matches_location] for more details
    pub fn emit_possibilities() {
        autocfg::emit_possibility(&AssertMatchesLocation::Root.to_string());
        autocfg::emit_possibility(&AssertMatchesLocation::Module.to_string());
    }
}

/// Known features have custom implementation
#[allow(non_camel_case_types, reason = "shadowing feature naming")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnstableFeature {
    assert_matches,
    iterator_try_collect,
    never_type,
    proc_macro_diagnostic,
    try_trait_v2,
    try_trait_v2_residual,
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
    pub mod assert_matches {
        pub const AVAILABLE: &str = r#"
#![allow(stable_features)]
#![feature(assert_matches)]
use std::assert_matches;
"#;

        pub const ROOT: &str = r#"
#![allow(stable_features)]
#![feature(assert_matches)]
use std::assert_matches;

#[allow(dead_code)]
fn main() {
    assert_matches!(Some(4), Some(_));
}
"#;

        pub const MODULE: &str = r#"
#![allow(stable_features)]
#![feature(assert_matches)]
use std::assert_matches::assert_matches;

#[allow(dead_code)]
fn main() {
    assert_matches!(Some(4), Some(_));
}
"#;
    }
    pub mod iterator_try_collect {
        // vec! not array: https://internals.rust-lang.org/t/code-compiles-on-playground-but-fails-when-passed-via-stdin-to-rustc/24393
        pub const AVAILABLE: &str = r#"
#![allow(stable_features)]
#![allow(unused)]
#![feature(iterator_try_collect)]
fn try_collect() {
    let _: Option<Vec<_>> = std::iter::Iterator::try_collect(&mut vec![Some(1)].into_iter());
}
"#;
    }
    pub mod never_type {
        pub const AVAILABLE: &str = r#"
#![allow(stable_features)]
#![allow(unused)]
#![feature(never_type)]
type Bang = !;
"#;
    }
    pub mod proc_macro_diagnostic {
        pub const UNSTABLE: &str = r#"
#![deny(stable_features)]
#![allow(unused)]
#![feature(proc_macro_diagnostic)]
extern crate proc_macro;
"#;

        pub const AVAILABLE: &str = r#"
#![allow(stable_features)]
#![allow(unused)]
#![feature(proc_macro_diagnostic)]
extern crate proc_macro;
use proc_macro::Diagnostic;      
"#;
    }

    pub mod try_trait_v2 {
        pub const AVAILABLE: &str = r#"
#![allow(stable_features)]
#![allow(unused)]
#![feature(try_trait_v2)]
use std::ops::Try;
"#;
    }

    pub mod try_trait_v2_residual {
        pub const AVAILABLE: &str = r#"
#![allow(stable_features)]
#![allow(unused)]
#![feature(try_trait_v2_residual)]
use std::ops::Residual;
"#;
    }
}

pub trait Nightly {
    /// Identify whether a an experimental feature flag is available _and_ required on nightly.
    /// Always fails if feature flags are unavailable.
    ///
    /// ## Usage:
    /// - To be used at top-level crate via `#![cfg_attr(unstable_foo, feature(foo))]`
    ///
    /// ## Cfg-gating
    /// Do **not** rely on `cfg(not(unstable_foo))` to suggest that `feature(foo)` is stable! There are 3
    /// reasons that `cfg(unstable_foo)` could be `false`:
    ///   1. The build is using `stable`/`beta`
    ///   2. The feature has been stabilised
    ///   3. The compiler is from before the feature was implemented
    ///
    /// If you need to test that a feature is available in order to cfg-gate your code then use or
    /// create one of the dedicated check functions. See [AutoCfg::assert_matches_location] for an
    /// example.
    fn emit_unstable_feature(&self, feature: &'static str);

    /// Location of assert_matches!() macro. Stabilisation was reverted at last minute
    /// on 2026-04-10, leaving the macro in the new planned location.
    ///
    /// This will give you one of two config flags (never both)
    /// - `assert_matches_in_root`
    /// - `assert_matches_in_module`
    ///
    /// If you need a `has_assert_matches` flag you can construct it easily: If this value is
    /// `Some(location)` then you can guarantee that the `assert_matches!` macro *is* available
    /// for use in the current configuration.
    ///
    /// ### Recommended usage
    /// #### In your build script:
    /// ```no_run
    /// use autocfg::AutoCfg;
    /// use ninja_build_rs::nightly::{AssertMatchesLocation, Nightly};
    ///
    /// let ac = autocfg::new();
    ///
    /// AssertMatchesLocation::emit_possibilities();
    /// if let Some(location) = ac.assert_matches_location() {
    ///     autocfg::emit(&location.to_string())
    /// }
    /// ```
    ///
    /// #### In the main code
    /// ```
    /// #[cfg(assert_matches_in_root)]
    /// use std::assert_matches;
    ///
    /// #[cfg(assert_matches_in_module)]
    /// use std::assert_matches::assert_matches;
    /// ```
    #[deprecated(since = "0.1.1", note = "handled by `emit_unstable_feature`")]
    #[expect(deprecated)]
    fn assert_matches_location(&self) -> Option<AssertMatchesLocation>;
}

fn default_unstable_cfg(ac: &AutoCfg, feature: &'static str) {
    let cfg = format!("unstable_{feature}");
    let code = format!(
        r#"
#![deny(stable_features)]
#![allow(unused)]
#![feature({feature})]
"#
    );
    autocfg::emit_possibility(&cfg);
    if ac.probe_raw(&code).is_ok() {
        autocfg::emit(&cfg);
    }
}

impl Nightly for AutoCfg {
    fn emit_unstable_feature(&self, feature: &'static str) {
        dbg!(&feature);
        match UnstableFeature::from(feature) {
            UnstableFeature::assert_matches => {
                default_unstable_cfg(self, feature);
                autocfg::emit_possibility("has_assert_matches");
                if self.probe_raw(probes::assert_matches::AVAILABLE).is_ok() {
                    autocfg::emit("has_assert_matches");
                }
                autocfg::emit_possibility("assert_matches_location, values(\"root\", \"module\")");
                if self.probe_raw(probes::assert_matches::ROOT).is_ok() {
                    autocfg::emit("assert_matches_location=\"root\"")
                } else if self.probe_raw(probes::assert_matches::MODULE).is_ok() {
                    autocfg::emit("assert_matches_location=\"module\"");
                }
            }
            UnstableFeature::iterator_try_collect => {
                default_unstable_cfg(self, feature);
                autocfg::emit_possibility("has_iterator_try_collect");
                if self
                    .probe_raw(probes::iterator_try_collect::AVAILABLE)
                    .is_ok()
                {
                    autocfg::emit("has_iterator_try_collect");
                }
            }
            UnstableFeature::never_type => {
                default_unstable_cfg(self, feature);
                autocfg::emit_possibility("has_never_type");
                if self.probe_raw(probes::never_type::AVAILABLE).is_ok() {
                    autocfg::emit("has_never_type");
                }
            }
            UnstableFeature::proc_macro_diagnostic => {
                autocfg::emit_possibility("unstable_proc_macro_diagnostic");
                if self
                    .probe_raw(probes::proc_macro_diagnostic::UNSTABLE)
                    .is_ok()
                {
                    autocfg::emit("unstable_proc_macro_diagnostic");
                }
                autocfg::emit_possibility("has_proc_macro_diagnostic");
                if self
                    .probe_raw(probes::proc_macro_diagnostic::AVAILABLE)
                    .is_ok()
                {
                    autocfg::emit("has_proc_macro_diagnostic");
                }
            }
            UnstableFeature::try_trait_v2 => {
                default_unstable_cfg(self, feature);
                autocfg::emit_possibility("has_try_trait_v2");
                if self.probe_raw(probes::try_trait_v2::AVAILABLE).is_ok() {
                    autocfg::emit("has_try_trait_v2");
                }
            }
            UnstableFeature::try_trait_v2_residual => {
                default_unstable_cfg(self, feature);
                autocfg::emit_possibility("has_try_trait_v2_residual");
                if self
                    .probe_raw(probes::try_trait_v2_residual::AVAILABLE)
                    .is_ok()
                {
                    autocfg::emit("has_try_trait_v2_residual");
                }
            }
            UnstableFeature::Other(feature) => default_unstable_cfg(self, feature),
        }
    }

    #[expect(deprecated)]
    fn assert_matches_location(&self) -> Option<AssertMatchesLocation> {
        let in_root = r#"
        #![allow(stable_features)]
        #![feature(assert_matches)]
        use std::assert_matches;

        #[allow(dead_code)]
        fn main() {
            assert_matches!(Some(4), Some(_));
        }
            "#;

        let in_module = r#"
        #![allow(stable_features)]
        #![feature(assert_matches)]
        use std::assert_matches::assert_matches;

        #[allow(dead_code)]
        fn main() {
            assert_matches!(Some(4), Some(_));
        }
            "#;

        if self.probe_raw(in_root).is_ok() {
            Some(AssertMatchesLocation::Root)
        } else if self.probe_raw(in_module).is_ok() {
            Some(AssertMatchesLocation::Module)
        } else {
            None
        }
    }
}

pub fn cargo_unstable() -> Result<bool> {
    Ok(Command::new(get_var("CARGO")?)
        .args(["-Zunstable-options", "help"])
        .status()
        .map_err(|err| BuildError::Other(err.to_string()))?
        .success())
}

/// Return the comma-separated list of `unstable.allow-features` from cargo config
///
/// ## Note
/// - `unstable-options` will NEVER be in this list, as we need to add it to call `cargo config`
/// - pass `None` to use current working directory (you probably always want to do this!)
pub fn cargo_allowed_features<P: AsRef<Path>>(current_dir: Option<P>) -> Result<AllowedFeatures> {
    if !cargo_unstable()? {
        return Ok(AllowedFeatures::None);
    }

    let mut cargo = Command::new(get_var("CARGO")?);
    if let Some(dir) = &current_dir {
        dbg!(&dir.as_ref());
        cargo.current_dir(dir);
    }
    cargo.args(["-Zunstable-options", "config", "get"]);
    let mut output = cargo
        .output()
        .map_err(|err| BuildError::Other(err.to_string()))?;

    if !output.status.success() {
        // Maybe there is a restricted list which doesn't include unstable-options
        let mut cargo = Command::new(get_var("CARGO")?);
        if let Some(dir) = &current_dir {
            dbg!(&dir.as_ref());
            cargo.current_dir(dir);
        }
        cargo.args([
            "-Zunstable-options",
            "--config",
            "unstable.allow-features=[\"unstable-options\"]",
            "config",
            "get",
            "unstable.allow-features",
        ]);
        output = cargo
            .output()
            .map_err(|err| BuildError::Other(err.to_string()))?;

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
        Some(features) => {
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
                .split(", ")
                .filter(|feature| *feature != "unstable-options")
                .map(ToString::to_string)
                .collect();
            if features.is_empty() {
                AllowedFeatures::None
            } else {
                AllowedFeatures::Some(features)
            }
        }
        None => AllowedFeatures::All,
    };
    Ok(allowed)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllowedFeatures {
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
        let allowed = cargo_allowed_features(Some(&tmp));
        if cargo_unstable().expect("cargo_unstable") {
            assert_matches!(allowed, Ok(AllowedFeatures::All))
        } else {
            assert_matches!(allowed, Ok(AllowedFeatures::None))
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
        writeln!(config, "unstable.allow-features = [\"try_trait_v2\"]").expect("added to config");

        let allowed = cargo_allowed_features(Some(&tmp));
        if cargo_unstable().expect("cargo_unstable") {
            assert_matches!(
                allowed,
                Ok(AllowedFeatures::Some(features))
                if features == vec!["try_trait_v2"]
            );
        } else {
            assert_matches!(allowed, Ok(AllowedFeatures::None));
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

        let allowed = cargo_allowed_features(Some(&tmp));
        assert_matches!(allowed, Ok(AllowedFeatures::None));
    }
}
