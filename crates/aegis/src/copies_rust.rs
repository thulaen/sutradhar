//! Rust source for `copies`: its tokens, with `use` items left out.

/// The tokens of Rust source, leaving out `use` items, which list what
/// a file uses and are not code to copy. A `use` starts an item only
/// where an item may start (at the top, or after `;`, `{`, `}`, an
/// attribute's `]`, or `pub`) and not before `<` (as in `+ use<'a>`).
/// The item ends at its `;`, or at the first token no `use` item holds.
pub fn code(text: &str) -> Vec<(String, usize)> {
    let all = crate::rust_tokens::tokens(text);
    let mut out: Vec<(String, usize)> = Vec::with_capacity(all.len());
    let mut at = 0_usize;
    while let Some((token, line)) = all.get(at) {
        let next = all.get(at.saturating_add(1)).map(|t| t.0.as_str());
        if token == "use" && next != Some("<") && item_starts(&out) {
            at = past_use(&all, at.saturating_add(1));
        } else {
            out.push((token.clone(), *line));
            at = at.saturating_add(1);
        }
    }
    out
}

/// Whether an item may start after the tokens kept so far.
fn item_starts(before: &[(String, usize)]) -> bool {
    // How far back a `pub(...)` may start.
    const LOOK_BACK: usize = 8;
    let last = |n: usize| {
        before
            .len()
            .checked_sub(n)
            .and_then(|i| before.get(i))
            .map(|t| t.0.as_str())
    };

    match last(1) {
        None | Some(";" | "{" | "}" | "]" | "pub") => true,
        // `pub(crate)`, `pub(super)` or `pub(in path)`.
        Some(")") => (2..LOOK_BACK).any(|n| {
            last(n) == Some("(") && last(n.saturating_add(1)) == Some("pub")
        }),
        _ => false,
    }
}

/// Where a `use` item that starts at `from` ends: just past its `;`, or
/// at the first token no `use` item holds.
fn past_use(all: &[(String, usize)], from: usize) -> usize {
    let mut at = from;
    while let Some((token, _)) = all.get(at) {
        if token == ";" {
            return at.saturating_add(1);
        }
        let held = token.chars().all(|c| c.is_alphanumeric() || c == '_')
            || matches!(token.as_str(), ":" | "{" | "}" | "," | "*");
        if !held {
            return at;
        }
        at = at.saturating_add(1);
    }
    at
}
