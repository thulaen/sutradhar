//! The `chaukidar` command:
//! - `chaukidar record LIST SUMS` writes the fingerprints, after an
//!   install;
//! - `chaukidar check LIST SUMS STATE` compares, and raises the alarm
//!   when what it finds differs from the last check, kept in `STATE`.
//!   It then runs the sync watch, which keeps its own clock in
//!   `STATE.sync` and raises its own alarm (see `sync_watch`);
//! - `chaukidar raise TEXT` raises the alarm for one line of text from
//!   another guard.
//!
//! The alarm needs `CHAUKIDAR_FORGEJO` (Forgejo's API address for the
//! repository), `CHAUKIDAR_HEADER` (a private file with Forgejo's
//! sign-in header) and `CHAUKIDAR_GITHUB` (`OWNER/NAME`). The sync watch
//! reads both heads with the same settings.

use std::io::{self, Write};
use std::process::ExitCode;

use chaukidar::{alarm, changes, parse, record, sync_watch};
use forge_sync::fetch::Config;
use forge_sync::run::System;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let refs: Vec<&str> = args.iter().map(String::as_str).collect();
    let result = match refs.as_slice() {
        ["record", list, sums] => record_sums(list, sums),
        ["check", list, sums, state] => check(list, sums, state),
        ["raise", text] => raise_named(text),
        _ => {
            say("usage: chaukidar record LIST SUMS | check LIST SUMS STATE \
                 | raise TEXT\n");
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
    let tampered = tamper_check(list, sums, state);
    // The sync watch runs after the tamper check has raised what it found,
    // so a failing sync watch cannot drop or change that alarm.
    if let Err(problem) = sync_check(state) {
        say(&format!("chaukidar: the sync watch failed: {problem}\n"));
    }
    tampered
}

fn tamper_check(list: &str, sums: &str, state: &str) -> Result<bool, String> {
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

/// The sync watch. Its clock is kept beside `STATE`, in `STATE.sync`.
/// It needs the alarm's own settings; without them there is nowhere to
/// raise anything, so it names the missing setting and stops.
fn sync_check(state: &str) -> Result<(), String> {
    let config = match config() {
        Ok(config) => config,
        Err(problem) => {
            say(&format!("chaukidar: the sync watch is off: {problem}\n"));
            return Ok(());
        }
    };
    let path = format!("{state}.sync");
    let clock = match std::fs::read_to_string(&path) {
        Ok(text) => sync_watch::Clock::parse(&text),
        Err(e) if e.kind() == io::ErrorKind::NotFound => None,
        Err(e) => return Err(format!("cannot read {path}: {e}")),
    };
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| format!("the clock is before 1970: {e}"))?
        .as_secs();
    let run = System::default();
    let (forgejo, github) = sync_watch::heads(&run, &config);
    for (side, head) in [("Forgejo", &forgejo), ("GitHub", &github)] {
        if let Err(problem) = head {
            say(&format!(
                "chaukidar: cannot read {side}'s main: {problem}\n"
            ));
        }
    }
    let (clock, body) = sync_watch::judge(&forgejo, &github, clock, now);
    // Keep the clock, marked raised, before raising, so an alarm that
    // fails in one place is not tried again every five minutes; the
    // tamper alarm does the same with its state file.
    write(&path, &clock.map(|c| c.render()).unwrap_or_default())?;
    if let Some(body) = body {
        say("chaukidar: GitHub's main has stopped following Forgejo's\n");
        let title = forge_sync::plan::SYNC_ALARM_TITLE;
        alarm::raise_titled(&run, &config, title, &body)?;
    }
    Ok(())
}

/// Raises the alarm for one line of text from another guard, such as the
/// nightly noisy-neighbour test. The text must be one public line.
fn raise_named(text: &str) -> Result<bool, String> {
    if text.is_empty() || text.contains('\n') {
        return Err("the alarm text must be one line".to_owned());
    }
    let body = format!(
        "{text}.\n\nRead the guard's log on Dell before any further push."
    );
    alarm::raise_text(&System::default(), &config()?, &body)?;
    Ok(true)
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
