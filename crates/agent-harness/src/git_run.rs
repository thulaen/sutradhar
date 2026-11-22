//! Running git on the harness's own repository with everything turned
//! down that could run a program (spec 0155, "Push", step 1). Arguments
//! are always a separate list, never a shell.

use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

/// The most seconds one git call may run (proposed).
const GIT_SECONDS: &str = "60";
/// The most bytes of output one git call may give (proposed).
const OUTPUT_MAX: usize = 4 * 1024 * 1024;
/// The settings given to every call, which beat the repository's own.
const SAFE_SETTINGS: [&str; 14] = [
    "-c",
    "core.fsmonitor=false",
    "-c",
    "core.hooksPath=/dev/null",
    "-c",
    "core.pager=cat",
    "-c",
    "diff.external=",
    "-c",
    "core.attributesFile=/dev/null",
    "-c",
    "protocol.allow=never",
    "-c",
    "core.alternateRefsCommand=",
];

/// The only environment a call gets: no system or user configuration,
/// no prompts, no optional locks.
const SAFE_ENVIRONMENT: [(&str, &str); 10] = [
    ("PATH", "/usr/bin:/bin"),
    ("HOME", "/nonexistent"),
    ("GIT_CONFIG_NOSYSTEM", "1"),
    ("GIT_CONFIG_GLOBAL", "/dev/null"),
    ("GIT_TERMINAL_PROMPT", "0"),
    ("GIT_OPTIONAL_LOCKS", "0"),
    ("GIT_NO_REPLACE_OBJECTS", "1"),
    ("GIT_NO_LAZY_FETCH", "1"),
    ("GIT_GRAFT_FILE", "/dev/null/grafts"),
    ("GIT_ALLOW_PROTOCOL", "none"),
];

/// What one git call gave.
pub struct Ran {
    /// The exit code, or none if it was stopped.
    pub code: Option<i32>,
    /// What it wrote.
    pub stdout: Vec<u8>,
}

/// Runs `git -C repo ARGS` with the safe settings, no environment but
/// the bare minimum, no system or user configuration, a time limit and
/// a limit on the output.
///
/// # Errors
/// A text saying git could not be run, or gave too much output.
pub fn git(
    repo: &Path,
    args: &[&str],
    stdin: Option<&str>,
) -> Result<Ran, String> {
    git_with(repo, args, stdin, &[])
}

/// As [`git`], with these settings added to the environment. A later
/// setting beats an earlier one, so a caller can allow one protocol.
///
/// # Errors
/// A text saying git could not be run, or gave too much output.
pub fn git_with(
    repo: &Path,
    args: &[&str],
    stdin: Option<&str>,
    extra: &[(&str, &str)],
) -> Result<Ran, String> {
    let mut command = Command::new("timeout");
    let _timeout = command.args(["--kill-after=5", GIT_SECONDS, "git"]);
    let _settings = command.args(SAFE_SETTINGS).arg("-C").arg(repo);
    let _args = command.arg("--no-pager").args(args).env_clear();
    for (key, value) in SAFE_ENVIRONMENT.iter().chain(extra) {
        let _set = command.env(key, value);
    }
    let _stdin = command.stdin(if stdin.is_some() {
        Stdio::piped()
    } else {
        Stdio::null()
    });
    let _out = command.stdout(Stdio::piped()).stderr(Stdio::null());
    let mut child = command
        .spawn()
        .map_err(|_| "git could not be started".to_owned())?;
    // The input is written from its own thread while the output is read,
    // so a long list cannot fill both pipes and stall them.
    let writer = stdin.zip(child.stdin.take()).map(|(text, mut pipe)| {
        let bytes = text.as_bytes().to_vec();
        std::thread::spawn(move || pipe.write_all(&bytes))
    });
    let mut stdout = Vec::new();
    if let Some(pipe) = child.stdout.take() {
        let limit = u64::try_from(OUTPUT_MAX).unwrap_or(u64::MAX);
        let _read = pipe
            .take(limit.saturating_add(1))
            .read_to_end(&mut stdout)
            .map_err(|_| "git's output could not be read".to_owned())?;
    }
    if stdout.len() > OUTPUT_MAX {
        let _killed = child.kill();
        let _status = child.wait();
        return Err("git gave too much output".to_owned());
    }
    let status = child.wait().map_err(|_| "git did not finish".to_owned())?;
    if let Some(thread) = writer {
        let _joined = thread.join();
    }
    Ok(Ran {
        code: status.code(),
        stdout,
    })
}
