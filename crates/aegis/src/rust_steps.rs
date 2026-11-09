//! The Rust steps of the gate: fetching what the checks need, how cargo
//! is started, and the cargo checks themselves (AGENTS.md section 7).

use crate::repo::Repo;
use crate::{Finding, tool};

/// Fetches the libraries named in `folder`'s lock file, and nothing else.
#[must_use]
pub fn fetch(folder: &std::path::Path) -> Option<Finding> {
    fetch_libraries(folder).or_else(|| {
        tool::run(folder, "rust.deny", "cargo", &["deny", "fetch", "db"])
    })
}

/// Fetches only the exact libraries in the lock file.
#[must_use]
pub fn fetch_libraries(folder: &std::path::Path) -> Option<Finding> {
    tool::run(folder, "rust.fetch", "cargo", &["fetch", "--locked"])
}

/// How to start `cargo`: through the queue `queue` when one is named,
/// sealed, since the code under check is not yet trusted.
#[must_use]
pub fn cargo_command(queue: Option<&str>) -> Vec<String> {
    match queue {
        Some(q) if !q.is_empty() => [q, "run", "--sealed", "--", "cargo"]
            .map(str::to_owned)
            .to_vec(),
        _ => vec!["cargo".to_owned()],
    }
}

/// [`rust_checks`], run with the given command in place of `cargo`. The
/// checks run on the repository's files; build output goes to the
/// repository's own `target` folder so it is reused between runs.
#[must_use]
pub fn rust_checks_using(repo: &Repo, cargo: &[&str]) -> Vec<Finding> {
    if repo.read("Cargo.toml").is_none() {
        return Vec::new();
    }
    let target = repo.root().join("target").display().to_string();
    let lints = [
        "clippy",
        "--workspace",
        "--all-targets",
        "--target-dir",
        &target,
        "--",
        "-D",
        "warnings",
    ];
    let tests = ["test", "--workspace", "--quiet", "--target-dir", &target];
    let deny = [
        "deny",
        "--offline",
        "check",
        "advisories",
        "licenses",
        "bans",
        "sources",
    ];
    let steps: [(&'static str, &[&str]); 3] = [
        ("rust.format", &["fmt", "--all", "--check"]),
        ("rust.lints", &lints),
        ("rust.tests", &tests),
    ];
    // The library rules (AGENTS.md section 5.7) must be written down.
    let mut out = Vec::new();
    if repo.read("deny.toml").is_none() {
        out.push(Finding::new(
            "rust.deny",
            "deny.toml",
            0,
            "a Rust project needs deny.toml, its library rules",
        ));
    }
    let Some((program, before)) = cargo.split_first() else {
        return vec![Finding::new(
            "rust.tests",
            "",
            0,
            "no command was given to run the Rust checks".to_owned(),
        )];
    };
    out.extend(steps.iter().filter_map(|(check, args)| {
        let all: Vec<&str> =
            before.iter().chain(args.iter()).copied().collect();
        tool::run(repo.content(), check, program, &all)
    }));
    // Library rules and unused libraries: these tools only read files and
    // run no project code (cargo-deny reads cargo's metadata, which builds
    // nothing), so they run directly. cargo-deny also locks its advisory
    // list, which the seal would keep read-only.
    // The command's last word is cargo itself, without the seal.
    let direct = cargo.last().copied().unwrap_or("cargo");
    out.extend(tool::run(repo.content(), "rust.deny", direct, &deny));
    out.extend(tool::run(
        repo.content(),
        "rust.unused-deps",
        "cargo-machete",
        &["--skip-target-dir"],
    ));
    out
}
