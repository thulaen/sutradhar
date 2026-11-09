//! The proof-tricks check (task #59; Phase 2 item 2). Kani, Verus and
//! Creusot can pass while a goal is skipped or assumed true. The file is
//! split into code words by `rust_tokens` (comments and every kind of
//! literal are dropped), and these are refused:
//!
//! - assuming false, or a precondition of false, in any of the three;
//! - renaming assume, by `as` or by binding it to a name, so a later
//!   trick hides behind the new name;
//! - an attribute (outer, inner, by path or inside `cfg_attr`) naming
//!   external, external body, trusted, Kani's should panic (a plain
//!   test's should panic is fine), the no-decreases allowance, a stub,
//!   or unwinding loops zero times in any spelling;
//! - Verus's and Creusot's ways of stating a specification for code
//!   that is not proved, and admitting a goal.
//!
//! The words are built from parts, so this file does not match itself.
//! An always-false condition written another way, such as `1 == 2`,
//! needs a solver and is a review matter, and so is assume hidden inside
//! a helper function or closure that is later given false.

use crate::Finding;
use crate::file_kinds::Kind;

const CHECK: &str = "proofs.no-tricks";

use crate::rust_tokens::{Token, tokens};

/// The word that ends the names of trusting proof markers.
const SPEC: &str = "specification";

fn word(parts: [&str; 2]) -> String {
    parts.concat()
}

/// The words that are tricks anywhere in code.
fn tricky_words() -> [String; 5] {
    [
        word(["external_fn_", SPEC]),
        word(["external_type_", SPEC]),
        word(["external_trait_", SPEC]),
        word(["assume_", SPEC]),
        word(["extern_", "spec"]),
    ]
}

/// The words that are tricks inside an attribute.
fn tricky_attribute_words() -> [String; 5] {
    [
        word(["exter", "nal"]),
        word(["external_", "body"]),
        word(["trust", "ed"]),
        word(["exec_allows_no_", "decreases_clause"]),
        word(["st", "ub"]),
    ]
}

/// The word at `i`, or nothing.
fn at(t: &[Token], i: usize) -> &str {
    t.get(i).map_or("", |(w, _)| w.as_str())
}

/// Whether the tokens from `i` are `( false`, `! ( false`, or the same
/// with more brackets, such as `((false))`.
fn called_with_false(t: &[Token], i: usize) -> bool {
    let mut j = i.saturating_add(usize::from(at(t, i) == "!"));
    if at(t, j) != "(" {
        return false;
    }
    while at(t, j) == "(" {
        j = j.saturating_add(1);
    }
    at(t, j) == "false"
}

/// Whether a requires clause from `i` has a bare `false` as any of its
/// parts, such as `requires x > 0, false,`. The clause ends at `{`,
/// `;` or `]`, or when its own brackets close.
fn requires_false(t: &[Token], i: usize) -> bool {
    let top = usize::from(at(t, i) == "(");
    let mut depth = 0_usize;
    let mut j = i;
    while let Some((w, _)) = t.get(j) {
        let part_start =
            j == i || at(t, j.wrapping_sub(1)) == "," && depth == top;
        let part_end = matches!(at(t, j.saturating_add(1)), "," | ")" | "{");
        if w == "false" && part_start && part_end {
            return true;
        }
        match w.as_str() {
            "(" => depth = depth.saturating_add(1),
            ")" if depth == 0 => return false,
            ")" => depth = depth.saturating_sub(1),
            "{" | ";" | "]" => return false,
            _ => {}
        }
        j = j.saturating_add(1);
    }
    false
}

/// Whether a number word is zero in any spelling: `0`, `00`, `0x0`,
/// `0_u32`, `0b0usize` and so on.
fn is_zero(w: &str) -> bool {
    let (body, hex) = match w.get(..2) {
        Some("0x") => (w.get(2..).unwrap_or(""), true),
        Some("0o" | "0b") => (w.get(2..).unwrap_or(""), false),
        _ => (w, false),
    };
    let digits: Vec<char> = body
        .chars()
        .take_while(|c| {
            *c == '_' || c.is_ascii_digit() || hex && c.is_ascii_hexdigit()
        })
        .filter(|c| *c != '_')
        .collect();
    w.starts_with('0') && !digits.is_empty() && digits.iter().all(|c| *c == '0')
}

/// Whether assume at `i` is used as a value, such as `let f = assume;`,
/// so a later call through the new name would hide a trick. Inside a
/// `use` it is only brought in, which is fine.
fn bound_to_name(t: &[Token], i: usize) -> bool {
    let next = at(t, i.saturating_add(1));
    let method = i > 0 && at(t, i.wrapping_sub(1)) == ".";
    let path = next == ":" && at(t, i.saturating_add(2)) == ":";
    let skip = matches!(next, "(" | "<" | "!" | "as" | ":") && !path;
    !method && !skip
}

/// The tricks in plain code, as (line, what it does).
fn find_words(t: &[Token], out: &mut Vec<(usize, &'static str)>) {
    let (assume_w, requires, admit) = (
        word(["ass", "ume"]),
        word(["requ", "ires"]),
        word(["adm", "it"]),
    );
    let words = tricky_words();
    let mut in_use = false;
    for (i, (w, line)) in t.iter().enumerate() {
        match w.as_str() {
            "use" => in_use = true,
            ";" => in_use = false,
            _ => {}
        }
        let after = i.saturating_add(1);
        if *w == assume_w && !in_use && bound_to_name(t, i) {
            out.push((*line, "binds assume to a name, which hides tricks"));
        }
        let false_given = (*w == assume_w || *w == requires)
            && called_with_false(t, after)
            || *w == requires && requires_false(t, after);
        if false_given {
            out.push((*line, "assumes false, so any proof passes"));
        }
        if *w == assume_w && at(t, after) == "as" {
            out.push((*line, "renames assume, which hides later tricks"));
        }
        let method = i > 0 && at(t, i.wrapping_sub(1)) == ".";
        if *w == admit && at(t, after) == "(" && !method {
            out.push((*line, "admits a goal without proving it"));
        }
        if words.contains(w) {
            out.push((*line, "states a specification for code not proved"));
        }
    }
}

/// The tricks in attributes, as (line, what it does).
fn find_attributes(t: &[Token], out: &mut Vec<(usize, &'static str)>) {
    const CLOSE_AT: usize = 3;
    const KANI_BACK: usize = 3;
    let words = tricky_attribute_words();
    let unwind = word(["unw", "ind"]);
    let mut i = 0_usize;
    while i < t.len() {
        let inner = usize::from(at(t, i.saturating_add(1)) == "!");
        let open = i.saturating_add(1).saturating_add(inner);
        if at(t, i) != "#" || at(t, open) != "[" {
            i = i.saturating_add(1);
            continue;
        }
        let mut depth = 0_usize;
        let mut j = open;
        while let Some((w, line)) = t.get(j) {
            depth = match w.as_str() {
                "[" => depth.saturating_add(1),
                "]" => depth.saturating_sub(1),
                _ => depth,
            };
            let kani = at(t, j.wrapping_sub(KANI_BACK)) == "kani"
                && at(t, j.wrapping_sub(1)) == ":";
            if words.contains(w) || kani && *w == word(["should_", "panic"]) {
                out.push((*line, "trusts or skips code in the proof"));
            }
            let zero = at(t, j.saturating_add(1)) == "("
                && is_zero(at(t, j.saturating_add(2)))
                && at(t, j.saturating_add(CLOSE_AT)) == ")";
            if *w == unwind && zero {
                out.push((*line, "skips loops in the proof"));
            }
            j = j.saturating_add(1);
            if depth == 0 {
                break;
            }
        }
        i = j;
    }
}

/// Checks one file.
#[must_use]
pub fn check(path: &str, kind: Kind, text: &str) -> Vec<Finding> {
    if kind != Kind::Rust {
        return Vec::new();
    }
    let t = tokens(text);
    let mut found = Vec::new();
    find_words(&t, &mut found);
    find_attributes(&t, &mut found);
    found.sort_unstable();
    found
        .into_iter()
        .map(|(line, what)| {
            Finding::new(
                CHECK,
                path,
                line,
                format!("a proof trick: this {what}"),
            )
        })
        .collect()
}

#[cfg(test)]
#[path = "proofs_tests.rs"]
mod tests;
