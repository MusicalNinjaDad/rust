use std::path::Path;

use clap::{Parser, Subcommand};
use clap_cargo::style::CLAP_STYLING as CARGO_STYLING;
use ninja_xtask::{
    Exit,
    commands::{build, clippy, clippy_tests, fmt, git_add, test},
};

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
#[command(styles = CARGO_STYLING)]
enum CargoCmd {
    #[command(subcommand)]
    Ninja(Command),
}

#[derive(Subcommand)]
#[command(version)]
enum Command {
    /// fmt, lint & test then stage everything in git if all is good
    Stage,
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

fn main() -> Exit<()> {
    let CargoCmd::Ninja(xtask) = CargoCmd::try_parse()?;
    let root = Path::new(".");

    match &xtask {
        Command::Stage => {
            let fmt = fmt(root);
            Exit::from(fmt)?;
            let clippy = clippy(root);
            let clippy_tests = clippy_tests(root);
            let tests = test(root);
            let checks = vec![clippy, clippy_tests, tests];
            Exit::from(checks)?;
            let git = git_add(root);
            Exit::from(git)
        }
        Command::Build { glibc, release, target } => {
            let build = build(root, target, glibc, release);
            Exit::from(build)
        }
    }
}
