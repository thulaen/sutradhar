#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use super::{database, holes, scan_with};
use crate::tools_list::images;

pub(super) const HEX: &str =
    "3cd6bc2b74c87cbde0a0fd527c9665cb416f5a0540dee54fe5afb3fa2df9f40c";

/// One hole in the scanner's `CycloneDX` report.
fn hit(id: &str, severity: &str) -> String {
    format!(
        "{{\"id\": \"{id}\", \"ratings\": [{{\"score\": 0.1}}, \
         {{\"severity\": \"{severity}\"}}], \"affects\": [{{\"ref\": \
         \"pkg:apk/alpine/zlib@1.3?arch=x86_64\"}}]}}"
    )
}

fn report(hits: &[String]) -> String {
    format!(
        "{{\"bomFormat\": \"CycloneDX\", \"vulnerabilities\": [{}]}}",
        hits.join(", ")
    )
}

/// A list with one image entry, `image` and `digest` as given.
pub(super) fn tools(image: &str, digest: &str) -> String {
    format!(
        "[[tool]]\nname = \"forgejo\"\nversion = \"15\"\nsource = \"s\"\n\
         {image}\n{digest}\n\n[[tool]]\nname = \"gh\"\nversion = \"2\"\n\
         source = \"s\"\nsha256 = \"{HEX}\"\n"
    )
}

pub(super) fn good() -> String {
    tools(
        "image = \"codeberg.org/forgejo/forgejo\"",
        &format!("digest = \"sha256:{HEX}\""),
    )
}

#[test]
fn only_serious_holes_count_and_a_clean_report_has_none() {
    let text = report(&[
        hit("CVE-1", "critical"),
        hit("CVE-2", "High"),
        hit("CVE-2", "High"),
        hit("CVE-4", "medium"),
    ]);
    assert_eq!(
        holes(&text).unwrap(),
        [
            "CVE-1 (critical) in pkg:apk/alpine/zlib@1.3",
            "CVE-2 (high) in pkg:apk/alpine/zlib@1.3"
        ]
    );
    // Grype leaves the list out when it finds nothing.
    assert_eq!(
        holes("{\"bomFormat\": \"CycloneDX\"}").unwrap(),
        Vec::<String>::new()
    );
    assert!(holes("not json").is_err());
    assert!(holes("{\"matches\": []}").is_err());
}

#[test]
fn every_image_entry_must_be_whole() {
    assert_eq!(
        images(&good()).unwrap(),
        [(
            "forgejo".to_owned(),
            format!("codeberg.org/forgejo/forgejo@sha256:{HEX}")
        )]
    );
    let digest = format!("digest = \"sha256:{HEX}\"");
    let bad = [
        tools("", &digest),
        tools("image = \"codeberg.org/forgejo/forgejo\"", ""),
        tools("image = \"evil.example/x\"", &digest),
        tools("image = \"codeberg.org/x --config=y\"", &digest),
        tools("image = \"codeberg.org/x\"", "digest = \"sha256:12\""),
        tools("image = \"codeberg.org/x\"", "Digest = \"x\""),
    ];
    for list in bad {
        assert!(images(&list).is_err(), "{list}");
    }
}

#[test]
fn the_list_of_holes_lies_outside_the_repository() {
    let top = std::env::temp_dir()
        .join(format!("aegis-images-where-{}", std::process::id()));
    let _ = fs::remove_dir_all(&top);
    let root = top.join("repo");
    fs::create_dir_all(&root).unwrap();
    std::os::unix::fs::symlink(&root, top.join("link")).unwrap();
    assert!(database(&root, None).is_err());
    for bad in [
        PathBuf::from("db"),
        root.join("target/db"),
        top.join("link/db"),
        top.join("x/../repo/db"),
        top.clone(),
    ] {
        assert!(database(&root, Some(bad.clone())).is_err(), "{bad:?}");
    }
    assert!(database(&root, Some(top.join("db"))).is_ok());
    let _ = fs::remove_dir_all(&top);
}

/// A stand-in scanner in a fresh folder. It writes each call's
/// arguments, folder and settings to `log`; `db update` does what
/// `db_ok` says, and a scan prints `report`.
fn scanner(name: &str, db_ok: bool, report: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join(format!("aegis-images-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("db")).unwrap();
    fs::write(dir.join("report.json"), report).unwrap();
    let db = if db_ok {
        "exit 0"
    } else {
        "echo offline >&2; exit 1"
    };
    let here = dir.display();
    let script = format!(
        "#!/bin/sh\n{{ echo \"$@\"; pwd; env | grep -E '^(GRYPE_|HOME=)'; }} \
         >>{here}/log\ncase \"$3\" in\n  db) {db} ;;\n  \
         registry:*) cat {here}/report.json ;;\nesac\n"
    );
    let program = dir.join("grype");
    fs::write(&program, script).unwrap();
    fs::set_permissions(&program, fs::Permissions::from_mode(0o755)).unwrap();
    program
}

/// Runs the check with a stand-in scanner: its findings and its log.
fn run(name: &str, db_ok: bool, text: &str) -> (Vec<crate::Finding>, String) {
    let program = scanner(name, db_ok, text);
    let dir = program.parent().unwrap().to_path_buf();
    let found =
        scan_with(program.to_str().unwrap(), Ok(dir.join("db")), &good(), &[]);
    let log = fs::read_to_string(dir.join("log")).unwrap_or_default();
    let _ = fs::remove_dir_all(&dir);
    (found, log)
}

#[test]
fn a_planted_hole_is_refused_and_a_clean_image_passes() {
    let (planted, log) = run("planted", true, &report(&[hit("CVE-9", "high")]));
    assert_eq!(planted.len(), 1);
    assert!(planted[0].message.contains("CVE-9"), "{planted:?}");
    // The scan is asked for fixed holes only, for the host's platform,
    // with an empty settings file, in its own folder, with no update.
    for want in [
        "--only-fixed",
        "--platform linux/amd64",
        "-c ",
        "settings.yaml",
        "aegis-work",
        "GRYPE_DB_AUTO_UPDATE=false",
        "HOME=",
    ] {
        assert!(log.contains(want), "{want}: {log}");
    }
    let (clean, _) = run("clean", true, "{\"bomFormat\": \"CycloneDX\"}");
    assert_eq!(clean, Vec::new());
    let (offline, _) = run("offline", true, "");
    assert!(offline[0].message.contains("could not be scanned"));
    let (no_list, _) = run("nolist", false, "");
    assert!(no_list[0].message.contains("list of holes"), "{no_list:?}");
}

#[test]
fn a_missing_scanner_or_an_unpinned_image_fails() {
    let db = std::env::temp_dir()
        .join(format!("aegis-images-db-{}", std::process::id()));
    let missing = scan_with("/no/such/grype", Ok(db.clone()), &good(), &[]);
    let _ = fs::remove_dir_all(&db);
    assert_eq!(missing.len(), 1);
    assert!(missing[0].message.contains("/no/such/grype"), "{missing:?}");
    let other = "a".repeat(64);
    let used = [("deploy/x.sh".to_owned(), format!("pull x@sha256:{other}\n"))];
    let found = scan_with("/no/grype", Err("no db".into()), &good(), &used);
    assert_eq!(found.len(), 2, "{found:?}");
    assert!(found[0].message.contains("not pinned"), "{found:?}");
}

/// The findings for one compose file whose image is `image`, with the
/// list pinning the Forgejo image.
fn compose(image: &str) -> Vec<crate::Finding> {
    let text = format!("services:\n  forgejo:\n    image: {image}\n");
    let used = [("docker/f/compose.yaml".to_owned(), text)];
    scan_with("/no/grype", Err("no db".into()), &good(), &used)
        .into_iter()
        .filter(|f| f.path != "gate/tools.toml")
        .collect()
}

#[test]
fn the_images_that_run_must_be_the_images_pinned() {
    // The pinned image, with or without its tag, passes; so does a
    // digest broken across lines in a quoted string.
    let pinned =
        format!("codeberg.org/forgejo/forgejo:15-rootless@sha256:{HEX}");
    assert_eq!(compose(&pinned), Vec::new());
    let (head, tail) = HEX.split_at(20);
    let broken = format!(
        "\"codeberg.org/forgejo/forgejo@sha256:{head}\\\n      {tail}\""
    );
    assert_eq!(compose(&broken), Vec::new());
    // A tag alone, a digest given by a setting, and another image are
    // refused.
    for bad in [
        "codeberg.org/forgejo/forgejo:15.0.11-rootless".to_owned(),
        "codeberg.org/forgejo/forgejo@sha256:${D}".to_owned(),
        format!("codeberg.org/other/forgejo@sha256:{HEX}"),
    ] {
        assert_eq!(compose(&bad).len(), 1, "{bad}");
    }
    // A runner label by tag is refused, by pinned digest it passes.
    let label = |image: &str| {
        let text =
            format!("runner:\n  labels:\n    - \"dell:docker://{image}\"\n");
        let used = [("deploy/runner/config.yml".to_owned(), text)];
        scan_with("/no/grype", Err("no db".into()), &ubuntu(), &used).len()
    };
    assert_eq!(label("ubuntu:26.04"), 2);
    assert_eq!(label(&format!("ubuntu@sha256:{HEX}")), 1);
}

/// A list pinning the official Ubuntu image.
fn ubuntu() -> String {
    tools(
        "image = \"docker.io/library/ubuntu\"",
        &format!("digest = \"sha256:{HEX}\""),
    )
}
