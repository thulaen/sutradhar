//! The door's rules, tested on scratch repositories.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

mod common;

use std::process::Command;

use aegis::repo::NO_COMMIT;
use aegis_door::gate::{ACCEPTED, Door, Push};
use common::{GOOD, commit, door_work, folder, git, main_push, pack, setup};

#[test]
fn a_good_push_passes_and_is_remembered() {
    let (source, door, first) = setup("good");
    let next = commit(&source, "b.md", "# B\n", GOOD);
    let body = pack(&source, &first, &next);
    let found = door.check(main_push(&first, &next), &body).unwrap();
    assert_eq!(found.len(), 0, "{found:?}");
    assert_eq!(git(&door_work("good"), &["rev-parse", ACCEPTED]), next);
}

#[test]
fn a_first_push_checks_every_commit() {
    let (source, _door, first) = setup("first");
    let empty = folder("first-empty");
    let door = Door::new(&empty);
    let body = pack(&source, NO_COMMIT, &first);
    let push = main_push(NO_COMMIT, &first);
    // The setup's docs-only first commit also holds the list of allowed
    // numbers, which is not a doc; that one finding is set aside.
    let found: Vec<_> = door
        .check(push, &body)
        .unwrap()
        .into_iter()
        .filter(|f| !(f.check == "commit.tdd" && f.path == "gate/metrics.toml"))
        .collect();
    assert_eq!(found.len(), 0, "{found:?}");
}

#[test]
fn a_bad_message_is_refused_and_not_remembered() {
    let (source, door, first) = setup("bad");
    let next = commit(&source, "b.md", "# B\n", "stuff");
    let body = pack(&source, &first, &next);
    let found = door.check(main_push(&first, &next), &body).unwrap();
    assert!(
        found.iter().all(|f| f.check == "commit.message"),
        "{found:?}"
    );
    assert_eq!(found.len(), 5);
    let work = door_work("bad");
    let remembered = Command::new("git")
        .arg("-C")
        .arg(work)
        .args(["rev-parse", "--verify", "-q", ACCEPTED])
        .output()
        .unwrap();
    assert_eq!(remembered.stdout.len(), 0);
}

#[test]
fn rewriting_deleting_and_other_refs_are_refused() {
    let (source, door, first) = setup("rewrite");
    let _ = git(&source, &["checkout", "-q", "--orphan", "other"]);
    let other = commit(&source, "c.md", "# C\n", GOOD);
    let body = pack(&source, &first, &other);
    let found = door.check(main_push(&first, &other), &body).unwrap();
    assert_eq!(found[0].check, "door.history");
    let gone = door.check(main_push(&first, NO_COMMIT), &[]).unwrap();
    assert_eq!(gone[0].check, "door.delete");
    let side = Push {
        name: "refs/heads/side",
        ..main_push(&first, &other)
    };
    assert_eq!(door.check(side, &[]).unwrap()[0].check, "door.ref");
}

#[test]
fn release_tags_must_be_new_and_on_what_the_door_accepted() {
    let (source, door, first) = setup("tags");
    let _ = git(&source, &["tag", "-a", "v1.0.0", "-m", "v1.0.0"]);
    let tag = git(&source, &["rev-parse", "v1.0.0"]);
    let body = pack(&source, &first, &tag);
    let new_tag = Push {
        name: "refs/tags/v1.0.0",
        old: NO_COMMIT,
        new: &tag,
        main: &first,
    };
    // A door that has accepted no main yet releases nothing.
    assert_eq!(door.check(new_tag, &body).unwrap()[0].check, "door.tag");
    let _ok = git(&door_work("tags"), &["update-ref", ACCEPTED, &first]);
    assert_eq!(door.check(new_tag, &body).unwrap().len(), 0);
    let replaced = Push {
        old: &tag,
        ..new_tag
    };
    assert_eq!(door.check(replaced, &[]).unwrap()[0].check, "door.tag");
    // The commit must be on Forgejo's main as well as the door's record.
    let no_main = Push {
        main: NO_COMMIT,
        ..new_tag
    };
    assert_eq!(door.check(no_main, &[]).unwrap()[0].check, "door.tag");
    let _ = git(&source, &["checkout", "-q", "--orphan", "loose"]);
    let loose = commit(&source, "l.md", "# L\n", GOOD);
    let body = pack(&source, &first, &loose);
    let off_main = Push {
        new: &loose,
        ..new_tag
    };
    assert_eq!(door.check(off_main, &body).unwrap()[0].check, "door.tag");
}

#[test]
fn bad_ids_missing_commits_and_bad_packs_are_errors() {
    let (_source, door, first) = setup("errors");
    assert!(door.check(main_push("main", &first), &[]).is_err());
    let missing = "ab".repeat(20);
    assert!(door.check(main_push(&first, &missing), &[]).is_err());
    assert!(
        door.check(main_push(&first, &first), b"not a pack")
            .is_err()
    );
}

#[test]
fn once_main_is_accepted_a_lost_main_cannot_start_history_again() {
    let (source, door, first) = setup("lost");
    let next = commit(&source, "b.md", "# B\n", GOOD);
    let body = pack(&source, &first, &next);
    assert_eq!(
        door.check(main_push(&first, &next), &body).unwrap().len(),
        0
    );
    // Forgejo now says main does not exist, and an unrelated commit
    // arrives: the door still measures it from what it accepted.
    let _ok = git(&source, &["checkout", "-q", "--orphan", "other"]);
    let lone = commit(&source, "c.md", "# C\n", GOOD);
    let body = pack(&source, NO_COMMIT, &lone);
    let found = door.check(main_push(NO_COMMIT, &lone), &body).unwrap();
    assert!(found.iter().any(|f| f.check == "door.history"), "{found:?}");
}

#[test]
fn a_push_of_too_many_commits_is_refused_before_anything_is_written() {
    let (source, door, first) = setup("many");
    for n in 0..501 {
        let _ = git(
            &source,
            &["commit", "-q", "--allow-empty", "-m", &format!("c{n}")],
        );
    }
    let head = git(&source, &["rev-parse", "HEAD"]);
    let body = pack(&source, &first, &head);
    let found = door.check(main_push(&first, &head), &body).unwrap();
    assert!(found.iter().any(|f| f.check == "door.size"), "{found:?}");
}

#[test]
fn a_tree_that_names_one_file_too_many_times_is_refused_before_checkout() {
    use std::fmt::Write as _;
    let (source, door, first) = setup("bomb");
    // One small file, named 1,000 times in a folder, and that folder
    // named 300 times: 300,000 files from a pack of a few KB.
    let blob = git(&source, &["hash-object", "-w", "--stdin"]);
    let mut rows = String::new();
    for n in 0..1000 {
        let _ok = writeln!(rows, "100644 blob {blob}\tf{n}");
    }
    let inner = git_input(&source, &["mktree"], &rows);
    let mut top = String::new();
    for n in 0..300 {
        let _ok = writeln!(top, "040000 tree {inner}\td{n}");
    }
    let tree = git_input(&source, &["mktree"], &top);
    let bomb = git(&source, &["commit-tree", "-p", &first, "-m", "x", &tree]);
    let body = pack(&source, &first, &bomb);
    let found = door.check(main_push(&first, &bomb), &body).unwrap();
    assert!(found.iter().any(|f| f.check == "door.size"), "{found:?}");
}

fn git_input(dir: &std::path::Path, args: &[&str], input: &str) -> String {
    use std::io::Write;
    let mut child = Command::new("git")
        .args(args)
        .current_dir(dir)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let out = child.wait_with_output().unwrap();
    String::from_utf8_lossy(&out.stdout).trim().to_owned()
}

#[test]
fn one_file_that_unpacks_too_big_is_refused_before_checkout() {
    let (source, door, first) = setup("bigfile");
    // Two million short lines: about 4 MB, but a pack of a few KB.
    let big = commit(&source, "big.txt", &"a\n".repeat(2_000_000), GOOD);
    let body = pack(&source, &first, &big);
    assert!(body.len() < 100_000);
    let found = door.check(main_push(&first, &big), &body).unwrap();
    assert!(found.iter().any(|f| f.check == "door.size"), "{found:?}");
}

#[test]
fn a_big_file_in_an_earlier_commit_of_the_push_is_refused_too() {
    let (source, door, first) = setup("earlier");
    // The big file is added, then removed, so the newest tree is small.
    let _added = commit(&source, "big.txt", &"a\n".repeat(700_000), GOOD);
    let _ = git(&source, &["rm", "-q", "big.txt"]);
    let _ = git(&source, &["commit", "-q", "-m", GOOD]);
    let head = git(&source, &["rev-parse", "HEAD"]);
    let body = pack(&source, &first, &head);
    let found = door.check(main_push(&first, &head), &body).unwrap();
    assert!(found.iter().any(|f| f.check == "door.size"), "{found:?}");
}

#[test]
fn a_filter_named_by_a_pushed_project_never_runs_at_checkout() {
    let (source, door, first) = setup("filter");
    let work = door_work("filter");
    let marker = work.join("filter-ran");
    let run = format!("touch {}", marker.display());
    let _ = git(&work, &["config", "filter.evil.smudge", &run]);
    let _ = git(&work, &["config", "filter.evil.required", "true"]);
    std::fs::write(source.join(".gitattributes"), "*.md filter=evil\n")
        .unwrap();
    let next = commit(&source, "c.md", "# C\n", GOOD);
    let body = pack(&source, &first, &next);
    let _found = door.check(main_push(&first, &next), &body).unwrap();
    assert!(!marker.exists(), "the pushed filter ran");
}
