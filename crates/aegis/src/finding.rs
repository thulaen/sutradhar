//! One problem found by a check.

use std::fmt;

/// One problem found by a check, with where it was found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// The check that found it, such as `docs.line-length`.
    pub check: &'static str,
    /// The file it is in, relative to the repository root. Empty when the
    /// problem is not in a file, such as a commit message.
    pub path: String,
    /// The line number, counting from 1. Zero when there is no line.
    pub line: usize,
    /// What is wrong, in plain words.
    pub message: String,
}

impl Finding {
    /// Makes a finding.
    #[must_use]
    pub fn new(
        check: &'static str,
        path: &str,
        line: usize,
        message: impl Into<String>,
    ) -> Self {
        Self {
            check,
            path: path.to_owned(),
            line,
            message: message.into(),
        }
    }
}

/// Text with every control character but line breaks and tabs removed,
/// so a file name or tool output cannot send codes to the screen.
fn shown(text: &str) -> String {
    text.chars()
        .filter(|c| !c.is_control() || matches!(c, '\n' | '\t'))
        .collect()
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // A path is one line: a line break in a name cannot fake a
        // finding of its own.
        let path = shown(&self.path).replace(['\n', '\t'], " ");
        let message = shown(&self.message);
        match (path.is_empty(), self.line) {
            (true, _) => write!(f, "[{}] {message}", self.check),
            (false, 0) => write!(f, "[{}] {path}: {message}", self.check),
            (false, n) => write!(f, "[{}] {path}:{n}: {message}", self.check),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Finding;

    #[test]
    fn shows_path_and_line() {
        let f = Finding::new("x.y", "a.md", 3, "bad");
        assert_eq!(f.to_string(), "[x.y] a.md:3: bad");
        let codes = Finding::new("x.y", "a\u{1b}[31m.md", 0, "b\u{7}ad\nnext");
        assert_eq!(codes.to_string(), "[x.y] a[31m.md: bad\nnext");
    }

    #[test]
    fn shows_path_without_line() {
        let f = Finding::new("x.y", "a.md", 0, "bad");
        assert_eq!(f.to_string(), "[x.y] a.md: bad");
    }

    #[test]
    fn shows_message_without_path() {
        let f = Finding::new("x.y", "", 0, "bad");
        assert_eq!(f.to_string(), "[x.y] bad");
    }
}
