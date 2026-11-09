//! Reads one YAML file, such as a compose or workflow file, and records
//! each setting it declares, as an `env_declared` fact: a key under an
//! `environment:` or `env:` block (map or `- NAME=value` list), and each
//! `${NAME}` it reads from outside. It reads lines, not the whole YAML
//! tree, so it needs no library and keeps each line number.

use crate::facts::{Facts, number, text};

fn indent(line: &str) -> usize {
    line.len().saturating_sub(line.trim_start().len())
}

/// The setting a line in an environment block declares, if any.
fn declared(item: &str) -> Option<&str> {
    let item = item.strip_prefix("- ").unwrap_or(item).trim();
    let end = item.find(['=', ':']).unwrap_or(item.len());
    let name = item.get(..end)?.trim().trim_matches('"');
    let ok = !name.is_empty()
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
    ok.then_some(name)
}

/// Every `${NAME}`, `${NAME:-x}` or `${NAME:?x}` in a line.
fn interpolated(line: &str) -> Vec<&str> {
    line.split("${")
        .skip(1)
        .filter_map(|rest| {
            let end =
                rest.find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))?;
            rest.get(..end).filter(|n| !n.is_empty())
        })
        .collect()
}

/// An open settings block: the indent of its key, and of its first
/// child once seen.
type Block = Option<(usize, Option<usize>)>;

/// The block after a line at indent `here`, and whether that line is
/// one of the block's own children (deeper lines belong to a child).
fn step(block: Block, here: usize) -> (Block, bool) {
    match block {
        Some((key, _)) if here <= key => (None, false),
        Some((key, child)) if child.is_none_or(|c| c == here) => {
            (Some((key, Some(here))), true)
        }
        Some(open) => (Some(open), false),
        None => (None, false),
    }
}

/// Adds the facts of one YAML file at `path` to `facts`.
pub fn read(path: &str, source: &str, facts: &mut Facts) {
    facts.add("file", &[("path", text(path)), ("language", text("yaml"))]);
    let mut add = |name: &str, at: usize| {
        facts.add(
            "env_declared",
            &[
                ("path", text(path)),
                ("name", text(name)),
                ("line", number(at)),
            ],
        );
    };
    let mut block: Block = None;
    for (index, line) in source.lines().enumerate() {
        let at = index.saturating_add(1);
        let trimmed = line.trim();
        for name in interpolated(line) {
            add(name, at);
        }
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let here = indent(line);
        let (next, inside) = step(block, here);
        block = next;
        if let Some(name) = declared(trimmed).filter(|_| inside) {
            add(name, at);
        }
        if inside {
            continue;
        }
        if matches!(trimmed, "environment:" | "env:") {
            block = Some((here, None));
        }
    }
}

#[cfg(test)]
#[path = "yaml_facts_tests.rs"]
mod tests;
