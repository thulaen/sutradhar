//! Image data proofs must cover runner pins and removal of old exceptions.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

mod common;
use common::{git, repo, write};

const RUNNER: &str = "deploy/runner/config.yml";
const HOLES: &str = "gate/image-holes.toml";
const MESSAGE: &str = "fix: repair image data proof\n\nTDD: none (docs only)\n";

fn runner(hex: &str) -> String {
    format!(
        "runner:\n  labels:\n    - \"dell:docker://ubuntu@sha256:{hex}\"\n\
         container:\n  privileged: false\n"
    )
}

fn hole(image: &str, id: &str) -> String {
    format!(
        "[[hole]]\nimage = \"{image}\"\nid = \"{id}\"\ntask = 130\n\
         until = \"2026-12-09\"\n"
    )
}

fn proof(name: &str, path: &str, before: &str, after: &str) -> usize {
    let dir = repo("tdd-runner-holes", name);
    write(&dir, path, before);
    let _ok = git(&dir, &["add", "--", path]);
    let _ok = git(&dir, &["commit", "-qm", "base"]);
    write(&dir, path, after);
    let _ok = git(&dir, &["add", "--", path]);
    let repo = aegis::repo::Repo::at(&dir);
    aegis::tdd::check(&repo, MESSAGE, &[path.into()], "HEAD", "").len()
}

#[test]
fn runner_digest_and_removed_exception_are_proved_data() {
    let before = runner(&"a".repeat(64));
    let after = runner(&"b".repeat(64));
    let kept = hole("forgejo", "CVE-2026-4738");
    let removed = hole("ubuntu-image", "CVE-2026-109518");
    let results = [
        proof("runner", RUNNER, &before, &after),
        proof(
            "holes",
            HOLES,
            &format!("{kept}\n{removed}"),
            &format!("{kept}\n"),
        ),
    ];
    assert_eq!(results, [0, 0]);
}

#[test]
fn removed_final_exception_may_take_its_blank_separator() {
    let kept = hole("forgejo", "CVE-2026-4738");
    let removed = hole("ubuntu-image", "CVE-2026-109518");
    assert_eq!(
        proof(
            "final-separator",
            HOLES,
            &format!("{kept}\n{removed}"),
            &kept
        ),
        0
    );
}

#[test]
fn runner_executable_or_ambiguous_changes_stay_refused() {
    let before = runner(&"a".repeat(64));
    let after = runner(&"b".repeat(64));
    let cases = [
        after.replace("dell:", "other:"),
        after.replace("ubuntu@", "other@"),
        after.replace("false", "true"),
        after.replace("runner:", "runner: !!map"),
        after.replace("labels:", "labels: &alias"),
        format!("{after}runner: {{}}\n"),
        format!("{after}# changed\n"),
        runner("bad"),
        after.replace("labels:", "unknown:"),
    ];
    for (n, changed) in cases.iter().enumerate() {
        assert_eq!(
            proof(&format!("runner-bad-{n}"), RUNNER, &before, changed),
            1
        );
    }
}

#[test]
fn exception_edits_additions_and_malformed_content_stay_refused() {
    let kept = hole("forgejo", "CVE-2026-4738");
    let removed = hole("ubuntu-image", "CVE-2026-109518");
    let before = format!("{kept}\n{removed}");
    let cases = [
        format!("{before}\n{removed}"),
        kept.replace("2026-12-09", "2026-12-10"),
        kept.replace("130", "39"),
        kept.replace("4738", "9999"),
        format!("{kept}unknown = true\n"),
        format!("{kept}task = 130\n"),
        kept.replace("image =", "image = ["),
        format!("# changed\n{kept}"),
        kept.trim_end().to_owned(),
        kept.replace("image =", "image  ="),
    ];
    for (n, changed) in cases.iter().enumerate() {
        assert_eq!(
            proof(&format!("holes-bad-{n}"), HOLES, &before, changed),
            1
        );
    }
}
