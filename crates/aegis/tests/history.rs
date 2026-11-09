//! How the gate reads a push's changes: limits on size, lines and file
//! names, and files that `.gitattributes` would hide.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use aegis::repo::Repo;

fn git(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(["-c", "user.name=t", "-c", "user.email=t@t"])
        .args(["-c", "core.hooksPath=/dev/null"])
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .args(args)
        .current_dir(dir)
        .env_remove("GIT_INDEX_FILE")
        .output()
        .unwrap();
    assert!(out.status.success(), "git {args:?}: {out:?}");
    String::from_utf8_lossy(&out.stdout).trim().to_owned()
}

fn write(dir: &Path, path: &str, text: &str) {
    let path = dir.join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join(format!("aegis-reviews-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let _ok = git(&dir, &["init", "-q", "-b", "main"]);
    write(&dir, "crates/aegis/src/reviews.rs", "// the check\n");
    let _ok = git(&dir, &["add", "-A"]);
    let _ok = git(&dir, &["commit", "-qm", "start"]);
    dir
}

#[test]
fn a_change_too_long_or_naming_a_very_long_file_is_refused() {
    let dir = scratch("patch");
    let base = git(&dir, &["rev-parse", "HEAD"]);
    // One name may hold at most 255 bytes on disk, so the long path is
    // three folders deep.
    let part = "n".repeat(200);
    let long = format!("{part}/{part}/{part}.txt");
    write(&dir, &long, "x\n");
    let _ok = git(&dir, &["add", "-A"]);
    let _ok = git(&dir, &["commit", "-qm", "long name"]);
    let head = git(&dir, &["rev-parse", "HEAD"]);
    let refused = aegis::history::added_lines(&Repo::at(&dir), &base, &head);
    assert!(refused.is_err_and(|e| e.contains("longer than 512")));
    // A rename from a short name to a long one is measured too.
    let _ok = git(&dir, &["reset", "-q", "--hard", &base]);
    write(&dir, "x.txt", "same text\n");
    let _ok = git(&dir, &["add", "-A"]);
    let _ok = git(&dir, &["commit", "-qm", "short"]);
    let short_id = git(&dir, &["rev-parse", "HEAD"]);
    let folder = dir.join(&long);
    fs::create_dir_all(folder.parent().unwrap()).unwrap();
    let _ok = git(&dir, &["mv", "x.txt", &long]);
    // A rename that also adds a line has a `+++` line naming the new file,
    // which is measured.
    write(&dir, &long, "same text\nand one more\n");
    let _ok = git(&dir, &["add", "-A"]);
    let _ok = git(&dir, &["commit", "-qm", "renamed"]);
    let renamed = git(&dir, &["rev-parse", "HEAD"]);
    let refused =
        aegis::history::added_lines(&Repo::at(&dir), &short_id, &renamed);
    assert!(refused.is_err_and(|e| e.contains("longer than 512")));
    // A pure rename to the long name shows, since renames are not
    // looked for, as an add of every line under the new name, which is
    // measured too.
    let _ok = git(&dir, &["reset", "-q", "--hard", &short_id]);
    fs::create_dir_all(folder.parent().unwrap()).unwrap();
    let _ok = git(&dir, &["mv", "x.txt", &long]);
    let _ok = git(&dir, &["commit", "-qm", "only renamed"]);
    let pure = git(&dir, &["rev-parse", "HEAD"]);
    let refused =
        aegis::history::added_lines(&Repo::at(&dir), &short_id, &pure);
    assert!(refused.is_err_and(|e| e.contains("longer than 512")));
    // A short name in another script is not refused: 50 letters of 3
    // bytes each, which git would quote as 600 bytes of escapes.
    let _ok = git(&dir, &["reset", "-q", "--hard", &short_id]);
    write(&dir, &format!("{}.txt", "\u{915}".repeat(50)), "x\n");
    let _ok = git(&dir, &["add", "-A"]);
    let _ok = git(&dir, &["commit", "-qm", "hindi name"]);
    let hindi = git(&dir, &["rev-parse", "HEAD"]);
    let seen = aegis::history::added_lines(&Repo::at(&dir), &short_id, &hindi);
    assert!(seen.is_ok_and(|l| l.iter().any(|(_, line)| line == "x")));
    // A change of mode only names the file only on the `diff --git` line,
    // with both its names, which is measured too.
    let _ok = git(&dir, &["reset", "-q", "--hard", &pure]);
    let _ok = git(&dir, &["update-index", "--chmod=+x", &long]);
    let _ok = git(&dir, &["commit", "-qm", "mode only"]);
    let mode = git(&dir, &["rev-parse", "HEAD"]);
    let refused = aegis::history::added_lines(&Repo::at(&dir), &pure, &mode);
    assert!(refused.is_err_and(|e| e.contains("longer than 512")));
    let short = aegis::history::added_lines(&Repo::at(&dir), &base, &base);
    assert_eq!(short.unwrap(), Vec::new());
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_file_marked_not_to_diff_still_shows_its_lines() {
    let dir = scratch("nodiff");
    let base = git(&dir, &["rev-parse", "HEAD"]);
    write(&dir, ".gitattributes", "hidden.dat -diff\n");
    write(&dir, "hidden.dat", "a line to see\n");
    let _ok = git(&dir, &["add", "-A"]);
    let _ok = git(&dir, &["commit", "-qm", "hidden"]);
    let head = git(&dir, &["rev-parse", "HEAD"]);
    let lines =
        aegis::history::added_lines(&Repo::at(&dir), &base, &head).unwrap();
    assert!(
        lines
            .iter()
            .any(|(p, l)| p == "hidden.dat" && l == "a line to see")
    );
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn an_empty_file_just_over_the_limit_or_a_name_not_text_is_refused() {
    use std::os::unix::ffi::OsStrExt;
    let dir = scratch("edge");
    let base = git(&dir, &["rev-parse", "HEAD"]);
    // 200 + 1 + 200 + 1 + 112 = 514 bytes: an empty file has no `+++` line.
    let path = format!(
        "{}/{}/{}",
        "e".repeat(200),
        "e".repeat(200),
        "e".repeat(112)
    );
    write(&dir, &path, "");
    let _ok = git(&dir, &["add", "-A"]);
    let _ok = git(&dir, &["commit", "-qm", "empty"]);
    let empty = git(&dir, &["rev-parse", "HEAD"]);
    let refused = aegis::history::added_lines(&Repo::at(&dir), &base, &empty);
    assert!(refused.is_err_and(|e| e.contains("longer than 512")));
    let _ok = git(&dir, &["reset", "-q", "--hard", &base]);
    let odd = std::ffi::OsStr::from_bytes(b"odd-\xff\xfe.txt");
    fs::write(dir.join(odd), "x\n").unwrap();
    let _ok = git(&dir, &["add", "-A"]);
    let _ok = git(&dir, &["commit", "-qm", "odd name"]);
    let odd_id = git(&dir, &["rev-parse", "HEAD"]);
    let refused = aegis::history::added_lines(&Repo::at(&dir), &base, &odd_id);
    assert!(refused.is_err());
    fs::remove_dir_all(&dir).unwrap();
}
