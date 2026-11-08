//! The size limits (gate item GI-04): a source file has at most 300 lines
//! (AGENTS.md section 6, item 11), each at most 80 characters wide (the
//! `max_width` in `rustfmt.toml`). rustfmt cannot shorten long quoted
//! text or comments, so this check catches them. For Rust, Clippy checks
//! function length, arguments, nesting and complexity.

use crate::Finding;
use crate::file_kinds::Kind;

/// The most lines a source file may have.
pub const MAX_FILE_LINES: usize = 300;

/// The most characters a line of source may have.
pub const MAX_WIDTH: usize = 80;

/// How many long lines are listed for one file.
const LISTED: usize = 20;

/// Checks one file. Only source files are limited.
#[must_use]
pub fn check(path: &str, kind: Kind, text: &str) -> Vec<Finding> {
    let source = matches!(
        kind,
        Kind::Rust
            | Kind::Css
            | Kind::TypeScript
            | Kind::Php
            | Kind::Sql
            | Kind::Shell
    );
    if !source {
        return Vec::new();
    }
    let mut out = Vec::new();
    let count = text.lines().count();
    if count > MAX_FILE_LINES {
        out.push(Finding::new(
            "limits.file-length",
            path,
            0,
            format!("{count} lines; the most is {MAX_FILE_LINES}"),
        ));
    }
    let wide: Vec<(usize, &str)> = text
        .lines()
        .enumerate()
        .filter(|(_, line)| line.chars().count() > MAX_WIDTH)
        .collect();
    out.extend(wide.iter().take(LISTED).map(|&(i, line)| {
        Finding::new(
            "limits.line-width",
            path,
            i.saturating_add(1),
            format!(
                "{} characters; the most is {MAX_WIDTH}",
                line.chars().count()
            ),
        )
    }));
    if wide.len() > LISTED {
        out.push(Finding::new(
            "limits.line-width",
            path,
            0,
            format!(
                "and {} more wide lines",
                wide.len().saturating_sub(LISTED)
            ),
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{MAX_FILE_LINES, MAX_WIDTH, check};
    use crate::file_kinds::Kind;

    #[test]
    fn refuses_a_long_source_file() {
        let text = "x\n".repeat(MAX_FILE_LINES + 1);
        assert_eq!(check("a.rs", Kind::Rust, &text).len(), 1);
    }

    #[test]
    fn passes_short_files_and_long_docs() {
        let long = "x\n".repeat(MAX_FILE_LINES + 1);
        assert_eq!(check("a.rs", Kind::Rust, "x\n"), Vec::new());
        assert_eq!(check("a.md", Kind::Markdown, &long), Vec::new());
    }

    #[test]
    fn refuses_a_wide_line_with_its_number() {
        let text = format!("ok\n{}\n", "x".repeat(MAX_WIDTH + 1));
        let found = check("a.sh", Kind::Shell, &text);
        assert_eq!(found.len(), 1);
        assert_eq!(found.first().map(|f| f.line), Some(2));
        let edge = format!("{}\n", "é".repeat(MAX_WIDTH));
        assert_eq!(check("a.rs", Kind::Rust, &edge), Vec::new());
    }

    #[test]
    fn at_most_twenty_wide_lines_are_listed_then_counted() {
        let text = format!("{}\n", "x".repeat(MAX_WIDTH + 1)).repeat(23);
        let found = check("a.rs", Kind::Rust, &text);
        assert_eq!(found.len(), 21);
        assert!(found.last().is_some_and(|f| f.message.contains("3 more")));
    }

    #[test]
    fn settings_files_may_hold_long_lines() {
        let text = format!("{}\n", "x".repeat(MAX_WIDTH + 1));
        assert_eq!(check("a.toml", Kind::Config, &text), Vec::new());
    }
}
