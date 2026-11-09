//! The smell facts read from text, for `smell_facts`: line comments,
//! and text written three or more times in a file's code.

use std::collections::BTreeMap;

use crate::facts::{Facts, number, text};
use crate::shapes::in_test_file;

/// The shortest text counted when it is written again.
const SHORTEST: usize = 8;

/// How many copies make text repeated.
const COPIES: usize = 3;

/// The comments of `source` on a line of their own (not doc comments:
/// `///` and `//!`; `////` is a plain comment), each with its line, and
/// `test` when it is in a test file or between `tests` lines.
pub fn comments(
    path: &str,
    source: &str,
    tests: &[(usize, usize)],
    facts: &mut Facts,
) {
    let test_file = in_test_file(path);
    for (n, line) in source.lines().enumerate() {
        let at = n.saturating_add(1);
        let Some(body) = line.trim_start().strip_prefix("//") else {
            continue;
        };
        let doc = (body.starts_with('/') && !body.starts_with("//"))
            || body.starts_with('!');
        if doc {
            continue;
        }
        let test =
            test_file || tests.iter().any(|(a, b)| (*a..=*b).contains(&at));
        let kind = if test { "test" } else { "code" };
        facts.add(
            "comment",
            &[
                ("path", text(path)),
                ("kind", text(kind)),
                ("text", text(body.trim())),
                ("line", number(at)),
            ],
        );
    }
}

/// A fact for every copy of text of [`SHORTEST`] or more characters
/// written [`COPIES`] or more times in the file's code, two copies on
/// one line counting as two.
pub fn repeated(
    path: &str,
    texts: &BTreeMap<String, Vec<usize>>,
    facts: &mut Facts,
) {
    for (written, lines) in texts {
        if written.chars().count() < SHORTEST || lines.len() < COPIES {
            continue;
        }
        for line in lines {
            facts.add(
                "repeated",
                &[
                    ("path", text(path)),
                    ("kind", text("code")),
                    ("text", text(written.clone())),
                    ("copies", number(lines.len())),
                    ("line", number(*line)),
                ],
            );
        }
    }
}
