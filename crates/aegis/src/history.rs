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

/// The most text, and lines, the change of one push may hold. The
/// project's whole history is about 2.5 MB and 60,000 lines.
const MOST_PATCH_BYTES: usize = 32 * 1024 * 1024;
const MOST_PATCH_LINES: usize = 1_000_000;
/// The longest file name a change may hold; ours is 58 bytes.
const MOST_PATH_BYTES: usize = 512;

/// The length of the name on a `--- a/NAME` or `+++ b/NAME` line: git
/// ends a name holding a space with a tab, which is not part of it. A
/// name git must still quote (one holding `"`, `\` or a control byte) is
/// measured as quoted, so it can only be refused early, never let in;
/// one that is not valid text counts as too long.
fn name_bytes(line: &[u8]) -> usize {
    let name = line.get(6..).unwrap_or_default();
    let name = name.strip_suffix(b"\t").unwrap_or(name);
    // A name that is not valid text grows when it is read, so it cannot
    // be measured honestly and is refused.
    if std::str::from_utf8(name).is_err() {
        return usize::MAX;
    }
    name.len()
}

/// Every line added by the commits after `base` up to `head`, with
/// the file it was added to.
///
/// # Errors
/// When git fails, or the change is over 32 MiB, a million lines or
/// names a file longer than 512 bytes.
pub fn added_lines(
    repo: &Repo,
    base: &str,
    head: &str,
) -> Result<Vec<(String, String)>, String> {
    let range = crate::repo::range(base, head);
    let args = [
        // Names as they are, not as escape codes, so a short name in
        // another script is not measured as a long one.
        "-c",
        "core.quotePath=false",
        "log",
        "-p",
        // A rename as a delete and an add, so the new name shows with
        // every line it adds (an empty file adds none, so copies none).
        "--no-renames",
        // Every file as text, whatever `.gitattributes` says, so a file
        // marked `-diff` or given a text filter cannot hide its lines.
        "--text",
        "--no-textconv",
        "--no-ext-diff",
        "--no-color",
        "--diff-merges=first-parent",
        "--format=",
        "--end-of-options",
        &range,
    ];
    let mut command = repo.command();
    let _args = command.args(args);
    let raw = crate::git_env::read_capped(&mut command, MOST_PATCH_BYTES)?;
    // A small pack can unpack into a huge change, so the lines are
    // counted before they are split up and copied.
    // Each added line keeps its file's name, so a long name is refused
    // too. Each name is measured on its own `--- a/` or `+++ b/` line in
    // a file's header (before its first `@@`), so a rename cannot carry
    // a name twice as long.
    let mut lines = 0_usize;
    let mut in_header = false;
    for line in raw.split(|b| *b == b'\n') {
        lines = lines.saturating_add(1);
        if line.starts_with(b"diff --git ") {
            in_header = true;
            // A change of mode only, or an empty new file, has no `---`
            // or `+++` lines, so the line holding both names is measured
            // too: `diff --git a/NAME b/NAME` is twice the name plus 16
            // bytes of its own. A line that is not valid text is refused.
            let both = MOST_PATH_BYTES.saturating_mul(2).saturating_add(16);
            if line.len() > both || std::str::from_utf8(line).is_err() {
                return Err(format!(
                    "the push names a file longer than {MOST_PATH_BYTES} bytes"
                ));
            }
        } else if line.starts_with(b"@@") {
            in_header = false;
        } else if in_header
            && (line.starts_with(b"--- ") || line.starts_with(b"+++ "))
            && name_bytes(line) > MOST_PATH_BYTES
        {
            return Err(format!(
                "the push names a file longer than {MOST_PATH_BYTES} bytes"
            ));
        }
    }
    // The piece after the last line break is not a line.
    let lines = lines.saturating_sub(1);
    if lines > MOST_PATCH_LINES {
        return Err(format!(
            "the push changes {lines} lines; the most is {MOST_PATCH_LINES}"
        ));
    }
    Ok(patch::added_lines(&String::from_utf8_lossy(&raw)))
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
        if !in_new_crate(dest, path) {
            lay_over(repo, &commit.id, path, dest)?;
        }
    }
    Ok(true)
}

/// Whether `path` is in a crate the parent tree in `dest` lacks, such as
/// `crates/new/tests/x.rs` when `crates/new/Cargo.toml` is not there.
/// Its tests cannot run on the parent, and laying them over would leave a
/// crate folder with no manifest, which stops cargo for the whole tree.
fn in_new_crate(dest: &Path, path: &str) -> bool {
    let mut parts = path.split('/');
    match (parts.next(), parts.next()) {
        (Some("crates"), Some(name)) => {
            !dest.join("crates").join(name).join("Cargo.toml").exists()
        }
        _ => false,
    }
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

/// Makes a new, empty folder for one red tree inside `base`, with a name
/// no other run uses, so nothing can be waiting there.
///
/// # Errors
/// When no new folder can be made.
pub fn fresh_folder(base: &Path) -> Result<std::path::PathBuf, String> {
    fs::create_dir_all(base)
        .map_err(|e| format!("cannot make {}: {e}", base.display()))?;
    for attempt in 0_u32..100 {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.subsec_nanos());
        let name = format!("red-{}-{nanos}-{attempt}", std::process::id());
        let path = base.join(name);
        if fs::create_dir(&path).is_ok() {
            return Ok(path);
        }
    }
    Err("cannot make a new folder for the red tree".to_owned())
}

/// Where the checks keep their working copies: inside the build folder,
/// which git ignores and a sealed check may write.
#[must_use]
pub fn work_folder(root: &Path) -> std::path::PathBuf {
    root.join("target").join("aegis-work")
}
