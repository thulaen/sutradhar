//! End-to-end tests of the commit and push tiers on scratch repositories.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use aegis::repo::{NO_COMMIT, Repo};
use aegis::rule_file::POINTER;
use aegis::tiers;

const GOOD_MSG: &str = "docs: start\n\nTDD: none (docs only)\nDefects: none\n\
    Task: bootstrap\nSigned-off-by: A <a@example.com>\n";

fn git(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .env_remove("GIT_DIR")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_WORK_TREE")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .unwrap();
    assert!(out.status.success(), "git {args:?}: {out:?}");
    String::from_utf8(out.stdout).unwrap().trim().to_owned()
}

/// A scratch repository holding the two pointer files and a README.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join(format!("aegis-tiers-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let _ = git(&dir, &["init", "-q", "-b", "main"]);
    let _ = git(&dir, &["config", "user.email", "a@example.com"]);
    let _ = git(&dir, &["config", "user.name", "A"]);
    fs::write(dir.join("CLAUDE.md"), POINTER).unwrap();
    fs::write(dir.join("GEMINI.md"), POINTER).unwrap();
    fs::write(dir.join("README.md"), "# Read me\n").unwrap();
    fs::write(dir.join("AGENTS.md"), "# Rules\n").unwrap();
    let _ = git(&dir, &["add", "-A"]);
    dir
}

fn checks(found: &[aegis::Finding]) -> Vec<&'static str> {
    found.iter().map(|f| f.check).collect()
}

#[test]
fn commit_tier_passes_a_clean_repo() {
    let dir = scratch("clean");
    let found = tiers::commit(&Repo::at(&dir)).unwrap();
    assert_eq!(checks(&found).len(), 0, "{found:?}");
}

#[test]
fn commit_tier_refuses_planted_faults() {
    let dir = scratch("faults");
    fs::write(dir.join("a.py"), "x = 1\n").unwrap();
    fs::write(dir.join("b.md"), "This is in progress.\n").unwrap();
    fs::write(dir.join("c.rs"), "\n".repeat(301)).unwrap();
    let key = ["-----BEGIN", " ", "PRIVATE KEY-----\n"].concat();
    fs::write(dir.join("d.md"), key).unwrap();
    fs::write(dir.join("GEMINI.md"), "Also do X.\n").unwrap();
    let _ = git(&dir, &["add", "-A"]);
    let found = tiers::commit(&Repo::at(&dir)).unwrap();
    let mut got = checks(&found);
    got.sort_unstable();
    assert_eq!(
        got,
        [
            "docs.one-rule-file",
            "docs.status-words",
            "files.allowed-kind",
            "limits.file-length",
            "secrets.private-data",
        ],
        "{found:?}"
    );
}

#[test]
fn push_tier_checks_every_commit_message() {
    let dir = scratch("push");
    let _ = git(&dir, &["commit", "-q", "-m", GOOD_MSG]);
    let first = git(&dir, &["rev-parse", "HEAD"]);
    fs::write(dir.join("README.md"), "# Read me\n\nMore.\n").unwrap();
    let _ = git(&dir, &["commit", "-q", "-am", "fixed stuff"]);
    let head = git(&dir, &["rev-parse", "HEAD"]);
    let repo = Repo::at(&dir);
    let all = tiers::push(&repo, NO_COMMIT, &head).unwrap();
    assert!(
        checks(&all).iter().all(|c| *c == "commit.message"),
        "{all:?}"
    );
    assert_eq!(all.len(), 5, "{all:?}");
    let first_only = tiers::push(&repo, NO_COMMIT, &first).unwrap();
    assert_eq!(first_only.len(), 0, "{first_only:?}");
}

#[test]
fn git_errors_are_reported() {
    let dir = std::env::temp_dir()
        .join(format!("aegis-tiers-none-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    assert!(tiers::commit(&Repo::at(&dir)).is_err());
    assert!(tiers::push(&Repo::at(&dir), "a", "b").is_err());
}

#[test]
fn typescript_switches_on_with_its_compiler() {
    let dir = scratch("ts");
    let repo = Repo::at(&dir);
    assert!(!tiers::typescript_on(&repo));
    fs::create_dir_all(dir.join("gate")).unwrap();
    fs::write(dir.join("gate/tools.toml"), "[[tool]]\nname = \"tsgo\"\n")
        .unwrap();
    assert!(tiers::typescript_on(&repo));
}

#[test]
fn rust_checks_run_only_with_a_workspace() {
    let dir = scratch("rust");
    let repo = Repo::at(&dir);
    assert_eq!(tiers::rust_checks(&repo).len(), 0);
    fs::write(dir.join("Cargo.toml"), "[workspace]\n").unwrap();
    let missing = tiers::rust_checks_using(&repo, &["true"]);
    assert_eq!(
        missing.iter().map(|f| f.check).collect::<Vec<_>>(),
        ["rust.deny"]
    );
    fs::write(dir.join("deny.toml"), "").unwrap();
    assert_eq!(tiers::rust_checks_using(&repo, &["true"]).len(), 0);
    let failed = tiers::rust_checks_using(&repo, &["false"]);
    let got: Vec<&str> = failed.iter().map(|f| f.check).collect();
    assert_eq!(
        got,
        ["rust.format", "rust.lints", "rust.tests", "rust.deny"]
    );
}

#[test]
fn heavy_checks_wait_in_the_queue_when_one_is_set() {
    assert_eq!(tiers::cargo_command(None), ["cargo"]);
    assert_eq!(tiers::cargo_command(Some("")), ["cargo"]);
    assert_eq!(
        tiers::cargo_command(Some("/q/pankti")),
        ["/q/pankti", "run", "--sealed", "--", "cargo"]
    );
    let dir = scratch("queued");
    fs::write(dir.join("Cargo.toml"), "[workspace]\n").unwrap();
    fs::write(dir.join("deny.toml"), "").unwrap();
    let repo = Repo::at(&dir);
    assert_eq!(
        tiers::rust_checks_using(&repo, &["sh", "-c", "true"]).len(),
        0
    );
    assert_eq!(tiers::rust_checks_using(&repo, &[]).len(), 1);
}

#[test]
fn commit_tier_reads_the_staged_copy_not_the_folder() {
    let dir = scratch("links");
    fs::create_dir_all(dir.join("docs")).unwrap();
    fs::write(dir.join("docs/x.md"), "[up](../README.md) [d](../docs)\n")
        .unwrap();
    fs::write(dir.join("run.sh"), "#!/bin/sh\necho \"ok\"\n").unwrap();
    let _ = git(&dir, &["add", "-A"]);
    fs::remove_file(dir.join("run.sh")).unwrap();
    let found = tiers::commit(&Repo::at(&dir)).unwrap();
    assert_eq!(found.len(), 0, "{found:?}");
}

#[test]
fn kept_scripts_and_broken_links_are_checked() {
    let dir = scratch("broken");
    fs::write(dir.join("run.sh"), "#!/bin/sh\necho \"ok\"\n").unwrap();
    fs::write(dir.join("x.md"), "[gone](gone.md)\n").unwrap();
    let _ = git(&dir, &["add", "-A"]);
    let found = tiers::commit(&Repo::at(&dir)).unwrap();
    let got: Vec<&str> = found.iter().map(|f| f.check).collect();
    assert_eq!(got, ["docs.links"], "{found:?}");
}

#[test]
fn push_tier_checks_only_new_commits_and_needs_a_clean_tree() {
    let dir = scratch("range");
    let _ = git(&dir, &["commit", "-q", "-m", "bad first message"]);
    let first = git(&dir, &["rev-parse", "HEAD"]);
    fs::write(dir.join("README.md"), "# Read me\n\nMore.\n").unwrap();
    let _ = git(&dir, &["commit", "-q", "-am", GOOD_MSG]);
    let head = git(&dir, &["rev-parse", "HEAD"]);
    let repo = Repo::at(&dir);
    let found = tiers::push(&repo, &first, &head).unwrap();
    assert_eq!(found.len(), 0, "{found:?}");
    fs::write(dir.join("README.md"), "# Changed\n").unwrap();
    let dirty = tiers::push(&repo, &first, &head).unwrap();
    assert_eq!(checks(&dirty), ["push.clean-tree"]);
}

#[test]
fn push_tier_finds_secrets_in_history_and_messages() {
    let dir = scratch("history");
    let key = ["-----BEGIN", " ", "PRIVATE KEY-----\n"].concat();
    fs::write(dir.join("k.md"), &key).unwrap();
    let _ = git(&dir, &["add", "-A"]);
    let _ = git(&dir, &["commit", "-q", "-m", GOOD_MSG]);
    fs::remove_file(dir.join("k.md")).unwrap();
    let message = format!("{GOOD_MSG}\n{}", ["/srv", "/data/x"].concat());
    let _ = git(&dir, &["commit", "-q", "-am", &message]);
    let head = git(&dir, &["rev-parse", "HEAD"]);
    let found = tiers::push(&Repo::at(&dir), NO_COMMIT, &head).unwrap();
    let paths: Vec<&str> = found.iter().map(|f| f.path.as_str()).collect();
    assert!(paths.contains(&"k.md (history)"), "{found:?}");
    assert!(paths.contains(&"(commit message)"), "{found:?}");
}

#[test]
fn an_untracked_file_makes_the_tree_unclean() {
    let dir = scratch("untracked");
    let _ = git(&dir, &["commit", "-q", "-m", GOOD_MSG]);
    let head = git(&dir, &["rev-parse", "HEAD"]);
    fs::write(dir.join("new.md"), "# New\n").unwrap();
    let found = tiers::push(&Repo::at(&dir), NO_COMMIT, &head).unwrap();
    assert_eq!(checks(&found), ["push.clean-tree"]);
}

#[test]
fn push_tier_scans_changes_made_in_merges() {
    let dir = scratch("merge");
    let _ = git(&dir, &["commit", "-q", "-m", GOOD_MSG]);
    let base = git(&dir, &["rev-parse", "HEAD"]);
    let _ = git(&dir, &["checkout", "-q", "-b", "side"]);
    fs::write(dir.join("side.md"), "# Side\n").unwrap();
    let _ = git(&dir, &["add", "-A"]);
    let _ = git(&dir, &["commit", "-q", "-m", GOOD_MSG]);
    let _ = git(&dir, &["checkout", "-q", "main"]);
    let _ = git(&dir, &["merge", "-q", "--no-ff", "--no-commit", "side"]);
    let key = ["-----BEGIN", " ", "PRIVATE KEY-----\n"].concat();
    fs::write(dir.join("m.md"), key).unwrap();
    let _ = git(&dir, &["add", "-A"]);
    let _ = git(&dir, &["commit", "-q", "-m", GOOD_MSG]);
    fs::remove_file(dir.join("m.md")).unwrap();
    let _ = git(&dir, &["commit", "-q", "-am", GOOD_MSG]);
    let head = git(&dir, &["rev-parse", "HEAD"]);
    let found = tiers::push(&Repo::at(&dir), &base, &head).unwrap();
    let paths: Vec<&str> = found.iter().map(|f| f.path.as_str()).collect();
    assert!(paths.contains(&"m.md (history)"), "{found:?}");
}

#[test]
fn file_checks_refuse_hidden_characters_and_text_that_is_not_utf8() {
    let dir = scratch("text");
    fs::write(dir.join("a.rs"), "fn f() {} // a\u{202e}b\n").unwrap();
    fs::write(dir.join("b.rs"), b"// caf\xe9\n").unwrap();
    fs::write(dir.join("c.rs"), "fn g() {}\n").unwrap();
    let repo = Repo::at(&dir);
    let paths = ["a.rs", "b.rs", "c.rs"].map(str::to_owned);
    let found = tiers::file_checks(&repo, &paths);
    let text: Vec<&str> = found
        .iter()
        .filter(|f| f.check == "text.clean")
        .map(|f| f.path.as_str())
        .collect();
    assert_eq!(text, ["a.rs", "b.rs"]);
}

#[test]
fn no_rust_step_runs_on_a_tree_with_cargo_settings() {
    let dir = scratch("cargo-settings");
    fs::write(dir.join("Cargo.toml"), "[workspace]\n").unwrap();
    fs::create_dir_all(dir.join("crates/a/.cargo")).unwrap();
    fs::write(dir.join("crates/a/.cargo/config.toml"), "").unwrap();
    let found = tiers::rust_checks(&Repo::at(&dir));
    let names: Vec<&str> = found.iter().map(|f| f.check).collect();
    assert_eq!(names, ["rust.cargo-settings"]);
    fs::remove_dir_all(dir.join("crates/a/.cargo")).unwrap();
    fs::create_dir_all(dir.join("target/.cargo")).unwrap();
    assert!(!aegis::file_kinds::has_cargo_settings(&dir));
}

#[test]
fn the_top_folder_is_always_a_full_path() {
    let repo = Repo::at(".");
    assert!(repo.root().is_absolute());
    assert!(Repo::for_hook(".").root().is_absolute());
}
