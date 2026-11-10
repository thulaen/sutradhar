//! Helpers the hook tests share: stand-in programs, and runs of the
//! repository's pre-push hook on scratch repositories.

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::{git, repo, save};

/// Writes a stand-in program at `path` that says `word` and exits with
/// `code`.
pub fn stand_in(path: &Path, word: &str, code: i32) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    let script = format!("#!/bin/sh\necho {word} \"$@\" >&2\nexit {code}\n");
    fs::write(path, script).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
}

/// Runs the repository's pre-push hook for one push of `sha` to `main`,
/// as `codex`: its exit code and what it said. It runs in `dir`, or, when
/// `outside` is set, in `dir/sub` with `GIT_DIR` naming `dir/.git`.
pub fn pre_push(dir: &Path, sha: &str, outside: bool) -> (i32, String) {
    let hook =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.githooks/pre-push");
    let path = format!("{}:/usr/bin:/bin", dir.join("bin").display());
    let zero = "0".repeat(40);
    let line = format!("refs/heads/main {sha} refs/heads/main {zero}\n");
    let mut child = Command::new("sh")
        .args([hook.to_str().unwrap(), "origin", "url"])
        .current_dir(if outside { dir.join("sub") } else { dir.into() })
        .env("PATH", path)
        .env("SUTRADHAR_AGENT", "codex")
        .env_remove("GIT_DIR")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_WORK_TREE")
        .envs(outside.then(|| ("GIT_DIR", dir.join(".git"))))
        .stdin(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    std::io::Write::write_all(child.stdin.as_mut().unwrap(), line.as_bytes())
        .unwrap();
    let out = child.wait_with_output().unwrap();
    let said = String::from_utf8_lossy(&out.stderr).into_owned();
    (out.status.code().unwrap_or(-1), said)
}

/// A test repository for the push hook, with stand-in `cargo` and
/// mailbox, a planted git hook, and one commit: the folder, the commit
/// and where the hook's clean copy goes. The stand-in `cargo` says where
/// it ran and its `GIT_DIR`, and exits with the number in `code`.
pub fn push_repo(name: &str) -> (PathBuf, String, String) {
    let dir = repo("hooks", name);
    let mailbox = dir.join("mailbox").display().to_string();
    let _ok = git(&dir, &["config", "sutradhar.mailbox", &mailbox]);
    stand_in(&dir.join("mailbox/bin/agent-mailbox"), "checked", 0);
    let code = dir.join("code").display().to_string();
    let cargo = format!(
        "#!/bin/sh\necho ran-cargo \"$@\" in \"$(pwd)\" \
         git-dir \"${{GIT_DIR-unset}}\" >&2\n\
         exit \"$(cat {code} 2>/dev/null || echo 0)\"\n"
    );
    fs::create_dir_all(dir.join("bin")).unwrap();
    fs::write(dir.join("bin/cargo"), cargo).unwrap();
    let run_mode = fs::Permissions::from_mode(0o755);
    fs::set_permissions(dir.join("bin/cargo"), run_mode).unwrap();
    // A hook planted for git to run must never run while the copy is made.
    let planted = dir.join("planted");
    stand_in(&planted.join("post-checkout"), "planted-hook-ran", 0);
    let hooks = planted.display().to_string();
    let _ok = git(&dir, &["config", "core.hooksPath", &hooks]);
    let sha = save(&dir, &[("a.txt", "a\n")], "start");
    let copy = format!("{}.push-work", dir.canonicalize().unwrap().display());
    (dir, sha, copy)
}

/// Removes a push test's repository, clean copy and lock.
pub fn tidy(dir: &Path, copy: &str) {
    let _copy = fs::remove_dir_all(copy);
    let _lock = fs::remove_file(format!("{copy}.lock"));
    fs::remove_dir_all(dir).unwrap();
}

/// Pushes `sha` from `dir` and asserts the push is refused because the
/// copy is not this checkout's, leaving `kept` and the checkout alone.
pub fn refused_as_not_ours(dir: &Path, sha: &str, kept: &str) {
    let (code, said) = pre_push(dir, sha, false);
    assert_eq!(code, 1, "{said}");
    assert!(said.contains("is not this checkout's push copy"), "{said}");
    assert!(!said.contains("planted-hook-ran"), "{said}");
    assert!(Path::new(kept).exists(), "{kept} was removed");
    assert!(dir.join("unsaved.txt").exists());
    let head = git(dir, &["symbolic-ref", "HEAD"]);
    assert_eq!(head, "refs/heads/main");
}
