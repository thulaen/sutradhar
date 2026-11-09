//! The docs checks: line length, links and status words (AGENTS.md
//! sections 2 and 12).

use crate::Finding;

const MAX_LINE: usize = 80;
const STATUS_WORDS: [&str; 8] = [
    "in progress",
    "not started",
    "work in progress",
    "is done",
    "are done",
    "todo",
    "tbd",
    "wip",
];

/// What a path in the repository is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lookup {
    /// No such path.
    Missing,
    /// A folder.
    Folder,
    /// A file, with its text.
    File(String),
}

/// Runs every docs check on one Markdown file.
#[must_use]
pub fn check(
    path: &str,
    text: &str,
    lookup: &dyn Fn(&str) -> Lookup,
) -> Vec<Finding> {
    let mut out = Vec::new();
    for (index, line) in prose_lines(text) {
        let number = index.saturating_add(1);
        out.extend(long_line(path, number, line));
        out.extend(status_words(path, number, line));
        out.extend(links(path, number, line, lookup));
    }
    out
}

/// The lines outside fenced code blocks (```` ``` ```` or `~~~`), with
/// their index.
fn prose_lines(text: &str) -> Vec<(usize, &str)> {
    let mut fence: Option<&str> = None;
    let mut out = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let trimmed = line.trim_start();
        let mark = ["```", "~~~"].into_iter().find(|m| trimmed.starts_with(m));
        match (fence, mark) {
            (None, Some(m)) => fence = Some(m),
            (Some(open), Some(m)) if open == m => fence = None,
            (None, None) => out.push((index, line)),
            _ => {}
        }
    }
    out
}

fn long_line(path: &str, number: usize, line: &str) -> Option<Finding> {
    let exempt = line.starts_with('|') || line.contains("://");
    (line.chars().count() > MAX_LINE && !exempt).then(|| {
        Finding::new(
            "docs.line-length",
            path,
            number,
            format!("the line is longer than {MAX_LINE} characters"),
        )
    })
}

/// The line with every `code span` taken out. A backtick with no partner
/// is plain text, as in Markdown.
fn without_code(line: &str) -> String {
    let parts: Vec<&str> = line.split('`').collect();
    let ticks = parts.len().saturating_sub(1);
    let paired = ticks.saturating_sub(ticks % 2);
    let mut kept = Vec::new();
    for (index, part) in parts.iter().enumerate() {
        if index % 2 == 0 || index > paired {
            kept.push(*part);
        }
    }
    kept.join(" ")
}

fn status_words(path: &str, number: usize, line: &str) -> Option<Finding> {
    let words = format!(" {} ", normalise(&without_code(line)));
    STATUS_WORDS
        .iter()
        .find(|w| words.contains(&format!(" {w} ")))
        .map(|w| {
            Finding::new(
                "docs.status-words",
                path,
                number,
                format!("`{w}` states status; status belongs in Forgejo"),
            )
        })
}

fn normalise(text: &str) -> String {
    let cleaned: String = text
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                ' '
            }
        })
        .collect();
    cleaned.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn links(
    path: &str,
    number: usize,
    line: &str,
    lookup: &dyn Fn(&str) -> Lookup,
) -> Vec<Finding> {
    link_targets(&without_code(line))
        .into_iter()
        .filter_map(|target| check_target(path, &target, lookup))
        .map(|message| Finding::new("docs.links", path, number, message))
        .collect()
}

/// The targets of every `[text](target)` link in the line.
fn link_targets(line: &str) -> Vec<String> {
    line.split("](")
        .skip(1)
        .filter_map(|rest| rest.split_once(')').map(|(t, _)| t))
        .filter_map(|t| t.split_whitespace().next())
        .map(str::to_owned)
        .collect()
}

fn check_target(
    path: &str,
    target: &str,
    lookup: &dyn Fn(&str) -> Lookup,
) -> Option<String> {
    if target.contains("://") || target.starts_with("mailto:") {
        return None;
    }
    let (file, anchor) = match target.split_once('#') {
        Some((f, a)) => (f, Some(a)),
        None => (target, None),
    };
    let resolved = if file.is_empty() {
        path.to_owned()
    } else {
        let Some(found) = resolve(path, file) else {
            return Some(format!("`{target}` points above the repository"));
        };
        found
    };
    match (lookup(&resolved), anchor) {
        (Lookup::Missing, _) => Some(format!("`{target}` does not exist")),
        (Lookup::File(text), Some(a)) if !has_anchor(&text, a) => {
            Some(format!("`{target}` has no heading `#{a}`"))
        }
        _ => None,
    }
}

/// Joins a link to the folder of the file it is in. `None` when it
/// climbs above the repository root.
#[must_use]
pub fn resolve(from: &str, link: &str) -> Option<String> {
    let mut parts: Vec<&str> = from.split('/').collect();
    let _file = parts.pop();
    for piece in link.split('/') {
        match piece {
            "" | "." => {}
            ".." => {
                let _parent = parts.pop()?;
            }
            other => parts.push(other),
        }
    }
    Some(parts.join("/"))
}

fn has_anchor(text: &str, anchor: &str) -> bool {
    anchors(text).iter().any(|a| a == anchor)
}

/// Every heading anchor in a file, as GitHub makes them: headings in
/// code blocks do not count, and a repeated heading gets `-1`, `-2`.
#[must_use]
pub fn anchors(text: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for (_, line) in prose_lines(text) {
        let Some(heading) = heading_text(line) else {
            continue;
        };
        let base = slug(heading);
        let seen = out
            .iter()
            .filter(|a| **a == base || is_numbered(a, &base))
            .count();
        out.push(if seen == 0 {
            base
        } else {
            format!("{base}-{seen}")
        });
    }
    out
}

fn is_numbered(anchor: &str, base: &str) -> bool {
    anchor
        .strip_prefix(base)
        .and_then(|rest| rest.strip_prefix('-'))
        .is_some_and(|n| !n.is_empty() && n.chars().all(|c| c.is_ascii_digit()))
}

/// The text of an ATX heading (`#` to `######`, then a space), without
/// any closing `#` marks.
fn heading_text(line: &str) -> Option<&str> {
    const MOST_HASHES: usize = 6;
    let hashes = line.chars().take_while(|c| *c == '#').count();
    if !(1..=MOST_HASHES).contains(&hashes) {
        return None;
    }
    let rest = line.get(hashes..)?.strip_prefix(' ')?;
    Some(rest.trim().trim_end_matches('#').trim_end())
}

/// GitHub's anchor for a heading.
#[must_use]
pub fn slug(heading: &str) -> String {
    heading
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || matches!(c, ' ' | '-' | '_'))
        .map(|c| if c == ' ' { '-' } else { c })
        .collect()
}

#[cfg(test)]
#[path = "docs_tests.rs"]
mod tests;
