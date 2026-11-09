//! Finding copied code (task #115). Every source file is split into
//! tokens, with comments and quoted text dropped; numbers are kept as
//! written. In Rust every name that is not a keyword (or `Some`, `None`,
//! `Ok` or `Err`) becomes the same token, so a copy with its names
//! changed still matches; in other languages names are kept as written.
//! Rust `use` items, which list what a file uses, are left out. A table
//! (one short piece written again and again, names and all) is squeezed
//! to its first piece. Two runs of at least [`MIN_TOKENS`] equal tokens,
//! in one file or two, are a copy, and the gate refuses it: copied code
//! is written once and shared.

use std::collections::HashMap;

use crate::Finding;
use crate::file_kinds::Kind;

const CHECK: &str = "code.copies";

/// The fewest tokens a run must share to be a copy: about ten lines.
pub const MIN_TOKENS: usize = 100;

/// The most places one run of tokens is looked at, so a file of the
/// same line many times cannot make the check slow.
const MOST_PLACES: usize = 64;

/// The most tokens the check reads in all, several times what the
/// project holds today, so a push of huge files cannot
/// run the server out of memory. More is refused, never skipped.
pub const MOST_TOKENS: usize = 1_000_000;

/// Rust's keywords and the four names Rust code uses everywhere, kept as
/// they are; every other name becomes the same token.
const KEYWORDS: [&str; 42] = [
    "as", "async", "await", "break", "const", "continue", "crate", "dyn",
    "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in", "let",
    "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self",
    "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
    "use", "where", "while", "Some", "None", "Ok", "Err",
];

/// One copied run: where it is in each file (path, first and last line)
/// and its length in tokens.
#[derive(Debug, PartialEq, Eq)]
pub struct Copied {
    /// The first place, earlier in path order.
    pub first: (String, usize, usize),
    /// The second place.
    pub second: (String, usize, usize),
    /// How many tokens the two places share.
    pub tokens: usize,
}

/// What `/* */` is in a language: a comment, dropped, or plain text,
/// kept (as in a shell script, where `/*` is a file pattern).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Blocks {
    Dropped,
    Kept,
}

/// The tokens of a file in a language other than Rust: words and
/// symbols, dropping comments (those `marks` start, to the end of the
/// line, and `/* */` when `block` is `Dropped`) and quoted text. Names are
/// kept as written.
fn other_tokens(
    text: &str,
    marks: &[&str],
    block: Blocks,
) -> Vec<(String, usize)> {
    let mut out = Vec::new();
    let mut lex = Lexer {
        rest: text,
        line: 1,
        marks,
        block: block == Blocks::Dropped,
    };
    while let Some(c) = lex.rest.chars().next() {
        if c.is_alphanumeric() || c == '_' {
            let end = lex
                .rest
                .find(|c: char| !(c.is_alphanumeric() || c == '_'))
                .unwrap_or(lex.rest.len());
            let (word, rest) = lex.rest.split_at(end);
            out.push((word.to_owned(), lex.line));
            lex.rest = rest;
        } else if !lex.skip(c) && !c.is_whitespace() {
            out.push((c.to_string(), lex.line));
        }
    }
    out
}

/// Walks through text for [`other_tokens`], keeping the line.
struct Lexer<'a> {
    rest: &'a str,
    line: usize,
    marks: &'a [&'a str],
    /// Whether `/* */` is a comment, as in SQL, CSS, TypeScript and PHP;
    /// in a shell script `/*` is a file pattern.
    block: bool,
}

impl Lexer<'_> {
    /// Moves on past the first `n` bytes, counting the lines in them.
    fn advance(&mut self, n: usize) {
        let (gone, rest) = self.rest.split_at(n.min(self.rest.len()));
        let lines = gone.matches('\n').count();
        self.line = self.line.saturating_add(lines);
        self.rest = rest;
    }

    /// Moves past a comment or quoted text at the start, or past the one
    /// character `c`; says whether it was a comment or quoted text.
    fn skip(&mut self, c: char) -> bool {
        let to_end = |r: &str, from: usize, close: &str| {
            r.get(from..)
                .and_then(|t| t.find(close))
                .map_or(r.len(), |at| from.saturating_add(at))
        };
        let rest = self.rest;
        let n = if self.marks.iter().any(|m| rest.starts_with(m)) {
            to_end(rest, 0, "\n")
        } else if self.block && rest.starts_with("/*") {
            to_end(rest, 2, "*/").saturating_add(2)
        } else if c == '"' || c == '\'' {
            quoted(rest, c)
        } else {
            self.advance(c.len_utf8());
            return false;
        };
        self.advance(n);
        true
    }
}

/// The length of the quoted text at the start of `rest`, quotes and all,
/// stepping over a `\` and the character after it.
fn quoted(rest: &str, quote: char) -> usize {
    let mut chars = rest.char_indices().skip(1);
    while let Some((at, c)) = chars.next() {
        if c == '\\' {
            let _escaped = chars.next();
        } else if c == quote {
            return at.saturating_add(c.len_utf8());
        }
    }
    rest.len()
}

/// One token: as compared (in Rust, each name merged into one), as
/// written, and its line.
pub type Token = (u32, u32, usize);

/// A file's tokens, as numbers. `None` for a file that is not source
/// code.
fn tokens(
    kind: Kind,
    text: &str,
    names: &mut HashMap<String, u32>,
) -> Option<Vec<Token>> {
    let raw = match kind {
        Kind::Rust => rust::code(text),
        Kind::Shell => other_tokens(text, &["#"], Blocks::Kept),
        // Rego comments start with `#`, Cedar ones with `//`.
        Kind::Policy => other_tokens(text, &["#", "//"], Blocks::Kept),
        Kind::Sql => other_tokens(text, &["--"], Blocks::Dropped),
        Kind::Css => other_tokens(text, &[], Blocks::Dropped),
        Kind::TypeScript | Kind::Php => {
            other_tokens(text, &["//"], Blocks::Dropped)
        }
        _ => return None,
    };
    let mut id = |t: String| {
        let next = u32::try_from(names.len()).unwrap_or(u32::MAX);
        *names.entry(t).or_insert(next)
    };
    let mut out = Vec::with_capacity(raw.len());
    for (t, line) in raw {
        let name = kind == Kind::Rust
            && t.starts_with(|c: char| c.is_alphabetic() || c == '_')
            && !KEYWORDS.contains(&t.as_str());
        let merged = if name {
            id("_".to_owned())
        } else {
            id(t.clone())
        };
        out.push((merged, id(t), line));
    }
    Some(out)
}

/// Every copy among `files` (path, kind, text).
///
/// # Errors
/// With the token count reached, when the files hold more than
/// [`MOST_TOKENS`] tokens in all.
pub fn find(files: &[(String, Kind, String)]) -> Result<Vec<Copied>, usize> {
    let mut names = HashMap::new();
    let mut all: Vec<(&str, Vec<Token>)> = Vec::new();
    let mut count = 0_usize;
    for (path, kind, text) in files {
        if let Some(t) = tokens(*kind, text, &mut names) {
            count = count.saturating_add(t.len());
            if count > MOST_TOKENS {
                return Err(count);
            }
            all.push((path, t));
        }
    }
    all.sort_by(|a, b| a.0.cmp(b.0));
    Ok(scan::scan(&all, MIN_TOKENS, MOST_PLACES))
}

/// The findings for `files`: one for each copy, named at the later
/// place and pointing at the earlier one.
#[must_use]
pub fn check(files: &[(String, Kind, String)]) -> Vec<Finding> {
    let found = match find(files) {
        Ok(found) => found,
        Err(count) => {
            return vec![Finding::new(
                CHECK,
                "",
                0,
                format!(
                    "the source files hold at least {count} tokens, more \
                     than the {MOST_TOKENS} the copy check reads; this is \
                     refused: remove large files, or ask the owner to raise \
                     the limit"
                ),
            )];
        }
    };
    found
        .into_iter()
        .map(|c| {
            let (other, at, end) = &c.first;
            let (path, from, to) = &c.second;
            Finding::new(
                CHECK,
                path,
                *from,
                format!(
                    "lines {from}-{to} copy {other} lines {at}-{end} \
                     ({} tokens); write it once and share it",
                    c.tokens
                ),
            )
        })
        .collect()
}

#[path = "copies_rust.rs"]
mod rust;

#[path = "copies_scan.rs"]
mod scan;

#[cfg(test)]
#[path = "copies_tests.rs"]
mod tests;
