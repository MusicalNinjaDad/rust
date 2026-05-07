use std::{fs, path::PathBuf};

use dircpy::copy_dir;
use ninja_xtask::{
    Exit,
    commands::{fmt, test},
};
use tempfile::tempdir;

#[test]
fn fmt_fixture() {
    let tmp = tempdir().expect("couldn't create temp dir for test");
    copy_dir("tests/fixture", tmp.path()).expect("couldn't copy fixture");
    let original = fs::read_to_string("tests/fixture/src/lib.rs").unwrap();
    let copied = fs::read_to_string(tmp.path().join("src/lib.rs")).unwrap();
    assert_eq!(original, copied);
    let cmd = fmt(tmp.path());
    let output = cmd.result.expect("`cargo fmt` failed to run");
    assert!(
        output.status.success(),
        "`cargo fmt` exited with status {:?}\nstderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr),
    );
    let formatted = fs::read_to_string(tmp.path().join("src/lib.rs")).unwrap();
    assert_ne!(original, formatted);
    dbg!(tmp.path());
}

#[test]
fn fail_output() {
    let fixture = PathBuf::from("tests/fixture");
    let run_tests = test(&fixture);
    let exit = Exit::from(run_tests);
    let Exit::Error(output) = exit else {
        panic!("test didn't fail")
    };
    assert!(output.contains("====== tests exited with"));
    assert!(output.contains("test printed to stdout"));
    assert!(output.contains("test dbg"));
    assert!(output.contains("left: 5"));
    println!("{output}");
}
