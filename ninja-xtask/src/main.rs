use std::path::Path;

use clap::Parser;
use ninja_xtask::{
    CargoCmd, CheckFlags, Exit, NinjaCommand,
    commands::{build, clippy, clippy_tests, fmt, git_add, test, test_examples},
};

fn main() -> Exit<()> {
    let CargoCmd::Ninja(xtask) = CargoCmd::try_parse()?;
    let root = Path::new(".");
    let checkflags = CheckFlags::from(&xtask);

    match &xtask {
        NinjaCommand::Stage { .. } => {
            let fmt = fmt(root);
            Exit::from(fmt)?;

            let checks = [
                clippy(root, checkflags),
                clippy_tests(root, checkflags),
                test(root, checkflags),
                test_examples(root, checkflags),
            ];
            Exit::from_iter(checks)?;

            let git = git_add(root);
            Exit::from(git)
        }
        NinjaCommand::Build {
            glibc,
            release,
            target,
        } => {
            let build = build(root, target, glibc, release);
            Exit::from(build)
        }
    }
}
