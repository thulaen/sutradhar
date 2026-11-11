//! Container image holes (task #114; AGENTS.md section 5.7).
//! Pinned Grype scans every pinned image. Fixable high or critical holes
//! are refused unless `allowed` permits them. Images named in `docker/`
//! and `deploy/` must be pinned.
//!
//! The list of holes is kept outside the repository, in the folder
//! `AEGIS_GRYPE_DB` names, and fetched there first, outside any seal.
//! Grype runs with an empty settings file, in its own folder, with no
//! outside `GRYPE_` setting and a time limit. A missing scanner, a
//! failed or late scan, or an image entry that is not whole fails the
//! check; it never skips it (AGENTS.md section 7).

use std::path::{Component, Path, PathBuf};
use std::process::Command;

use crate::Finding;
use crate::tools_list;

#[path = "images_report.rs"]
mod report;

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
    report::holes(report)
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

/// Scans one image (`name@digest`) and gives its serious holes.
fn scan_image(
    program: &str,
    db: &Path,
    image: &str,
) -> Result<Vec<String>, String> {
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
    grype(program, db, SCAN_SECONDS, &args).and_then(|r| holes(&r))
}

/// The message for an image's serious holes, naming the first few.
fn serious(name: &str, image: &str, holes: &[String]) -> String {
    let more = holes.len().saturating_sub(NAMED);
    let mut named: Vec<String> = holes.iter().take(NAMED).cloned().collect();
    if more > 0 {
        named.push(format!("and {more} more"));
    }
    format!(
        "`{name}` ({image}) has {} serious hole(s) with a fix: {}; pin a \
         fixed image",
        holes.len(),
        named.join("; ")
    )
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
    scan_allowing(program, db, tools, used, &[])
}

/// As [`scan_with`], letting through the holes `allowed` lists (the
/// module `allowed`).
#[must_use]
pub fn scan_allowing(
    program: &str,
    db: Result<PathBuf, String>,
    tools: &str,
    used: &[(String, String)],
    allowed: &[allowed::Entry],
) -> Vec<Finding> {
    let problem = |why: String| Finding::new(CHECK, tools_list::PATH, 0, why);
    let images = match tools_list::images(tools) {
        Ok(images) => images,
        Err(why) => return vec![problem(why)],
    };
    let mut out = refs::unpinned(&images, used);
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
    let (mut seen, mut scanned) = (Vec::new(), Vec::new());
    for (name, image) in images {
        let found = scan_image(program, &db, &image)
            .inspect(|_| scanned.push(name.clone()))
            .map(|h| allowed::not_allowed(&name, h, allowed, &mut seen));
        match found {
            Ok(holes) if holes.is_empty() => {}
            Ok(holes) => out.push(problem(serious(&name, &image, &holes))),
            Err(why) => {
                out.push(problem(format!(
                    "`{name}` could not be scanned: {why}"
                )));
            }
        }
    }
    out.extend(
        allowed::stale(allowed, &seen, &scanned)
            .into_iter()
            .map(problem),
    );
    out
}

/// Scans every image pinned in `tools` with the pinned `grype`, its list
/// of holes in the folder `AEGIS_GRYPE_DB` names, letting through the
/// holes the list in `allowed_text` allows. No list means no images.
#[must_use]
pub fn scan(
    root: &Path,
    tools: Option<&str>,
    used: &[(String, String)],
    allowed_text: Option<&str>,
) -> Vec<Finding> {
    let entries = match allowed::for_scan(tools, allowed_text) {
        Ok(entries) => entries,
        Err(why) => {
            return why
                .into_iter()
                .map(|w| Finding::new(CHECK, allowed::PATH, 0, w))
                .collect();
        }
    };
    let named = std::env::var_os("AEGIS_GRYPE_DB").map(PathBuf::from);
    tools.map_or_else(Vec::new, |t| {
        scan_allowing("grype", database(root, named), t, used, &entries)
    })
}

#[path = "images_allowed.rs"]
pub mod allowed;

#[path = "images_refs.rs"]
mod refs;

#[cfg(test)]
#[path = "images_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "images_files_tests.rs"]
mod files_tests;
