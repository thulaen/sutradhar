//! The `pankti` command:
//! `pankti run [--both] [--sealed] -- COMMAND...` waits in line for a
//! place, then runs the command capped in the heavy-job slice and passes
//! on its exit code. `--both` takes both places, for a benchmark.
//! `--sealed` runs code under check with no network, no way to gain
//! rights and a read-only system (see the specification).
//!
//! The places live in `PANKTI_DIR`, or else in the folder above the one
//! that holds the program, as the install sets it up: `QUEUE_DIR/bin`.

use std::io::{self, Write};
use std::os::unix::fs::MetadataExt;
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, ExitCode};
use std::time::Duration;

use pankti::{
    KEPT, Seal, Size, check_space, check_writable, free_bytes, launch,
    sealed_launch, take, writable,
};

const USAGE: &str = "usage: pankti run [--both] [--sealed] -- COMMAND...\n";

/// What was asked for: how many places, whether sealed, and the command.
struct Asked {
    size: Size,
    sealed: bool,
    cmd: Vec<String>,
}

fn parse(args: &[String]) -> Option<Asked> {
    let (first, rest) = args.split_first()?;
    if first != "run" {
        return None;
    }
    let split = rest.iter().position(|a| a == "--")?;
    let (flags, cmd) = rest.split_at(split);
    let cmd = cmd.get(1..).unwrap_or_default();
    let known = flags.iter().all(|f| f == "--both" || f == "--sealed");
    (known && !cmd.is_empty()).then(|| Asked {
        size: if flags.iter().any(|f| f == "--both") {
            Size::Both
        } else {
            Size::One
        },
        sealed: flags.iter().any(|f| f == "--sealed"),
        cmd: cmd.to_vec(),
    })
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(asked) = parse(&args) else {
        say(USAGE);
        return ExitCode::from(2);
    };
    match run(&asked) {
        Ok(code) => ExitCode::from(code),
        Err(problem) => {
            say(&format!("pankti: {problem}\n"));
            ExitCode::from(2)
        }
    }
}

fn line(asked: &Asked) -> Result<Vec<String>, String> {
    let me = std::fs::metadata("/proc/self")
        .map_err(|e| format!("cannot find who is running: {e}"))?;
    let kept: Vec<(String, String)> = KEPT
        .iter()
        .filter_map(|name| {
            Some(((*name).to_owned(), std::env::var(name).ok()?))
        })
        .collect();
    if !asked.sealed {
        return Ok(launch(asked.size, me.uid(), me.gid(), &kept, &asked.cmd));
    }
    let cwd = std::env::current_dir()
        .map_err(|e| format!("cannot find the working folder: {e}"))?
        .display()
        .to_string();
    let seal = Seal {
        writable: writable(&kept, &asked.cmd),
        cwd,
    };
    check_writable(&seal.writable)?;
    check_writable(std::slice::from_ref(&seal.cwd))?;
    let ids = (me.uid(), me.gid());
    Ok(sealed_launch(asked.size, ids, &seal, &kept, &asked.cmd))
}

fn run(asked: &Asked) -> Result<u8, String> {
    let dir = match std::env::var("PANKTI_DIR") {
        Ok(dir) => dir,
        Err(_) => home()?,
    };
    let line = line(asked)?;
    check_space(free_here()?)?;
    let held = take(&dir, asked.size, Duration::from_secs(1))?;
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

/// The free bytes on the disk of the working folder, where builds go.
fn free_here() -> Result<u64, String> {
    let out = Command::new("df")
        .args(["-B1", "--output=avail", "."])
        .output()
        .map_err(|e| format!("cannot run df: {e}"))?;
    free_bytes(&String::from_utf8_lossy(&out.stdout))
        .ok_or_else(|| "cannot read the free disk space".to_owned())
}
