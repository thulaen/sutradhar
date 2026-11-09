//! The required reviews on real git repositories: the commit and push
//! checks agree, and merges, amends, lost history and replace objects
//! cannot skip them.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::fs;
use std::path::{Path, PathBuf};

use aegis::repo::{NO_COMMIT, Repo};
use aegis::reviews::{REQUIRED, of_pushed, of_staged, staged};

mod common;
use common::{git, repo, save, write};

fn record(skill: &str, change: &str, verdict: &str) -> String {
    format!(
        "skill = \"{skill}\"\nchange = \"{change}\"\nverdict = \"{verdict}\"\n\
         agent = \"claude\"\nsummary = \"Read every changed line twice.\"\n"
    )
}

/// Writes passing records for the staged content, as the skills do.
fn review(dir: &Path) {
    let change = staged(&Repo::at(dir)).unwrap();
    for skill in REQUIRED {
        write(
            dir,
            &format!("reviews/{skill}.toml"),
            &record(skill, &change, "pass"),
        );
    }
    let _ok = git(dir, &["add", "-A"]);
}

fn scratch(name: &str) -> PathBuf {
    let dir = repo("reviews", name);
    let _start = save(
        &dir,
        &[("crates/aegis/src/reviews.rs", "// the check\n")],
        "start",
    );
    dir
}

#[test]
fn commit_and_push_agree_for_plain_commits_amends_and_merges() {
    let dir = scratch("agree");
    let repo = Repo::at(&dir);
    write(&dir, "src/a.rs", "fn a() {}\n");
    let _ok = git(&dir, &["add", "-A"]);
    assert_eq!(of_staged(&repo).unwrap().len(), 8);
    review(&dir);
    assert_eq!(of_staged(&repo).unwrap(), Vec::new());
    let _ok = git(&dir, &["commit", "-qm", "a"]);
    assert_eq!(
        of_pushed(&repo, NO_COMMIT, &git(&dir, &["rev-parse", "HEAD"]))
            .unwrap(),
        Vec::new()
    );
    write(&dir, "src/a.rs", "fn a() { b(); }\n");
    let _ok = git(&dir, &["add", "-A"]);
    review(&dir);
    let _ok = git(&dir, &["commit", "-q", "--amend", "-m", "a again"]);
    assert_eq!(
        of_pushed(&repo, NO_COMMIT, &git(&dir, &["rev-parse", "HEAD"]))
            .unwrap(),
        Vec::new()
    );
    // A side line of work, merged honestly: reviewed, so it passes.
    let _ok = git(&dir, &["checkout", "-q", "-b", "side", "HEAD~1"]);
    write(&dir, "src/side.rs", "fn s() {}\n");
    let _ok = git(&dir, &["add", "src/side.rs"]);
    let _ok = git(&dir, &["commit", "-qm", "side"]);
    let _ok = git(&dir, &["checkout", "-q", "main"]);
    let _ok = git(&dir, &["merge", "-q", "--no-commit", "side"]);
    review(&dir);
    let _ok = git(&dir, &["commit", "-qm", "merge"]);
    let merged = git(&dir, &["rev-parse", "HEAD"]);
    assert_eq!(of_pushed(&repo, NO_COMMIT, &merged).unwrap(), Vec::new());
    // The same merge with a file slipped in after review is refused.
    let _ok = git(&dir, &["reset", "-q", "--hard", "HEAD~1"]);
    let _ok = git(&dir, &["merge", "-q", "--no-commit", "side"]);
    review(&dir);
    write(&dir, "src/extra.rs", "fn unreviewed() {}\n");
    let _ok = git(&dir, &["add", "src/extra.rs"]);
    let _ok = git(&dir, &["commit", "-qm", "merge"]);
    let slipped = git(&dir, &["rev-parse", "HEAD"]);
    assert_eq!(of_pushed(&repo, NO_COMMIT, &slipped).unwrap().len(), 8);
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn deleting_the_check_needs_reviews_too() {
    let dir = scratch("delete");
    let repo = Repo::at(&dir);
    let main = git(&dir, &["rev-parse", "HEAD"]);
    let _ok = git(&dir, &["rm", "-q", "crates/aegis/src/reviews.rs"]);
    assert_eq!(of_staged(&repo).unwrap().len(), 8);
    let _ok = git(&dir, &["commit", "-qm", "gone"]);
    let gone = git(&dir, &["rev-parse", "HEAD"]);
    assert_eq!(of_pushed(&repo, NO_COMMIT, &gone).unwrap().len(), 8);
    // A later commit, pushed onto the main that held the check, needs
    // reviews too, though its own parent no longer holds it.
    let after = save(&dir, &[("src/after.rs", "fn later() {}\n")], "after");
    assert_eq!(of_pushed(&repo, &main, &after).unwrap().len(), 8);
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_merge_onto_an_unrelated_line_cannot_leave_the_check_behind() {
    let dir = scratch("orphan");
    let repo = Repo::at(&dir);
    let base = git(&dir, &["rev-parse", "HEAD"]);
    let _ok = git(&dir, &["checkout", "-q", "--orphan", "other"]);
    let _ok = git(&dir, &["rm", "-rq", "--cached", "."]);
    fs::remove_dir_all(dir.join("crates")).unwrap();
    write(&dir, "payload.rs", "fn p() {}\n");
    let _ok = git(&dir, &["add", "payload.rs"]);
    let _ok = git(&dir, &["commit", "-qm", "root"]);
    let root = git(&dir, &["rev-parse", "HEAD"]);
    let merge = git(
        &dir,
        &[
            "commit-tree",
            "-p",
            &root,
            "-p",
            &base,
            "-m",
            "m",
            &format!("{root}^{{tree}}"),
        ],
    );
    assert_eq!(of_pushed(&repo, &base, &root).unwrap().len(), 8);
    assert_eq!(of_pushed(&repo, &base, &merge).unwrap().len(), 8);
    assert_eq!(of_pushed(&repo, NO_COMMIT, &merge).unwrap().len(), 8);
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_file_only_marked_for_adding_does_not_change_the_fingerprint() {
    let dir = scratch("intent");
    let repo = Repo::at(&dir);
    let before = staged(&Repo::at(&dir)).unwrap();
    write(&dir, "later.rs", "fn l() {}\n");
    let _ok = git(&dir, &["add", "-N", "later.rs"]);
    assert_eq!(staged(&repo).unwrap(), before);
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn commits_made_before_the_check_need_no_reviews() {
    let dir = repo("reviews", "before");
    write(&dir, "a.txt", "a\n");
    let _ok = git(&dir, &["add", "-A"]);
    let _ok = git(&dir, &["commit", "-qm", "old"]);
    write(&dir, "b.txt", "b\n");
    let _ok = git(&dir, &["add", "-A"]);
    let _ok = git(&dir, &["commit", "-qm", "older still"]);
    let repo = Repo::at(&dir);
    let id = git(&dir, &["rev-parse", "HEAD"]);
    assert_eq!(of_pushed(&repo, NO_COMMIT, &id).unwrap(), Vec::new());
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_replaced_commit_does_not_switch_the_reviews_off() {
    let dir = repo("reviews", "replace");
    fs::create_dir_all(dir.join("crates/aegis/src")).unwrap();
    fs::write(dir.join("a.txt"), "a\n").unwrap();
    let _ok = git(&dir, &["add", "-A"]);
    let _ok = git(&dir, &["commit", "-qm", "before the check"]);
    let before = git(&dir, &["rev-parse", "HEAD"]);
    fs::write(dir.join("crates/aegis/src/reviews.rs"), "// check\n").unwrap();
    let _ok = git(&dir, &["add", "-A"]);
    let _ok = git(&dir, &["commit", "-qm", "the check"]);
    let _ok = git(&dir, &["replace", "HEAD", &before]);
    fs::write(dir.join("b.txt"), "b\n").unwrap();
    let _ok = git(&dir, &["add", "b.txt"]);
    assert_eq!(of_staged(&Repo::at(&dir)).unwrap().len(), 8);
    fs::remove_dir_all(&dir).unwrap();
}
