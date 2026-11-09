//! Holes in container images (task #114; the rule is in AGENTS.md
//! section 5.7). Every image pinned in `gate/tools.toml` is scanned with
//! the pinned scanner, Grype, against its list of known holes, and a
//! high or critical hole that has a fix is refused. Every image digest
//! named in the set-up files under `docker/` and `deploy/` must be one
//! the list pins, so the images that run are the images scanned.
//!
//! The list of holes is kept outside the repository, in the folder
//! `AEGIS_GRYPE_DB` names, and fetched there first, outside any seal.
//! Grype is run with an empty settings file, in its own folder, with no
//! `GRYPE_` setting from outside, and with a time limit. A missing
//! scanner, a failed or late scan, or an image entry that is not whole
//! fails the check; it never skips it (AGENTS.md section 7).

use std::path::{Component, Path, PathBuf};
use std::process::Command;

use serde_json::Value;

use crate::Finding;
use crate::tools_list;

const CHECK: &str = "images.holes";

/// How many holes are named for one image; the rest are counted.
const NAMED: usize = 12;

/// The most seconds fetching the list of holes may take.
const UPDATE_SECONDS: &str = "900";

/// The most seconds one image's scan may take; a cold scan has taken
/// 220.
const SCAN_SECONDS: &str = "600";

/// The high and critical holes in the scanner's `CycloneDX` report, each
/// as one line of plain text. The scan is made with `--only-fixed`, so
/// every hole in the report has a fix. A report with no holes has no
/// list of them.
///
/// # Errors
/// When the text is not a `CycloneDX` report.
pub fn holes(report: &str) -> Result<Vec<String>, String> {
    let report: Value = serde_json::from_str(report)
        .map_err(|e| format!("the scanner's report cannot be read: {e}"))?;
    if report.get("bomFormat").and_then(Value::as_str) != Some("CycloneDX") {
        return Err("the scanner's report is not a CycloneDX report".into());
    }
    let text = |v: Option<&Value>| {
        v.and_then(Value::as_str).unwrap_or_default().to_owned()
    };
    let none = Vec::new();
    let found = report
        .get("vulnerabilities")
        .and_then(Value::as_array)
        .unwrap_or(&none);
    let mut out = Vec::new();
    for hole in found {
        let ratings = hole.get("ratings").and_then(Value::as_array);
        let serious = ratings.into_iter().flatten().find_map(|r| {
            let level = text(r.get("severity")).to_ascii_lowercase();
            matches!(level.as_str(), "high" | "critical").then_some(level)
        });
        let Some(level) = serious else { continue };
        let id = text(hole.get("id"));
        let parts = hole.get("affects").and_then(Value::as_array);
        let mut named: Vec<String> = parts
            .into_iter()
            .flatten()
            .map(|p| text(p.get("ref")))
            .map(|r| r.split('?').next().unwrap_or_default().to_owned())
            .collect();
        if named.is_empty() {
            named.push("a part the report does not name".to_owned());
        }
        out.extend(
            named.iter().map(|what| format!("{id} ({level}) in {what}")),
        );
    }
    out.sort();
    out.dedup();
    Ok(out)
}

/// The folder for the list of holes: it must be named, and lie outside
/// the repository, where no pushed code can write to it.
///
/// # Errors
/// When it is not named, is not a full path, holds `..`, cannot be
/// made or resolved, lies inside `root`, or holds `root`.
pub fn database(
    root: &Path,
    named: Option<PathBuf>,
) -> Result<PathBuf, String> {
    let db = named.ok_or(
        "AEGIS_GRYPE_DB must name the folder for the list of holes \
         (the setting sutradhar.grypedb)",
    )?;
    let plain = db
        .components()
        .all(|c| matches!(c, Component::RootDir | Component::Normal(_)));
    if !db.is_absolute() || !plain || db.file_name().is_none() {
        return Err(format!(
            "AEGIS_GRYPE_DB must be a full path with no `..`: {}",
            db.display()
        ));
    }
    // The folder is made first, so links on the way are followed when
    // it is compared with the repository.
    let full = std::fs::create_dir_all(&db)
        .and_then(|()| std::fs::canonicalize(&db))
        .map_err(|e| format!("AEGIS_GRYPE_DB {}: {e}", db.display()))?;
    let top = std::fs::canonicalize(root).unwrap_or_else(|_| root.to_owned());
    if full.starts_with(&top) || top.starts_with(&full) {
        return Err(
            "AEGIS_GRYPE_DB must lie outside the repository, and not hold it"
                .into(),
        );
    }
    Ok(full)
}

/// Runs the scanner `program` with `args` and a time limit, its list of
/// holes in `db`, and gives what it printed. It runs in a fresh folder,
/// `aegis-work` inside the list's folder, on disk, with an empty
/// settings file and none of
/// the caller's `GRYPE_` settings, so no settings file can hide holes.
fn grype(
    program: &str,
    db: &Path,
    seconds: &str,
    args: &[&str],
) -> Result<String, String> {
    // Inside the list's own folder, which lies outside the repository
    // and does not hold it, so clearing it touches nothing else.
    let work = db.join("aegis-work");
    let _old = std::fs::remove_dir_all(&work);
    let empty = work.join("settings.yaml");
    std::fs::create_dir_all(work.join("home"))
        .and_then(|()| std::fs::write(&empty, ""))
        .map_err(|e| format!("cannot make {}: {e}", work.display()))?;
    let mut command = Command::new("timeout");
    let _set = command
        .args(["--kill-after=10", seconds, program, "-c"])
        .arg(&empty)
        .args(args)
        .current_dir(&work);
    for (key, _) in std::env::vars_os() {
        if key.to_string_lossy().starts_with("GRYPE_") {
            let _gone = command.env_remove(key);
        }
    }
    let _set = command
        .env("HOME", work.join("home"))
        .env("XDG_CONFIG_HOME", work.join("home"))
        .env("GRYPE_DB_CACHE_DIR", db)
        .env("GRYPE_DB_AUTO_UPDATE", "false")
        .env("GRYPE_CHECK_FOR_APP_UPDATE", "false")
        .env("TMPDIR", &work);
    crate::git_env::run_with_input(&mut command, b"", program, &args.join(" "))
}

/// The images the set-up files name that the list does not pin.
fn unpinned(
    images: &[(String, String)],
    used: &[(String, String)],
) -> Vec<Finding> {
    let mut out = Vec::new();
    for (path, text) in used {
        let mut say = |why: String| out.push(Finding::new(CHECK, path, 0, why));
        match refs::named(path, text) {
            Ok(named) => {
                named
                    .iter()
                    .filter_map(|i| refs::problem(i, images))
                    .for_each(&mut say);
            }
            Err(why) => say(why),
        }
        for digest in refs::loose_digests(text, images) {
            say(format!(
                "the image {digest} is not pinned in gate/tools.toml, so it \
                 is never scanned"
            ));
        }
    }
    out
}

/// Scans every image pinned in `tools` with the scanner `program`, the
/// list of holes in `db`; `used` holds the set-up files (path, text).
#[must_use]
pub fn scan_with(
    program: &str,
    db: Result<PathBuf, String>,
    tools: &str,
    used: &[(String, String)],
) -> Vec<Finding> {
    let problem = |why: String| Finding::new(CHECK, tools_list::PATH, 0, why);
    let images = match tools_list::images(tools) {
        Ok(images) => images,
        Err(why) => return vec![problem(why)],
    };
    let mut out = unpinned(&images, used);
    if images.is_empty() {
        return out;
    }
    let db = match db {
        Ok(db) => db,
        Err(why) => {
            out.push(problem(why));
            return out;
        }
    };
    // Fetching the list of holes runs no code from the repository.
    if let Err(why) =
        grype(program, &db, UPDATE_SECONDS, &["db", "update", "-q"])
    {
        out.push(problem(format!("the list of holes: {why}")));
        return out;
    }
    for (name, image) in images {
        let source = format!("registry:{image}");
        let args = [
            source.as_str(),
            "--platform",
            "linux/amd64",
            "--only-fixed",
            "-o",
            "cyclonedx-json",
            "-q",
        ];
        match grype(program, &db, SCAN_SECONDS, &args).and_then(|r| holes(&r)) {
            Ok(holes) if holes.is_empty() => {}
            Ok(holes) => {
                let more = holes.len().saturating_sub(NAMED);
                let mut named: Vec<String> =
                    holes.iter().take(NAMED).cloned().collect();
                if more > 0 {
                    named.push(format!("and {more} more"));
                }
                out.push(problem(format!(
                    "`{name}` ({image}) has {} serious hole(s) with a fix: \
                     {}; pin a fixed image",
                    holes.len(),
                    named.join("; ")
                )));
            }
            Err(why) => {
                out.push(problem(format!(
                    "`{name}` could not be scanned: {why}"
                )));
            }
        }
    }
    out
}

/// Scans every image pinned in `tools` with the pinned `grype`, its list
/// of holes in the folder `AEGIS_GRYPE_DB` names. No list means no
/// images.
#[must_use]
pub fn scan(
    root: &Path,
    tools: Option<&str>,
    used: &[(String, String)],
) -> Vec<Finding> {
    let named = std::env::var_os("AEGIS_GRYPE_DB").map(PathBuf::from);
    tools.map_or_else(Vec::new, |t| {
        scan_with("grype", database(root, named), t, used)
    })
}

#[path = "images_refs.rs"]
mod refs;

#[cfg(test)]
#[path = "images_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "images_files_tests.rs"]
mod files_tests;
