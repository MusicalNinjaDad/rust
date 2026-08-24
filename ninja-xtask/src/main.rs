use std::path::Path;

use clap::Parser;
use ninja_xtask::{
    CargoCmd, CheckFlags, Exit, NinjaCommand, WithJson,
    commands::{build, clippy, clippy_tests, fmt, git_add, test, test_examples},
};

fn main() -> Exit<WithJson<()>> {
    let CargoCmd::Ninja(xtask) = CargoCmd::try_parse()?;
    let root = Path::new(".");
    let checkflags = CheckFlags::from(&xtask);

    match &xtask {
        NinjaCommand::Stage { .. } => {
            let fmt = fmt(root, checkflags);
            let fmt = Exit::from(fmt)?;

            let clippy = [clippy(root, checkflags), clippy_tests(root, checkflags)];
            let clippy_result = Exit::from_iter(clippy);

            // Cannot run in parallel with clippy as --json leads to deadlock on build dir
            let tests = [test(root, checkflags), test_examples(root, checkflags)];
            let test_result = Exit::from_iter(tests);

            // But we do want to collect all the results before returning or staging
            let checks = Exit::from_iter([Exit::Ok(fmt), clippy_result, test_result])?;

            let git = git_add(root, checkflags);
            Exit::from_iter([Exit::Ok(checks), Exit::from(git)])
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
