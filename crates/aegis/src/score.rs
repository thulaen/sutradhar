//! One quality score for a change (task #116; the rule is in AGENTS.md
//! section 7). Every finding the gate makes falls in one [`Part`]. The
//! score is the share of parts with no finding, on a scale of [`SCALE`],
//! and it is shown with each part's count after every `aegis commit`
//! and `aegis push`. The bar is [`BAR`]; every check blocks (AGENTS.md
//! section 7), so a change under the bar is refused by its findings.
//!
//! Complexity is its own part. It holds a file over the length limit
//! (`limits.file-length`), and clippy's complexity lints (a function
//! too long, too complex, too deep or with too many inputs), which come
//! in through `rust.lints`. Coverage of new lines is not yet a part: no
//! coverage tool exists until Phase 2 item 5 (task #135).

use crate::Finding;

/// The scale the score is on.
pub const SCALE: usize = 100;

/// The score a change must reach.
pub const BAR: usize = SCALE;

/// The parts a finding can fall in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {
    /// Secrets, holes, unsafe code and unsafe set-up.
    Security,
    /// Tests that are missing, fail, come late or prove nothing.
    Tests,
    /// Copied code.
    Copies,
    /// Code smells (the analyser's M rules).
    Smells,
    /// Files too long, and functions too long, too complex, too deep or
    /// with too many inputs.
    Complexity,
    /// The required reviews.
    Reviews,
    /// Files, libraries and set-up files.
    SetUp,
    /// Style, text and docs.
    StyleDocs,
}

/// Every part, in the order it is shown.
pub const PARTS: [Part; 8] = [
    Part::Security,
    Part::Tests,
    Part::Copies,
    Part::Smells,
    Part::Complexity,
    Part::Reviews,
    Part::SetUp,
    Part::StyleDocs,
];

impl Part {
    /// The part's name as shown.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Security => "security",
            Self::Tests => "tests",
            Self::Copies => "copies",
            Self::Smells => "smells",
            Self::Complexity => "complexity",
            Self::Reviews => "reviews",
            Self::SetUp => "set-up",
            Self::StyleDocs => "style and docs",
        }
    }
}

#[path = "score_parts.rs"]
mod parts;

pub(crate) use parts::COMPLEX;
pub use parts::part;

/// A change's score: the count of findings in each part, and the score.
#[derive(Debug, PartialEq, Eq)]
pub struct Score {
    /// Each part and its count of findings, in [`PARTS`] order.
    pub counts: Vec<(Part, usize)>,
    /// The share of parts with no finding, on a scale of [`SCALE`].
    pub value: usize,
}

/// The score of a change with these findings.
#[must_use]
pub fn of(findings: &[Finding]) -> Score {
    let mut counts: Vec<(Part, usize)> =
        PARTS.iter().map(|p| (*p, 0)).collect();
    for f in findings {
        let at = part(f);
        if let Some(c) = counts.iter_mut().find(|(p, _)| *p == at) {
            c.1 = c.1.saturating_add(1);
        }
    }
    let clean = counts.iter().filter(|(_, n)| *n == 0).count();
    let value = clean
        .saturating_mul(SCALE)
        .checked_div(PARTS.len())
        .unwrap_or(0);
    Score { counts, value }
}

/// The score as one plain line.
#[must_use]
pub fn line(score: &Score) -> String {
    let parts: Vec<String> = score
        .counts
        .iter()
        .map(|(p, n)| format!("{} {n}", p.name()))
        .collect();
    format!(
        "aegis: quality score {} of {SCALE} (problems: {})",
        score.value,
        parts.join(", ")
    )
}

#[cfg(test)]
#[path = "score_tests.rs"]
mod tests;
