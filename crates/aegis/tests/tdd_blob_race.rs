//! A Git wrapper changes the index between the mode check and blob read.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::io::Write;
use std::path::Path;
use std::process::{Command, ExitCode};

mod common;
use common::{git, repo, write};

const PATH: &str = "docker/forgejo/compose.yaml";
const MESSAGE: &str = "fix: pin an image\n\nTDD: none (docs only)\n";

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if !args.is_empty() {
        return changing_git(&args);
    }
    let (done, waiting) = std::sync::mpsc::channel();
    let watchdog = std::thread::spawn(move || {
        if waiting
            .recv_timeout(std::time::Duration::from_secs(30))
            .is_err()
        {
            std::process::exit(1);
        }
    });
    checked_mode_and_content_belong_to_the_same_blob();
    done.send(()).unwrap();
    watchdog.join().unwrap();
    ExitCode::SUCCESS
}

fn changing_git(args: &[String]) -> ExitCode {
    let [flag, setting, at, directory, rest @ ..] = args else {
        return ExitCode::FAILURE;
    };
    let dir = Path::new(directory);
    if flag != "-c"
        || setting != "core.fsmonitor=false"
        || at != "-C"
        || rest.is_empty()
        || !dir.join(".git/next-blob").is_file()
    {
        return ExitCode::FAILURE;
    }
    let out = Command::new("/usr/bin/git").args(args).output().unwrap();
    if out.status.success() && args.iter().any(|arg| arg == "ls-files") {
        let object =
            std::fs::read_to_string(dir.join(".git/next-blob")).unwrap();
        let entry = format!("100755,{},{}", object.trim(), PATH);
        let _ok = git(dir, &["update-index", "--cacheinfo", &entry]);
    }
    std::io::stdout().write_all(&out.stdout).unwrap();
    std::io::stderr().write_all(&out.stderr).unwrap();
    if out.status.success() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn configuration(hex: char, command: &str) -> String {
    format!(
        "services:\n  forgejo:\n    image: \
         codeberg.org/forgejo/forgejo@sha256:{}\n    command: {command}\n",
        hex.to_string().repeat(64)
    )
}

fn checked_mode_and_content_belong_to_the_same_blob() {
    let dir = repo("tdd-blob", "changing-index");
    write(&dir, PATH, &configuration('a', "original"));
    let _ok = git(&dir, &["add", "--", PATH]);
    let _ok = git(&dir, &["commit", "-q", "-m", MESSAGE]);
    write(&dir, PATH, &configuration('b', "original"));
    let clean = git(&dir, &["hash-object", "-w", "--", PATH]);
    write(&dir, ".git/next-blob", &clean);
    write(&dir, PATH, &configuration('b', "changed"));
    let _ok = git(&dir, &["add", "--", PATH]);
    let executable = std::env::current_exe().unwrap();
    let repo = aegis::repo::Repo::with_git(&dir, executable.to_str().unwrap());
    let found = aegis::tdd::check(&repo, MESSAGE, &[PATH.into()], "HEAD", "");
    assert_eq!(
        found.len(),
        1,
        "a replacement index blob must not borrow the original plain mode"
    );
    assert_eq!(
        found.first().map(|finding| finding.check),
        Some("commit.tdd")
    );
    assert!(
        git(&dir, &["ls-files", "--stage", "--", PATH]).starts_with("100755 ")
    );
}
