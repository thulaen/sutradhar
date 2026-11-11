//! Each new commit, with its message and the paths it changed.

use crate::patch;
use crate::repo::Repo;

/// One commit: its message and the paths it added, changed or renamed.
/// A merge's paths are those it changed against its first parent, and a
/// renamed file is listed under both its names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commit {
    /// The commit's id.
    pub id: String,
    /// The full message.
    pub message: String,
    /// Every path it touched, deletions included.
    pub paths: Vec<String>,
}

/// Every commit after `base` up to `head`. A `base` of
/// [`NO_COMMIT`](crate::repo::NO_COMMIT) means every commit up to `head`.
///
/// # Errors
/// When git fails.
pub fn commits(
    repo: &Repo,
    base: &str,
    head: &str,
) -> Result<Vec<Commit>, String> {
    let range = crate::repo::range(base, head);
    let ids = repo.git(&["rev-list", "--end-of-options", &range])?;
    ids.lines()
        .filter(|id| !id.is_empty())
        .map(|id| {
            let message = repo.git(&["log", "-1", "--format=%B", id])?;
            let names = repo.git(&[
                "diff-tree",
                "--no-commit-id",
                "--name-only",
                // Without this a merge lists no paths at all, so a merge
                // whose `TDD:` line says docs only could bring in code.
                "--diff-merges=first-parent",
                "-r",
                "-z",
                "--root",
                id,
            ])?;
            Ok(Commit {
                id: id.to_owned(),
                message,
                paths: patch::nul_list(&names),
            })
        })
        .collect()
}
