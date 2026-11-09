//! The code analyser as a gate check (task #68). nirikshak reads facts
//! about every Rust file and page in the project and applies its rules,
//! which see across files and languages: a test that checks nothing
//! through all its helpers, a setting read in code that no page
//! explains. Every finding blocks, and so does a file it cannot read or
//! a rule it cannot run.

use crate::Finding;
use crate::repo::Repo;

const CHECK: &str = "analysis.nirikshak";

/// Runs every rule over the files among `all` it reads: Rust and
/// Markdown so far.
#[must_use]
pub fn check(repo: &Repo, all: &[String]) -> Vec<Finding> {
    let files: Vec<(String, String)> = all
        .iter()
        .filter(|p| nirikshak::reads(p))
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
