//! A whole sync run when things go well.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

mod common;

use common::{Fake, ISSUES, MILESTONES, Pair, RELEASES, config};
use forge_sync::sync::once;

const MADE: Pair = ("POST repos/g/r/issues --input", Ok(r#"{"number":5}"#));

#[test]
fn copies_everything_github_lacks() {
    let fake = Fake::new(&[
        ("o/r/milestones", Ok(MILESTONES)),
        ("o/r/issues?state=all", Ok(ISSUES)),
        ("o/r/releases", Ok(RELEASES)),
        ("tag --merged main", Ok("v0.1.0\n")),
        MADE,
    ]);
    let (done, problems) = once(&fake, &config("copies"));
    assert_eq!(problems.len(), 0, "{problems:?}");
    assert_eq!(done.len(), 6, "{done:?}");
    assert!(fake.called("gitleaks stdin"));
    assert!(fake.called("gitleaks git --no-banner --redact --log-opts="));
    assert!(fake.called("git -C /m push --quiet github main refs/tags/v0.1.0"));
    assert!(fake.called("PATCH repos/g/r/issues/5"));
    assert!(fake.called("POST repos/g/r/releases"));
}

#[test]
fn updates_stale_copies() {
    let fake = Fake::new(&[
        ("o/r/milestones", Ok(MILESTONES)),
        ("o/r/issues?state=all", Ok(ISSUES)),
        (
            "api repos/g/r/milestones?state=all",
            Ok(r#"[{"number":2,"title":"Later","description":"old",
                    "state":"open"}]"#),
        ),
        (
            "api repos/g/r/issues?state=all",
            Ok("[{\"number\":40,\"title\":\"Task\",\
                 \"user\":{\"login\":\"owner\"},\
             \"body\":\"b\\n\\n<!-- forgejo-task: 3 -->\",\
             \"state\":\"open\"}]"),
        ),
    ]);
    let (_done, problems) = once(&fake, &config("updates"));
    assert_eq!(problems.len(), 0, "{problems:?}");
    assert!(fake.called("PATCH repos/g/r/milestones/2"));
    assert!(fake.called("PATCH repos/g/r/issues/40"));
    assert!(fake.called("\"milestone\":2"));
}

#[test]
fn brings_new_github_issues_and_comments_back() {
    let fake = Fake::new(&[
        ("o/r/milestones", Ok(MILESTONES)),
        (
            "o/r/issues?state=all",
            Ok(r#"[{"number":12,"title":"[From GitHub #8] Idea",
                "body":"x <!-- github-issue: 8 -->","state":"open"}]"#),
        ),
        (
            "api repos/g/r/issues?state=all",
            Ok(r#"[{"number":9,"title":"New","body":"x","state":"open",
                "labels":[],"user":{"login":"pat"}}]"#),
        ),
        (
            "issues/comments",
            Ok(r#"[{"id":70,"issue_url":"https://x/issues/8",
                "user":{"login":"pat"},"body":"more"}]"#),
        ),
    ]);
    let config = config("imports");
    let (done, problems) = once(&fake, &config);
    assert_eq!(problems.len(), 0, "{problems:?}");
    assert!(fake.called("[From GitHub #9] New"));
    assert!(fake.called("<!-- github-issue: 9 -->"));
    assert!(fake.called("repos/g/r/issues/9/labels"));
    assert!(fake.called("o/r/issues/12/comments"));
    assert_eq!(done.len(), 6, "{done:?}");
    let again = Fake::new(&fake.rules);
    let _ = once(&again, &config);
    assert!(!again.called("o/r/issues/12/comments"));
    assert!(!again.called("[From GitHub #9]"));
}

#[test]
fn the_record_stops_a_second_copy() {
    let config = config("record");
    let rules: Vec<Pair> = vec![
        ("o/r/milestones", Ok(MILESTONES)),
        ("o/r/issues?state=all", Ok(ISSUES)),
        MADE,
    ];
    let (_d, problems) = once(&Fake::new(&rules), &config);
    assert_eq!(problems.len(), 0, "{problems:?}");
    let again = Fake::new(&rules);
    let _ = once(&again, &config);
    assert!(!again.called("POST repos/g/r/issues --input"));
}

#[test]
fn a_release_is_made_only_for_a_tag_sent_to_github() {
    let fake = Fake::new(&[
        ("o/r/milestones", Ok(MILESTONES)),
        ("o/r/releases", Ok(RELEASES)),
        ("tag --merged main", Ok("")),
    ]);
    let (_done, problems) = once(&fake, &config("offmain"));
    assert_eq!(problems.len(), 0, "{problems:?}");
    assert!(!fake.called("POST repos/g/r/releases"));
}
