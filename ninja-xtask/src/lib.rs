#![cfg_attr(unstable_never_type, feature(never_type))]
#![cfg_attr(unstable_try_trait_v2, feature(try_trait_v2))]
#![cfg_attr(unstable_try_trait_v2_residual, feature(try_trait_v2_residual))]

use std::{
    fmt::Debug,
    io,
    process::{Child, Output},
};

use bitflags::bitflags;
use clap::{Parser, Subcommand};
use clap_cargo::style::CLAP_STYLING as CARGO_STYLING;
use serde_json::{Value, json};

pub mod commands;
pub mod exit_with_json;

pub use exit_with_json::{Exit, WithJson};

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
#[command(styles = CARGO_STYLING)]
pub enum CargoCmd {
    #[command(subcommand)]
    Ninja(NinjaCommand),
}

#[derive(Subcommand)]
#[command(version)]
pub enum NinjaCommand {
    /// fmt, lint & test then stage everything in git if all is good
    Stage {
        /// add --deny warnings to clippy invocations
        #[arg(long)]
        strict: bool,
        /// output in json format
        #[arg(long)]
        json: bool,
    },
    /// build (optionally with zigbuild for a given glibc version)
    Build {
        /// build for a specific glibc version (WSL-Ubuntu is 2.35)
        #[arg(short, long)]
        glibc: Option<String>,
        /// build a release build (default is cargo's default profile, usually debug)
        #[arg(short, long)]
        release: bool,
        /// build for a given target
        #[arg(long)]
        target: Option<String>,
    },
}

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CheckFlags: u32 {
        const STRICT = 0b00000001;
        const JSON = 0b00000010;
    }
}

impl From<&NinjaCommand> for CheckFlags {
    fn from(xtask: &NinjaCommand) -> Self {
        match xtask {
            NinjaCommand::Stage { strict, json } => {
                let mut flags = Self::default();
                flags.set(Self::STRICT, *strict);
                flags.set(Self::JSON, *json);
                flags
            }
            NinjaCommand::Build { .. } => CheckFlags::default(),
        }
    }
}


