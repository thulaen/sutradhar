//! The door's decision for one pushed ref (AGENTS.md sections 7 to 9).
//! Only `main` and release tags may be pushed, never deleted or forced;
//! a push of `main` must pass the push tier.

use std::path::PathBuf;

use aegis::Finding;
use aegis::repo::{NO_COMMIT, Repo};
use aegis::tiers;

/// The ref that records the last `main` the door accepted. Every accepted
/// commit stays in the door's copy for good, and this ref, not Forgejo's
/// report, is the base for history, the push tier and release tags.
pub const ACCEPTED: &str = "refs/door/main";

/// The most commits one push may bring.
const MOST_COMMITS: usize = 500;
/// The most files, and bytes in all, a pushed tree may hold, counted
/// before anything is written out: a small pack can name one file a
/// million times.
const MOST_FILES: usize = 200_000;
const MOST_BYTES: u64 = 64 * 1024 * 1024;
/// The largest one file may be; ours is about 130 KB.
const MOST_FILE_BYTES: u64 = 1024 * 1024;

/// One pushed ref, as Forgejo's hook reports it.
#[derive(Debug, Clone, Copy)]
pub struct Push<'a> {
    /// The ref name, such as `refs/heads/main`.
    pub name: &'a str,
    /// The commit the ref pointed at, or [`NO_COMMIT`].
    pub old: &'a str,
    /// The commit or tag the ref will point at, or [`NO_COMMIT`].
    pub new: &'a str,
    /// Forgejo's `main` before this push, or [`NO_COMMIT`]. Used only for
    /// release tags, which must also be on the door's own record.
    pub main: &'a str,
}

/// The door's own copy of the repository, where pushed objects are
/// unpacked and checked.
#[derive(Debug, Clone)]
pub struct Door {
    work: PathBuf,
}

impl Door {
    /// A door that works in `work`, a git repository holding at least the
    /// commits already accepted.
    #[must_use]
    pub fn new(work: impl Into<PathBuf>) -> Self {
        Self { work: work.into() }
    }

    /// Checks one pushed ref. `pack` holds the new objects, as
    /// `git pack-objects` writes them; it may be empty.
    ///
    /// # Errors
    /// When the push cannot be checked at all; it is then refused.
    pub fn check(
        &self,
        push: Push<'_>,
        pack: &[u8],
    ) -> Result<Vec<Finding>, String> {
        for id in [push.old, push.new, push.main] {
            if !is_object_id(id) {
                return Err(format!("`{id}` is not an object id"));
            }
        }
        if push.new == NO_COMMIT {
            return Ok(refused("door.delete", "deleting a ref is refused"));
        }
        let repo = Repo::at(&self.work);
        if !pack.is_empty() {
            let _out = repo.git_with_input(&["unpack-objects", "-q"], pack)?;
        }
        let commit =
            repo.git(&["rev-parse", "--verify", &commit_of(push.new)])?;
        let commit = commit.trim();
        if push.name == "refs/heads/main" {
            check_main(&repo, push, commit)
        } else if is_release_tag(push.name) {
            check_tag(&repo, push, commit)
        } else {
            Ok(refused(
                "door.ref",
                "only main and release tags may be pushed",
            ))
        }
    }
}

fn check_main(
    repo: &Repo,
    push: Push<'_>,
    commit: &str,
) -> Result<Vec<Finding>, String> {
    let accepted = accepted(repo)?;
    let base = accepted.as_deref().unwrap_or(push.old);
    if base != NO_COMMIT && !is_ancestor(repo, base, commit) {
        return Ok(refused(
            "door.history",
            "rewriting main's history is refused",
        ));
    }
    // Before anything is written out: too many commits, or a tree too
    // big, is refused, so a small push cannot fill the disk or keep the
    // door busy for hours. The very first push, of a whole history, has
    // no commit limit.
    if base != NO_COMMIT {
        let range = format!("{base}..{commit}");
        let count = repo.git(&["rev-list", "--count", &range, "--"])?;
        if count
            .trim()
            .parse::<usize>()
            .map_or(true, |n| n > MOST_COMMITS)
        {
            return Ok(refused(
                "door.size",
                "a push may bring at most 500 commits; push in smaller parts",
            ));
        }
    }
    if let Some(why) = big_object(repo, base, commit) {
        return Ok(refused("door.size", &why));
    }
    if let Some(why) = too_big(repo, commit) {
        return Ok(refused("door.size", &why));
    }
    // git's own override of `.gitattributes`: no filter a pushed project
    // names (such as git-lfs) runs during checkout (task #119).
    let found = repo.git(&["rev-parse", "--git-path", "info/attributes"])?;
    let attributes = repo.root().join(found.trim());
    attributes
        .parent()
        .map_or(Ok(()), std::fs::create_dir_all)
        .and_then(|()| std::fs::write(&attributes, "* -filter\n"))
        .map_err(|e| format!("cannot switch filters off: {e}"))?;
    let _out = repo.git(&["checkout", "-q", "--detach", "--force", commit])?;
    let _out =
        repo.git(&["clean", "-q", "-f", "-d", "-x", "-e", "/target/"])?;
    let found = tiers::push(repo, base, commit)?;
    if found.is_empty() {
        let _out = repo.git(&["update-ref", ACCEPTED, commit])?;
    }
    Ok(found)
}
fn check_tag(
    repo: &Repo,
    push: Push<'_>,
    commit: &str,
) -> Result<Vec<Finding>, String> {
    if push.old != NO_COMMIT {
        return Ok(refused("door.tag", "replacing a release tag is refused"));
    }
    // Only a commit the door accepted, and that is on Forgejo's main too,
    // may be released: the door can accept a main that Forgejo then
    // turns down when another part of the same push fails.
    let on_main = push.main != NO_COMMIT
        && is_ancestor(repo, commit, push.main)
        && accepted(repo)?.is_some_and(|a| is_ancestor(repo, commit, &a));
    if !on_main {
        return Ok(refused(
            "door.tag",
            "a release tag must name a commit the door accepted on main",
        ));
    }
    Ok(Vec::new())
}

/// The last `main` the door accepted, which is the base for every check:
/// Forgejo's own record of `main` could be lost or reset outside the
/// door. `None` only when the door has accepted nothing yet; any other
/// failure to read it refuses the push.
fn accepted(repo: &Repo) -> Result<Option<String>, String> {
    // `for-each-ref` prints nothing for a missing ref and fails on a
    // fault, so the two cannot be mixed up.
    let listed =
        repo.git(&["for-each-ref", "--format=%(objectname)", ACCEPTED])?;
    if listed.trim().is_empty() {
        return Ok(None);
    }
    let id = repo.git(&["rev-parse", "--verify", &commit_of(ACCEPTED)])?;
    Ok(Some(id.trim().to_owned()))
}

/// Why some file the push brings, in any of its commits, is too big, if
/// one is: git's own size filter lists every object of the push and
/// marks each file over the limit, before any of them is read.
fn big_object(repo: &Repo, base: &str, commit: &str) -> Option<String> {
    let filter =
        format!("--filter=blob:limit={}", MOST_FILE_BYTES.saturating_add(1));
    let mut args = vec![
        "rev-list",
        "--objects",
        "--filter-print-omitted",
        &filter,
        commit,
    ];
    let not_base = format!("^{base}");
    if base != NO_COMMIT {
        args.push(&not_base);
    }
    match aegis::reviews::capped(repo, &args) {
        Ok(listed)
            if listed.split(|b| *b == b'\n').any(|l| l.starts_with(b"~")) =>
        {
            Some(format!(
                "the push brings a file over {MOST_FILE_BYTES} bytes"
            ))
        }
        Ok(_) => None,
        Err(why) => Some(format!("the push cannot be listed: {why}")),
    }
}

/// Why the tree of `commit` is too big to write out, if it is.
fn too_big(repo: &Repo, commit: &str) -> Option<String> {
    let (files, bytes, largest) =
        match aegis::reviews::tree_totals(repo, commit) {
            Ok(totals) => totals,
            Err(why) => {
                return Some(format!(
                    "the pushed tree cannot be listed: {why}"
                ));
            }
        };
    if largest > MOST_FILE_BYTES {
        return Some(format!(
            "the pushed tree holds a file of {largest} bytes; the most is \
             {MOST_FILE_BYTES}"
        ));
    }
    (files > MOST_FILES || bytes > MOST_BYTES).then(|| {
        format!(
            "the pushed tree holds {files} files and {bytes} bytes; the most \
             is {MOST_FILES} files and {MOST_BYTES} bytes"
        )
    })
}

fn refused(check: &'static str, message: &str) -> Vec<Finding> {
    vec![Finding::new(check, "", 0, message)]
}

fn commit_of(id: &str) -> String {
    format!("{id}^{{commit}}")
}

/// True for 40 lowercase hexadecimal characters.
#[must_use]
pub fn is_object_id(id: &str) -> bool {
    id.len() == 40 && id.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f'))
}

/// True for `refs/tags/v` followed by a digit and then only digits,
/// letters, `.` or `-`, such as `refs/tags/v1.0.0-rc.1`.
#[must_use]
pub fn is_release_tag(name: &str) -> bool {
    name.strip_prefix("refs/tags/v").is_some_and(|rest| {
        rest.starts_with(|c: char| c.is_ascii_digit())
            && rest
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
    })
}

fn is_ancestor(repo: &Repo, old: &str, new: &str) -> bool {
    repo.git(&["merge-base", "--is-ancestor", old, new]).is_ok()
}

#[cfg(test)]
mod tests {
    use super::{is_object_id, is_release_tag};

    #[test]
    fn knows_an_object_id() {
        assert!(is_object_id(&"a1".repeat(20)));
        assert!(!is_object_id("a1"));
        assert!(!is_object_id(&"A1".repeat(20)));
        assert!(!is_object_id(&"g1".repeat(20)));
    }

    #[test]
    fn knows_a_release_tag() {
        assert!(is_release_tag("refs/tags/v1.0.0"));
        assert!(is_release_tag("refs/tags/v1.0.0-rc.1"));
        for name in [
            "refs/tags/vanity",
            "refs/tags/v",
            "refs/tags/v1/x",
            "refs/heads/v1",
            "refs/tags/1.0",
        ] {
            assert!(!is_release_tag(name), "{name}");
        }
    }
}
