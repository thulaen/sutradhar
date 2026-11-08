//! The commits being pushed, each with its message and the paths it
//! changed, so a commit's `TDD:` line can be checked against them.

use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::patch;
use crate::red_green::is_test_file;
use crate::repo::{NO_COMMIT, Repo};

/// One commit: its message and the paths it added, changed or renamed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commit {
    /// The commit's id.
    pub id: String,
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
                id: id.to_owned(),
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

/// Writes into `dest` the commit's parent with the commit's own test
/// files laid over it: the tree where the named test must be red.
/// Returns `false` when the commit has no parent.
///
/// # Errors
/// When git or tar fails, `dest` cannot be written, or a test file's
/// path would pass through a link in the parent tree.
pub fn parent_with_tests(
    repo: &Repo,
    commit: &Commit,
    dest: &Path,
) -> Result<bool, String> {
    let parent = format!("{}^", commit.id);
    let check = format!("{parent}^{{commit}}");
    if repo
        .git(&["rev-parse", "--verify", "--quiet", &check])
        .is_err()
    {
        return Ok(false);
    }
    let tar = repo.git_bytes(&["archive", "--format=tar", &parent])?;
    let mut unpack = Command::new("tar")
        .args(["-x", "-m", "-C"])
        .arg(dest)
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|e| format!("cannot start tar: {e}"))?;
    let sent = unpack
        .stdin
        .take()
        .map(|mut input| std::io::Write::write_all(&mut input, &tar));
    let unpacked = unpack.wait().map_err(|e| format!("tar: {e}"))?;
    if !unpacked.success() || !matches!(sent, Some(Ok(()))) {
        return Err(format!("cannot unpack the parent of {}", commit.id));
    }
    for path in commit.paths.iter().filter(|p| is_test_file(p)) {
        lay_over(repo, &commit.id, path, dest)?;
    }
    Ok(true)
}

/// Refuses a path that passes through, or ends at, a link inside `dest`,
/// so a link planted in the parent tree cannot make a write land
/// outside it.
fn no_links(dest: &Path, path: &str) -> Result<(), String> {
    let mut at = dest.to_path_buf();
    for part in Path::new(path).components() {
        at.push(part);
        if fs::symlink_metadata(&at).is_ok_and(|m| m.file_type().is_symlink()) {
            return Err(format!("{path} passes through a link"));
        }
    }
    Ok(())
}

/// Writes the commit's copy of `path` into `dest`, or removes it there
/// when the commit deleted it.
fn lay_over(
    repo: &Repo,
    id: &str,
    path: &str,
    dest: &Path,
) -> Result<(), String> {
    no_links(dest, path)?;
    let to = dest.join(path);
    let object = format!("{id}:{path}");
    if repo.git(&["cat-file", "-e", &object]).is_err() {
        let _ignored = fs::remove_file(&to);
        return Ok(());
    }
    let bytes = repo.git_bytes(&["show", &object])?;
    if let Some(folder) = to.parent() {
        fs::create_dir_all(folder)
            .map_err(|e| format!("cannot make {}: {e}", folder.display()))?;
    }
    fs::write(&to, bytes)
        .map_err(|e| format!("cannot write {}: {e}", to.display()))
}

/// Makes a new, empty folder for one red tree, with a name no other run
/// uses, so nothing can be waiting there.
///
/// # Errors
/// When no new folder can be made.
pub fn fresh_folder() -> Result<std::path::PathBuf, String> {
    let base = std::env::temp_dir();
    for attempt in 0_u32..100 {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.subsec_nanos());
        let name =
            format!("aegis-red-{}-{nanos}-{attempt}", std::process::id());
        let path = base.join(name);
        if fs::create_dir(&path).is_ok() {
            return Ok(path);
        }
    }
    Err("cannot make a new folder for the red tree".to_owned())
}
