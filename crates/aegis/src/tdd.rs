//! The `TDD:` line of a commit message (AGENTS.md section 9). It names
//! the test that failed before the change, as a Rust path whose first
//! part is the crate, such as
//! `shodhan::score::tests::overlap_ranks_exact_phrase_first`. A source
//! change needs that proof. A commit that changes no code says
//! `TDD: none (docs only)`. Proved image-digest data changes may use that
//! form too, without exempting executable configuration.

use crate::Finding;
use crate::repo::Repo;

#[path = "tdd_config.rs"]
mod config;

/// The words for a commit that changes no code.
pub const DOCS_ONLY: &str = "none (docs only)";

/// What a `TDD:` line says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tdd {
    /// The commit declares no source change.
    DocsOnly,
    /// The test that failed before: its crate and the rest of its path.
    Test {
        /// The crate, as Rust writes it (`forge_sync`).
        krate: String,
        /// The path inside the crate, such as `plan::tests::x`.
        path: String,
    },
}

fn is_segment(s: &str) -> bool {
    s.chars()
        .next()
        .is_some_and(|c| c.is_ascii_lowercase() || c == '_')
        && s.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

/// Reads the value of a `TDD:` line, or `None` when it is neither form.
#[must_use]
pub fn parse(value: &str) -> Option<Tdd> {
    if value == DOCS_ONLY {
        return Some(Tdd::DocsOnly);
    }
    let (krate, path) = value.split_once("::")?;
    let all = std::iter::once(krate).chain(path.split("::"));
    all.clone().all(is_segment).then(|| Tdd::Test {
        krate: krate.to_owned(),
        path: path.to_owned(),
    })
}

/// The value of the message's `TDD:` line, read as the message check
/// reads it.
#[must_use]
pub fn value(message: &str) -> Option<String> {
    crate::commit_msg::trailer(message, "TDD:")
}

/// Checks that a commit saying `TDD: none (docs only)` touches only
/// Markdown files and the required review records, which every commit
/// carries (AGENTS.md section 7.1).
#[must_use]
pub fn check_paths(message: &str, paths: &[String]) -> Vec<Finding> {
    with_changes(message, paths, |_| None)
}

/// Checks exact Git blobs as well as paths. Only proved image-digest
/// deltas may extend the context-free no-code allowance. An empty
/// revision names the index, including the index supplied to a hook.
#[must_use]
pub fn check(
    repo: &Repo,
    message: &str,
    paths: &[String],
    before: &str,
    after: &str,
) -> Vec<Finding> {
    if value(message).as_deref().and_then(parse) != Some(Tdd::DocsOnly) {
        return Vec::new();
    }
    let before = tree(repo, before);
    let after = tree(repo, after);
    with_changes(message, paths, |path| {
        blob(repo, before.as_deref()?, path).zip(blob(
            repo,
            after.as_deref()?,
            path,
        ))
    })
}

/// Applies the same proof to one commit and its first parent.
#[must_use]
pub fn committed(repo: &Repo, commit: &crate::history::Commit) -> Vec<Finding> {
    check(
        repo,
        &commit.message,
        &commit.paths,
        &format!("{}^", commit.id),
        &commit.id,
    )
}

fn tree(repo: &Repo, revision: &str) -> Option<String> {
    if revision.is_empty() {
        return Some(String::new());
    }
    let tree = repo
        .git(&["rev-parse", "--verify", &format!("{revision}^{{tree}}")])
        .ok()?;
    object_id(tree.trim())
}

fn object_id(value: &str) -> Option<String> {
    (matches!(value.len(), 40 | 64)
        && value.bytes().all(|byte| byte.is_ascii_hexdigit()))
    .then(|| value.to_owned())
}

fn blob(repo: &Repo, revision: &str, path: &str) -> Option<String> {
    // The index may change. Its accepted mode, size and bytes must all
    // refer to the one immutable object named by its listing.
    let object = plain_blob(repo, revision, path)?;
    let size = repo.git(&["cat-file", "-s", &object]).ok()?;
    if size.trim().parse::<u64>().ok()? > crate::repo::MOST_READ {
        return None;
    }
    String::from_utf8(repo.git_bytes(&["cat-file", "blob", &object]).ok()?).ok()
}

fn plain_blob(repo: &Repo, revision: &str, path: &str) -> Option<String> {
    let entry = if revision.is_empty() {
        repo.git(&[
            "--literal-pathspecs",
            "ls-files",
            "--stage",
            "-z",
            "--",
            path,
        ])
    } else {
        repo.git(&[
            "--literal-pathspecs",
            "ls-tree",
            "-z",
            revision,
            "--",
            path,
        ])
    };
    let entry = entry.ok()?;
    let (metadata, name) = entry
        .strip_suffix('\0')
        .and_then(|record| record.split_once('\t'))?;
    if name != path {
        return None;
    }
    let fields: Vec<_> = metadata.split(' ').collect();
    match fields.as_slice() {
        ["100644", object, "0"] if revision.is_empty() => object_id(object),
        ["100644", "blob", object] if !revision.is_empty() => object_id(object),
        _ => None,
    }
}

fn with_changes(
    message: &str,
    paths: &[String],
    read: impl Fn(&str) -> Option<(String, String)>,
) -> Vec<Finding> {
    if value(message).as_deref().and_then(parse) != Some(Tdd::DocsOnly) {
        return Vec::new();
    }
    paths
        .iter()
        .filter(|p| {
            !std::path::Path::new(p)
                .extension()
                .is_some_and(|e| e.eq_ignore_ascii_case("md"))
                && !crate::reviews::is_record(p.as_bytes())
                && !config::data_only(p, &read)
        })
        .map(|p| {
            Finding::new(
                "commit.tdd",
                p,
                0,
                "this path needs a named test that failed before the \
                 change; its no-code content has not been proved",
            )
        })
        .collect()
}

#[cfg(test)]
#[path = "tdd_tests.rs"]
mod tests;
