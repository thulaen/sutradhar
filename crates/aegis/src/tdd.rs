//! The `TDD:` line of a commit message (AGENTS.md section 9). It names
//! the test that failed before the change, as a Rust path whose first
//! part is the crate, such as
//! `shodhan::score::tests::overlap_ranks_exact_phrase_first`. A commit
//! that changes no code says `TDD: none (docs only)`, and then every
//! path it touches must be a Markdown file.

use crate::Finding;

/// The words for a commit that changes no code.
pub const DOCS_ONLY: &str = "none (docs only)";

/// What a `TDD:` line says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tdd {
    /// The commit changes only docs.
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
        })
        .map(|p| {
            Finding::new(
                "commit.tdd",
                p,
                0,
                "the message says `TDD: none (docs only)`, but this is not a \
                 doc: name the test that failed before the change",
            )
        })
        .collect()
}

#[cfg(test)]
#[path = "tdd_tests.rs"]
mod tests;
