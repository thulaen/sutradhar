//! The nightly sweep script, run on scratch folders.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::fs;
use std::path::Path;
use std::process::Command;

const SWEEP: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../deploy/pankti/sweep.sh");

const NAMES: [&str; 4] =
    ["aegis-old", "forge-sync-old", "aegis-new", "mine-old"];

fn age(path: &Path) {
    let done = Command::new("touch")
        .args(["-d", "2 hours ago"])
        .arg(path)
        .status()
        .unwrap();
    assert!(done.success());
}

#[test]
fn old_test_leftovers_in_the_temp_folder_go_and_nothing_else_does() {
    let base = std::env::temp_dir()
        .join(format!("pankti-sweep-{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    let (queue, tmp) = (base.join("queue"), base.join("tmp"));
    fs::create_dir_all(&queue).unwrap();
    for name in ["line", "place-1", "place-2"] {
        fs::write(queue.join(name), "").unwrap();
    }
    for name in NAMES {
        fs::create_dir_all(tmp.join(name)).unwrap();
        fs::write(tmp.join(name).join("f"), "x").unwrap();
    }
    for name in ["aegis-old", "forge-sync-old", "mine-old"] {
        age(&tmp.join(name));
    }
    let out = Command::new("bash")
        .arg(SWEEP)
        .arg(&queue)
        .args(["1", "0"])
        .env("TMPDIR", &tmp)
        .current_dir(&base)
        .output()
        .unwrap();
    assert!(out.status.success(), "{out:?}");
    let left: Vec<bool> = NAMES.iter().map(|n| tmp.join(n).exists()).collect();
    assert_eq!(left, [false, false, true, true]);
    fs::remove_dir_all(&base).unwrap();
}
