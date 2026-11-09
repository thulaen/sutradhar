//! The rules, written in Rego and built into the program, and how they
//! are run. Each rule file starts with comment lines naming its `id`,
//! `title`, `why` and `fix`, then the package `nirikshak.<id in lower
//! case>`, whose set `findings` holds objects with a `path`, a `line`
//! and a `what`. A rule that cannot be read or run, or that gives a
//! finding of any other shape, blocks the run: it never passes unseen.

use serde_json::Value;

use crate::facts::Facts;

/// One rule: its file name and text.
pub type Source = (&'static str, &'static str);

/// Every rule, in order.
pub const ALL: [Source; 20] = [
    (
        "r05_environment.rego",
        include_str!("../rules/r05_environment.rego"),
    ),
    (
        "r08_timeouts.rego",
        include_str!("../rules/r08_timeouts.rego"),
    ),
    (
        "r09_swallowed_errors.rego",
        include_str!("../rules/r09_swallowed_errors.rego"),
    ),
    (
        "r14_tests_prove_nothing.rego",
        include_str!("../rules/r14_tests_prove_nothing.rego"),
    ),
    (
        "r18_unbounded_channels.rego",
        include_str!("../rules/r18_unbounded_channels.rego"),
    ),
    (
        "r19_sql_from_strings.rego",
        include_str!("../rules/r19_sql_from_strings.rego"),
    ),
    (
        "s01_shell_commands.rego",
        include_str!("../rules/s01_shell_commands.rego"),
    ),
    (
        "s02_tls_checks.rego",
        include_str!("../rules/s02_tls_checks.rego"),
    ),
    (
        "s03_weak_hashes.rego",
        include_str!("../rules/s03_weak_hashes.rego"),
    ),
    (
        "h06_open_permissions.rego",
        include_str!("../rules/h06_open_permissions.rego"),
    ),
    (
        "h11_unbuffered_writes.rego",
        include_str!("../rules/h11_unbuffered_writes.rego"),
    ),
    (
        "h22_destructive_sql.rego",
        include_str!("../rules/h22_destructive_sql.rego"),
    ),
    (
        "h23_check_then_set.rego",
        include_str!("../rules/h23_check_then_set.rego"),
    ),
    (
        "h25_cache_never_cleared.rego",
        include_str!("../rules/h25_cache_never_cleared.rego"),
    ),
    (
        "h35_panic_in_drop.rego",
        include_str!("../rules/h35_panic_in_drop.rego"),
    ),
    (
        "h39_writes_outside_transaction.rego",
        include_str!("../rules/h39_writes_outside_transaction.rego"),
    ),
    (
        "h42_too_few_cases.rego",
        include_str!("../rules/h42_too_few_cases.rego"),
    ),
    (
        "h44_local_clock.rego",
        include_str!("../rules/h44_local_clock.rego"),
    ),
    ("h53_leaks.rego", include_str!("../rules/h53_leaks.rego")),
    (
        "h56_platform_paths.rego",
        include_str!("../rules/h56_platform_paths.rego"),
    ),
];

/// What a rule says about itself.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct About {
    /// Its short name, such as `R08`.
    pub id: String,
    /// What it finds, in a few words.
    pub title: String,
    /// Why that matters.
    pub why: String,
    /// How to put it right.
    pub fix: String,
}

/// One thing a rule found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Found {
    /// The rule that found it.
    pub rule: About,
    /// The file, relative to the analysed root.
    pub path: String,
    /// The line, from 1.
    pub line: usize,
    /// What exactly was found there.
    pub what: String,
}

/// The rule's own description, from its first comment lines.
///
/// # Errors
/// When a part is missing or empty.
pub fn about(name: &str, text: &str) -> Result<About, String> {
    let field = |key: &str| {
        text.lines()
            .map_while(|l| l.strip_prefix('#'))
            .find_map(|l| l.trim().strip_prefix(&format!("{key}:")))
            .map(|v| v.trim().to_owned())
            .filter(|v| !v.is_empty())
            .ok_or_else(|| format!("rule {name} does not say its `{key}`"))
    };
    Ok(About {
        id: field("id")?,
        title: field("title")?,
        why: field("why")?,
        fix: field("fix")?,
    })
}

fn found(rule: &About, item: &Value) -> Option<Found> {
    let path = item.get("path")?.as_str()?.to_owned();
    let line = usize::try_from(item.get("line")?.as_u64()?).ok()?;
    let what = item.get("what")?.as_str()?.to_owned();
    (item.as_object()?.len() == 3).then(|| Found {
        rule: rule.clone(),
        path,
        line,
        what,
    })
}

/// Runs every rule in `sources` over the facts, in one engine, so the
/// facts are handed over once. Each rule's findings, or why it could not
/// run, come back in order.
#[must_use]
pub fn run_all(
    sources: &[Source],
    facts: &Facts,
) -> Vec<Result<Vec<Found>, String>> {
    let mut engine = regorus::Engine::new();
    let input = regorus::Value::from_json_str(&facts.to_json().to_string());
    match input {
        Ok(input) => engine.set_input(input),
        Err(e) => {
            let why = format!("the facts cannot be handed over: {e}");
            return sources.iter().map(|_| Err(why.clone())).collect();
        }
    }
    let mut loaded = Vec::new();
    for (name, text) in sources {
        let added = about(name, text).and_then(|rule| {
            engine
                .add_policy((*name).to_owned(), (*text).to_owned())
                .map(|_package| rule)
                .map_err(|e| format!("rule {name}: {e}"))
        });
        loaded.push(added);
    }
    sources
        .iter()
        .zip(loaded)
        .map(|((name, _), rule)| {
            rule.and_then(|r| findings(&mut engine, name, &r))
        })
        .collect()
}

/// Runs one rule over the facts.
///
/// # Errors
/// When the rule cannot be read or run, or gives a finding of the wrong
/// shape.
pub fn run(source: Source, facts: &Facts) -> Result<Vec<Found>, String> {
    run_all(&[source], facts)
        .pop()
        .unwrap_or_else(|| Err("no rule was run".to_owned()))
}

fn findings(
    engine: &mut regorus::Engine,
    name: &str,
    rule: &About,
) -> Result<Vec<Found>, String> {
    let bad = |e: &dyn std::fmt::Display| format!("rule {name}: {e}");
    let query = format!("data.nirikshak.{}.findings", rule.id.to_lowercase());
    let result = engine.eval_rule(query).map_err(|e| bad(&e))?;
    let json: Value = result
        .to_json_str()
        .map_err(|e| bad(&e))
        .and_then(|t| serde_json::from_str(&t).map_err(|e| bad(&e)))?;
    let items = json
        .as_array()
        .ok_or_else(|| bad(&"`findings` is not a set"))?;
    items
        .iter()
        .map(|item| {
            found(rule, item)
                .ok_or_else(|| bad(&"a finding has the wrong shape"))
        })
        .collect()
}

#[cfg(test)]
#[path = "rules_tests_common.rs"]
mod tests_common;

#[cfg(test)]
#[path = "rules_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "rules_h_tests.rs"]
mod h_tests;
