//! Snapshot test: the exact words the gate shows for a set of planted
//! faults are kept in `tests/snapshots`, so any change to what an agent
//! or the owner reads is seen in review. To accept a deliberate change,
//! run the test with `AEGIS_UPDATE_SNAPSHOTS=1` and commit the new file.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::fs;
use std::path::Path;

use aegis::repo::Repo;
use aegis::tiers;

mod common;

const SNAPSHOT: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/snapshots/file_checks.txt"
);

#[test]
fn the_report_for_planted_faults_matches_its_snapshot() {
    let dir = std::env::temp_dir()
        .join(format!("aegis-snapshot-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::create_dir_all(dir.join("gate")).unwrap();
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let list = "gate/metrics.toml";
    let _bytes = fs::copy(root.join(list), dir.join(list)).unwrap();
    // Built from parts, so this file holds none of the faults whole.
    let marker = ["TO", "DO"].concat();
    let trick = ["#[kani::unw", "ind(0)]"].concat();
    let vain = ["total", "_visits"].concat();
    let code = format!(
        "// {marker} later\n{trick}\nfn f() {{\n    let {vain} = 1;\n}}\n"
    );
    let long = format!("{}\n", "x".repeat(90));
    for (path, text) in [("src/a.rs", code), ("src/b.rs", long)] {
        fs::write(dir.join(path), text).unwrap();
    }
    let paths = ["src/a.rs".to_owned(), "src/b.rs".to_owned()];
    let lines: Vec<String> = tiers::file_checks(&Repo::at(&dir), &paths)
        .iter()
        .map(ToString::to_string)
        .collect();
    let report = format!("{}\n", lines.join("\n"));
    fs::remove_dir_all(&dir).unwrap();
    if std::env::var_os("AEGIS_UPDATE_SNAPSHOTS").is_some() {
        fs::write(SNAPSHOT, &report).unwrap();
    }
    let kept = fs::read_to_string(SNAPSHOT).unwrap_or_default();
    assert_eq!(report, kept, "the report changed; see the test's notes");
}

const SCORE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/snapshots/score_line.txt"
);

/// Runs the `aegis` command in `dir`, with none of the caller's git
/// settings, so a hook's own index is never touched: its exit code and
/// what it said.
fn aegis(dir: &Path, args: &[&str]) -> (i32, String) {
    let out = std::process::Command::new(env!("CARGO_BIN_EXE_aegis"))
        .env_remove("GIT_DIR")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_WORK_TREE")
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap();
    let text = String::from_utf8_lossy(&out.stderr).into_owned();
    (out.status.code().unwrap_or(-1), text)
}

#[test]
fn a_commit_shows_its_score_and_a_message_check_does_not() {
    let dir = common::repo("score", "line");
    fs::write(dir.join("a.txt"), "plain text\n").unwrap();
    let _ok = common::git(&dir, &["add", "a.txt"]);
    let (code, said) = aegis(&dir, &["commit"]);
    let score: Vec<&str> = said
        .lines()
        .filter(|l| l.contains("quality score"))
        .collect();
    let kept = format!("{}\n", score.join("\n"));
    if std::env::var_os("AEGIS_UPDATE_SNAPSHOTS").is_some() {
        fs::write(SCORE, &kept).unwrap();
    }
    assert_eq!(code, 1, "{said}");
    assert_eq!(score.len(), 1, "{said}");
    assert_eq!(kept, fs::read_to_string(SCORE).unwrap_or_default());
    fs::write(dir.join("m"), "docs: x\n").unwrap();
    let (_, said) = aegis(&dir, &["msg", "m"]);
    assert!(!said.contains("quality score"), "{said}");
    fs::remove_dir_all(&dir).unwrap();
}
