//! The one-library-per-job check (task #21; the old gate's overlap rule).
//! `gate/packages.toml` lists the only outside libraries the project may
//! use, each with its job. Every library a crate uses, in any section
//! (normal, dev, build, and their `target` forms) and through the root's
//! `[workspace.dependencies]`, must be listed by its real package name;
//! no two listed libraries may share a job; and a listed library nothing
//! uses is refused. Links to our own crates count only when they stay
//! inside the repository and name no other source. Git and other
//! registries, and any `[patch]` or `[replace]`, are refused. A library
//! declared but never used in code is caught by `rust.unused-deps`.

use serde::Deserialize;
use toml::{Table, Value};

use crate::Finding;

const CHECK: &str = "gate.packages";

/// Where the list lives.
pub const PATH: &str = "gate/packages.toml";

const SECTIONS: [&str; 3] =
    ["dependencies", "dev-dependencies", "build-dependencies"];

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct List {
    package: Vec<Package>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Package {
    name: String,
    job: String,
}

fn problem(path: &str, message: impl Into<String>) -> Finding {
    Finding::new(CHECK, path, 0, message)
}

/// Whether a path link from `manifest` leads to one of our own crates,
/// in the repository's `crates` folder.
fn inside(manifest: &str, link: &str) -> bool {
    if link.starts_with('/') {
        return false;
    }
    let mut depth: Vec<&str> = manifest.split('/').collect();
    let _file = depth.pop();
    for part in link.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                if depth.pop().is_none() {
                    return false;
                }
            }
            other => depth.push(other),
        }
    }
    depth.first() == Some(&"crates") && depth.len() == 2
}

/// One dependency entry: its real package name, or why it is refused,
/// or `None` for a link to our own crate.
fn entry(
    manifest: &str,
    key: &str,
    spec: &Value,
    shared: &Table,
) -> Result<Option<String>, String> {
    let Some(t) = spec.as_table() else {
        return Ok(Some(key.to_owned()));
    };
    if t.get("workspace").and_then(Value::as_bool) == Some(true) {
        let root = shared.get(key).ok_or_else(|| {
            format!("`{key}` names a workspace library the root lacks")
        })?;
        return entry("Cargo.toml", key, root, &Table::new());
    }
    if ["git", "registry", "registry-index"]
        .iter()
        .any(|k| t.contains_key(*k))
    {
        return Err(format!("`{key}` comes from outside crates.io"));
    }
    if let Some(link) = t.get("path").and_then(Value::as_str) {
        if t.contains_key("version") || !inside(manifest, link) {
            return Err(format!("`{key}` links outside our own crates"));
        }
        return Ok(None);
    }
    let real = t.get("package").and_then(Value::as_str).unwrap_or(key);
    Ok(Some(real.to_owned()))
}

/// Every dependency table in a manifest, plain and per target.
fn tables(manifest: &Table) -> Vec<&Table> {
    let mut out: Vec<&Table> = SECTIONS
        .iter()
        .filter_map(|s| manifest.get(*s)?.as_table())
        .collect();
    if let Some(targets) = manifest.get("target").and_then(Value::as_table) {
        for target in targets.values().filter_map(Value::as_table) {
            out.extend(
                SECTIONS.iter().filter_map(|s| target.get(*s)?.as_table()),
            );
        }
    }
    out
}

/// The outside libraries one manifest uses, by real package name.
fn used_by(
    path: &str,
    text: &str,
    shared: &Table,
    out: &mut Vec<Finding>,
) -> Vec<String> {
    let manifest: Table = match toml::from_str(text) {
        Ok(m) => m,
        Err(e) => {
            out.push(problem(path, format!("cannot read it: {e}")));
            return Vec::new();
        }
    };
    if manifest.contains_key("patch") || manifest.contains_key("replace") {
        out.push(problem(path, "libraries may not be patched or replaced"));
    }
    let mut names = Vec::new();
    for table in tables(&manifest) {
        for (key, spec) in table {
            match entry(path, key, spec, shared) {
                Ok(Some(name)) => names.push(name),
                Ok(None) => {}
                Err(e) => out.push(problem(path, e)),
            }
        }
    }
    names
}

fn check_list(list: &List, out: &mut Vec<Finding>) {
    for (i, p) in list.package.iter().enumerate() {
        let earlier = list.package.get(..i).unwrap_or_default();
        if earlier.iter().any(|q| q.name == p.name) {
            out.push(problem(PATH, format!("`{}` is listed twice", p.name)));
        }
        if earlier.iter().any(|q| q.job.eq_ignore_ascii_case(&p.job)) {
            out.push(problem(
                PATH,
                format!("two libraries do the job `{}`", p.job),
            ));
        }
    }
}

/// Checks the list against the root `Cargo.toml` (`root`, when there is
/// one) and every crate's, given as `(path, text)`.
#[must_use]
pub fn check(
    list: Option<&str>,
    root: Option<&str>,
    manifests: &[(String, String)],
) -> Vec<Finding> {
    let Some(text) = list else {
        return vec![problem(PATH, "the list of allowed libraries is missing")];
    };
    let list: List = match toml::from_str(text) {
        Ok(l) => l,
        Err(e) => return vec![problem(PATH, format!("cannot read it: {e}"))],
    };
    let mut out = Vec::new();
    check_list(&list, &mut out);
    let root_table: Table = root
        .and_then(|r| toml::from_str(r).ok())
        .unwrap_or_default();
    let shared = root_table
        .get("workspace")
        .and_then(|w| w.get("dependencies"))
        .and_then(Value::as_table)
        .cloned()
        .unwrap_or_default();
    let mut used = Vec::new();
    if let Some(r) = root {
        used.extend(used_by("Cargo.toml", r, &shared, &mut out));
    }
    for (path, manifest) in manifests {
        used.extend(used_by(path, manifest, &shared, &mut out));
    }
    for name in used
        .iter()
        .filter(|n| !list.package.iter().any(|p| &p.name == *n))
    {
        out.push(problem(
            PATH,
            format!("`{name}` is not on the list of allowed libraries"),
        ));
    }
    for p in list.package.iter().filter(|p| !used.contains(&p.name)) {
        out.push(problem(
            PATH,
            format!("`{}` is listed but nothing uses it", p.name),
        ));
    }
    out
}

#[cfg(test)]
#[path = "packages_tests.rs"]
mod tests;
