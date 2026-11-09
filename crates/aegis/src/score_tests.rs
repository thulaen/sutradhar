#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use super::parts::named;
use super::{BAR, PARTS, Part, line, of, part};
use crate::Finding;

#[test]
fn every_check_the_gate_reports_names_its_part() {
    let mut named = named();
    named.sort_unstable();
    let mut reported = crate::catalogue::REPORTED.to_vec();
    reported.sort_unstable();
    assert_eq!(named, reported);
}

#[test]
fn every_analyser_rule_falls_in_the_right_part() {
    let one = |check: &'static str, message: &str| {
        part(&Finding::new(check, "a", 1, message))
    };
    for (name, text) in nirikshak::rules::ALL {
        let id = nirikshak::rules::about(name, text).unwrap().id;
        let got = one("analysis.nirikshak", &format!("{id} Title: x"));
        let want = match id.as_str() {
            "R05" => Part::StyleDocs,
            "R14" | "H42" | "M08" => Part::Tests,
            _ if id.starts_with('M') => Part::Smells,
            _ => Part::Security,
        };
        assert_eq!(got, want, "{id}");
    }
    assert_eq!(one("analysis.nirikshak", "a rule cannot run"), Part::SetUp);
    let complex = "error: this function has too many lines (67/60) \
                   clippy::too_many_lines";
    assert_eq!(one("rust.lints", complex), Part::Complexity);
    assert_eq!(one("rust.lints", "error: unused import"), Part::StyleDocs);
    assert_eq!(one("code.copies", "x"), Part::Copies);
    assert_eq!(one("rust.tests", "x"), Part::Tests);
    assert_eq!(one("door.size", "x"), Part::SetUp);
}

#[test]
fn a_clean_change_scores_the_bar_and_each_problem_counts_once() {
    assert_eq!(of(&[]).value, BAR);
    let found = [
        Finding::new("secrets.gitleaks", "a", 1, "a key"),
        Finding::new("analysis.nirikshak", "a", 2, "M02 Number"),
        Finding::new("analysis.nirikshak", "a", 3, "C02 Root"),
        Finding::new("limits.line-width", "a", 4, "long"),
        Finding::new("reviews.required", "a", 0, "no record"),
    ];
    let score = of(&found);
    assert_eq!(score.value, 50);
    assert_eq!(score.counts.len(), PARTS.len());
    let text = line(&score);
    assert_eq!(
        text,
        "aegis: quality score 50 of 100 (problems: security 2, tests 0, \
         copies 0, smells 1, complexity 0, reviews 1, set-up 0, \
         style and docs 1)"
    );
}
