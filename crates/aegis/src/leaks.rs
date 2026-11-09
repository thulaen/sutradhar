//! The leak scan (AGENTS.md section 9.1, gate item GI-46). It runs the
//! same pinned gitleaks that forge-sync runs before anything leaves
//! Dell, so a commit the gate lets in can never stop the copy to GitHub.
//! The commit tier scans the staged files; the push tier also scans every
//! new commit. A `gitleaks:allow` note in a file does not count: the only
//! way to pass a false alarm is the reviewed `.gitleaksignore` list at
//! the top of the repository.

use std::path::Path;

use crate::Finding;
use crate::repo::NO_COMMIT;

const CHECK: &str = "secrets.gitleaks";

const COMMON: [&str; 3] =
    ["--no-banner", "--redact", "--ignore-gitleaks-allow"];

/// The scan of the files, as gitleaks arguments. It runs inside the
/// folder, so the names in `.gitleaksignore` are the repository's own.
#[must_use]
pub fn files_args() -> Vec<String> {
    let mut args = vec!["dir".to_owned()];
    args.extend(COMMON.map(str::to_owned));
    args.push(".".to_owned());
    args
}

/// The scan of the commits from `base` to `head`, as gitleaks arguments.
#[must_use]
pub fn history_args(base: &str, head: &str) -> Vec<String> {
    let range = if base == NO_COMMIT {
        head.to_owned()
    } else {
        format!("{base}..{head}")
    };
    let mut args = vec!["git".to_owned()];
    args.extend(COMMON.map(str::to_owned));
    args.push(format!("--log-opts={range}"));
    args.push(".".to_owned());
    args
}

fn run(folder: &Path, args: &[String]) -> Option<Finding> {
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    crate::tool::run(folder, CHECK, "gitleaks", &args)
}

/// Scans the files in `folder`.
#[must_use]
pub fn files(folder: &Path) -> Option<Finding> {
    run(folder, &files_args())
}

/// Scans the commits from `base` to `head` in `root`.
#[must_use]
pub fn history(root: &Path, base: &str, head: &str) -> Option<Finding> {
    run(root, &history_args(base, head))
}

#[cfg(test)]
mod tests {
    use super::{files_args, history_args};
    use crate::repo::NO_COMMIT;

    #[test]
    fn allow_notes_never_count_and_the_ignore_list_is_the_repos() {
        let f = files_args();
        let h = history_args("a", "b");
        for args in [&f, &h] {
            assert!(args.iter().any(|a| a == "--ignore-gitleaks-allow"));
            assert!(args.iter().any(|a| a == "--redact"));
            assert_eq!(args.last().map(String::as_str), Some("."));
        }
        assert!(h.iter().any(|a| a == "--log-opts=a..b"));
        let new = history_args(NO_COMMIT, "b");
        assert!(new.iter().any(|a| a == "--log-opts=b"));
    }
}
