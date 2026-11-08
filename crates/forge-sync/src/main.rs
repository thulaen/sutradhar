//! The `forge-sync` command: `forge-sync once`. Its settings come from
//! the environment, as docs/specs/0002-forge-sync.md lists:
//! `FORGE_SYNC_FORGEJO`, `FORGE_SYNC_HEADER`, `FORGE_SYNC_GITHUB`,
//! `FORGE_SYNC_GITHUB_USER`, `FORGE_SYNC_MIRROR`, `FORGE_SYNC_RECORD`,
//! `FORGE_SYNC_GITLEAKS` and `FORGE_SYNC_PRIVATE`, a file of words that
//! must never leave Dell, one a line.

use std::io::{self, Write};
use std::process::ExitCode;

use forge_sync::fetch::Config;
use forge_sync::run::System;
use forge_sync::sync;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args != ["once"] {
        say("usage: forge-sync once\n");
        return ExitCode::from(2);
    }
    let config = match config() {
        Ok(c) => c,
        Err(problem) => {
            say(&format!("forge-sync: {problem}\n"));
            return ExitCode::from(2);
        }
    };
    let (done, problems) = sync::once(&System::default(), &config);
    for line in &done {
        say(&format!("forge-sync: {line}\n"));
    }
    for line in &problems {
        say(&format!("forge-sync: problem: {line}\n"));
    }
    if problems.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn setting(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|_| format!("{name} is not set"))
}

fn config() -> Result<Config, String> {
    let private = match std::env::var("FORGE_SYNC_PRIVATE") {
        Ok(path) => std::fs::read_to_string(&path)
            .map_err(|e| format!("cannot read {path}: {e}"))?
            .lines()
            .map(str::to_owned)
            .collect(),
        Err(_) => Vec::new(),
    };
    Ok(Config {
        forgejo: setting("FORGE_SYNC_FORGEJO")?,
        header_file: setting("FORGE_SYNC_HEADER")?,
        github: setting("FORGE_SYNC_GITHUB")?,
        mirror: setting("FORGE_SYNC_MIRROR")?,
        record: setting("FORGE_SYNC_RECORD")?,
        github_user: setting("FORGE_SYNC_GITHUB_USER")?,
        gitleaks: setting("FORGE_SYNC_GITLEAKS")?,
        private,
    })
}

fn say(text: &str) {
    let _ignored = io::stderr().write_all(text.as_bytes());
}
