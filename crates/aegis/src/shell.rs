//! The shell checks: `ShellCheck` and shfmt on every shell script
//! (AGENTS.md section 4). A missing tool fails the check.

use crate::repo::Repo;
use crate::{Finding, tool};

/// Runs `ShellCheck` and shfmt on the given shell scripts.
#[must_use]
pub fn check(repo: &Repo, scripts: &[String]) -> Vec<Finding> {
    if scripts.is_empty() {
        return Vec::new();
    }
    let names: Vec<&str> = scripts.iter().map(String::as_str).collect();
    let lint: Vec<&str> = ["--"].into_iter().chain(names.clone()).collect();
    let format: Vec<&str> =
        ["-d", "-i", "2", "--"].into_iter().chain(names).collect();
    [
        tool::run(repo.content(), "shell.lint", "shellcheck", &lint),
        tool::run(repo.content(), "shell.format", "shfmt", &format),
    ]
    .into_iter()
    .flatten()
    .collect()
}

#[cfg(test)]
mod tests {
    use super::check;
    use crate::repo::Repo;
    use std::fs;

    fn folder(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir()
            .join(format!("aegis-shell-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn passes_a_clean_script() {
        let dir = folder("clean");
        fs::write(dir.join("a.sh"), "#!/bin/sh\necho \"hi\"\n").unwrap();
        let found = check(&Repo::at(&dir), &["a.sh".to_owned()]);
        assert_eq!(found.len(), 0, "{found:?}");
    }

    #[test]
    fn refuses_a_planted_fault() {
        let dir = folder("bad");
        fs::write(dir.join("a.sh"), "#!/bin/sh\necho $1\nif true;then\nfi\n")
            .unwrap();
        let found = check(&Repo::at(&dir), &["a.sh".to_owned()]);
        assert_eq!(found.len(), 2, "{found:?}");
        assert!(found.iter().all(|f| !f.message.contains("cannot run")));
    }

    #[test]
    fn nothing_to_check_passes() {
        assert_eq!(check(&Repo::at("."), &[]).len(), 0);
    }
}
