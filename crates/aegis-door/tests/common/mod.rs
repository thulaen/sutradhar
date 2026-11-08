//! Helpers shared by the door tests.
#![allow(dead_code, reason = "each test file uses some of the helpers")]
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use aegis::repo::NO_COMMIT;
use aegis_door::gate::{Door, Push};

pub const GOOD: &str = "docs: start\n\nTDD: none (docs only)\nDefects: none\n\
    Task: bootstrap\nSigned-off-by: A <a@example.com>\n";
pub const POINTER: &str =
    "Read [AGENTS.md](AGENTS.md). It is the only rule file.\n";

pub fn git(dir: &Path, args: &[&str]) -> String {
    aegis::repo::Repo::at(dir)
        .git(args)
        .unwrap()
        .trim()
        .to_owned()
}

pub fn folder(name: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join(format!("aegis-door-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let _ = git(&dir, &["init", "-q", "-b", "main"]);
    let _ = git(&dir, &["config", "user.email", "a@example.com"]);
    let _ = git(&dir, &["config", "user.name", "A"]);
    dir
}

/// A source repository with one good commit, and a door whose copy has
/// that commit.
pub fn setup(name: &str) -> (PathBuf, Door, String) {
    let source = folder(&format!("{name}-src"));
    for (file, text) in [
        ("AGENTS.md", "# Rules\n"),
        ("CLAUDE.md", POINTER),
        ("GEMINI.md", POINTER),
        ("README.md", "# Read me\n"),
    ] {
        fs::write(source.join(file), text).unwrap();
    }
    let _ = git(&source, &["add", "-A"]);
    let _ = git(&source, &["commit", "-q", "-m", GOOD]);
    let first = git(&source, &["rev-parse", "HEAD"]);
    let work = folder(&format!("{name}-work"));
    let url = source.display().to_string();
    let _ = git(&work, &["fetch", "-q", &url, "main"]);
    (source, Door::new(&work), first)
}

/// The objects in `new` that `old` lacks, as a pack.
pub fn pack(source: &Path, old: &str, new: &str) -> Vec<u8> {
    let mut child = Command::new("git")
        .arg("-C")
        .arg(source)
        .args(["pack-objects", "--revs", "--stdout", "-q"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let revs = if old == NO_COMMIT {
        format!("{new}\n")
    } else {
        format!("{new}\n--not\n{old}\n")
    };
    child
        .stdin
        .take()
        .unwrap()
        .write_all(revs.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap().stdout
}

pub fn commit(source: &Path, file: &str, text: &str, message: &str) -> String {
    fs::write(source.join(file), text).unwrap();
    let _ = git(source, &["add", "-A"]);
    let _ = git(source, &["commit", "-q", "-m", message]);
    git(source, &["rev-parse", "HEAD"])
}

pub const MAIN: &str = "refs/heads/main";

pub fn main_push<'a>(old: &'a str, new: &'a str) -> Push<'a> {
    Push {
        name: MAIN,
        old,
        new,
        main: old,
    }
}

pub fn door_work(name: &str) -> PathBuf {
    std::env::temp_dir()
        .join(format!("aegis-door-{name}-work-{}", std::process::id()))
}
