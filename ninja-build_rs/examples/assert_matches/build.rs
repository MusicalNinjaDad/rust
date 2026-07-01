use autocfg::AutoCfg;
use ninja_build_rs::{
    Result,
    nightly::{Nightly, cargo_allowed_features},
};

fn main() -> Result<()> {
    let ac = AutoCfg::new()?;
    let allowed_features = cargo_allowed_features()?;
    ac.emit_unstable_feature("assert_matches", &allowed_features);
    Ok(())
}
