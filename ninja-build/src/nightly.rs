use std::fmt::Display;

use autocfg::AutoCfg;

/// Location of assert_matches!() macro. Stabilisation was reverted at last minute
/// on 2026-04-10, leaving the macro in the new planned location.
///
/// See [AutoCfg::assert_matches_location] for more details
pub enum AssertMatchesLocation {
    /// Macro is at `std::assert_matches`
    Root,
    /// Macro is at `std::assert_matches::assert_matches`
    Module,
}

impl Display for AssertMatchesLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssertMatchesLocation::Root => write!(f, "assert_matches_in_root"),
            AssertMatchesLocation::Module => write!(f, "assert_matches_in_module"),
        }
    }
}

impl AssertMatchesLocation {
    /// See [AutoCfg::assert_matches_location] for more details
    pub fn emit_possibilities() {
        autocfg::emit_possibility(&AssertMatchesLocation::Root.to_string());
        autocfg::emit_possibility(&AssertMatchesLocation::Module.to_string());
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
    /// use ninja_build::nightly::{AssertMatchesLocation, Nightly};
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
    fn assert_matches_location(&self) -> Option<AssertMatchesLocation>;
}

impl Nightly for AutoCfg {
    fn emit_unstable_feature(&self, feature: &'static str) {
        let cfg = format!("unstable_{feature}");
        // #![allow(unused)] is required to avoid this failing for `cargo clippy -- -D warnings`
        let code = format!(
            r#"
        #![deny(stable_features)]
        #![allow(unused)]
        #![feature({feature})]
        "#
        );
        autocfg::emit_possibility(&cfg);
        if self.probe_raw(&code).is_ok() {
            autocfg::emit(&cfg);
        }
    }

    fn assert_matches_location(&self) -> Option<AssertMatchesLocation> {
        let in_root = r#"
        #![allow(stable_features)]
        #![feature(assert_matches)]
        use std::assert_matches;

        fn main() {
            assert_matches!(Some(4), Some(_));
        }
            "#;

        let in_module = r#"
        #![allow(stable_features)]
        #![feature(assert_matches)]
        use std::assert_matches::assert_matches;

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
