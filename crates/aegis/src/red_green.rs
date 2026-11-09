//! The test-first proof (task #39, from the old gate's test-first check).
//! A commit's `TDD:` line names the test that failed before the change.
//! At push, for each such commit, the named test is run twice:
//!
//! - red: on the commit's parent with the commit's own test files laid
//!   over it. It must fail, or not exist there yet. A compile error
//!   counts only when it is in one of those test files;
//! - green: on the pushed code. It must run and pass.
//!
//! A test file is a file under a `tests/` folder or one whose name ends in
//! `_tests.rs`. Tests written inside a source file are not laid over, so
//! a new one counts as not existing before.

use std::path::Path;
use std::process::{Command, Stdio};

use crate::Finding;
use crate::history::{Commit, fresh_folder, parent_with_tests};
use crate::repo::Repo;
use crate::tdd::{self, Tdd};

pub use crate::run_outcome::{Counted, Outcome, outcome};

const CHECK: &str = "commit.test-first";

/// The ways to run a named test, as `cargo test` arguments. A path whose
/// first part names a file in the crate's `tests/` folder may be that
/// integration test or a module of the library, so both are tried.
#[must_use]
pub fn targets(root: &Path, krate: &str, path: &str) -> Vec<Vec<String>> {
    let package = krate.replace('_', "-");
    let folder = root.join("crates").join(&package);
    if !folder.join("Cargo.toml").is_file() {
        return Vec::new();
    }
    let run = |kind: &[&str], name: &str| -> Vec<String> {
        let mut args =
            vec!["test".to_owned(), "-p".to_owned(), package.clone()];
        args.extend(kind.iter().map(|s| (*s).to_owned()));
        args.extend(["--".to_owned(), "--exact".to_owned(), name.to_owned()]);
        args
    };
    let mut out = Vec::new();
    if let Some((file, rest)) = path.split_once("::")
        && folder.join("tests").join(format!("{file}.rs")).is_file()
    {
        out.push(run(&["--test", file], rest));
    }
    if folder.join("src").join("lib.rs").is_file() {
        out.push(run(&["--lib"], path));
    }
    out
}

/// Runs the named test in `folder` every way it can be run, with
/// `cargo` as the command that starts cargo (perhaps through the
/// queue) and build output in `target`. A pass anywhere is a pass;
/// otherwise a failure anywhere is a failure.
#[must_use]
pub fn run(
    folder: &Path,
    cargo: &[&str],
    target: &Path,
    test: (&str, &str),
    counted: Counted<'_>,
) -> Outcome {
    let (krate, path) = test;
    let Some((program, before)) = cargo.split_first() else {
        return Outcome::Failed;
    };
    let mut seen = Vec::new();
    for args in targets(folder, krate, path) {
        let result = Command::new(program)
            .args(before)
            .args(&args)
            .current_dir(folder)
            .env("CARGO_TARGET_DIR", target)
            .stdin(Stdio::null())
            .output();
        seen.push(match result {
            Ok(o) => outcome(
                o.status.success(),
                &String::from_utf8_lossy(&o.stdout),
                &String::from_utf8_lossy(&o.stderr),
                counted,
            ),
            Err(e) => Outcome::Broken(format!("cannot start cargo: {e}")),
        });
    }
    if seen.contains(&Outcome::Passed) {
        return Outcome::Passed;
    }
    if let Some(broken) = seen
        .iter()
        .find(|o| matches!(o, Outcome::Broken(_)))
        .cloned()
    {
        return broken;
    }
    if seen.contains(&Outcome::Failed) {
        Outcome::Failed
    } else {
        Outcome::Absent
    }
}

/// Whether `path` is a test file that is laid over the parent.
#[must_use]
pub fn is_test_file(path: &str) -> bool {
    path.starts_with("tests/")
        || path.contains("/tests/")
        || path.ends_with("_tests.rs")
}

/// The findings for one commit, given its red and green outcomes.
#[must_use]
pub fn judge(test: &Tdd, red: &Outcome, green: &Outcome) -> Vec<Finding> {
    let Tdd::Test { krate, path } = test else {
        return Vec::new();
    };
    let name = format!("{krate}::{path}");
    let mut out = Vec::new();
    for (when, run) in [("before", &red), ("after", &green)] {
        if let Outcome::Broken(why) = run {
            out.push(Finding::new(
                CHECK,
                "",
                0,
                format!("`{name}` could not be run {when} the change: {why}"),
            ));
        }
    }
    if *red == Outcome::Passed {
        out.push(Finding::new(
            CHECK,
            "",
            0,
            format!(
                "`{name}` already passed before the change, so it does \
                 not prove it"
            ),
        ));
    }
    if !matches!(green, Outcome::Passed | Outcome::Broken(_)) {
        out.push(Finding::new(
            CHECK,
            "",
            0,
            format!("`{name}` must run and pass on the pushed code"),
        ));
    }
    out
}

/// Makes the build folders first: a sealed job may write only to
/// folders that already exist, so a missing one would only show as a
/// broken run.
fn make_folders(folders: &[&Path]) -> Option<Finding> {
    folders.iter().find_map(|folder| {
        std::fs::create_dir_all(folder).err().map(|e| {
            Finding::new(
                CHECK,
                "",
                0,
                format!("cannot make {}: {e}", folder.display()),
            )
        })
    })
}

/// The commit's own test files. Before the change, only a compile error
/// in one of these counts as the test failing; one elsewhere is not
/// proof.
fn own_tests(commit: &Commit) -> Vec<String> {
    commit
        .paths
        .iter()
        .filter(|p| is_test_file(p))
        .cloned()
        .collect()
}

/// The test-first proof for every commit that names a test.
#[must_use]
pub fn check(repo: &Repo, commits: &[Commit], cargo: &[&str]) -> Vec<Finding> {
    // Cargo judges freshness by file times, so the old tree never shares
    // a build folder with the pushed one.
    let target = repo.root().join("target");
    let red_target = target.join("red-green");
    let mut out = Vec::new();
    if let Some(failed) = make_folders(&[&target, &red_target]) {
        return vec![failed];
    }
    for commit in commits {
        let Some(test) =
            tdd::value(&commit.message).as_deref().and_then(tdd::parse)
        else {
            continue;
        };
        let Tdd::Test { krate, path } = &test else {
            continue;
        };
        let name = (krate.as_str(), path.as_str());
        let own = own_tests(commit);
        let work = crate::history::work_folder(repo.root());
        let dest = match fresh_folder(&work) {
            Ok(dest) => dest,
            Err(e) => {
                out.push(Finding::new(CHECK, "", 0, e));
                continue;
            }
        };
        // The old tree is fetched outside the seal first, so a missing
        // library never passes for a red test.
        let red = parent_with_tests(repo, commit, &dest).and_then(|has| {
            if !has {
                return Ok(Outcome::Absent);
            }
            if crate::file_kinds::has_cargo_settings(&dest) {
                return Err("the old tree holds cargo settings".to_owned());
            }
            match crate::tiers::fetch_libraries(&dest) {
                Some(failed) => Err(failed.message),
                None => Ok(run(&dest, cargo, &red_target, name, Some(&own))),
            }
        });
        let _ignored = std::fs::remove_dir_all(&dest);
        let red = match red {
            Ok(red) => red,
            Err(e) => {
                out.push(Finding::new(CHECK, "", 0, e));
                continue;
            }
        };
        let green = run(repo.content(), cargo, &target, name, None);
        out.extend(judge(&test, &red, &green));
    }
    out
}

#[cfg(test)]
#[path = "red_green_tests.rs"]
mod tests;
