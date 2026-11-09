//! Tests of the `aegis` command itself.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::fs;
use std::process::Command;

const GOOD_MSG: &str = "docs: start\n\nTDD: none (docs only)\nDefects: none\n\
    Task: bootstrap\nSigned-off-by: A <a@example.com>\n";

fn aegis(dir: &std::path::Path, args: &[&str]) -> (i32, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_aegis"))
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap();
    let text = String::from_utf8_lossy(&out.stderr).into_owned();
    (out.status.code().unwrap_or(-1), text)
}

fn folder(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir()
        .join(format!("aegis-cmd-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn msg_passes_a_good_message_and_refuses_a_bad_one() {
    let dir = folder("msg");
    let init = Command::new("git")
        .args(["init", "-q"])
        .current_dir(&dir)
        .status()
        .unwrap();
    assert!(init.success());
    fs::write(dir.join("good"), GOOD_MSG).unwrap();
    fs::write(dir.join("bad"), "stuff\n").unwrap();
    let (code, text) = aegis(&dir, &["msg", "good"]);
    assert_eq!((code, text.as_str()), (0, "aegis: every check passed\n"));
    let (code, text) = aegis(&dir, &["msg", "bad"]);
    assert_eq!(code, 1);
    assert!(text.contains("nothing may go in"), "{text}");
}

#[test]
fn bad_use_and_missing_files_stop_with_code_2() {
    let dir = folder("usage");
    let (code, text) = aegis(&dir, &[]);
    assert_eq!(code, 2);
    assert!(text.contains("usage:"), "{text}");
    let (code, text) = aegis(&dir, &["msg", "nope"]);
    assert_eq!(code, 2);
    assert!(text.contains("cannot read nope"), "{text}");
}

#[test]
fn commit_and_push_outside_git_stop_with_code_2() {
    let dir = folder("nogit");
    assert_eq!(aegis(&dir, &["commit"]).0, 2);
    assert_eq!(aegis(&dir, &["push", "a", "b"]).0, 2);
}

fn git(dir: &std::path::Path, args: &[&str], index: Option<&str>) {
    let mut command = Command::new("git");
    let _ = command
        .env_remove("GIT_DIR")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_WORK_TREE");
    if let Some(i) = index {
        let _ = command.env("GIT_INDEX_FILE", i);
    }
    let out = command.arg("-C").arg(dir).args(args).output().unwrap();
    assert!(out.status.success(), "git {args:?}: {out:?}");
}

#[test]
fn commit_checks_the_index_git_gives_the_hook() {
    let dir = folder("index");
    git(&dir, &["init", "-q", "-b", "main"], None);
    let pointer = "Read [AGENTS.md](AGENTS.md). It is the only rule file.\n";
    for (name, text) in [
        ("AGENTS.md", "# Rules\n"),
        ("CLAUDE.md", pointer),
        ("GEMINI.md", pointer),
        ("README.md", "# Read me\n"),
    ] {
        fs::write(dir.join(name), text).unwrap();
    }
    git(&dir, &["add", "-A"], None);
    let _ =
        fs::copy(dir.join(".git/index"), dir.join(".git/alt-index")).unwrap();
    fs::write(dir.join("README.md"), "This is in progress.\n").unwrap();
    git(&dir, &["add", "README.md"], Some(".git/alt-index"));
    let run = |index: Option<&str>| {
        let mut command = Command::new(env!("CARGO_BIN_EXE_aegis"));
        let _ = command
            .current_dir(&dir)
            .arg("commit")
            .env_remove("GIT_INDEX_FILE");
        if let Some(i) = index {
            let _ = command.env("GIT_INDEX_FILE", i);
        }
        command.output().unwrap().status.code()
    };
    assert_eq!(run(None), Some(0));
    assert_eq!(run(Some(".git/alt-index")), Some(1));
    let full = dir.join(".git/alt-index").display().to_string();
    assert_eq!(run(Some(&full)), Some(1));
}

#[test]
fn commit_refuses_a_private_word_without_showing_it() {
    let dir = folder("private");
    git(&dir, &["init", "-q", "-b", "main"], None);
    let lists = folder("private-lists");
    // Built from parts, so this file holds no private-looking word whole.
    let word = ["my", "forumname"].concat();
    for (name, text) in [
        ("deny-list.txt", format!("{word}\n")),
        ("xenforo-phrases.txt", String::new()),
        ("xenforo-index.txt", String::new()),
        ("credential-files.txt", String::new()),
    ] {
        fs::write(lists.join(name), text).unwrap();
    }
    let pointer = "Read [AGENTS.md](AGENTS.md). It is the only rule file.\n";
    for (name, text) in [
        ("AGENTS.md", "# Rules\n".to_owned()),
        ("CLAUDE.md", pointer.to_owned()),
        ("GEMINI.md", pointer.to_owned()),
        ("README.md", format!("# Read me\n\nSee {word} for more.\n")),
    ] {
        fs::write(dir.join(name), text).unwrap();
    }
    git(&dir, &["add", "-A"], None);
    let out = Command::new(env!("CARGO_BIN_EXE_aegis"))
        .current_dir(&dir)
        .arg("commit")
        .env_remove("GIT_INDEX_FILE")
        .env("AEGIS_PRIVATE", &lists)
        .output()
        .unwrap();
    let shown = String::from_utf8_lossy(&out.stderr).to_lowercase();
    assert_eq!(out.status.code(), Some(1), "{shown}");
    assert!(shown.contains("secrets.private-lists"), "{shown}");
    assert!(!shown.contains(&word), "the word was shown");
    let run = |args: &[&str], lists: &std::path::Path| {
        let out = Command::new(env!("CARGO_BIN_EXE_aegis"))
            .current_dir(&dir)
            .args(args)
            .env_remove("GIT_INDEX_FILE")
            .env("AEGIS_PRIVATE", lists)
            .output()
            .unwrap();
        let shown = String::from_utf8_lossy(&out.stderr).to_lowercase();
        assert!(!shown.contains(&word), "the word was shown");
        (out.status.code(), shown)
    };
    // A clean tree passes; missing lists fail rather than pass unseen.
    fs::write(dir.join("README.md"), "# Read me\n").unwrap();
    git(&dir, &["add", "-A"], None);
    assert_eq!(run(&["commit"], &lists).0, Some(0));
    assert_eq!(run(&["commit"], &dir.join("no-such-folder")).0, Some(1));
    // A commit message with the word is refused.
    let message = dir.join("message");
    fs::write(&message, format!("{GOOD_MSG}\n{word}\n")).unwrap();
    let (code, shown) = run(&["msg", "message"], &lists);
    assert_eq!(code, Some(1));
    assert!(shown.contains("secrets.private-lists"), "{shown}");
}

#[test]
fn push_refuses_a_private_word_left_in_history() {
    let dir = folder("private-history");
    git(&dir, &["init", "-q", "-b", "main"], None);
    git(&dir, &["config", "user.email", "a@example.com"], None);
    git(&dir, &["config", "user.name", "A"], None);
    let lists = folder("private-history-lists");
    let word = ["my", "forumname"].concat();
    for (name, text) in [
        ("deny-list.txt", format!("{word}\n")),
        ("xenforo-phrases.txt", String::new()),
        ("xenforo-index.txt", String::new()),
        ("credential-files.txt", String::new()),
    ] {
        fs::write(lists.join(name), text).unwrap();
    }
    let save = |text: &str| {
        fs::write(dir.join("notes.md"), text).unwrap();
        git(&dir, &["add", "-A"], None);
        git(&dir, &["commit", "-q", "-m", GOOD_MSG], None);
    };
    save("# Notes\n");
    let base = String::from_utf8(
        Command::new("git")
            .args(["-C"])
            .arg(&dir)
            .args(["rev-parse", "HEAD"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    save(&format!("# Notes\n\n{word}\n"));
    save("# Notes\n");
    let out = Command::new(env!("CARGO_BIN_EXE_aegis"))
        .current_dir(&dir)
        .args(["push", base.trim(), "HEAD"])
        .env("AEGIS_PRIVATE", &lists)
        .output()
        .unwrap();
    let shown = String::from_utf8_lossy(&out.stderr).to_lowercase();
    assert!(!shown.contains(&word), "the word was shown");
    assert!(shown.contains("notes.md (history)"), "{shown}");
}
