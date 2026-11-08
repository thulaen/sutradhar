//! The real Forgejo hook, run by git on a scratch bare repository, in
//! front of the real door program.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

use common::{GOOD, commit, door_work, folder, git, setup};

const HOOK: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docker/forgejo/door-pre-receive.sh"
);

/// A bare repository holding the source's `main`, with the hook in
/// place, and a running door for `count` requests (one is used up by
/// the check that the door is listening).
fn forge(name: &str, source: &Path, count: usize) -> (PathBuf, Child) {
    let bare = std::env::temp_dir()
        .join(format!("aegis-door-{name}-bare-{}", std::process::id()));
    let _ = fs::remove_dir_all(&bare);
    let url = source.display().to_string();
    let _ = Command::new("git")
        .args(["clone", "-q", "--bare", &url])
        .arg(&bare)
        .status()
        .unwrap();
    let hook = bare.join("hooks/pre-receive");
    let _ = fs::copy(HOOK, &hook).unwrap();
    let socket = folder(&format!("{name}-sock")).join("door.sock");
    let door = Command::new(env!("CARGO_BIN_EXE_aegis-door"))
        .arg(&socket)
        .arg(door_work(name))
        .arg(count.to_string())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    while std::os::unix::net::UnixStream::connect(&socket).is_err() {
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    let _ = git(
        &bare,
        &["config", "aegis.socket", &socket.to_string_lossy()],
    );
    (bare, door)
}

/// Pushes `what` from `source` to the bare repository; true if accepted.
fn push(source: &Path, bare: &Path, what: &str) -> bool {
    let socket = git(bare, &["config", "aegis.socket"]);
    Command::new("git")
        .env("DOOR_SOCKET", socket)
        .arg("-C")
        .arg(source)
        .args(["push", "-q", "--no-verify"])
        .arg(bare)
        .arg(what)
        .stderr(Stdio::null())
        .status()
        .unwrap()
        .success()
}

#[test]
fn the_hook_lets_good_main_in_and_keeps_bad_pushes_out() {
    let (source, _door, _first) = setup("hook");
    let (bare, mut door) = forge("hook", &source, 10);
    let _ = commit(&source, "b.md", "# B\n", GOOD);
    assert!(push(&source, &bare, "main"));
    let _ = commit(&source, "c.md", "# C\n", "stuff");
    assert!(!push(&source, &bare, "main"));
    let _ = git(&source, &["reset", "-q", "--hard", "HEAD~1"]);
    assert!(!push(&source, &bare, "HEAD:refs/heads/{main}"));
    assert!(!push(&source, &bare, "HEAD:refs/heads/side"));
    assert!(!push(&source, &bare, ":refs/heads/main"));
    let _ = door.kill();
}
