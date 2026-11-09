//! Helpers the gate's tests share: scratch git repositories. The door's
//! tests use them too.
#![allow(dead_code, reason = "each test file uses some of the helpers")]
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Runs git in `dir`, with no hooks and none of the caller's git
/// settings, and gives what it printed, trimmed. A failure stops the
/// test.
pub fn git(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .env_remove("GIT_DIR")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_WORK_TREE")
        .args(["-c", "core.hooksPath=/dev/null"])
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .unwrap();
    assert!(out.status.success(), "git {args:?}: {out:?}");
    String::from_utf8_lossy(&out.stdout).trim().to_owned()
}

/// Writes one file, making its folders.
pub fn write(dir: &Path, path: &str, text: &str) {
    let path = dir.join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

/// Writes `files`, commits everything with `message`, and gives the
/// new commit's id.
pub fn save(dir: &Path, files: &[(&str, &str)], message: &str) -> String {
    for (path, text) in files {
        write(dir, path, text);
    }
    let _ok = git(dir, &["add", "-A"]);
    let _ok = git(dir, &["commit", "-q", "-m", message]);
    git(dir, &["rev-parse", "HEAD"])
}

/// An empty repository on `main` in a fresh scratch folder named from
/// `group` and `name`, with an author set.
pub fn repo(group: &str, name: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join(format!("aegis-{group}-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let _ok = git(&dir, &["init", "-q", "-b", "main"]);
    let _ok = git(&dir, &["config", "user.email", "a@example.com"]);
    let _ok = git(&dir, &["config", "user.name", "A"]);
    dir
}
