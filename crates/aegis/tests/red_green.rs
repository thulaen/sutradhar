//! The test-first proof on a real, tiny Cargo workspace.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::fs;
use std::path::PathBuf;

use aegis::history::commits;
use aegis::red_green::check;
use aegis::repo::Repo;

mod common;
use common::{repo, save};

fn workspace(name: &str) -> PathBuf {
    repo("red-green", name)
}

fn message(test: &str) -> String {
    format!("fix(demo): x\n\nTDD: {test}\nDefects: none\nTask: #1\n")
}

#[test]
fn a_named_test_must_fail_before_and_pass_after() {
    let dir = workspace("real");
    let base = save(
        &dir,
        &[
            ("Cargo.toml", "[workspace]\nmembers = [\"crates/*\"]\n"),
            (
                "Cargo.lock",
                "version = 4\n\n[[package]]\nname = \"demo\"\n\
                 version = \"0.1.0\"\n",
            ),
            (
                "crates/demo/Cargo.toml",
                "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\
                 edition = \"2024\"\n",
            ),
            (
                "crates/demo/src/lib.rs",
                "pub fn add(a: i32, b: i32) -> i32 { a - b }\n",
            ),
        ],
        "feat(demo): start",
    );
    let fixed = save(
        &dir,
        &[
            (
                "crates/demo/src/lib.rs",
                "pub fn add(a: i32, b: i32) -> i32 { a + b }\n",
            ),
            (
                "crates/demo/tests/sum.rs",
                "#[test]\nfn adds() { assert_eq!(demo::add(1, 2), 3); }\n",
            ),
        ],
        &message("demo::sum::adds"),
    );
    let repo = Repo::at(&dir);
    let red_then_green = commits(&repo, &base, &fixed).unwrap();
    assert_eq!(check(&repo, &red_then_green, &["cargo"]), Vec::new());

    let late = save(
        &dir,
        &[(
            "crates/demo/tests/sum.rs",
            "#[test]\nfn adds() { assert_eq!(demo::add(1, 2), 3); }\n\
             #[test]\nfn still() { assert_eq!(demo::add(2, 2), 4); }\n",
        )],
        &message("demo::sum::still"),
    );
    let already_green = commits(&repo, &fixed, &late).unwrap();
    let found = check(&repo, &already_green, &["cargo"]);
    assert_eq!(found.len(), 1);
    assert!(found.iter().all(|f| f.message.contains("already passed")));

    let missing = commits(&repo, &base, &fixed).unwrap();
    let mut wrong = missing.clone();
    if let Some(first) = wrong.first_mut() {
        first.message = message("demo::sum::no_such_test");
    }
    let found = check(&repo, &wrong, &["cargo"]);
    assert!(
        found
            .iter()
            .any(|f| f.message.contains("must run and pass"))
    );
}

#[test]
fn a_link_in_the_old_tree_cannot_send_a_write_outside() {
    let dir = workspace("link");
    let outside = dir.with_extension("outside");
    let _ = fs::remove_dir_all(&outside);
    fs::create_dir_all(&outside).unwrap();
    fs::create_dir_all(dir.join("crates/demo")).unwrap();
    std::os::unix::fs::symlink(&outside, dir.join("crates/demo/tests"))
        .unwrap();
    let base = save(&dir, &[("Cargo.toml", "[workspace]\n")], "feat(demo): a");
    fs::remove_file(dir.join("crates/demo/tests")).unwrap();
    let head = save(
        &dir,
        &[("crates/demo/tests/x.rs", "#[test]\nfn x() {}\n")],
        &message("demo::x::x"),
    );
    let repo = Repo::at(&dir);
    let found =
        check(&repo, &commits(&repo, &base, &head).unwrap(), &["cargo"]);
    assert!(
        found.iter().any(|f| f.message.contains("link")),
        "{found:?}"
    );
    assert_eq!(fs::read_dir(&outside).unwrap().count(), 0);
}

#[test]
fn a_commit_that_adds_a_crate_with_tests_can_still_prove_test_first() {
    let dir = workspace("new-crate");
    let base = save(
        &dir,
        &[
            ("Cargo.toml", "[workspace]\nmembers = [\"crates/*\"]\n"),
            (
                "Cargo.lock",
                "version = 4\n\n[[package]]\nname = \"demo\"\n\
                 version = \"0.1.0\"\n",
            ),
            (
                "crates/demo/Cargo.toml",
                "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\
                 edition = \"2024\"\n",
            ),
            (
                "crates/demo/src/lib.rs",
                "pub fn add(a: i32, b: i32) -> i32 { a - b }\n",
            ),
        ],
        "feat(demo): start",
    );
    let fixed = save(
        &dir,
        &[
            (
                "crates/demo/src/lib.rs",
                "pub fn add(a: i32, b: i32) -> i32 { a + b }\n",
            ),
            (
                "crates/demo/tests/sum.rs",
                "#[test]\nfn adds() { assert_eq!(demo::add(1, 2), 3); }\n",
            ),
            (
                "crates/extra/Cargo.toml",
                "[package]\nname = \"extra\"\nversion = \"0.1.0\"\n\
                 edition = \"2024\"\n",
            ),
            ("crates/extra/src/lib.rs", "//! x\n"),
            ("crates/extra/tests/x.rs", "#[test]\nfn x() {}\n"),
            (
                "Cargo.lock",
                "version = 4\n\n[[package]]\nname = \"demo\"\n\
                 version = \"0.1.0\"\n\n[[package]]\nname = \"extra\"\n\
                 version = \"0.1.0\"\n",
            ),
        ],
        &message("demo::sum::adds"),
    );
    let repo = Repo::at(&dir);
    let found =
        check(&repo, &commits(&repo, &base, &fixed).unwrap(), &["cargo"]);
    assert_eq!(found, Vec::new());
    fs::remove_dir_all(&dir).unwrap();
}
