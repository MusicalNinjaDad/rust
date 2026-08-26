use std::fmt::Debug;

use bitflags::bitflags;
use clap::{Parser, Subcommand};
use clap_cargo::style::CLAP_STYLING as CARGO_STYLING;

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
    /// 
    /// To provide any specfic environment variables for the executed cargo commands
    /// create a *workspace* metadata section in Cargo.toml (if this is a single crate, you
    /// can add the section to the crate Cargo.toml without a problem, if it's a workspace
    /// add it to the workspace Cargo.toml). For example:
    /// 
    /// ```toml
    /// [workspace.metadata.ninja-xtask]
    /// env = {
    ///     "LD_LIBRARY_PATH" = false,  # unset variable, do not inherit
    ///     "RUSTFLAGS" = "flag"        # set variable, overwriting inherited value
    /// }
    /// ```
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
