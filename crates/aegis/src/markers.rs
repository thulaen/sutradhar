//! The marker check (task #16, carried from the old gate's check 42).
//! Status lives only in Forgejo (AGENTS.md section 2), so a marker word
//! for unfinished work in code must name the Forgejo task that tracks
//! it, as `MARKER(#16)`. A bare marker is refused.
//!
//! Only code, settings and git files are read. Markdown has its own
//! status-word check, and licence texts and tool-written lock files are
//! never ours to change, which keeps the scan to the files that matter.

use crate::Finding;
use crate::file_kinds::Kind;

const CHECK: &str = "code.markers";

/// The marker words, built from parts so this file holds none whole.
fn markers() -> [String; 4] {
    [
        ["TO", "DO"].concat(),
        ["FIX", "ME"].concat(),
        ["X", "XX"].concat(),
        ["HA", "CK"].concat(),
    ]
}

/// Whether this kind of file is read.
fn read(kind: Kind) -> bool {
    !matches!(kind, Kind::Markdown | Kind::Licence | Kind::Lock)
}

/// Whether `rest`, the text just after a marker, names a task: `(#N)`.
fn names_a_task(rest: &str) -> bool {
    let Some(inner) = rest.strip_prefix("(#") else {
        return false;
    };
    let digits = inner.chars().take_while(char::is_ascii_digit).count();
    digits > 0 && inner.get(digits..).is_some_and(|r| r.starts_with(')'))
}

/// Whether `c` can be part of a word.
fn wordish(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// The bare markers on one line.
fn bare(line: &str, word: &str) -> usize {
    line.match_indices(word)
        .filter(|(at, _)| {
            let before = line.get(..*at).and_then(|b| b.chars().last());
            let rest = line.get(at.saturating_add(word.len())..).unwrap_or("");
            let after = rest.chars().next();
            !before.is_some_and(wordish)
                && !after.is_some_and(wordish)
                && !names_a_task(rest)
        })
        .count()
}

/// Checks one file.
#[must_use]
pub fn check(path: &str, kind: Kind, text: &str) -> Vec<Finding> {
    if !read(kind) {
        return Vec::new();
    }
    let words = markers();
    let mut out = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let count: usize = words.iter().map(|w| bare(line, w)).sum();
        for _ in 0..count {
            out.push(Finding::new(
                CHECK,
                path,
                index.saturating_add(1),
                "a marker for unfinished work must name its Forgejo task, \
                 as in MARKER(#16); keep unfinished work out of main",
            ));
        }
    }
    out
}

#[cfg(test)]
#[path = "markers_tests.rs"]
mod tests;
