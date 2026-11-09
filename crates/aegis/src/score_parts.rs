//! Which part of the score each check, and each analyser rule, falls
//! in. Every check the gate reports (`catalogue::REPORTED`) is named
//! here, and a test keeps the list whole, so a new check cannot fall in
//! a part by chance. Of the analyser's rules, R05, R14, H42 and M08
//! are named, the other M rules are smells, and every other rule counts
//! as security, the strictest reading; a test checks every rule.

use super::Part;
use crate::Finding;

/// The checks the gate reports, by part.
const CHECKS: [(Part, &[&str]); 7] = [
    (
        Part::Security,
        &[
            "secrets.gitleaks",
            "secrets.private-data",
            "secrets.private-lists",
            "workflows.sealed",
            "rust.deny",
            "supply.sbom",
            "images.holes",
        ],
    ),
    (
        Part::Tests,
        &[
            "commit.tdd",
            "commit.test-first",
            "rust.tests",
            "proofs.no-tricks",
        ],
    ),
    (Part::Copies, &["code.copies"]),
    (Part::Complexity, &["limits.file-length"]),
    (Part::Reviews, &["reviews.required"]),
    (
        Part::SetUp,
        &[
            "files.allowed-kind",
            "files.no-links",
            "files.too-large",
            "rust.fetch",
            "rust.cargo-settings",
            "rust.unused-deps",
            "push.clean-tree",
            "analysis.nirikshak",
            "gate.packages",
        ],
    ),
    (
        Part::StyleDocs,
        &[
            "commit.message",
            "docs.line-length",
            "docs.links",
            "docs.names",
            "docs.one-rule-file",
            "docs.status-words",
            "limits.line-width",
            "code.markers",
            "text.clean",
            "shell.format",
            "shell.lint",
            "rust.format",
            "rust.lints",
            "text.typos",
            "metrics.no-vanity",
            "gate.catalogue",
        ],
    ),
];

/// Each analyser rule, by its id, and its part. A rule not named here
/// counts as security, the strictest reading.
const RULES: [(&str, Part); 4] = [
    ("R05", Part::StyleDocs),
    ("R14", Part::Tests),
    ("H42", Part::Tests),
    ("M08", Part::Tests),
];

/// The clippy lints that measure complexity: a function too long, too
/// complex, too deep or with too many inputs.
const COMPLEX: [&str; 5] = [
    "too_many_lines",
    "cognitive_complexity",
    "excessive_nesting",
    "too_many_arguments",
    "fn_params_excessive_bools",
];

/// The part one finding falls in.
#[must_use]
pub fn part(f: &Finding) -> Part {
    // A rule id is a letter and two digits, such as `M02`.
    const ID_LEN: usize = 3;
    match f.check {
        // An analyser finding starts with its rule's id; a finding with
        // none means the analyser could not run.
        "analysis.nirikshak" => {
            let id = f.message.split(' ').next().unwrap_or_default();
            let is_id =
                id.len() == ID_LEN && id.starts_with(char::is_uppercase);
            let named = RULES.iter().find(|(r, _)| *r == id).map(|r| r.1);
            match named {
                _ if !is_id => Part::SetUp,
                Some(part) => part,
                None if id.starts_with('M') => Part::Smells,
                None => Part::Security,
            }
        }
        "rust.lints" if COMPLEX.iter().any(|l| f.message.contains(l)) => {
            Part::Complexity
        }
        check if check.starts_with("door.") => Part::SetUp,
        check => CHECKS
            .iter()
            .find(|(_, names)| names.contains(&check))
            .map_or(Part::StyleDocs, |(p, _)| *p),
    }
}

/// The checks named here, for the test that keeps the list whole.
#[cfg(test)]
pub fn named() -> Vec<&'static str> {
    CHECKS
        .iter()
        .flat_map(|(_, names)| names.iter().copied())
        .collect()
}
