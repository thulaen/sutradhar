//! The commit and push tiers (AGENTS.md section 7).

use crate::docs::{self, Lookup};
use crate::repo::Repo;
use crate::{
    Finding, commit_msg, file_kinds, limits, markers, rule_file, secrets,
    shell, tool, workflows,
};

/// Whether the TypeScript compiler is pinned, which switches TypeScript
/// on.
#[must_use]
pub fn typescript_on(repo: &Repo) -> bool {
    repo.read("gate/tools.toml")
        .is_some_and(|t| t.lines().any(|l| l.trim() == "name = \"tsgo\""))
}

/// The checks for a set of files: kind, clean text, size, markers,
/// secrets, workflows and the shell checks. Files that are missing are
/// only checked for their kind.
#[must_use]
pub fn file_checks(repo: &Repo, paths: &[String]) -> Vec<Finding> {
    let ts = typescript_on(repo);
    let mut out = file_kinds::check(paths.iter().map(String::as_str), ts);
    let private = match crate::private::Private::from_env() {
        Ok(private) => private,
        Err(e) => {
            out.push(crate::private::unreadable(e));
            None
        }
    };
    let mut scripts = Vec::new();
    for path in paths {
        let (Some(kind), Some(bytes)) =
            (file_kinds::kind_of(path, ts), repo.read_bytes(path))
        else {
            continue;
        };
        out.extend(crate::text::check(path, kind, &bytes));
        if let Some(lists) = &private {
            out.extend(crate::private::check(path, &bytes, lists));
        }
        let text = String::from_utf8_lossy(&bytes);
        out.extend(limits::check(path, kind, &text));
        out.extend(markers::check(path, kind, &text));
        out.extend(crate::proofs::check(path, kind, &text));
        out.extend(secrets::check(path, &text));
        if workflows::is_workflow(path) {
            out.extend(workflows::check(path, &text));
        }
        if kind == file_kinds::Kind::Shell {
            scripts.push(path.clone());
        }
    }
    out.extend(shell::check(repo, &scripts));
    out
}

/// The checks that always cover the whole project because any change
/// can break them: every Markdown file and the pointer files.
#[must_use]
pub fn whole_project_docs(repo: &Repo, all: &[String]) -> Vec<Finding> {
    let lookup = |p: &str| match repo.read(p) {
        Some(text) => Lookup::File(text),
        None if repo.is_folder(p) => Lookup::Folder,
        None => Lookup::Missing,
    };
    let mut out = Vec::new();
    for path in all.iter().filter(|p| is_markdown(p)) {
        if let Some(text) = repo.read(path) {
            out.extend(docs::check(path, &text, &lookup));
        }
    }
    for path in rule_file::POINTER_FILES {
        out.extend(rule_file::check(path, repo.read(path).as_deref()));
    }
    // Spelling, in British English, across the whole project.
    out.extend(tool::run(repo.content(), "text.typos", "typos", &[]));
    out.extend(crate::leaks::files(repo.content()));
    // The gate's own repository keeps its catalogue in step with its code.
    let gate = all.iter().any(|p| p.starts_with("crates/aegis/"));
    if gate || all.iter().any(|p| p == crate::catalogue::PATH) {
        let rust: Vec<(String, String)> = all
            .iter()
            .filter(|p| p.starts_with("crates/") && is_rust(p))
            .filter_map(|p| Some((p.clone(), repo.read(p)?)))
            .collect();
        out.extend(crate::catalogue::check(&|p| repo.read(p), &rust));
        let manifests: Vec<(String, String)> = all
            .iter()
            .filter(|p| p.starts_with("crates/") && p.ends_with("/Cargo.toml"))
            .filter_map(|p| Some((p.clone(), repo.read(p)?)))
            .collect();
        let glossary = repo.read(crate::names::GLOSSARY);
        out.extend(crate::names::check(glossary.as_deref(), all));
        let list = repo.read(crate::packages::PATH);
        let root = repo.read("Cargo.toml");
        out.extend(crate::packages::check(
            list.as_deref(),
            root.as_deref(),
            &manifests,
        ));
    }
    out
}

/// The Rust checks: format, lints and tests, for the whole workspace.
/// They run only when the repository has a Cargo workspace. When
/// `AEGIS_QUEUE` names the heavy-job queue, pankti, each one waits in it
/// (AGENTS.md section 11).
#[must_use]
pub fn rust_checks(repo: &Repo) -> Vec<Finding> {
    let queue = std::env::var("AEGIS_QUEUE").ok();
    let command = cargo_command(queue.as_deref());
    let refs: Vec<&str> = command.iter().map(String::as_str).collect();
    if file_kinds::has_cargo_settings(repo.content()) {
        return vec![Finding::new(
            "rust.cargo-settings",
            ".cargo",
            0,
            "cargo settings are in the tree, so no Rust step runs on it",
        )];
    }
    let mut out = Vec::new();
    if queue.is_some_and(|q| !q.is_empty()) && repo.read("Cargo.toml").is_some()
    {
        // A sealed check is offline, so the exact libraries in the lock file
        // are fetched first, outside the seal; fetching runs no build code.
        out.extend(fetch(repo.content()));
    }
    out.extend(rust_checks_using(repo, &refs));
    out
}

/// Fetches the libraries named in `folder`'s lock file, and nothing else.
#[must_use]
pub fn fetch(folder: &std::path::Path) -> Option<Finding> {
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
    let deny = ["deny", "--offline", "check", "licenses", "bans", "sources"];
    let steps: [(&'static str, &[&str]); 4] = [
        ("rust.format", &["fmt", "--all", "--check"]),
        ("rust.lints", &lints),
        ("rust.tests", &tests),
        ("rust.deny", &deny),
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
    // Unused libraries: the tool only reads files and runs no project
    // code, so it runs directly.
    out.extend(tool::run(
        repo.content(),
        "rust.unused-deps",
        "cargo-machete",
        &["--skip-target-dir"],
    ));
    out
}

/// The commit tier: the staged files, every doc, and the Rust checks,
/// all run on an exact copy of what is staged.
///
/// # Errors
/// When git fails or the copy cannot be made.
pub fn commit(repo: &Repo) -> Result<Vec<Finding>, String> {
    let staged = repo.staged_files()?;
    let all = repo.tracked_files()?;
    let copy = repo.staged_copy()?;
    let snapshot = copy.repo();
    let mut out = file_checks(snapshot, &staged);
    out.extend(whole_project_docs(snapshot, &all));
    out.extend(rust_checks(snapshot));
    Ok(out)
}

/// The push tier: every commit from `base` to `head` (its message and
/// every line it added), then every file, every doc and the Rust checks.
/// The repository must have `head` checked out with no uncommitted
/// changes.
///
/// # Errors
/// When git fails.
pub fn push(
    repo: &Repo,
    base: &str,
    head: &str,
) -> Result<Vec<Finding>, String> {
    if !repo.is_clean()? {
        return Ok(vec![Finding::new(
            "push.clean-tree",
            "",
            0,
            "there are changes not yet committed; commit or remove them first",
        )]);
    }
    let mut out = Vec::new();
    let commits = crate::history::commits(repo, base, head)?;
    let lists = crate::private::Private::from_env().unwrap_or_else(|e| {
        out.push(crate::private::unreadable(e));
        None
    });
    let private = |path: &str, text: &str| {
        lists.as_ref().map_or_else(Vec::new, |l| {
            crate::private::check_text(path, text, l)
        })
    };
    for commit in &commits {
        out.extend(commit_msg::check(&commit.message));
        out.extend(crate::tdd::check_paths(&commit.message, &commit.paths));
        out.extend(secrets::check("(commit message)", &commit.message));
        out.extend(private("(commit message)", &commit.message));
    }
    for (path, line) in repo.added_lines(base, head)? {
        let at = format!("{path} (history)");
        out.extend(secrets::check(&at, &line));
        out.extend(private(&at, &line));
    }
    out.extend(crate::leaks::history(repo.root(), base, head));
    let all = repo.tracked_files()?;
    out.extend(file_checks(repo, &all));
    out.extend(whole_project_docs(repo, &all));
    out.extend(rust_checks(repo));
    let queue = std::env::var("AEGIS_QUEUE").ok();
    let cargo = cargo_command(queue.as_deref());
    let cargo: Vec<&str> = cargo.iter().map(String::as_str).collect();
    if !file_kinds::has_cargo_settings(repo.content()) {
        out.extend(crate::red_green::check(repo, &commits, &cargo));
    }
    Ok(out)
}

fn is_rust(path: &str) -> bool {
    std::path::Path::new(path)
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("rs"))
}

fn is_markdown(path: &str) -> bool {
    std::path::Path::new(path)
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("md"))
}
