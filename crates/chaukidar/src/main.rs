//! The `chaukidar` command:
//! - `chaukidar record LIST SUMS` writes the fingerprints, after an
//!   install;
//! - `chaukidar check LIST SUMS STATE` compares, and raises the alarm
//!   when what it finds differs from the last check, kept in `STATE`.
//!
//! The alarm needs `CHAUKIDAR_FORGEJO` (Forgejo's API address for the
//! repository), `CHAUKIDAR_HEADER` (a private file with Forgejo's
//! sign-in header) and `CHAUKIDAR_GITHUB` (`OWNER/NAME`).

use std::io::{self, Write};
use std::process::ExitCode;

use chaukidar::{alarm, changes, parse, record};
use forge_sync::fetch::Config;
use forge_sync::run::System;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let refs: Vec<&str> = args.iter().map(String::as_str).collect();
    let result = match refs.as_slice() {
        ["record", list, sums] => record_sums(list, sums),
        ["check", list, sums, state] => check(list, sums, state),
        _ => {
            say("usage: chaukidar record LIST SUMS | check LIST SUMS STATE\n");
            return ExitCode::from(2);
        }
    };
    match result {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(problem) => {
            say(&format!("chaukidar: {problem}\n"));
            ExitCode::from(2)
        }
    }
}

fn read(path: &str) -> Result<String, String> {
    std::fs::read_to_string(path)
        .map_err(|e| format!("cannot read {path}: {e}"))
}

fn write(path: &str, text: &str) -> Result<(), String> {
    std::fs::write(path, text).map_err(|e| format!("cannot write {path}: {e}"))
}

fn record_sums(list: &str, sums: &str) -> Result<bool, String> {
    let watched = parse(&read(list)?)?;
    write(sums, &record(&System::default(), &watched)?)?;
    say(&format!(
        "chaukidar: recorded {} fingerprints\n",
        watched.len()
    ));
    Ok(true)
}

fn check(list: &str, sums: &str, state: &str) -> Result<bool, String> {
    let run = System::default();
    let watched = parse(&read(list)?)?;
    let found = changes(&run, &watched, &read(sums)?);
    let text = found.join("\n");
    let previous = std::fs::read_to_string(state).unwrap_or_default();
    if found.is_empty() {
        write(state, "")?;
        return Ok(true);
    }
    for line in &found {
        say(&format!("chaukidar: {line}\n"));
    }
    if text != previous {
        // Remember the alarm as raised once it has been tried, so a place
        // that keeps failing cannot make the other one repeat it.
        let raised = alarm::raise(&run, &config()?, &found);
        write(state, &text)?;
        raised?;
    }
    Ok(false)
}

fn setting(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|_| format!("{name} is not set"))
}

fn config() -> Result<Config, String> {
    Ok(Config {
        forgejo: setting("CHAUKIDAR_FORGEJO")?,
        header_file: setting("CHAUKIDAR_HEADER")?,
        github: setting("CHAUKIDAR_GITHUB")?,
        mirror: String::new(),
        record: String::new(),
        github_user: String::new(),
        gitleaks: String::new(),
        private: Vec::new(),
    })
}

fn say(text: &str) {
    let _ignored = io::stderr().write_all(text.as_bytes());
}
