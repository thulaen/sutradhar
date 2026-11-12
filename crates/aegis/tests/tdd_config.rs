//! Only proved image data changes may use the no-code TDD form.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::path::{Path, PathBuf};
use std::process::Command;

mod common;
use common::{git, repo, write};

const MESSAGE: &str = "fix: pin an image\n\nTDD: none (docs only)\n\
    Defects: none\nTask: #39\nSigned-off-by: A <a@example.com>\n";
const PATHS: [&str; 3] = [
    "gate/tools.toml",
    "gate/sbom.cdx.json",
    "docker/forgejo/compose.yaml",
];

fn configuration(path: &str, hex: &str) -> String {
    match path {
        "gate/tools.toml" => format!(
            "[[tool]]\nname = \"forgejo\"\nversion = \"15\"\n\
             source = \"official\"\nimage = \"codeberg.org/forgejo/forgejo\"\n\
             digest = \"sha256:{hex}\"\n"
        ),
        "gate/sbom.cdx.json" => format!(
            "{}\n",
            serde_json::to_string_pretty(&serde_json::json!({
                "bomFormat": "CycloneDX",
                "components": [{ "type": "container", "hashes": [{
                    "alg": "SHA-256", "content": hex,
                }] }],
            }))
            .unwrap()
        ),
        _ => format!(
            "services:\n  forgejo:\n    image: \
             \"codeberg.org/forgejo/forgejo@\\\n      sha256:{hex}\"\n\
             \x20   command: [sleep, '10']\n"
        ),
    }
}

fn staged(name: &str) -> PathBuf {
    let dir = repo("tdd-config", name);
    for path in PATHS {
        write(&dir, path, &configuration(path, &"a".repeat(64)));
    }
    for path in PATHS {
        let _ok = git(&dir, &["add", "--", path]);
    }
    dir
}

fn stage_changes(dir: &Path) {
    for path in PATHS {
        write(dir, path, &configuration(path, &"b".repeat(64)));
        let _ok = git(dir, &["add", "--", path]);
    }
}

fn msg(dir: &Path) -> (i32, String) {
    msg_using(dir, None)
}

fn msg_using(dir: &Path, index: Option<&str>) -> (i32, String) {
    write(dir, ".git/tdd-message", MESSAGE);
    let mut command = Command::new(env!("CARGO_BIN_EXE_aegis"));
    let _args = command
        .current_dir(dir)
        .args(["msg", ".git/tdd-message"])
        .env_remove("GIT_INDEX_FILE");
    if let Some(index) = index {
        let _index = command.env("GIT_INDEX_FILE", index);
    }
    let out = command.output().unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn configuration_proof_keeps_the_index_supplied_to_the_hook() {
    let dir = staged("hook-index");
    let _ok = git(&dir, &["commit", "-q", "-m", MESSAGE]);
    stage_changes(&dir);
    let _bytes =
        std::fs::copy(dir.join(".git/index"), dir.join(".git/other-index"))
            .unwrap();
    let path = "docker/forgejo/compose.yaml";
    let executable = configuration(path, &"b".repeat(64))
        .replace("[sleep, '10']", "[sleep, '20']");
    write(&dir, path, &executable);
    let _ok = git(&dir, &["add", "--", path]);
    assert_eq!(msg(&dir).0, 1);
    let (code, text) = msg_using(&dir, Some(".git/other-index"));
    assert_eq!(code, 0, "{text}");
}

#[test]
fn digest_only_configuration_needs_no_named_test() {
    let dir = staged("digest-only");
    let _ok = git(&dir, &["commit", "-q", "-m", MESSAGE]);
    stage_changes(&dir);
    let (code, text) = msg(&dir);
    assert_eq!(code, 0, "{text}");
}

#[test]
fn configuration_proof_reads_the_index_not_the_worktree() {
    let dir = staged("index-content");
    let _ok = git(&dir, &["commit", "-q", "-m", MESSAGE]);
    stage_changes(&dir);
    let path = "docker/forgejo/compose.yaml";
    let executable = configuration(path, &"b".repeat(64))
        .replace("[sleep, '10']", "[sleep, '20']");
    write(&dir, path, &executable);
    assert_eq!(msg(&dir).0, 0, "unstaged code must not alter the proof");
    let _ok = git(&dir, &["add", "--", path]);
    let (code, text) = msg(&dir);
    assert_eq!(code, 1, "{text}");
    assert!(text.contains(path), "{text}");
}

#[test]
fn non_ascii_comments_preserve_digest_proof() {
    let path = "docker/forgejo/compose.yaml";
    let prefix = "# café — नमस्ते\n";
    let before = format!("{prefix}{}", configuration(path, &"a".repeat(64)));
    let after = format!("{prefix}{}", configuration(path, &"b".repeat(64)));
    let dir = changed("unicode-comment", path, &before, &after);
    let (code, text) = msg(&dir);
    assert_eq!(code, 0, "{text}");
}

fn changed(name: &str, path: &str, before: &str, after: &str) -> PathBuf {
    let dir = repo("tdd-config", name);
    write(&dir, path, before);
    let _ok = git(&dir, &["add", "--", path]);
    let _ok = git(&dir, &["commit", "-q", "-m", MESSAGE]);
    write(&dir, path, after);
    let _ok = git(&dir, &["add", "--", path]);
    dir
}

#[test]
fn executable_ambiguous_and_unrecognised_config_stays_refused() {
    let path = "docker/forgejo/compose.yaml";
    let before = configuration(path, &"a".repeat(64));
    let after = configuration(path, &"b".repeat(64));
    let cases = [
        (path, after.replace("[sleep, '10']", "[sleep, '20']")),
        (path, format!("{after}    entrypoint: [echo, changed]\n")),
        (
            path,
            format!("{after}    healthcheck:\n      test: changed\n"),
        ),
        (path, format!("{after}    image: other\n")),
        (path, format!("{after}# changed comment\n")),
        (path, format!("{after}    note: {}\n", "b".repeat(64))),
        (path, "services: [\n".to_owned()),
        (path, after.replace("services:", "services: !!map")),
        (path, format!("{after}x: &copy 1\ny: *copy\n")),
        (path, format!("{after}---\nother: document\n")),
        (path, format!("{after}services: {{}}\n")),
        (path, "services: []\n".to_owned()),
        (path, "services: {}\n".to_owned()),
        (".github/workflows/a.yml", after.clone()),
        (".forgejo/workflows/a.yaml", after.clone()),
        (".gitea/workflows/a.yml", after.clone()),
        ("Cargo.toml", after.clone()),
        ("deploy/runner/settings.yml", after.clone()),
        ("gate/image-holes.toml", after),
    ];
    for (index, (file, content)) in cases.into_iter().enumerate() {
        let dir = changed(&format!("refused-{index}"), file, &before, &content);
        let (code, text) = msg(&dir);
        assert_eq!(code, 1, "{file}: {text}");
        assert!(text.contains("commit.tdd"), "{text}");
    }
}

#[test]
fn extra_or_ambiguous_digest_data_changes_stay_refused() {
    for (index, path) in PATHS.into_iter().take(2).enumerate() {
        let before = configuration(path, &"a".repeat(64));
        let after = configuration(path, &"b".repeat(64));
        let changed_data = if path == PATHS[0] {
            after.replace("version = \"15\"", "version = \"16\"")
        } else {
            after.replace("CycloneDX", "Other")
        };
        let dir = changed(
            &format!("extra-data-{index}"),
            path,
            &before,
            &changed_data,
        );
        assert_eq!(msg(&dir).0, 1);
        let duplicate = if path == PATHS[0] {
            format!("{after}digest = \"sha256:{}\"\n", "b".repeat(64))
        } else {
            after.replace("\"SHA-256\"", "\"SHA-256\", \"alg\": \"SHA-256\"")
        };
        let dir = changed(
            &format!("duplicate-data-{index}"),
            path,
            &before,
            &duplicate,
        );
        assert_eq!(msg(&dir).0, 1);
    }
}

#[test]
fn unchanged_bytes_with_a_mode_change_do_not_prove_a_digest_delta() {
    let dir = staged("mode-only");
    let _ok = git(&dir, &["commit", "-q", "-m", MESSAGE]);
    let _ok = git(&dir, &["update-index", "--chmod=+x", PATHS[0]]);
    assert_eq!(msg(&dir).0, 1);
    stage_changes(&dir);
    let _ok = git(&dir, &["update-index", "--chmod=+x", PATHS[0]]);
    assert_eq!(msg(&dir).0, 1);
}

#[test]
fn pattern_paths_cannot_borrow_a_different_files_plain_mode() {
    let dir = repo("tdd-config", "pattern-mode");
    let path = "docker/z[!a]/compose.yaml";
    let before = configuration(path, &"a".repeat(64));
    for file in ["docker/z0/compose.yaml", path] {
        write(&dir, file, &before);
        let _ok = git(&dir, &["add", "--", file]);
    }
    let _ok = git(&dir, &["commit", "-q", "-m", MESSAGE]);
    write(&dir, path, &configuration(path, &"b".repeat(64)));
    let _ok = git(&dir, &["add", "--", path]);
    assert_eq!(msg(&dir).0, 0);
    let _ok = git(&dir, &["update-index", "--chmod=+x", "--", path]);
    let (code, text) = msg(&dir);
    assert_eq!(code, 1, "the exact path has executable mode: {text}");
    assert!(text.contains("commit.tdd"), "{text}");
    let _ok = git(&dir, &["commit", "-q", "-m", MESSAGE]);
    let repo = aegis::repo::Repo::at(&dir);
    assert_eq!(
        aegis::tdd::check(&repo, MESSAGE, &[path.into()], "HEAD^", "HEAD")
            .len(),
        1
    );
}

#[test]
fn missing_or_deleted_content_and_mixed_source_stay_refused() {
    let dir = repo("tdd-config", "missing-content");
    write(&dir, "README.md", "# Read me\n");
    let _ok = git(&dir, &["add", "--", "README.md"]);
    let _ok = git(&dir, &["commit", "-q", "-m", MESSAGE]);
    stage_changes(&dir);
    assert_eq!(msg(&dir).0, 1);
    let dir = staged("deleted-content");
    let _ok = git(&dir, &["commit", "-q", "-m", MESSAGE]);
    let _ok = git(&dir, &["rm", "--", PATHS[0]]);
    assert_eq!(msg(&dir).0, 1);
    let dir = staged("mixed-source");
    let _ok = git(&dir, &["commit", "-q", "-m", MESSAGE]);
    stage_changes(&dir);
    write(&dir, "crates/a/src/lib.rs", "pub fn changed() {}\n");
    let _ok = git(&dir, &["add", "--", "crates/a/src/lib.rs"]);
    let (code, text) = msg(&dir);
    assert_eq!(code, 1, "{text}");
    assert!(text.contains("crates/a/src/lib.rs"), "{text}");
    assert!(!text.contains(PATHS[0]), "{text}");
}

#[test]
fn push_proof_uses_each_commit_instead_of_the_final_tree() {
    let dir = staged("push-history");
    let _ok = git(&dir, &["commit", "-q", "-m", MESSAGE]);
    let base = git(&dir, &["rev-parse", "HEAD"]);
    stage_changes(&dir);
    let _ok = git(&dir, &["commit", "-q", "-m", MESSAGE]);
    let path = "docker/forgejo/compose.yaml";
    let clean = configuration(path, &"b".repeat(64));
    write(&dir, path, &clean.replace("[sleep, '10']", "[sleep, '20']"));
    let _ok = git(&dir, &["add", "--", path]);
    let _ok = git(&dir, &["commit", "-q", "-m", MESSAGE]);
    let bad = git(&dir, &["rev-parse", "HEAD"]);
    write(&dir, path, &clean);
    let _ok = git(&dir, &["add", "--", path]);
    let _ok = git(&dir, &["commit", "-q", "-m", MESSAGE]);
    let found = aegis::tiers::push(&aegis::repo::Repo::at(&dir), &base, "HEAD")
        .unwrap();
    let tdd: Vec<_> =
        found.iter().filter(|f| f.check == "commit.tdd").collect();
    assert_eq!(tdd.len(), 2, "{found:?}; bad commit {bad}");
    assert!(tdd.iter().all(|f| f.path == path), "{tdd:?}");
}
