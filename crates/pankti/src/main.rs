//! The `pankti` command:
//! `pankti run [--both] -- COMMAND...` waits in line for a place, then
//! runs the command capped in the heavy-job slice and passes on its exit
//! code. `--both` takes both places, for a benchmark.
//!
//! The places live in `PANKTI_DIR`, or else in the folder above the one
//! that holds the program, as the install sets it up: `QUEUE_DIR/bin`.

use std::io::{self, Write};
use std::os::unix::fs::MetadataExt;
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, ExitCode};
use std::time::Duration;

use pankti::{KEPT, Size, launch, take};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let refs: Vec<&str> = args.iter().map(String::as_str).collect();
    let (size, cmd) = match refs.as_slice() {
        ["run", "--both", "--", cmd @ ..] if !cmd.is_empty() => {
            (Size::Both, cmd)
        }
        ["run", "--", cmd @ ..] if !cmd.is_empty() => (Size::One, cmd),
        _ => {
            say("usage: pankti run [--both] -- COMMAND...\n");
            return ExitCode::from(2);
        }
    };
    let cmd: Vec<String> = cmd.iter().map(|s| (*s).to_owned()).collect();
    match run(size, &cmd) {
        Ok(code) => ExitCode::from(code),
        Err(problem) => {
            say(&format!("pankti: {problem}\n"));
            ExitCode::from(2)
        }
    }
}

fn run(size: Size, cmd: &[String]) -> Result<u8, String> {
    let dir = match std::env::var("PANKTI_DIR") {
        Ok(dir) => dir,
        Err(_) => home()?,
    };
    let me = std::fs::metadata("/proc/self")
        .map_err(|e| format!("cannot find who is running: {e}"))?;
    let kept: Vec<(String, String)> = KEPT
        .iter()
        .filter_map(|name| {
            Some(((*name).to_owned(), std::env::var(name).ok()?))
        })
        .collect();
    let held = take(&dir, size, Duration::from_secs(1))?;
    let line = launch(size, me.uid(), me.gid(), &kept, cmd);
    let (program, rest) = line.split_first().ok_or("nothing to run")?;
    let status = Command::new(program)
        .args(rest)
        .status()
        .map_err(|e| format!("cannot start {program}: {e}"))?;
    drop(held);
    // A job stopped by a signal, such as the memory cap, ends as 128 plus
    // the signal's number, as a shell would report it.
    let code = status
        .code()
        .or_else(|| status.signal().map(|s| s.saturating_add(128)))
        .unwrap_or(1);
    Ok(u8::try_from(code).unwrap_or(1))
}

fn say(text: &str) {
    let _ignored = io::stderr().write_all(text.as_bytes());
}

/// The queue's folder: the one above the program's own `bin` folder.
fn home() -> Result<String, String> {
    let exe = std::env::current_exe()
        .map_err(|e| format!("cannot find the program: {e}"))?;
    exe.parent()
        .and_then(std::path::Path::parent)
        .map(|p| p.display().to_string())
        .ok_or_else(|| "cannot find the queue's folder".to_owned())
}
