//! The catalogue check (roadmap Phase 2 item 2). `gate/catalogue.toml`
//! lists every check the gate can report, the owner's gate items it
//! covers, its tiers, and its proof: a test that plants a fault and sees
//! it refused. This check keeps the list and the code in step: every
//! check the code reports is listed and nothing else is, every gate item
//! named is real, and every proof test exists.

use serde::Deserialize;

use crate::Finding;
use crate::tdd::{self, Tdd};

const CHECK: &str = "gate.catalogue";

/// Where the catalogue lives.
pub const PATH: &str = "gate/catalogue.toml";

/// Where the owner's gate items are listed.
pub const ITEMS: &str = "docs/standards/gate-items.md";

/// Every check the gate can report. A test keeps this list and the code
/// in step.
pub const REPORTED: [&str; 37] = [
    "commit.message",
    "commit.tdd",
    "commit.test-first",
    "docs.line-length",
    "docs.links",
    "docs.names",
    "docs.one-rule-file",
    "docs.status-words",
    "files.allowed-kind",
    "files.no-links",
    "files.too-large",
    "limits.file-length",
    "limits.line-width",
    "code.markers",
    "text.clean",
    "secrets.gitleaks",
    "secrets.private-data",
    "secrets.private-lists",
    "workflows.sealed",
    "shell.format",
    "shell.lint",
    "rust.format",
    "rust.lints",
    "rust.tests",
    "rust.fetch",
    "rust.cargo-settings",
    "rust.deny",
    "rust.unused-deps",
    "supply.sbom",
    "text.typos",
    "push.clean-tree",
    "proofs.no-tricks",
    "metrics.no-vanity",
    "analysis.nirikshak",
    "reviews.required",
    "gate.catalogue",
    "gate.packages",
];

const TIERS: [&str; 3] = ["commit", "push", "release"];

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Catalogue {
    check: Vec<Entry>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Entry {
    id: String,
    /// The gate items it covers in full.
    #[serde(default)]
    items: Vec<String>,
    /// The gate items it covers only in part.
    #[serde(default)]
    partly: Vec<String>,
    tiers: Vec<String>,
    proof: String,
}

fn problem(message: impl Into<String>) -> Finding {
    Finding::new(CHECK, PATH, 0, message)
}

/// The gate item IDs in the items list, such as `GI-04`.
fn item_ids(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|l| l.strip_prefix("| GI-"))
        .filter_map(|rest| rest.split(' ').next())
        .map(|n| format!("GI-{n}"))
        .collect()
}

/// The files a test path may live in: an integration test file named by
/// its first part, or the library module it names (with this
/// repository's `x_tests.rs` file for a module's `tests`).
fn proof_files(krate: &str, path: &str) -> Vec<String> {
    let base = format!("crates/{}", krate.replace('_', "-"));
    let mods: Vec<&str> = path.split("::").collect();
    let mods = mods.get(..mods.len().saturating_sub(1)).unwrap_or_default();
    let mut out = Vec::new();
    if let [file] = mods {
        out.push(format!("{base}/tests/{file}.rs"));
    }
    let lib = match mods.split_last() {
        Some((&"tests", rest)) => rest,
        _ => mods,
    };
    if lib.is_empty() {
        out.push(format!("{base}/src/lib.rs"));
    } else {
        let module = format!("{base}/src/{}", lib.join("/"));
        out.extend(
            [".rs", "/mod.rs", "_tests.rs"].map(|e| format!("{module}{e}")),
        );
    }
    out
}

/// Whether `text` defines `name` as a test: `fn name(` with `#[test]`
/// among the attributes just above it.
fn is_test(text: &str, name: &str) -> bool {
    let wanted = format!("fn {name}(");
    let lines: Vec<&str> = text.lines().map(str::trim).collect();
    lines.iter().enumerate().any(|(i, line)| {
        line.starts_with(&wanted)
            && lines
                .get(..i)
                .unwrap_or_default()
                .iter()
                .rev()
                .take_while(|l| l.starts_with("#[") || l.starts_with("///"))
                .any(|l| *l == "#[test]")
    })
}

/// Whether the proof test exists: a `#[test]` fn of that name in the
/// file its path names.
fn proof_exists(proof: &str, files: &[(String, String)]) -> bool {
    let Some(Tdd::Test { krate, path }) = tdd::parse(proof) else {
        return false;
    };
    let name = path.rsplit("::").next().unwrap_or(&path);
    let homes = proof_files(&krate, &path);
    files
        .iter()
        .any(|(p, text)| homes.contains(p) && is_test(text, name))
}

fn check_entry(
    entry: &Entry,
    items: &[String],
    files: &[(String, String)],
) -> Vec<Finding> {
    let mut out = Vec::new();
    let id = &entry.id;
    if entry.items.is_empty() && entry.partly.is_empty() {
        out.push(problem(format!("`{id}` names no gate item")));
    }
    let named = entry.items.iter().chain(&entry.partly);
    for item in named.filter(|i| !items.contains(i)) {
        out.push(problem(format!(
            "`{id}` names `{item}`, which is not a gate item"
        )));
    }
    let bad_tier = |t: &String| !TIERS.contains(&t.as_str());
    if entry.tiers.is_empty() || entry.tiers.iter().any(bad_tier) {
        out.push(problem(format!(
            "`{id}` must run in commit, push or release"
        )));
    }
    if !proof_exists(&entry.proof, files) {
        out.push(problem(format!(
            "`{id}`: its proof `{}` is not a test that exists",
            entry.proof
        )));
    }
    out
}

/// Checks the catalogue. `read` gives a file's text; `rust_files` are
/// the repository's Rust files with their text, to find proof tests.
#[must_use]
pub fn check(
    read: &dyn Fn(&str) -> Option<String>,
    rust_files: &[(String, String)],
) -> Vec<Finding> {
    let Some(text) = read(PATH) else {
        return vec![problem("the catalogue is missing")];
    };
    let catalogue: Catalogue = match toml::from_str(&text) {
        Ok(c) => c,
        Err(e) => return vec![problem(format!("cannot read it: {e}"))],
    };
    let items = item_ids(&read(ITEMS).unwrap_or_default());
    let mut out = Vec::new();
    let mut seen: Vec<&str> = Vec::new();
    for entry in &catalogue.check {
        if seen.contains(&entry.id.as_str()) {
            out.push(problem(format!("`{}` is listed twice", entry.id)));
        }
        seen.push(&entry.id);
        out.extend(check_entry(entry, &items, rust_files));
    }
    for id in REPORTED.iter().filter(|id| !seen.contains(id)) {
        out.push(problem(format!(
            "`{id}` is reported by the gate but not listed"
        )));
    }
    for id in seen.iter().filter(|id| !REPORTED.contains(id)) {
        out.push(problem(format!(
            "`{id}` is listed but the gate never reports it"
        )));
    }
    out
}

/// The gate items no listed check covers in full yet, in order: first
/// those no check touches at all, then those covered only in part.
///
/// # Errors
/// When the catalogue cannot be read.
pub fn uncovered(
    catalogue: &str,
    items: &str,
) -> Result<(Vec<String>, Vec<String>), String> {
    let catalogue: Catalogue = toml::from_str(catalogue)
        .map_err(|e| format!("cannot read it: {e}"))?;
    let full: Vec<&String> = catalogue
        .check
        .iter()
        .flat_map(|e| e.items.iter())
        .collect();
    let part: Vec<&String> = catalogue
        .check
        .iter()
        .flat_map(|e| e.partly.iter())
        .collect();
    let listed = item_ids(items);
    if listed.is_empty() {
        return Err("the list of gate items is missing or empty".to_owned());
    }
    let (partly, none): (Vec<String>, Vec<String>) = listed
        .into_iter()
        .filter(|i| !full.contains(&i))
        .partition(|i| part.contains(&i));
    Ok((none, partly))
}

#[cfg(test)]
#[path = "catalogue_tests.rs"]
mod tests;
