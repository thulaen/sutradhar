//! Reading the repository through git.

use std::fs;

use crate::patch;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

/// A git repository on disk. Git runs in `root`; files are read from
/// `content`, which is `root` itself or a copy of the staged files.
#[derive(Debug, Clone)]
pub struct Repo {
    root: PathBuf,
    content: PathBuf,
    git: String,
    index: Option<PathBuf>,
}

/// The commit id git uses for "no commit", as in a first push.
pub const NO_COMMIT: &str = "0000000000000000000000000000000000000000";

/// A copy of the staged files in a temporary folder, removed when it is
/// dropped.
#[derive(Debug)]
pub struct StagedCopy {
    repo: Repo,
}

impl StagedCopy {
    /// The repository, reading files from the copy.
    #[must_use]
    pub fn repo(&self) -> &Repo {
        &self.repo
    }
}

impl Drop for StagedCopy {
    fn drop(&mut self) {
        let _ignored = fs::remove_dir_all(&self.repo.content);
    }
}

impl Repo {
    /// Opens the repository whose top folder is `root`.
    #[must_use]
    pub fn at(root: impl Into<PathBuf>) -> Self {
        Self::with_git(root, "git")
    }

    /// Opens the repository a git hook is running in. Git may point the
    /// hook at a temporary index, as `git commit -a` does; this keeps
    /// that index, so the checks see exactly what is being committed.
    #[must_use]
    pub fn for_hook(root: impl Into<PathBuf>) -> Self {
        let root = root.into();
        let index = std::env::var_os("GIT_INDEX_FILE").map(|i| {
            let path = PathBuf::from(i);
            if path.is_absolute() {
                path
            } else {
                root.join(path)
            }
        });
        Self {
            index,
            ..Self::at(root)
        }
    }

    /// Opens the repository, running `git` as the given program.
    #[must_use]
    pub fn with_git(root: impl Into<PathBuf>, git: &str) -> Self {
        let root = root.into();
        Self {
            content: root.clone(),
            root,
            git: git.to_owned(),
            index: None,
        }
    }

    /// The top folder, where git runs.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// The folder the files are read from.
    #[must_use]
    pub fn content(&self) -> &Path {
        &self.content
    }

    /// Runs git and returns what it printed.
    ///
    /// # Errors
    /// When git cannot start or ends with an error.
    pub fn git(&self, args: &[&str]) -> Result<String, String> {
        self.git_with_input(args, &[])
    }

    /// Runs git with `input` as what it reads, and returns what it
    /// printed. Git's hook variables are cleared, so git always works on
    /// this repository, except for the index a hook was given.
    ///
    /// # Errors
    /// When git cannot start or ends with an error.
    pub fn git_with_input(
        &self,
        args: &[&str],
        input: &[u8],
    ) -> Result<String, String> {
        let mut command = Command::new(&self.git);
        let _cleared = command
            .env_remove("GIT_DIR")
            .env_remove("GIT_INDEX_FILE")
            .env_remove("GIT_WORK_TREE");
        if let Some(index) = &self.index {
            let _kept = command.env("GIT_INDEX_FILE", index);
        }
        let mut child = command
            .arg("-C")
            .arg(&self.root)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("cannot run git: {e}"))?;
        let sent = child.stdin.take().map(|mut s| s.write_all(input));
        let out = child
            .wait_with_output()
            .map_err(|e| format!("git did not finish: {e}"))?;
        if out.status.success() && !matches!(sent, Some(Err(_))) {
            Ok(String::from_utf8_lossy(&out.stdout).into_owned())
        } else {
            Err(format!(
                "git {} failed: {}",
                args.join(" "),
                String::from_utf8_lossy(&out.stderr).trim()
            ))
        }
    }

    /// Copies exactly the staged files into a new temporary folder.
    ///
    /// # Errors
    /// When the folder cannot be made or git fails.
    pub fn staged_copy(&self) -> Result<StagedCopy, String> {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos());
        let dir = std::env::temp_dir()
            .join(format!("aegis-staged-{}-{nanos}", std::process::id()));
        fs::create_dir_all(&dir)
            .map_err(|e| format!("cannot make {}: {e}", dir.display()))?;
        let copy = StagedCopy {
            repo: Self {
                content: dir.clone(),
                ..self.clone()
            },
        };
        let prefix = format!("--prefix={}/", dir.display());
        let _out =
            self.git(&["checkout-index", "--all", "--force", &prefix])?;
        Ok(copy)
    }

    /// True when there are no changes that are not committed, and no new
    /// files that git does not track (ignored files do not count).
    ///
    /// # Errors
    /// When git fails.
    pub fn is_clean(&self) -> Result<bool, String> {
        let out =
            self.git(&["status", "--porcelain", "--untracked-files=normal"])?;
        Ok(out.trim().is_empty())
    }

    /// Every file git tracks, plus files added to the index.
    ///
    /// # Errors
    /// When git fails.
    pub fn tracked_files(&self) -> Result<Vec<String>, String> {
        Ok(patch::nul_list(&self.git(&["ls-files", "--cached", "-z"])?))
    }

    /// Files added, changed or renamed in the index.
    ///
    /// # Errors
    /// When git fails.
    pub fn staged_files(&self) -> Result<Vec<String>, String> {
        let args = [
            "diff",
            "--cached",
            "--name-only",
            "-z",
            "--diff-filter=ACMR",
        ];
        Ok(patch::nul_list(&self.git(&args)?))
    }

    /// Every line added by the commits after `base` up to `head`, with
    /// the file it was added to.
    ///
    /// # Errors
    /// When git fails.
    pub fn added_lines(
        &self,
        base: &str,
        head: &str,
    ) -> Result<Vec<(String, String)>, String> {
        let range = range(base, head);
        let args = [
            "log",
            "-p",
            "--no-color",
            "--diff-merges=first-parent",
            "--format=",
            "--end-of-options",
            &range,
        ];
        Ok(patch::added_lines(&self.git(&args)?))
    }

    /// The text of a file, or `None` when it cannot be read.
    #[must_use]
    pub fn read(&self, path: &str) -> Option<String> {
        let bytes = fs::read(self.content.join(path)).ok()?;
        Some(String::from_utf8_lossy(&bytes).into_owned())
    }

    /// The bytes of a file, or `None` when it cannot be read.
    #[must_use]
    pub fn read_bytes(&self, path: &str) -> Option<Vec<u8>> {
        fs::read(self.content.join(path)).ok()
    }

    /// True when the path is a folder.
    #[must_use]
    pub fn is_folder(&self, path: &str) -> bool {
        self.content.join(path).is_dir()
    }
}

fn range(base: &str, head: &str) -> String {
    if base == NO_COMMIT {
        head.to_owned()
    } else {
        format!("{base}..{head}")
    }
}

/// The added lines in a patch, with the file each was added to.
#[cfg(test)]
mod tests {
    use super::Repo;

    #[test]
    fn a_missing_git_is_an_error() {
        let repo = Repo::with_git(".", "no-such-git-here");
        assert!(repo.git(&["status"]).unwrap_err().contains("cannot run"));
    }

    #[test]
    fn a_staged_copy_that_cannot_be_made_is_an_error() {
        let repo = Repo::with_git(".", "no-such-git-here");
        assert!(repo.staged_copy().is_err());
    }
}
