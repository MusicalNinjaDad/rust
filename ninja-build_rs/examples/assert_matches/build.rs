use autocfg::AutoCfg;
use ninja_build_rs::prelude::*;

fn main() -> Result<()> {
    let ac = AutoCfg::new()?;
    let allowed_features = cargo_allowed_features()?;
    ac.emit_unstable_feature(assert_matches, &allowed_features);
    Ok(())
}
