//! A link in a project is refused and never followed (task #119).
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::fs;

use aegis::repo::Repo;
use aegis::tiers;

#[test]
fn a_link_is_refused_and_never_read() {
    let dir = std::env::temp_dir()
        .join(format!("aegis-links-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let outside = dir.join("outside-secret");
    fs::write(&outside, "host-only secret text\n").unwrap();
    std::os::unix::fs::symlink(&outside, dir.join("creds.md")).unwrap();
    let repo = Repo::at(&dir);
    assert_eq!(repo.read("creds.md"), None);
    let found = tiers::file_checks(&repo, &["creds.md".to_owned()]);
    assert!(
        found.iter().any(|f| f.check == "files.no-links"),
        "{found:?}"
    );
    assert!(
        found.iter().all(|f| !f.message.contains("host-only")),
        "{found:?}"
    );
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_file_too_large_to_check_is_refused_not_skipped() {
    let dir = std::env::temp_dir()
        .join(format!("aegis-large-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let big = fs::File::create(dir.join("big.md")).unwrap();
    big.set_len(aegis::repo::MOST_READ + 1).unwrap();
    let repo = Repo::at(&dir);
    let found = tiers::file_checks(&repo, &["big.md".to_owned()]);
    assert!(
        found.iter().any(|f| f.check == "files.too-large"),
        "{found:?}"
    );
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_path_that_climbs_out_or_starts_at_the_top_is_never_read() {
    let repo = Repo::at(std::env::temp_dir());
    for path in ["../outside", "/etc/hostname", "a/../../b"] {
        assert!(repo.is_link(path), "{path}");
        assert_eq!(repo.read(path), None, "{path}");
    }
}
