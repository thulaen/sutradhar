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
fn release_tags_must_be_new_and_on_main() {
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
    assert_eq!(door.check(new_tag, &body).unwrap().len(), 0);
    let replaced = Push {
        old: &tag,
        ..new_tag
    };
    assert_eq!(door.check(replaced, &[]).unwrap()[0].check, "door.tag");
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
