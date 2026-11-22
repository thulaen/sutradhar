//! A throwaway git repository for the git tests.
#![allow(dead_code, reason = "each test file uses some of these")]
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]
#![allow(unused_results, reason = "git calls are made for their effect")]

use std::path::{Path, PathBuf};
use std::process::Command;

/// A repository in a temporary folder, removed when it is dropped.
pub struct Repo {
    /// Where the repository is.
    pub dir: PathBuf,
}

impl Repo {
    /// A new repository with one commit, `README`, as `main`'s head.
    pub fn new(name: &str) -> Self {
        let dir = std::env::temp_dir()
            .join(format!("harness-git-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let repo = Self { dir };
        let _ = repo.git(&["init", "-q", "-b", "main"]);
        repo.commit_file("README", "hello\n", "docs: start", "Codex Agent");
        repo
    }

    /// Runs git here as a person with fixed names and gives its output.
    pub fn git(&self, args: &[&str]) -> String {
        self.git_as(args, "Codex Agent", "Codex Agent")
    }

    /// Runs git here as this author and committer.
    pub fn git_as(
        &self,
        args: &[&str],
        author: &str,
        committer: &str,
    ) -> String {
        let out = Command::new("git")
            .arg("-C")
            .arg(&self.dir)
            .args(args)
            .env("GIT_AUTHOR_NAME", author)
            .env("GIT_AUTHOR_EMAIL", "a@example.invalid")
            .env("GIT_COMMITTER_NAME", committer)
            .env("GIT_COMMITTER_EMAIL", "c@example.invalid")
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8(out.stdout).unwrap().trim().to_owned()
    }

    /// Writes a file and commits it.
    pub fn commit_file(
        &self,
        path: &str,
        text: &str,
        message: &str,
        who: &str,
    ) -> String {
        let file = self.dir.join(path);
        if let Some(parent) = file.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(file, text).unwrap();
        self.git(&["add", "-A"]);
        self.git_as(&["commit", "-q", "-m", message], who, who);
        self.head()
    }

    /// The commit `HEAD` names.
    pub fn head(&self) -> String {
        self.git(&["rev-parse", "HEAD"])
    }

    /// The repository folder.
    pub fn path(&self) -> &Path {
        &self.dir
    }
}

impl Drop for Repo {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

/// A program that touches a marker file when it runs, and that marker.
pub fn tripwire(name: &str) -> (std::path::PathBuf, std::path::PathBuf) {
    let pid = std::process::id();
    let marker = std::env::temp_dir().join(format!("harness-{name}-{pid}"));
    let script = std::env::temp_dir().join(format!("{name}-{pid}.sh"));
    let body = format!("#!/bin/sh\ntouch {}\n", marker.display());
    std::fs::write(&script, body).unwrap();
    let mode = std::os::unix::fs::PermissionsExt::from_mode(0o755);
    std::fs::set_permissions(&script, mode).unwrap();
    (script, marker)
}
