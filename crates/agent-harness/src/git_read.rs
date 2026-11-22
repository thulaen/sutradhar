//! Reading the commits of a push from the harness's own repository
//! (spec 0155, "Push", steps 1 and 2). The commits are read with git
//! turned down to what cannot run anything, then checked as plain data
//! by [`crate::push_check`]. A refusal names the rule, never a path.

use std::path::Path;

use crate::git_run::git;
use crate::push_check::NewCommit;
use crate::request_check::is_commit;

/// The limits on what a push may bring (all proposed).
#[derive(Debug, Clone, Copy)]
pub struct GitLimits {
    /// The most new commits.
    pub max_commits: usize,
    /// The largest file, in bytes.
    pub max_object_bytes: u64,
}

refusals! {
    /// Why the commits of a push could not be read or are not allowed.
    GitRefusal {
        /// An id is not a full commit id.
        BadCommitId => "an id is not a full commit id",
        /// The tip does not descend from `main`.
        NotDescendant => "the commit does not descend from main",
        /// More new commits than allowed.
        TooManyCommits => "the push has too many new commits",
        /// A change makes a symlink or a submodule.
        NotPlainFile => "a change is not a plain file",
        /// A change touches a `.gitmodules` file.
        ModulesFile => "a change touches a submodule file",
        /// A file is over the size limit.
        ObjectTooBig => "a file is over the size limit",
        /// Git could not read the commits, or gave odd output.
        Unreadable => "the commits could not be read",
    }
}

/// Reads the new commits from `main` (exclusive) to `tip`, oldest first.
///
/// # Errors
/// The rule the push breaks, or [`GitRefusal::Unreadable`].
pub fn read_new_commits(
    repo: &Path,
    main: &str,
    tip: &str,
    limits: &GitLimits,
) -> Result<Vec<NewCommit>, GitRefusal> {
    if !is_commit(main) || !is_commit(tip) {
        return Err(GitRefusal::BadCommitId);
    }
    let ancestor = git(repo, &["merge-base", "--is-ancestor", main, tip], None)
        .map_err(|_| GitRefusal::Unreadable)?;
    match ancestor.code {
        Some(0) => {}
        Some(1) => return Err(GitRefusal::NotDescendant),
        _ => return Err(GitRefusal::Unreadable),
    }
    let range = format!("{main}..{tip}");
    let cap = format!("--max-count={}", limits.max_commits.saturating_add(1));
    let ran = git(
        repo,
        &["rev-list", "--reverse", "--topo-order", &cap, &range],
        None,
    )
    .map_err(|_| GitRefusal::Unreadable)?;
    let listed = lines(&ran)?;
    if listed.len() > limits.max_commits {
        return Err(GitRefusal::TooManyCommits);
    }
    listed
        .iter()
        .map(|id| read_commit(repo, id, limits))
        .collect()
}

fn lines(ran: &crate::git_run::Ran) -> Result<Vec<String>, GitRefusal> {
    if ran.code != Some(0) {
        return Err(GitRefusal::Unreadable);
    }
    let text = String::from_utf8(ran.stdout.clone())
        .map_err(|_| GitRefusal::Unreadable)?;
    Ok(text.lines().map(str::to_owned).collect())
}

fn read_commit(
    repo: &Path,
    id: &str,
    limits: &GitLimits,
) -> Result<NewCommit, GitRefusal> {
    if !is_commit(id) {
        return Err(GitRefusal::Unreadable);
    }
    let raw = git(repo, &["cat-file", "commit", id], None)
        .map_err(|_| GitRefusal::Unreadable)?;
    if raw.code != Some(0) {
        return Err(GitRefusal::Unreadable);
    }
    let text =
        String::from_utf8(raw.stdout).map_err(|_| GitRefusal::Unreadable)?;
    let (headers, message) = text.split_once("\n\n").unwrap_or((&text, ""));
    let mut parents = 0_usize;
    let (mut author, mut committer) = (None, None);
    for line in headers.lines() {
        if line.starts_with("parent ") {
            parents = parents.saturating_add(1);
        } else if let Some(who) = line.strip_prefix("author ") {
            author = author.or_else(|| name_of(who));
        } else if let Some(who) = line.strip_prefix("committer ") {
            committer = committer.or_else(|| name_of(who));
        }
    }
    Ok(NewCommit {
        author: author.ok_or(GitRefusal::Unreadable)?,
        committer: committer.ok_or(GitRefusal::Unreadable)?,
        parents,
        message: message.to_owned(),
        paths: changed_paths(repo, id, limits)?,
    })
}

/// The name in `Name <email> time zone`.
fn name_of(line: &str) -> Option<String> {
    line.rsplit_once(" <").map(|(name, _)| name.to_owned())
}

fn changed_paths(
    repo: &Path,
    id: &str,
    limits: &GitLimits,
) -> Result<Vec<String>, GitRefusal> {
    let ran = git(
        repo,
        &[
            "diff-tree",
            "-r",
            "-z",
            "--raw",
            "-M",
            "--root",
            "--no-commit-id",
            "--no-ext-diff",
            id,
        ],
        None,
    )
    .map_err(|_| GitRefusal::Unreadable)?;
    if ran.code != Some(0) {
        return Err(GitRefusal::Unreadable);
    }
    let text =
        String::from_utf8(ran.stdout).map_err(|_| GitRefusal::Unreadable)?;
    let mut tokens = text.split('\0').filter(|t| !t.is_empty());
    let (mut paths, mut blobs) = (Vec::new(), Vec::new());
    while let Some(meta) = tokens.next() {
        let fields: Vec<&str> =
            meta.trim_start_matches(':').split(' ').collect();
        let [_old_mode, new_mode, _old, new_sha, status] = fields.as_slice()
        else {
            return Err(GitRefusal::Unreadable);
        };
        if !matches!(*new_mode, "100644" | "100755" | "000000") {
            return Err(GitRefusal::NotPlainFile);
        }
        let count = if status.starts_with(['R', 'C']) { 2 } else { 1 };
        for _ in 0..count {
            let path = tokens.next().ok_or(GitRefusal::Unreadable)?;
            if path.rsplit('/').next() == Some(".gitmodules") {
                return Err(GitRefusal::ModulesFile);
            }
            paths.push(path.to_owned());
        }
        if *new_mode != "000000" {
            blobs.push((*new_sha).to_owned());
        }
    }
    check_sizes(repo, &blobs, limits)?;
    Ok(paths)
}

fn check_sizes(
    repo: &Path,
    blobs: &[String],
    limits: &GitLimits,
) -> Result<(), GitRefusal> {
    if blobs.is_empty() {
        return Ok(());
    }
    let input = format!("{}\n", blobs.join("\n"));
    let ran = git(
        repo,
        &["cat-file", "--batch-check=%(objectsize)"],
        Some(&input),
    )
    .map_err(|_| GitRefusal::Unreadable)?;
    for line in lines(&ran)? {
        let size: u64 = line.parse().map_err(|_| GitRefusal::Unreadable)?;
        if size > limits.max_object_bytes {
            return Err(GitRefusal::ObjectTooBig);
        }
    }
    Ok(())
}
