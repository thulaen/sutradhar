//! The advisory lists planted projects read, so they need no network.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::fs;
use std::path::Path;
use std::process::Command;

/// Planted projects read an advisory list of their own, so they need no
/// network. The address is never fetched; the pinned cargo-deny keeps a
/// list for it in the folder named below.
pub const PLANTED_DENY: &str = "[licenses]\n\
    allow = [\"MIT\", \"Apache-2.0\"]\n\n\
    [advisories]\ndb-path = \"dbs\"\n\
    db-urls = [\"https://planted.invalid/advisory-db\"]\n";
const LIST: &str = "dbs/advisory-db-f14be4e4cecf18b7";

/// Writes the planted advisory list, one advisory per named library.
pub fn advisory_list(dir: &Path, vulnerable: &[&str]) {
    let list = dir.join(LIST);
    let _ = fs::remove_dir_all(&list);
    // cargo-deny refuses an empty list, so every list also names a
    // library no planted project uses.
    let unused = ["aegis-planted-unused"];
    for (n, name) in unused.iter().chain(vulnerable).enumerate() {
        let id = format!("RUSTSEC-2099-{:04}", n.saturating_add(1));
        let folder = list.join("crates").join(name);
        fs::create_dir_all(&folder).unwrap();
        let advisory = format!(
            "```toml\n[advisory]\nid = \"{id}\"\n\
             package = \"{name}\"\ndate = \"2099-01-01\"\n\n\
             [versions]\npatched = []\n```\n\n# A planted fault\n\n\
             Not a real advisory.\n"
        );
        fs::write(folder.join(format!("{id}.md")), advisory).unwrap();
    }
    for args in [
        &["init", "-q"][..],
        &["add", "-A"],
        &[
            "-c",
            "user.name=t",
            "-c",
            "user.email=t@t",
            "commit",
            "--allow-empty",
            "-qm",
            "list",
        ],
    ] {
        let ok = Command::new("git")
            .arg("-C")
            .arg(&list)
            .args(args)
            .status()
            .unwrap();
        assert!(ok.success());
    }
}
