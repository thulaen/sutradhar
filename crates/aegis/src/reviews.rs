//! The required reviews (task #111; AGENTS.md section 7.1). Every agent
//! follows the same skills in `.claude/skills/`, and each skill ends by
//! writing a review record in `reviews/`. The gate refuses a commit, and
//! the door a push, when any required skill's record is missing, does
//! not pass, has a finding not fixed, or was made for other content.
//!
//! A record names the fingerprint of the content it reviewed: SHA-256
//! over every file of the resulting tree (mode, content id and path,
//! each path with its length before it), leaving out only the record
//! files themselves. The same content gives the same fingerprint at
//! commit and at push, for a plain commit, an amend or a merge; any
//! other content gives another. Run `aegis fingerprint` to see the
//! staged one. Records are not signed yet (Phase 2 item 8), so an agent
//! with full sudo could write a false one; that is a broken rule.
//!
//! Reviews are needed for every commit pushed onto a `main` that holds
//! this check, and for every commit whose parent holds it; so no commit
//! can switch it off, and commits made before it existed need none.

use crate::Finding;
use crate::repo::{NO_COMMIT, Repo};

/// Where the records live, one file per skill.
pub const FOLDER: &str = "reviews/";

/// AGENTS.md section 7.1's eight required reviews.
pub const REQUIRED: [&str; 8] = [
    "architecture",
    "fast-performance",
    "defects",
    "security",
    "backlog",
    "style-docs",
    "red-team",
    "unsafe-patterns",
];

/// The file whose presence in the base's, a parent's or `HEAD`'s tree
/// turns the check on.
const MARK: &str = "crates/aegis/src/reviews.rs";

/// The most bytes a tree's file list may take; ours is a few dozen KB.
const LARGEST_TREE: usize = 64 * 1024 * 1024;

/// One file of a tree: its mode, content id and path.
pub type Entry = (String, String, Vec<u8>);

/// Whether a path is one of the record files.
#[must_use]
pub fn is_record(path: &[u8]) -> bool {
    // Called for every file of every tree, so nothing is built per call.
    path.strip_prefix(FOLDER.as_bytes())
        .and_then(|name| name.strip_suffix(b".toml"))
        .is_some_and(|skill| REQUIRED.iter().any(|s| skill == s.as_bytes()))
}

/// The fingerprint of a tree's files, records left out, in the order
/// git lists the tree: both sides list a tree the same way, and a tree
/// with a name twice cannot pose as another.
#[must_use]
pub fn fingerprint(entries: &[Entry]) -> String {
    let lines: Vec<Vec<u8>> = entries
        .iter()
        .filter(|(_, _, path)| !is_record(path))
        .map(|(mode, id, path)| {
            let mut line = format!("{mode} {id} {}:", path.len()).into_bytes();
            line.extend_from_slice(path);
            line
        })
        .collect();
    // One pass over the joined lines: at most the 64 MiB listing cap.
    let hex = crate::private::sha256(&lines.concat());
    format!("sha256:{hex}")
}

/// Splits git's `-z` listing into entries; `head` reads the part before
/// the tab into (mode, content id).
fn entries(
    raw: &[u8],
    head: fn(&str) -> Option<(String, String)>,
) -> Vec<Entry> {
    raw.split(|b| *b == 0)
        .filter_map(|item| {
            let tab = item.iter().position(|b| *b == b'\t')?;
            let front = item.get(..tab)?;
            let path = item.get(tab.saturating_add(1)..)?;
            let (mode, id) = head(std::str::from_utf8(front).ok()?)?;
            Some((mode, id, path.to_vec()))
        })
        .collect()
}

/// The staged tree's files: git writes the index as a tree, which
/// leaves out files only marked with `git add -N` and works from any
/// folder, then the tree is listed as a commit's would be.
///
/// # Errors
/// When git fails, as it does while a conflict is unresolved.
pub fn staged_entries(repo: &Repo) -> Result<Vec<Entry>, String> {
    let tree = repo.git(&["write-tree"])?;
    commit_entries(repo, tree.trim())
}

/// A commit's tree's files, from `git ls-tree -r` (mode, type, id).
///
/// # Errors
/// When git fails.
pub fn commit_entries(repo: &Repo, id: &str) -> Result<Vec<Entry>, String> {
    let mut command = repo.command();
    let _args = command.args(["ls-tree", "-r", "-z", "--full-tree", id]);
    let raw = crate::git_env::read_capped(&mut command, LARGEST_TREE)?;
    Ok(entries(&raw, |front| {
        let mut parts = front.split(' ');
        let mode = parts.next()?.to_owned();
        let _kind = parts.next()?;
        Some((mode, parts.next()?.to_owned()))
    }))
}

/// What a commit's tree holds: its file count, the sizes of its files
/// added up (a file named many times is counted each time) and its
/// largest file, read from a capped listing.
///
/// # Errors
/// When git fails or the listing is too long.
pub fn tree_totals(repo: &Repo, id: &str) -> Result<(usize, u64, u64), String> {
    const SIZE_FIELD: usize = 3;
    let mut command = repo.command();
    let _args = command.args(["ls-tree", "-r", "-l", "-z", "--full-tree", id]);
    let raw = crate::git_env::read_capped(&mut command, LARGEST_TREE)?;
    let (mut files, mut bytes, mut largest) = (0_usize, 0_u64, 0_u64);
    for item in raw.split(|b| *b == 0).filter(|i| !i.is_empty()) {
        let front = item.split(|b| *b == b'\t').next().unwrap_or_default();
        let size = std::str::from_utf8(front)
            .ok()
            .and_then(|f| f.split_whitespace().nth(SIZE_FIELD))
            .and_then(|s| s.parse::<u64>().ok())
            // Only a submodule has no size, and checkout writes none.
            .unwrap_or(0);
        files = files.saturating_add(1);
        bytes = bytes.saturating_add(size);
        largest = largest.max(size);
    }
    Ok((files, bytes, largest))
}

/// What a git command prints, capped like a tree listing.
///
/// # Errors
/// When git fails or prints too much.
pub fn capped(repo: &Repo, args: &[&str]) -> Result<Vec<u8>, String> {
    let mut command = repo.command();
    let _args = command.args(args);
    crate::git_env::read_capped(&mut command, LARGEST_TREE)
}

/// The staged content's fingerprint.
///
/// # Errors
/// When git fails.
pub fn staged(repo: &Repo) -> Result<String, String> {
    staged_entries(repo).map(|e| fingerprint(&e))
}

/// Whether the tree of `rev` holds the check. A direct look at that one
/// tree, with no search through history, so no merge can hide it.
fn has_check(repo: &Repo, rev: &str) -> Result<bool, String> {
    // `ls-tree` fails when `rev` is not a tree git can read, and prints
    // the path only when it is there: a clear yes or no, never a guess.
    let commit = format!("{rev}^{{commit}}");
    let found = repo.git(&["ls-tree", "--name-only", &commit, "--", MARK])?;
    Ok(found.lines().any(|l| l == MARK))
}

/// The parents of commit `id`.
fn parents(repo: &Repo, id: &str) -> Result<Vec<String>, String> {
    let line = repo.git(&["rev-list", "--parents", "-n", "1", id, "--"])?;
    Ok(line.split_whitespace().skip(1).map(str::to_owned).collect())
}

/// A blob's content, read only when it is small enough to be a record.
fn read_blob(repo: &Repo, spec: &str) -> Option<Result<Vec<u8>, String>> {
    let size = repo.git(&["cat-file", "-s", spec]).ok()?;
    match size.trim().parse::<usize>() {
        Ok(n) if n <= review_record::LARGEST => {
            Some(repo.git_bytes(&["show", spec]))
        }
        _ => Some(Err("the record is too large".to_owned())),
    }
}

/// The reviews of the staged content: needed when `HEAD` holds the
/// check. A repository with no commit needs none.
///
/// # Errors
/// When git fails.
pub fn of_staged(repo: &Repo) -> Result<Vec<Finding>, String> {
    let head = repo.git(&["rev-parse", "--verify", "--quiet", "HEAD"]);
    if head.is_err() || !has_check(repo, "HEAD")? {
        return Ok(Vec::new());
    }
    let entries = staged_entries(repo)?;
    let read = |path: &str| read_blob(repo, &format!(":{path}"));
    let change = fingerprint(&entries);
    Ok(review_record::check(
        &change,
        &entries,
        &read,
        "the staged change",
    ))
}

/// The reviews of one pushed commit, read as they are in that commit.
/// They are needed when `base`, the accepted `main`, holds the check, or
/// any of the commit's parents does; so neither a merge nor a commit
/// with no parent can leave the check behind.
///
/// # Errors
/// When git fails.
pub fn of_pushed(
    repo: &Repo,
    base: &str,
    id: &str,
) -> Result<Vec<Finding>, String> {
    const SHORT_ID: usize = 10;
    let mut needed = base != NO_COMMIT && has_check(repo, base)?;
    for parent in parents(repo, id)? {
        needed = needed || has_check(repo, &parent)?;
    }
    if !needed {
        return Ok(Vec::new());
    }
    let entries = commit_entries(repo, id)?;
    let read = |p: &str| read_blob(repo, &format!("{id}:{p}"));
    let short = id.get(..SHORT_ID).unwrap_or(id);
    let change = fingerprint(&entries);
    let at = format!("commit {short}");
    Ok(review_record::check(&change, &entries, &read, &at))
}

#[path = "review_record.rs"]
mod review_record;

#[cfg(test)]
#[path = "reviews_tests.rs"]
mod tests;
