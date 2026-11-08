//! The door's decision for one pushed ref (AGENTS.md sections 7 to 9).
//! Only `main` and release tags may be pushed, never deleted or forced;
//! a push of `main` must pass the push tier.

use std::path::PathBuf;

use aegis::Finding;
use aegis::repo::{NO_COMMIT, Repo};
use aegis::tiers;

/// The ref that records the last `main` the door accepted, so that every
/// accepted commit stays in the door's copy for good.
pub const ACCEPTED: &str = "refs/door/main";

/// One pushed ref, as Forgejo's hook reports it.
#[derive(Debug, Clone, Copy)]
pub struct Push<'a> {
    /// The ref name, such as `refs/heads/main`.
    pub name: &'a str,
    /// The commit the ref pointed at, or [`NO_COMMIT`].
    pub old: &'a str,
    /// The commit or tag the ref will point at, or [`NO_COMMIT`].
    pub new: &'a str,
    /// Forgejo's `main` before this push, or [`NO_COMMIT`].
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
            Ok(check_tag(&repo, push, commit))
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
    if push.old != NO_COMMIT && !is_ancestor(repo, push.old, commit) {
        return Ok(refused(
            "door.history",
            "rewriting main's history is refused",
        ));
    }
    let _out = repo.git(&["checkout", "-q", "--detach", "--force", commit])?;
    let _out =
        repo.git(&["clean", "-q", "-f", "-d", "-x", "-e", "/target/"])?;
    let found = tiers::push(repo, push.old, commit)?;
    if found.is_empty() {
        let _out = repo.git(&["update-ref", ACCEPTED, commit])?;
    }
    Ok(found)
}
fn check_tag(repo: &Repo, push: Push<'_>, commit: &str) -> Vec<Finding> {
    if push.old != NO_COMMIT {
        return refused("door.tag", "replacing a release tag is refused");
    }
    if push.main == NO_COMMIT || !is_ancestor(repo, commit, push.main) {
        return refused("door.tag", "a release tag must name a commit on main");
    }
    Vec::new()
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
