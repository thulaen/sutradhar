use std::fs;
use std::path::PathBuf;

use super::{Outcome, is_test_file, judge, outcome, targets};
use crate::tdd::Tdd;

fn tree(name: &str) -> PathBuf {
    let root = std::env::temp_dir()
        .join(format!("aegis-red-green-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    let pkg = root.join("crates/forge-sync");
    fs::create_dir_all(pkg.join("src")).unwrap();
    fs::create_dir_all(pkg.join("tests")).unwrap();
    fs::write(pkg.join("Cargo.toml"), "[package]\n").unwrap();
    fs::write(pkg.join("src/lib.rs"), "").unwrap();
    fs::write(pkg.join("tests/runs.rs"), "").unwrap();
    root
}

fn strings(args: &[&str]) -> Vec<String> {
    args.iter().map(|s| (*s).to_owned()).collect()
}

#[test]
fn a_path_maps_to_the_library_and_any_matching_test_file() {
    let root = tree("targets");
    let both = targets(&root, "forge_sync", "runs::a_test");
    assert_eq!(
        both,
        [
            strings(&[
                "test",
                "-p",
                "forge-sync",
                "--test",
                "runs",
                "--",
                "--exact",
                "a_test"
            ]),
            strings(&[
                "test",
                "-p",
                "forge-sync",
                "--lib",
                "--",
                "--exact",
                "runs::a_test"
            ]),
        ]
    );
    let lib = targets(&root, "forge_sync", "plan::tests::x");
    assert_eq!(lib.len(), 1);
    assert_eq!(targets(&root, "nothing", "a::b"), Vec::<Vec<String>>::new());
}

#[test]
fn reads_passed_failed_and_absent() {
    let ran = "running 1 test\n.\ntest result: ok. 1 passed; 0 failed;\n";
    let none = "test result: ok. 0 passed; 0 failed; 88 filtered out\n";
    assert_eq!(outcome(true, ran), Outcome::Passed);
    assert_eq!(outcome(true, none), Outcome::Absent);
    assert_eq!(outcome(true, ""), Outcome::Absent);
    assert_eq!(outcome(false, ran), Outcome::Failed);
}

#[test]
fn knows_test_files() {
    for yes in [
        "crates/a/tests/x.rs",
        "crates/a/src/x_tests.rs",
        "tests/a.rs",
    ] {
        assert!(is_test_file(yes), "{yes}");
    }
    for no in ["crates/a/src/x.rs", "crates/a/src/tests.md", "docs/a.md"] {
        assert!(!is_test_file(no), "{no}");
    }
}

#[test]
fn a_test_must_be_red_before_and_green_after() {
    let test = Tdd::Test {
        krate: "a".to_owned(),
        path: "b".to_owned(),
    };
    assert_eq!(
        judge(&test, Outcome::Failed, Outcome::Passed),
        Vec::<_>::new()
    );
    assert_eq!(
        judge(&test, Outcome::Absent, Outcome::Passed),
        Vec::<_>::new()
    );
    assert_eq!(judge(&test, Outcome::Passed, Outcome::Passed).len(), 1);
    assert_eq!(judge(&test, Outcome::Failed, Outcome::Failed).len(), 1);
    assert_eq!(judge(&test, Outcome::Failed, Outcome::Absent).len(), 1);
    assert_eq!(
        judge(&Tdd::DocsOnly, Outcome::Passed, Outcome::Absent),
        Vec::<_>::new()
    );
}
