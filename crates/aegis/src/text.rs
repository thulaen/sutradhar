//! The text check (task #18, carried from the old docs checker's text
//! rules). Every file we write is UTF-8 text with `\n` line breaks, a
//! final line break, and no space or tab at the end of a line. Hidden
//! characters are refused too: direction controls can make code read
//! one way and run another, and zero-width or odd line breaks can hide
//! text from a reviewer. Licence texts are kept exactly as published.

use crate::Finding;
use crate::file_kinds::Kind;

const CHECK: &str = "text.clean";

/// How many problems are listed for one file before the rest are
/// counted.
const LISTED: usize = 20;

/// Whether `c` is an invisible format character (Unicode's Cf class), an
/// invisible mark or variation selector (emoji styles included: no file
/// of ours needs them), or a filler that shows as blank. These include
/// direction controls, which can make code read one way and run another,
/// and tag characters, which can hide words from a reviewer.
fn format_character(c: char) -> bool {
    matches!(
        c,
        '\u{00ad}'
            | '\u{034f}'
            | '\u{17b4}'..='\u{17b5}'
            | '\u{180b}'..='\u{180f}'
            | '\u{fe00}'..='\u{fe0f}'
            | '\u{e0100}'..='\u{e01ef}'
            | '\u{0600}'..='\u{0605}'
            | '\u{061c}'
            | '\u{06dd}'
            | '\u{070f}'
            | '\u{0890}'..='\u{0891}'
            | '\u{08e2}'
            | '\u{115f}'..='\u{1160}'
            | '\u{200b}'..='\u{200f}'
            | '\u{2028}'..='\u{202e}'
            | '\u{2060}'..='\u{2064}'
            | '\u{2066}'..='\u{206f}'
            | '\u{3164}'
            | '\u{feff}'
            | '\u{ffa0}'
            | '\u{fff9}'..='\u{fffb}'
            | '\u{110bd}'
            | '\u{110cd}'
            | '\u{13430}'..='\u{1343f}'
            | '\u{1bca0}'..='\u{1bca3}'
            | '\u{1d173}'..='\u{1d17a}'
            | '\u{e0000}'..='\u{e007f}'
    )
}

/// Whether `c` is a letter or sign of an Indian script, where the
/// joiners U+200C and U+200D are part of correct spelling.
fn indic(c: char) -> bool {
    ('\u{0900}'..='\u{0dff}').contains(&c)
}

/// Whether the line holds a character no file of ours may hold.
fn has_hidden(line: &str) -> bool {
    let chars: Vec<char> = line.chars().collect();
    chars.iter().enumerate().any(|(i, &c)| {
        if c == '\u{200c}' || c == '\u{200d}' {
            let before = i.checked_sub(1).and_then(|j| chars.get(j));
            let after = chars.get(i.saturating_add(1));
            return !(before.is_some_and(|&b| indic(b))
                && after.is_some_and(|&a| indic(a)));
        }
        (c.is_control() && c != '\t') || format_character(c)
    })
}

/// What is wrong with one line, if anything.
fn problem(line: &str) -> Option<&'static str> {
    if line.ends_with('\r') {
        return Some("a Windows line break: use `\\n` only");
    }
    if line.chars().last().is_some_and(char::is_whitespace) {
        return Some("a space of any kind at the end of the line");
    }
    has_hidden(line).then_some("a hidden or control character")
}

/// Checks one file's bytes.
#[must_use]
pub fn check(path: &str, kind: Kind, bytes: &[u8]) -> Vec<Finding> {
    if kind == Kind::Licence {
        return Vec::new();
    }
    let Ok(text) = std::str::from_utf8(bytes) else {
        return vec![Finding::new(CHECK, path, 0, "the file is not UTF-8")];
    };
    let mut found: Vec<Finding> = text
        .split('\n')
        .enumerate()
        .filter_map(|(i, line)| {
            problem(line)
                .map(|p| Finding::new(CHECK, path, i.saturating_add(1), p))
        })
        .collect();
    if !text.is_empty() && !text.ends_with('\n') {
        let last = text.split('\n').count();
        found.push(Finding::new(CHECK, path, last, "no line break at the end"));
    }
    if found.len() > LISTED {
        let more = found.len().saturating_sub(LISTED);
        found.truncate(LISTED);
        found.push(Finding::new(
            CHECK,
            path,
            0,
            format!("and {more} more problems of the same kinds"),
        ));
    }
    found
}

#[cfg(test)]
#[path = "text_tests.rs"]
mod tests;
