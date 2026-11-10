//! The local git hooks (AGENTS.md section 8.2): with a mailbox named, a
//! commit or push needs the agent's name and a clear lock check, and a
//! push is checked on a clean copy beside the checkout.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::fs;
use std::path::Path;
use std::process::Command;

mod common;

use common::hooks::{pre_push, push_repo, stand_in, tidy};

/// Runs the repository's pre-commit hook in `dir`, as `agent` if any,
/// with a stand-in `cargo`: its exit code and what it said.
fn pre_commit(dir: &Path, agent: Option<&str>) -> (i32, String) {
    let hook = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../.githooks/pre-commit");
    let path = format!("{}:/usr/bin:/bin", dir.join("bin").display());
    let mut run = Command::new("sh");
    let _set = run
        .arg(hook)
        .current_dir(dir)
        .env("PATH", path)
        .env_remove("SUTRADHAR_AGENT")
        .env_remove("GIT_DIR")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_WORK_TREE");
    if let Some(name) = agent {
        let _name = run.env("SUTRADHAR_AGENT", name);
    }
    let out = run.output().unwrap();
    let said = String::from_utf8_lossy(&out.stderr).into_owned();
    (out.status.code().unwrap_or(-1), said)
}

#[test]
fn a_commit_needs_the_agents_name_and_free_locks() {
    let dir = common::repo("hooks", "locks");
    stand_in(&dir.join("bin/cargo"), "ran-cargo", 0);
    let program = dir.join("mailbox/bin/agent-mailbox");
    let mailbox = dir.join("mailbox").display().to_string();
    let _ok = common::git(&dir, &["config", "sutradhar.mailbox", &mailbox]);
    // No name: refused before any check runs.
    stand_in(&program, "checked", 0);
    let (code, said) = pre_commit(&dir, None);
    assert_eq!(code, 1, "{said}");
    assert!(said.contains("set SUTRADHAR_AGENT"), "{said}");
    assert!(!said.contains("ran-cargo"), "{said}");
    // A lock in the way: refused, and the gate does not run.
    stand_in(&program, "checked", 1);
    let (code, said) = pre_commit(&dir, Some("codex"));
    assert_eq!(code, 1, "{said}");
    assert!(said.contains("--agent codex"), "{said}");
    assert!(!said.contains("ran-cargo"), "{said}");
    // Locks clear: the gate runs.
    stand_in(&program, "checked", 0);
    let (code, said) = pre_commit(&dir, Some("codex"));
    assert_eq!(code, 0, "{said}");
    assert!(said.contains("ran-cargo run"), "{said}");
    assert!(said.contains("-- commit"), "{said}");
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_first_push_checks_every_file_on_a_clean_copy() {
    let (dir, sha, copy) = push_repo("push");
    let (code, said) = pre_push(&dir, &sha, false);
    assert_eq!(code, 0, "{said}");
    let empty = "4b825dc642cb6eb9a060e54bf8d69288fbee4904";
    assert!(said.contains(&format!("--range {empty}..{sha}")), "{said}");
    assert!(said.contains(&format!("in {copy} git-dir unset")), "{said}");
    assert!(!said.contains("planted-hook-ran"), "{said}");
    assert!(said.contains("--repo . --socket"), "{said}");
    assert!(said.contains("mailbox/run/mailbox.sock"), "{said}");
    assert!(said.contains("mailbox/tokens/codex"), "{said}");
    // A folder planted in the kept copy is gone before the next check;
    // a repository in the build folder, such as cargo-deny's advisory
    // list, is left alone; and a failing check fails the push.
    let advisories = format!("{copy}/target/advisory-dbs/x");
    let _ok = common::git(Path::new(&copy), &["init", "-q", &advisories]);
    let planted = format!("{copy}/.cargo");
    fs::create_dir_all(&planted).unwrap();
    fs::write(format!("{planted}/config.toml"), "[build]\n").unwrap();
    fs::write(dir.join("code"), "3\n").unwrap();
    assert_eq!(pre_push(&dir, &sha, false).0, 3);
    assert!(!Path::new(&planted).exists(), "{planted} survived");
    assert!(Path::new(&advisories).join(".git").exists());
    // A repository inside the copy may be another agent's work: the push
    // is refused, and it is kept.
    let _ok = common::git(Path::new(&copy), &["init", "-q", &planted]);
    let (code, said) = pre_push(&dir, &sha, false);
    assert_eq!(code, 1, "{said}");
    assert!(
        said.contains("is a repository inside the push copy"),
        "{said}"
    );
    assert!(Path::new(&planted).join(".git").exists());
    tidy(&dir, &copy);
}

#[test]
fn outside_git_settings_never_steer_the_copy() {
    let (dir, sha, copy) = push_repo("outside");
    // From a subfolder, with an outside `GIT_DIR`: the copy still sits
    // beside the checkout, and the check runs with no `GIT_DIR`.
    fs::create_dir_all(dir.join("sub")).unwrap();
    let (code, said) = pre_push(&dir, &sha, true);
    assert_eq!(code, 0, "{said}");
    assert!(said.contains(&format!("in {copy} git-dir unset")), "{said}");
    // Another agent's worktree inside the copy is never wiped.
    let other = format!("{copy}/other");
    let _ok = common::git(&dir, &["worktree", "add", "-q", "--detach", &other]);
    fs::write(format!("{other}/unsaved.txt"), "keep\n").unwrap();
    let (code, said) = pre_push(&dir, &sha, false);
    assert_eq!(code, 1, "{said}");
    assert!(said.contains("sits inside the push copy"), "{said}");
    assert!(Path::new(&format!("{other}/unsaved.txt")).exists());
    tidy(&dir, &copy);
}
