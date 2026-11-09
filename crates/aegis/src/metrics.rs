//! The ban on vanity numbers (task #65; ADR-0013). A vanity number looks
//! good but changes no decision. A number may be reported only when
//! `gate/metrics.toml` lists it with one approved source, the decision it
//! serves (at least four words) and what it is compared with. The list
//! also names known vanity names; each is refused in code, settings and
//! scripts in any spelling (snake, camel or kebab case, and as part of a
//! longer name), with its file and line. Test files may name them, so the
//! check itself can be tested; nothing else may, and there is no note
//! that switches the check off. A missing or broken list blocks.

use std::collections::BTreeSet;

use serde::Deserialize;

use crate::Finding;
use crate::file_kinds::Kind;

const CHECK: &str = "metrics.no-vanity";

/// Where the list lives.
pub const PATH: &str = "gate/metrics.toml";

/// A decision shorter than this says nothing about what would be done.
const MIN_DECISION_WORDS: usize = 4;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct List {
    sources: Vec<String>,
    vanity_names: Vec<String>,
    #[serde(default)]
    metric: Vec<Metric>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Metric {
    id: String,
    source: String,
    decision: String,
    compared_with: String,
}

/// The vanity names, each split into its lower-case parts.
#[derive(Debug, Default)]
pub struct Names(Vec<Vec<String>>);

fn problem(path: &str, line: usize, message: impl Into<String>) -> Finding {
    Finding::new(CHECK, path, line, message)
}

/// Splits a name into lower-case parts: `dayCount`, `day_count` and
/// `day-count` all give `day`, `count`.
fn parts(name: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut part = String::new();
    let mut lower_before = false;
    for c in name.chars() {
        let starts = c.is_uppercase() && lower_before;
        if (!c.is_alphanumeric() || starts) && !part.is_empty() {
            out.push(std::mem::take(&mut part));
        }
        if c.is_alphanumeric() {
            part.extend(c.to_lowercase());
        }
        // A digit carries on what came before it, so `A0A` is one part
        // and `a0A` is two.
        lower_before = c.is_lowercase() || (c.is_ascii_digit() && lower_before);
    }
    if !part.is_empty() {
        out.push(part);
    }
    out
}

fn check_list(list: &List) -> Vec<String> {
    let mut found = Vec::new();
    if list.sources.is_empty() || list.vanity_names.is_empty() {
        found.push("the list names no sources or no vanity names".to_owned());
    }
    let vanity: Vec<Vec<String>> =
        list.vanity_names.iter().map(|n| parts(n)).collect();
    let mut seen = BTreeSet::new();
    for m in &list.metric {
        let id = &m.id;
        if parts(id).is_empty() {
            found.push("a number has an empty id".to_owned());
        }
        if !seen.insert(id.as_str()) {
            found.push(format!("`{id}` is listed twice"));
        }
        if vanity.contains(&parts(id)) {
            found.push(format!("`{id}` is a vanity name"));
        }
        if !list.sources.contains(&m.source) {
            found.push(format!("`{id}` has no single approved source"));
        }
        if m.decision.split_whitespace().count() < MIN_DECISION_WORDS {
            found.push(format!("`{id}` names no real decision"));
        }
        if m.compared_with.trim().is_empty() {
            found.push(format!("`{id}` says nothing to compare it with"));
        }
    }
    found
}

/// Reads the list (`text`, or `None` when it is missing). Problems with
/// the list go into `out`.
#[must_use]
pub fn names(text: Option<&str>, out: &mut Vec<Finding>) -> Names {
    let Some(text) = text else {
        out.push(problem(PATH, 0, "the list of allowed numbers is missing"));
        return Names::default();
    };
    match toml::from_str::<List>(text) {
        Ok(list) => {
            out.extend(
                check_list(&list).into_iter().map(|m| problem(PATH, 0, m)),
            );
            Names(list.vanity_names.iter().map(|n| parts(n)).collect())
        }
        Err(e) => {
            out.push(problem(PATH, 0, format!("cannot read it: {e}")));
            Names::default()
        }
    }
}

/// Reads the list from the repository.
#[must_use]
pub fn from_repo(repo: &crate::repo::Repo, out: &mut Vec<Finding>) -> Names {
    names(repo.read(PATH).as_deref(), out)
}

/// Whether a file is a test, which may name vanity numbers.
fn is_test(path: &str) -> bool {
    path.starts_with("tests/")
        || path.contains("/tests/")
        || path.ends_with("_tests.rs")
}

/// Whether this kind of file is read.
fn read(path: &str, kind: Kind) -> bool {
    !matches!(
        kind,
        Kind::Markdown | Kind::Licence | Kind::Lock | Kind::Snapshot
    ) && path != PATH
        && !is_test(path)
}

/// Whether the parts of a name hold `vanity` as a run of parts.
fn holds(name: &[String], vanity: &[String]) -> bool {
    !vanity.is_empty() && name.windows(vanity.len()).any(|w| w == vanity)
}

/// Checks one file.
#[must_use]
pub fn check(
    path: &str,
    kind: Kind,
    text: &str,
    names: &Names,
) -> Vec<Finding> {
    if !read(path, kind) {
        return Vec::new();
    }
    let mut out = Vec::new();
    for (number, line) in text.lines().enumerate() {
        let words = line
            .split(|c: char| !(c.is_alphanumeric() || c == '_' || c == '-'));
        let hit = words.map(parts).find_map(|name| {
            names
                .0
                .iter()
                .find(|v| holds(&name, v))
                .map(|v| v.join("_"))
        });
        if let Some(name) = hit {
            out.push(problem(
                path,
                number.saturating_add(1),
                format!(
                    "`{name}` is a vanity number; report a number from {PATH} \
                     instead"
                ),
            ));
        }
    }
    out
}

#[cfg(test)]
#[path = "metrics_tests.rs"]
mod tests;
