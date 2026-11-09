//! nirikshak, the code analyser (task #68; docs/specs/0061-nirikshak.md).
//! It reads facts about the code, then applies rules written in Rego that
//! no single language's own tools can see. Every rule blocks. A file that
//! cannot be read, or a rule that cannot run, blocks too: it is never
//! taken as a pass.

pub mod facts;
pub mod rules;
pub mod rust_facts;
pub mod uses;

use facts::Facts;
use rules::Found;

/// What a run found.
#[derive(Debug, Default)]
pub struct Report {
    /// What the rules found.
    pub found: Vec<Found>,
    /// Why the run could not be trusted: a file not read, or a rule not
    /// run. Any entry blocks.
    pub blocked: Vec<String>,
}

/// Reads the facts of every file given as (path, text). Only Rust is
/// read so far; other files hold no facts any rule uses yet.
#[must_use]
pub fn read(files: &[(String, String)]) -> Facts {
    let mut facts = Facts::default();
    for (path, text) in files.iter().filter(|(p, _)| is_rust(p)) {
        rust_facts::read(path, text, &mut facts);
    }
    facts
}

fn is_rust(path: &str) -> bool {
    std::path::Path::new(path)
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("rs"))
}

/// Applies `rules` to `facts`.
#[must_use]
pub fn check(facts: &Facts, rules: &[rules::Source]) -> Report {
    let mut report = Report::default();
    for failure in facts.of("parse_failure") {
        report
            .blocked
            .push(format!("cannot read a file: {failure}"));
    }
    for rule in rules {
        match rules::run(*rule, facts) {
            Ok(found) => report.found.extend(found),
            Err(e) => report.blocked.push(e),
        }
    }
    report
        .found
        .sort_by(|a, b| (&a.path, a.line).cmp(&(&b.path, b.line)));
    report
}

/// Reads the files and applies every rule.
#[must_use]
pub fn analyse(files: &[(String, String)]) -> Report {
    check(&read(files), &rules::ALL)
}
