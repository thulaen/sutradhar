//! Snapshot test: the exact words the gate shows for a set of planted
//! faults are kept in `tests/snapshots`, so any change to what an agent
//! or the owner reads is seen in review. To accept a deliberate change,
//! run the test with `AEGIS_UPDATE_SNAPSHOTS=1` and commit the new file.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::fs;
use std::path::Path;

use aegis::repo::Repo;
use aegis::tiers;

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
