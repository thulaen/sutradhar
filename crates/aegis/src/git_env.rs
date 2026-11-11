//! The settings every git command the gate runs is started without, so
//! git shows the gate the repository as it really is: no replace objects
//! or grafts, no other object store, and no repository, index or
//! settings passed in through the environment (a hook's own index is set
//! again by `Repo`). The system-wide git settings are skipped and git's
//! file watcher is off. The person's own `~/.gitconfig`, at its usual
//! place, is still read: it is theirs to keep safe.

use std::io::Read;
use std::process::{Command, Stdio};

/// The settings a git command never takes from outside.
const CLEARED: [&str; 12] = [
    "GIT_DIR",
    "GIT_INDEX_FILE",
    "GIT_WORK_TREE",
    "GIT_COMMON_DIR",
    "GIT_NAMESPACE",
    "GIT_OBJECT_DIRECTORY",
    "GIT_ALTERNATE_OBJECT_DIRECTORIES",
    "GIT_REPLACE_REF_BASE",
    "GIT_CONFIG_PARAMETERS",
    "GIT_CONFIG_COUNT",
    "GIT_CONFIG_SYSTEM",
    "GIT_CONFIG_GLOBAL",
];

/// Clears those settings on `command`, shuts off replace objects, grafts
/// and system settings, and turns git's file watcher off.
pub fn shut_off(command: &mut Command) {
    for name in CLEARED {
        let _cleared = command.env_remove(name);
    }
    let _set = command
        .env("GIT_NO_REPLACE_OBJECTS", "1")
        .env("GIT_GRAFT_FILE", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .args(["-c", "core.fsmonitor=false"]);
}

/// Runs `command` and returns what it printed, but stops it and fails
/// once it has printed more than `cap` bytes, so a small push that
/// lists a huge tree cannot use up the memory. git is always stopped and
/// waited for, whatever happens.
///
/// # Errors
/// When git cannot start, ends with an error, or prints too much.
pub fn read_capped(
    command: &mut Command,
    cap: usize,
) -> Result<Vec<u8>, String> {
    let mut child = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("cannot run git: {e}"))?;
    let mut out = Vec::new();
    let limit = u64::try_from(cap.saturating_add(1)).unwrap_or(u64::MAX);
    let read = child
        .stdout
        .take()
        .map(|stdout| stdout.take(limit).read_to_end(&mut out));
    if !matches!(read, Some(Ok(_))) || out.len() > cap {
        let _killed = child.kill();
        let _ended = child.wait();
        return Err("the git listing was stopped: it was too long or could \
                    not be read"
            .to_owned());
    }
    match child.wait() {
        Ok(status) if status.success() => Ok(out),
        Ok(status) => Err(format!("a git listing failed: {status}")),
        Err(e) => Err(format!("a git listing failed: {e}")),
    }
}

/// Runs `command` with `input` as what it reads, and gives what it
/// printed. `what` is a trusted label for safe errors. Output is bounded;
/// raw child diagnostics and command arguments never enter an error.
/// `_args` preserves the existing callers but is never shown or inspected.
///
/// # Errors
/// When the program cannot start, or ends with an error, or does not
/// take all of `input`, output exceeds its limit, or is not UTF-8.
pub fn run_with_input(
    command: &mut Command,
    input: &[u8],
    what: &str,
    _args: &str,
) -> Result<String, String> {
    let output = crate::process_capture::run(command, input, what)?;
    if !output.status.success() {
        return Err(format!("{what} failed ({})", output.status));
    }
    String::from_utf8(output.stdout)
        .map_err(|_| format!("{what} output is not UTF-8"))
}

#[cfg(test)]
mod tests {
    use super::read_capped;
    use std::process::Command;

    #[test]
    fn a_command_that_prints_too_much_is_stopped_and_refused() {
        let mut endless = Command::new("yes");
        let refused = read_capped(&mut endless, 1000);
        assert!(refused.is_err());
        let mut short = Command::new("printf");
        let _args = short.arg("ab");
        assert_eq!(read_capped(&mut short, 1000), Ok(b"ab".to_vec()));
        let mut failing = Command::new("false");
        assert!(read_capped(&mut failing, 1000).is_err());
    }
}
