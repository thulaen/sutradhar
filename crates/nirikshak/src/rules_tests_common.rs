//! Helpers for the rule tests.

use super::{ALL, Source, about, run};
use crate::facts::Facts;

pub fn facts(files: &[(&str, &str)]) -> Facts {
    let owned: Vec<(String, String)> = files
        .iter()
        .map(|(p, t)| ((*p).to_owned(), (*t).to_owned()))
        .collect();
    crate::read(&owned)
}

pub fn rule(id: &str) -> Source {
    let found = ALL
        .iter()
        .find(|(_, text)| about("x", text).is_ok_and(|a| a.id == id));
    match found {
        Some(source) => *source,
        None => ("missing.rego", ""),
    }
}

/// The lines a rule finds in the files.
pub fn lines(id: &str, files: &[(&str, &str)]) -> Vec<usize> {
    let found = run(rule(id), &facts(files));
    found.map_or_else(
        |e| vec![usize::MAX; usize::from(!e.is_empty())],
        |f| f.iter().map(|x| x.line).collect(),
    )
}
