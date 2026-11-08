//! The test-first proof on a real, tiny Cargo workspace.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use aegis::history::commits;
use aegis::red_green::check;
use aegis::repo::Repo;

fn git(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(["-c", "core.hooksPath=/dev/null"])
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap();
    assert!(out.status.success(), "git {args:?}");
    String::from_utf8_lossy(&out.stdout).trim().to_owned()
}

fn save(dir: &Path, files: &[(&str, &str)], message: &str) -> String {
    for (path, text) in files {
        let path = dir.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }
    let _ = git(dir, &["add", "-A"]);
    let _ = git(dir, &["commit", "-q", "-m", message]);
    git(dir, &["rev-parse", "HEAD"])
}

fn workspace(name: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join(format!("aegis-red-green-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let _ = git(&dir, &["init", "-q", "-b", "main"]);
    let _ = git(&dir, &["config", "user.email", "a@example.com"]);
    let _ = git(&dir, &["config", "user.name", "A"]);
    dir
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
