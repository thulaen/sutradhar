//! The commits being pushed, each with its message and the paths it
//! changed, so a commit's `TDD:` line can be checked against them.

use crate::patch;
use crate::repo::{NO_COMMIT, Repo};

/// One commit: its message and the paths it added, changed or renamed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commit {
    /// The full message.
    pub message: String,
    /// Every path it touched, deletions included.
    pub paths: Vec<String>,
}

/// Every commit after `base` up to `head`. A `base` of [`NO_COMMIT`]
/// means every commit up to `head`.
///
/// # Errors
/// When git fails.
pub fn commits(
    repo: &Repo,
    base: &str,
    head: &str,
) -> Result<Vec<Commit>, String> {
    let range = if base == NO_COMMIT {
        head.to_owned()
    } else {
        format!("{base}..{head}")
    };
    let ids = repo.git(&["rev-list", "--end-of-options", &range])?;
    ids.lines()
        .filter(|id| !id.is_empty())
        .map(|id| {
            let message = repo.git(&["log", "-1", "--format=%B", id])?;
            let names = repo.git(&[
                "diff-tree",
                "--no-commit-id",
                "--name-only",
                "-r",
                "-z",
                "--root",
                id,
            ])?;
            Ok(Commit {
                message,
                paths: patch::nul_list(&names),
            })
        })
        .collect()
}

/// Every path the index changes against `HEAD`, deletions included and
/// renames given as both paths, as the push tier sees them.
///
/// # Errors
/// When git fails.
pub fn staged_paths(repo: &Repo) -> Result<Vec<String>, String> {
    let args = ["diff", "--cached", "--name-only", "--no-renames", "-z"];
    Ok(patch::nul_list(&repo.git(&args)?))
}
