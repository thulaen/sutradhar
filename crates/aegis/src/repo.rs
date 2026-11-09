//! Reading the repository through git.

use std::fs;

use crate::patch;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

/// The largest file the gate reads, in bytes: far above any source file,
/// low enough that a file cannot use up the memory. The door refuses
/// any file over 1 MiB before this is reached.
pub const MOST_READ: u64 = 8 * 1024 * 1024;

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
        let root = std::path::absolute(&root).unwrap_or(root);
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
        let root = root.into(); // made a full path, the same for every tool
        let root = std::path::absolute(&root).unwrap_or(root);
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

    /// A git command for this repository, with git's hook variables
    /// cleared but for the index a hook was given. Replace objects and
    /// outside settings, which could show another history, are shut off.
    pub(crate) fn command(&self) -> Command {
        let mut command = Command::new(&self.git);
        crate::git_env::shut_off(&mut command);
        if let Some(index) = &self.index {
            let _kept = command.env("GIT_INDEX_FILE", index);
        }
        let _at = command.arg("-C").arg(&self.root);
        command
    }

    /// Runs git and returns the bytes it printed, unchanged.
    ///
    /// # Errors
    /// When git cannot start or ends with an error.
    pub fn git_bytes(&self, args: &[&str]) -> Result<Vec<u8>, String> {
        let out = self
            .command()
            .args(args)
            .stdin(Stdio::null())
            .output()
            .map_err(|e| format!("cannot run git: {e}"))?;
        if out.status.success() {
            Ok(out.stdout)
        } else {
            Err(format!("git {} failed", args.join(" ")))
        }
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
        let mut command = self.command();
        let _args = command.args(args);
        crate::git_env::run_with_input(
            &mut command,
            input,
            "git",
            &args.join(" "),
        )
    }

    /// Copies exactly the staged files into a new folder under the build
    /// folder, `target/aegis-work`, where a sealed check may read it.
    ///
    /// # Errors
    /// When the folder cannot be made or git fails.
    pub fn staged_copy(&self) -> Result<StagedCopy, String> {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos());
        let dir = crate::history::work_folder(&self.root)
            .join(format!("staged-{}-{nanos}", std::process::id()));
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

    /// True when the path is a folder.
    #[must_use]
    pub fn is_folder(&self, path: &str) -> bool {
        !self.is_link(path) && self.content.join(path).is_dir()
    }
}

pub(crate) fn range(base: &str, head: &str) -> String {
    if base == NO_COMMIT {
        head.to_owned()
    } else {
        format!("{base}..{head}")
    }
}

// Reading files from disk: never through a link, never too large.
#[path = "repo_files.rs"]
mod files;

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
