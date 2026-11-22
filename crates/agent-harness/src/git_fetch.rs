//! Fetching the agent's commit into the harness's own repository (spec
//! 0155, "Push", step 1). The clone path comes from the owner's account
//! file, never from the request. Objects are checked in full as they
//! come in, and nothing from the clone is executed.

use std::path::Path;

use crate::agent::AgentName;
use crate::git_run::{git, git_with};
use crate::request_check::is_commit;

/// The limit on the harness's repository (proposed in spec 0155).
#[derive(Debug, Clone, Copy)]
pub struct FetchLimits {
    /// The most bytes the repository may hold.
    pub max_repo_bytes: u64,
}

/// Bytes in a kibibyte, for the sizes `git count-objects` gives.
const KIB: u64 = 1024;

/// Where the agents' branches are kept in the harness's repository.
const AGENTS: &str = "refs/agent";

refusals! {
    /// Why a commit could not be fetched.
    FetchRefusal {
        /// The id is not a full commit id.
        BadCommitId => "the id is not a full commit id",
        /// The clone path is not a plain absolute path to a folder.
        BadClonePath => "the clone path is not a plain absolute folder",
        /// The clone borrows objects, is a link or is not a repository.
        UnsafeClone => "the clone borrows objects or is not a plain repository",
        /// Git could not fetch from the clone, or an object was malformed.
        FetchFailed => "the fetch from the clone failed",
        /// The commit is on no branch the clone offers.
        CommitNotOffered => "the commit is on no branch the clone offers",
        /// The harness's repository is over its size limit.
        RepoTooBig => "the harness repository is over its size limit",
    }
}

/// A plain absolute path: letters, digits, `.`, `_`, `-` and `/`, with no
/// `..` part and no link in it, so it can neither hide an option nor
/// reach elsewhere.
fn is_plain(path: &Path) -> bool {
    let Some(text) = path.to_str() else {
        return false;
    };
    path.is_absolute()
        && text
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._-/".contains(&b))
        && text.split('/').all(|part| part != "..")
        && std::fs::canonicalize(path).is_ok_and(|real| real == path)
}

/// Whether the clone is a repository of its own: `.git` is a folder (or
/// the clone is bare), nothing in it is a link, and it borrows no
/// objects from anywhere else.
fn is_own_repository(path: &Path) -> bool {
    let git_dir = path.join(".git");
    let dir = if git_dir.is_dir() {
        git_dir
    } else if path.join("HEAD").is_file() {
        path.to_path_buf()
    } else {
        return false;
    };
    let is_real_dir =
        |p: &Path| std::fs::symlink_metadata(p).is_ok_and(|m| m.is_dir());
    let info = dir.join("objects").join("info");
    is_real_dir(&dir.join("objects"))
        && is_real_dir(&dir.join("refs"))
        && !info.join("alternates").exists()
        && !info.join("http-alternates").exists()
}

/// Fetches every branch of `clone` into `repo`, under the namespace of
/// `agent`, and checks that `commit` came with them. A refused fetch is
/// discarded, so it cannot fill the repository for the other agents.
///
/// # Errors
/// The rule the fetch breaks.
pub fn fetch_commit(
    repo: &Path,
    clone: &Path,
    commit: &str,
    agent: &AgentName,
    limits: FetchLimits,
) -> Result<(), FetchRefusal> {
    if !is_commit(commit) {
        return Err(FetchRefusal::BadCommitId);
    }
    if !is_plain(clone) || !clone.is_dir() {
        return Err(FetchRefusal::BadClonePath);
    }
    if !is_own_repository(clone) {
        return Err(FetchRefusal::UnsafeClone);
    }
    let heads = format!("{AGENTS}/{}/heads", agent.as_str());
    let result = fetch_into(repo, clone, &heads)
        .and_then(|()| check_offered(repo, commit, &heads))
        .and_then(|()| check_size(repo, limits));
    if result.is_err() {
        discard(repo, &heads);
    }
    result
}

fn fetch_into(
    repo: &Path,
    clone: &Path,
    heads: &str,
) -> Result<(), FetchRefusal> {
    let from = clone.to_str().ok_or(FetchRefusal::BadClonePath)?;
    let spec = format!("+refs/heads/*:{heads}/*");
    let safe = format!("safe.directory={from}");
    let args = [
        "-c",
        "fetch.fsckObjects=true",
        "-c",
        "transfer.fsckObjects=true",
        "-c",
        "protocol.file.allow=always",
        "-c",
        &safe,
        "fetch",
        "--quiet",
        "--no-tags",
        "--prune",
        "--no-auto-gc",
        "--no-recurse-submodules",
        "--no-write-fetch-head",
        "--",
        from,
        &spec,
    ];
    let ran = git_with(repo, &args, None, &[("GIT_ALLOW_PROTOCOL", "file")])
        .map_err(|_| FetchRefusal::FetchFailed)?;
    if ran.code == Some(0) {
        Ok(())
    } else {
        Err(FetchRefusal::FetchFailed)
    }
}

/// Deletes the agent's refs and then the objects nothing else holds.
fn discard(repo: &Path, heads: &str) {
    let listed = git(
        repo,
        &["for-each-ref", "--format=delete %(refname)", heads],
        None,
    );
    if let Ok(ran) = listed
        && let Ok(commands) = String::from_utf8(ran.stdout)
        && !commands.is_empty()
    {
        let _deleted = git(repo, &["update-ref", "--stdin"], Some(&commands));
    }
    tidy(repo);
}

/// Removes the objects no ref holds. The service also runs it on a
/// timer, so deleted branches do not leave objects behind.
pub fn tidy(repo: &Path) {
    let _gc = git(repo, &["gc", "--quiet", "--prune=now"], None);
}

fn check_offered(
    repo: &Path,
    commit: &str,
    heads: &str,
) -> Result<(), FetchRefusal> {
    let on = format!("--contains={commit}");
    let ran = git(
        repo,
        &["for-each-ref", &on, "--format=%(refname)", heads],
        None,
    )
    .map_err(|_| FetchRefusal::FetchFailed)?;
    if ran.code == Some(0) && !ran.stdout.is_empty() {
        Ok(())
    } else {
        Err(FetchRefusal::CommitNotOffered)
    }
}

fn check_size(repo: &Path, limits: FetchLimits) -> Result<(), FetchRefusal> {
    let ran = git(repo, &["count-objects", "-v"], None)
        .map_err(|_| FetchRefusal::FetchFailed)?;
    let text =
        String::from_utf8(ran.stdout).map_err(|_| FetchRefusal::FetchFailed)?;
    let kib: u64 = text
        .lines()
        .filter_map(|line| line.split_once(": "))
        .filter(|(key, _)| {
            matches!(*key, "size" | "size-pack" | "size-garbage")
        })
        .filter_map(|(_, value)| value.trim().parse::<u64>().ok())
        .fold(0, u64::saturating_add);
    if kib.saturating_mul(KIB) > limits.max_repo_bytes {
        Err(FetchRefusal::RepoTooBig)
    } else {
        Ok(())
    }
}
