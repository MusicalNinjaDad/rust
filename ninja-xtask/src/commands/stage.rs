use std::{
    path::Path,
    process::{Command, Stdio},
};

use super::{Cmd, CmdExt as _, Spawned, SpawnedExt as _};
use crate::CheckFlags;

pub fn fmt(root: &Path, flags: CheckFlags) -> Cmd {
    Command::new("cargo")
        .current_dir(root)
        .arg("fmt")
        .output()
        .into_cmd("fmt", Some(flags))
}

pub fn git_add(root: &Path, flags: CheckFlags) -> Cmd {
    Command::new("git")
        .current_dir(root)
        .arg("add")
        .arg(".")
        .output()
        .into_cmd("git add", Some(flags))
}

pub fn clippy(root: &Path, flags: CheckFlags) -> Spawned {
    let mut clippy = Command::new("cargo");
    clippy
        .current_dir(root)
        .arg("clippy")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped());
    if flags.contains(CheckFlags::JSON) {
        clippy.arg("--message-format=json");
    }
    if flags.contains(CheckFlags::STRICT) {
        clippy.args(["--", "--deny", "warnings"]);
    }
    clippy.spawn().into_spawned("clippy", Some(flags))
}

pub fn clippy_tests(root: &Path, flags: CheckFlags) -> Spawned {
    let mut clippy = Command::new("cargo");
    clippy
        .current_dir(root)
        .arg("clippy")
        .arg("--tests")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped());
    if flags.contains(CheckFlags::JSON) {
        clippy.arg("--message-format=json");
    }
    if flags.contains(CheckFlags::STRICT) {
        clippy.args(["--", "--deny", "warnings"]);
    }
    clippy.spawn().into_spawned("clippy the tests", Some(flags))
}

pub fn test(root: &Path, flags: CheckFlags) -> Spawned {
    let mut testlib = Command::new("cargo");
    testlib
        .current_dir(root)
        .arg("test")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped());
    if flags.contains(CheckFlags::JSON) {
        testlib.args([
            "--message-format=json",
            "--",
            "-Zunstable-options",
            "--format",
            "json",
        ]);
    }
    testlib.spawn().into_spawned("tests", Some(flags))
}

pub fn test_examples(root: &Path, flags: CheckFlags) -> Spawned {
    let mut testlib = Command::new("cargo");
    testlib
        .current_dir(root)
        .arg("test")
        .arg("--examples")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped());
    if flags.contains(CheckFlags::JSON) {
        testlib.args([
            "--message-format=json",
            "--",
            "-Zunstable-options",
            "--format",
            "json",
        ]);
    }
    testlib.spawn().into_spawned("test examples", Some(flags))
}
