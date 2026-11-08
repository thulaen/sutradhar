//! pankti, the heavy-job queue (AGENTS.md section 11). Heavy jobs, such
//! as builds, full test runs and runner jobs, wait in one line. At most
//! two run at once; a benchmark takes both places and runs alone.
//!
//! The places are lock files in one folder: `line`, `place-1` and
//! `place-2`. A job holds the `line` lock while it waits, so later jobs
//! queue behind it, then holds its places until it ends. The system
//! frees a lock when its holder ends, even after a crash, so no place is
//! ever lost.

use std::fs::{File, OpenOptions, TryLockError};
use std::thread::sleep;
use std::time::Duration;

/// How many places a job needs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size {
    /// One place: 4 GB and 8 CPU threads.
    One,
    /// Both places, for a benchmark: 8 GB and 16 CPU threads.
    Both,
}

/// The places a job holds. Dropping it frees them.
#[derive(Debug)]
pub struct Held {
    _places: Vec<File>,
}

/// The lock files in the queue's folder. The install makes them.
pub const FILES: [&str; 3] = ["line", "place-1", "place-2"];

/// The settings a job keeps. sudo clears the rest, which keeps the job's
/// set-up the same wherever it starts.
// `AEGIS_QUEUE` is left out on purpose: a job inside a job must not wait
// for a place, or two of them could wait for each other for ever.
pub const KEPT: [&str; 7] = [
    "PATH",
    "HOME",
    "LANG",
    "RUSTUP_HOME",
    "RUSTUP_TOOLCHAIN",
    "CARGO_HOME",
    "CARGO_TARGET_DIR",
];

fn open(dir: &str, name: &str) -> Result<File, String> {
    let path = format!("{dir}/{name}");
    // Read-only is enough for a lock, so every member of the queue's
    // group can use files that another member made.
    OpenOptions::new()
        .read(true)
        .open(&path)
        .map_err(|e| format!("cannot open {path} (install pankti first): {e}"))
}

fn lock(file: &File, name: &str) -> Result<(), String> {
    file.lock().map_err(|e| format!("cannot lock {name}: {e}"))
}

/// Tries a place without waiting.
fn try_place(file: &File) -> Result<bool, String> {
    match file.try_lock() {
        Ok(()) => Ok(true),
        Err(TryLockError::WouldBlock) => Ok(false),
        Err(TryLockError::Error(e)) => Err(format!("cannot lock a place: {e}")),
    }
}

/// Waits in line for this job's places, checking every `gap` for a
/// single place.
///
/// # Errors
/// When the folder or a lock file cannot be used.
pub fn take(dir: &str, size: Size, gap: Duration) -> Result<Held, String> {
    let line = open(dir, "line")?;
    lock(&line, "line")?;
    let first = open(dir, "place-1")?;
    let second = open(dir, "place-2")?;
    let held = match size {
        Size::Both => {
            lock(&first, "place-1")?;
            lock(&second, "place-2")?;
            vec![first, second]
        }
        Size::One => loop {
            if try_place(&first)? {
                break vec![first];
            }
            if try_place(&second)? {
                break vec![second];
            }
            sleep(gap);
        },
    };
    drop(line);
    Ok(Held { _places: held })
}

/// The command that runs `cmd` as user `uid` and group `gid`, capped
/// inside the heavy-job slice, with the `kept` settings.
#[must_use]
pub fn launch(
    size: Size,
    uid: u32,
    gid: u32,
    kept: &[(String, String)],
    cmd: &[String],
) -> Vec<String> {
    let (memory, cpu) = match size {
        Size::One => ("4G", "800%"),
        Size::Both => ("8G", "1600%"),
    };
    let mut out: Vec<String> = [
        "sudo",
        "-n",
        "systemd-run",
        "--quiet",
        "--scope",
        "--slice=sutradhar-heavy.slice",
    ]
    .map(str::to_owned)
    .to_vec();
    for property in [
        format!("MemoryMax={memory}"),
        "MemorySwapMax=0".to_owned(),
        format!("CPUQuota={cpu}"),
    ] {
        out.push("-p".to_owned());
        out.push(property);
    }
    // setpriv gives the job back its user, main group and every other
    // group the user is in, which systemd-run --uid would drop.
    out.push("--".to_owned());
    out.push("setpriv".to_owned());
    out.push(format!("--reuid={uid}"));
    out.push(format!("--regid={gid}"));
    out.push("--init-groups".to_owned());
    out.push("--".to_owned());
    out.push("env".to_owned());
    for (name, value) in kept {
        out.push(format!("{name}={value}"));
    }
    out.extend(cmd.iter().cloned());
    out
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
