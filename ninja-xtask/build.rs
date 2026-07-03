use ninja_build_rs::prelude::*;

fn main() -> Result<()> {
    let allowed_features = cargo_allowed_features()?;
    let ac = autocfg::new();

    ac.emit_unstable_feature(never_type, &allowed_features);
    ac.emit_unstable_feature(try_trait_v2, &allowed_features);
    ac.emit_unstable_feature(try_trait_v2_residual, &allowed_features);

    Ok(())
}
