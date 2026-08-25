#![cfg_attr(unstable_never_type, feature(never_type))]
#![cfg_attr(unstable_try_trait_v2, feature(try_trait_v2))]
#![cfg_attr(unstable_try_trait_v2_residual, feature(try_trait_v2_residual))]

pub mod cli;
pub mod commands;
pub mod exit_with_json;

pub use cli::*;
pub use exit_with_json::{Exit, WithJson};
