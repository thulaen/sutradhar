//! The pre-push hook's clean copy (AGENTS.md section 8.2): a copy that is
//! not this checkout's own, or was tampered with, is refused and kept,
//! and the copy always holds the pushed commit's bytes.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::fs;
use std::path::Path;

mod common;

use common::hooks::{pre_push, push_repo, refused_as_not_ours, tidy};

#[test]
fn a_copy_that_is_not_ours_is_refused_and_kept() {
    let (dir, sha, copy) = push_repo("forged");
    fs::write(dir.join("unsaved.txt"), "keep\n").unwrap();
    // A forged copy whose `.git` names this checkout.
    fs::create_dir_all(&copy).unwrap();
    let forged = format!("gitdir: {}\n", dir.join(".git").display());
    fs::write(format!("{copy}/.git"), forged).unwrap();
    refused_as_not_ours(&dir, &sha, &format!("{copy}/.git"));
    fs::remove_dir_all(&copy).unwrap();
    // Another clone sitting at the copy's place.
    let _ok = common::git(&dir, &["clone", "-q", ".", &copy]);
    fs::write(format!("{copy}/mine.txt"), "keep\n").unwrap();
    refused_as_not_ours(&dir, &sha, &format!("{copy}/mine.txt"));
    fs::remove_dir_all(&copy).unwrap();
    // Another agent's detached worktree at the copy's place.
    let _ok = common::git(&dir, &["worktree", "add", "-q", "--detach", &copy]);
    fs::write(format!("{copy}/mine.txt"), "keep\n").unwrap();
    refused_as_not_ours(&dir, &sha, &format!("{copy}/mine.txt"));
    let _ok = common::git(&dir, &["worktree", "remove", "--force", &copy]);
    // This repository's own worktree on a branch, at the copy's place.
    let _ok =
        common::git(&dir, &["worktree", "add", "-q", "-b", "feat", &copy]);
    fs::write(format!("{copy}/mine.txt"), "keep\n").unwrap();
    refused_as_not_ours(&dir, &sha, &format!("{copy}/mine.txt"));
    let branch = common::git(Path::new(&copy), &["symbolic-ref", "HEAD"]);
    assert_eq!(branch, "refs/heads/feat");
    tidy(&dir, &copy);
}

#[test]
fn a_tampered_copy_is_refused_and_kept() {
    let (dir, sha, copy) = push_repo("tampered");
    fs::write(dir.join("unsaved.txt"), "keep\n").unwrap();
    assert_eq!(pre_push(&dir, &sha, false).0, 0);
    // A file the copy's index hides from checkouts is refused, and the
    // shared settings file is never changed by a push.
    let copy_dir = Path::new(&copy);
    let config = fs::read_to_string(dir.join(".git/config")).unwrap();
    let hide = ["update-index", "--skip-worktree", "a.txt"];
    let _ok = common::git(copy_dir, &hide);
    fs::write(format!("{copy}/a.txt"), "changed\n").unwrap();
    let (code, said) = pre_push(&dir, &sha, false);
    assert_eq!(code, 1, "{said}");
    assert!(said.contains("hides a file from the check"), "{said}");
    assert_eq!(fs::read_to_string(dir.join(".git/config")).unwrap(), config);
    let show = ["update-index", "--no-skip-worktree", "a.txt"];
    let _ok = common::git(copy_dir, &show);
    let _ok = common::git(copy_dir, &["checkout", "--", "a.txt"]);
    // Settings of the copy's own are refused.
    let record = common::git(copy_dir, &["rev-parse", "--git-dir"]);
    let own_config = Path::new(&record).join("config.worktree");
    fs::write(&own_config, "[core]\n\tfsmonitor = true\n").unwrap();
    refused_as_not_ours(&dir, &sha, &own_config.display().to_string());
    fs::remove_file(&own_config).unwrap();
    // Our own copy with its mark taken off, or moved onto a branch, may
    // now be someone's work.
    fs::write(format!("{copy}/mine.txt"), "keep\n").unwrap();
    let _ok = common::git(&dir, &["worktree", "unlock", &copy]);
    refused_as_not_ours(&dir, &sha, &format!("{copy}/mine.txt"));
    let mark = "aegis push copy: made and used only by the pre-push hook";
    let _ok = common::git(&dir, &["worktree", "lock", "--reason", mark, &copy]);
    let _ok = common::git(copy_dir, &["switch", "-q", "-c", "side"]);
    refused_as_not_ours(&dir, &sha, &format!("{copy}/mine.txt"));
    let _ok = common::git(copy_dir, &["switch", "-q", "--detach"]);
    // A broken HEAD in the copy's record.
    fs::write(Path::new(&record).join("HEAD"), "ref: refs/heads/../bad\n")
        .unwrap();
    fs::write(format!("{copy}/mine.txt"), "keep\n").unwrap();
    refused_as_not_ours(&dir, &sha, &format!("{copy}/mine.txt"));
    // A copy deleted by hand is made again, though its record is locked.
    fs::remove_dir_all(&copy).unwrap();
    let (code, said) = pre_push(&dir, &sha, false);
    assert_eq!(code, 0, "{said}");
    tidy(&dir, &copy);
}

#[test]
fn the_copy_holds_the_committed_bytes() {
    let (dir, _first, copy) = push_repo("bytes");
    // Attributes in the commit, and the checkout's own settings that git
    // copies into a new copy, never change what the check reads.
    let attrs = "* text eol=crlf\n";
    let sha = common::save(&dir, &[(".gitattributes", attrs)], "attrs");
    let _ok =
        common::git(&dir, &["config", "extensions.worktreeConfig", "true"]);
    fs::write(dir.join(".git/config.worktree"), "[core]\n").unwrap();
    // A replaced object never stands in for the pushed bytes.
    let real = common::git(&dir, &["rev-parse", "HEAD:a.txt"]);
    fs::write(dir.join("fake.txt"), "fake\n").unwrap();
    let fake = common::git(&dir, &["hash-object", "-w", "fake.txt"]);
    let _ok = common::git(&dir, &["replace", &real, &fake]);
    for _ in 0..2 {
        let (code, said) = pre_push(&dir, &sha, false);
        assert_eq!(code, 0, "{said}");
        let text = fs::read_to_string(format!("{copy}/a.txt")).unwrap();
        assert_eq!(text, "a\n");
    }
    // The checkout's own attributes file would change them: refused.
    fs::create_dir_all(dir.join(".git/info")).unwrap();
    fs::write(dir.join(".git/info/attributes"), attrs).unwrap();
    let (code, said) = pre_push(&dir, &sha, false);
    assert_eq!(code, 1, "{said}");
    assert!(said.contains("info/attributes would change"), "{said}");
    tidy(&dir, &copy);
}
