//! The code analyser as a gate check (task #68). nirikshak reads facts
//! about every Rust file in the project and applies its rules, which see
//! across files: a test that checks nothing through all its helpers, an
//! error thrown away, a network call with no time limit. Every finding
//! blocks, and so does a file it cannot read or a rule it cannot run.

use crate::Finding;
use crate::repo::Repo;

const CHECK: &str = "analysis.nirikshak";

/// Runs every rule over the Rust files among `all`.
#[must_use]
pub fn check(repo: &Repo, all: &[String]) -> Vec<Finding> {
    let files: Vec<(String, String)> = all
        .iter()
        .filter(|p| {
            std::path::Path::new(p)
                .extension()
                .is_some_and(|e| e.eq_ignore_ascii_case("rs"))
        })
        .filter_map(|p| Some((p.clone(), repo.read(p)?)))
        .collect();
    let report = nirikshak::analyse(&files);
    let mut out: Vec<Finding> = report
        .blocked
        .into_iter()
        .map(|why| Finding::new(CHECK, "", 0, why))
        .collect();
    out.extend(report.found.into_iter().map(|f| {
        Finding::new(
            CHECK,
            &f.path,
            f.line,
            format!(
                "{} {}: `{}`. {} {}",
                f.rule.id, f.rule.title, f.what, f.rule.why, f.rule.fix
            ),
        )
    }));
    out
}
