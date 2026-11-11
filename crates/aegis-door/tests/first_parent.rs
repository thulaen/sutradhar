//! The last accepted main must stay on the first-parent line of a push:
//! histories that only a wrong version of that rule would let in, or
//! keep out.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

mod common;

use std::path::Path;
use std::process::Command;

use aegis_door::gate::Door;
use common::{GOOD, commit, git, main_push, pack, setup};

/// A commit of `tree` with `parents`, dated `date`, made without
/// touching the branch.
fn commit_tree(
    source: &Path,
    tree: &str,
    parents: &[&str],
    date: &str,
) -> String {
    let mut command = Command::new("git");
    let _dir = command.arg("-C").arg(source).args(["commit-tree", tree]);
    for parent in parents {
        let _parent = command.args(["-p", parent]);
    }
    let out = command
        .args(["-m", GOOD])
        .env("GIT_AUTHOR_DATE", date)
        .env("GIT_COMMITTER_DATE", date)
        .output()
        .unwrap();
    assert!(out.status.success(), "{out:?}");
    String::from_utf8(out.stdout).unwrap().trim().to_owned()
}

/// A source and door where `accepted`, one commit after `first`, is the
/// accepted main; returns them with the tree of `first`.
fn accepted_main(name: &str) -> (std::path::PathBuf, Door, String, String) {
    let (source, door, first) = setup(name);
    let accepted = commit(&source, "b.md", "# B\n", GOOD);
    let body = pack(&source, &first, &accepted);
    let found = door.check(main_push(&first, &accepted), &body).unwrap();
    assert_eq!(found.len(), 0, "{found:?}");
    let tree = git(&source, &["rev-parse", &format!("{first}^{{tree}}")]);
    (source, door, accepted, tree)
}

fn checks(
    door: &Door,
    source: &Path,
    base: &str,
    new: &str,
) -> Vec<&'static str> {
    let body = pack(source, base, new);
    let found = door.check(main_push(base, new), &body).unwrap();
    found.iter().map(|f| f.check).collect()
}

#[test]
fn a_side_line_older_by_date_cannot_stand_in_for_the_first_parent() {
    let (source, door, accepted, tree) = accepted_main("fp-dates");
    let first = git(&source, &["rev-parse", "HEAD~1"]);
    // `side` follows accepted main but is the oldest by date; the merge's
    // first-parent line goes through `skip`, which leaves main out.
    let side = commit_tree(&source, &tree, &[&accepted], "2001-01-01T00:00Z");
    let skip = commit_tree(&source, &tree, &[&first], "2002-01-01T00:00Z");
    let merge =
        commit_tree(&source, &tree, &[&skip, &side], "2003-01-01T00:00Z");
    assert_eq!(checks(&door, &source, &accepted, &merge), ["door.history"]);
}

#[test]
fn a_root_commit_cannot_start_the_first_parent_line() {
    let (source, door, accepted, tree) = accepted_main("fp-root");
    let root = commit_tree(&source, &tree, &[], "2002-01-01T00:00Z");
    let merge =
        commit_tree(&source, &tree, &[&root, &accepted], "2003-01-01T00:00Z");
    assert_eq!(checks(&door, &source, &accepted, &merge), ["door.history"]);
}

#[test]
fn pushing_the_accepted_main_again_passes() {
    let (source, door, accepted, _tree) = accepted_main("fp-again");
    let first = git(&source, &["rev-parse", "HEAD~1"]);
    // Forgejo may not have taken a commit the door accepted; the same
    // push then comes again, with no commit after the accepted main.
    assert_eq!(
        checks(&door, &source, &first, &accepted),
        Vec::<&str>::new()
    );
}

#[test]
fn the_rule_follows_the_accepted_main_when_forgejo_lags_behind() {
    let (source, door, accepted, tree) = accepted_main("fp-lag");
    let first = git(&source, &["rev-parse", "HEAD~1"]);
    // Forgejo still reports `first`; the door's own record says
    // `accepted`, and only that record is the base.
    let skip = commit_tree(&source, &tree, &[&first], "2002-01-01T00:00Z");
    let merge =
        commit_tree(&source, &tree, &[&skip, &accepted], "2003-01-01T00:00Z");
    assert_eq!(checks(&door, &source, &first, &merge), ["door.history"]);
    let next = commit(&source, "c.md", "# C\n", GOOD);
    assert_eq!(checks(&door, &source, &first, &next), Vec::<&str>::new());
}
