//! The file size limit (AGENTS.md section 6, gate item GI-04). Function
//! length is checked by Clippy's `too_many_lines`.

use crate::Finding;
use crate::file_kinds::Kind;

/// The most lines a source file may have.
pub const MAX_FILE_LINES: usize = 300;

/// Checks one file. Only source files are limited.
#[must_use]
pub fn check(path: &str, kind: Kind, text: &str) -> Option<Finding> {
    let source = matches!(
        kind,
        Kind::Rust
            | Kind::Css
            | Kind::TypeScript
            | Kind::Php
            | Kind::Sql
            | Kind::Shell
    );
    let count = text.lines().count();
    (source && count > MAX_FILE_LINES).then(|| {
        Finding::new(
            "limits.file-length",
            path,
            0,
            format!("{count} lines; the most is {MAX_FILE_LINES}"),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::{MAX_FILE_LINES, check};
    use crate::file_kinds::Kind;

    #[test]
    fn refuses_a_long_source_file() {
        let text = "x\n".repeat(MAX_FILE_LINES + 1);
        assert!(check("a.rs", Kind::Rust, &text).is_some());
    }

    #[test]
    fn passes_short_files_and_long_docs() {
        let long = "x\n".repeat(MAX_FILE_LINES + 1);
        assert!(check("a.rs", Kind::Rust, "x\n").is_none());
        assert!(check("a.md", Kind::Markdown, &long).is_none());
    }
}
