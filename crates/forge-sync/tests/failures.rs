//! A whole sync run when things go wrong: nothing is sent, and each new
//! problem raises one urgent task.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

mod common;

use common::{Fake, ISSUES, LABELS, MILESTONES, Pair, config};
use forge_sync::fetch::Config;
use forge_sync::plan::URGENT_TITLE;
use forge_sync::sync::{once, phase_number};

fn sent_nothing(fake: &Fake) -> bool {
    !fake.called("push --quiet") && !fake.called("api -X")
}

#[test]
fn a_failure_raises_one_urgent_task_in_the_first_phase() {
    let fake = Fake::new(&[
        ("fetch --quiet", Err("network down")),
        ("milestones?state=open", Ok(MILESTONES)),
        ("o/r/labels", Ok(LABELS)),
    ]);
    let config = config("urgent");
    let (_done, problems) = once(&fake, &config);
    assert_eq!(problems, ["network down"]);
    assert!(fake.called(URGENT_TITLE));
    assert!(fake.called("\"milestone\":9"));
    let again = Fake::new(&fake.rules);
    let _ = once(&again, &config);
    assert!(!again.called(URGENT_TITLE), "reported twice");
}

#[test]
fn a_known_urgent_task_gets_a_comment() {
    let open = Box::leak(
        format!(r#"[{{"number":12,"title":"{URGENT_TITLE}"}}]"#)
            .into_boxed_str(),
    );
    let fake = Fake::new(&[
        ("fetch --quiet", Err("refused")),
        ("issues?state=open", Ok(open)),
    ]);
    let (_done, problems) = once(&fake, &config("known"));
    assert_eq!(problems, ["refused"]);
    assert!(fake.called("o/r/issues/12/comments"));
}

#[test]
fn private_text_stops_everything() {
    let fake = Fake::new(&[
        ("o/r/milestones", Ok(MILESTONES)),
        (
            "o/r/issues?state=all",
            Ok(r#"[{"number":3,"title":"About MyForum","body":"b",
                    "state":"open"}]"#),
        ),
        ("o/r/labels", Ok(LABELS)),
    ]);
    let (_done, problems) = once(&fake, &config("private"));
    assert!(problems[0].contains("looks private"), "{problems:?}");
    assert!(sent_nothing(&fake));
}

#[test]
fn every_check_that_fails_stops_everything() {
    for (step, word) in [
        ("gitleaks stdin", "gitleaks did not pass the text"),
        ("gitleaks git", "gitleaks did not pass the new commits"),
        ("for-each-ref", "cannot read tags"),
    ] {
        let fake = Fake::new(&[
            (step, Err("cannot read tags")),
            ("o/r/milestones", Ok(MILESTONES)),
            ("o/r/labels", Ok(LABELS)),
        ]);
        let (_d, problems) = once(&fake, &config("checks"));
        assert!(problems[0].contains(word), "{step}: {problems:?}");
        assert!(sent_nothing(&fake), "{step}");
    }
    let leaky = Fake::new(&[
        ("log -p", Ok("by someone about MyForum")),
        ("o/r/milestones", Ok(MILESTONES)),
        ("o/r/labels", Ok(LABELS)),
    ]);
    let (_d, problems) = once(&leaky, &config("leaky"));
    assert!(problems[0].contains("commit or tag looks private"));
    assert!(sent_nothing(&leaky));
}

#[test]
fn an_empty_word_list_stops_everything() {
    let config = Config {
        private: vec![" ".to_owned()],
        ..config("empty")
    };
    let fake = Fake::new(&[("o/r/labels", Ok(LABELS))]);
    let (_d, problems) = once(&fake, &config);
    assert!(problems[0].contains("word list is empty"), "{problems:?}");
    assert!(!fake.called("fetch"));
}

#[test]
fn failed_writes_are_problems() {
    let import: Pair = (
        "api repos/g/r/issues?state=all",
        Ok(r#"[{"number":8,"title":"Idea","body":"x","state":"open",
            "labels":[],"user":{"login":"pat"}}]"#),
    );
    let made: Pair = ("POST repos/g/r/issues --input", Ok(r#"{"number":5}"#));
    let cases: [(Vec<Pair>, &str); 5] = [
        (
            vec![("POST repos/g/r/issues --input", Err("no create"))],
            "no create",
        ),
        (
            vec![("POST repos/g/r/issues --input", Ok("{}"))],
            "did not say",
        ),
        (
            vec![made, ("PATCH repos/g/r/issues/5", Err("no close"))],
            "no close",
        ),
        (
            vec![import, ("issues/8/labels", Err("no label"))],
            "no label",
        ),
        (
            vec![("POST repos/g/r/issues --input", Ok("nope"))],
            "not JSON",
        ),
    ];
    for (rules, want) in cases {
        let mut all: Vec<Pair> = vec![
            ("o/r/milestones", Ok(MILESTONES)),
            ("o/r/issues?state=all", Ok(ISSUES)),
        ];
        all.extend(rules);
        all.push(("o/r/labels", Ok(LABELS)));
        let (_d, problems) = once(&Fake::new(&all), &config("writes"));
        assert!(
            problems.iter().any(|p| p.contains(want)),
            "{want}: {problems:?}"
        );
    }
}

#[test]
fn missing_and_broken_pieces_are_problems() {
    let no_later = Fake::new(&[(
        "o/r/milestones",
        Ok(r#"[{"id":9,"title":"Phase 1","state":"open"}]"#),
    )]);
    let (_d, problems) = once(&no_later, &config("nolater"));
    assert!(problems[0].contains("no `Later`"), "{problems:?}");
    assert!(problems.iter().any(|p| p.contains("status/urgent")));
    let bad = Fake::new(&[("o/r/milestones", Ok("{}"))]);
    let (_d, problems) = once(&bad, &config("badjson"));
    assert!(problems[0].contains("not a JSON list"), "{problems:?}");
    let mut endless = Fake::new(&[("o/r/milestones", Ok(r#"[{"id":1}]"#))]);
    endless.endless = true;
    let (_d, problems) = once(&endless, &config("endless"));
    assert!(problems.iter().any(|p| p.contains("more than")));
}

#[test]
fn records_that_cannot_be_read_or_written_are_problems() {
    let damaged = config("damaged");
    std::fs::write(&damaged.record, "nope").unwrap();
    let (_d, problems) = once(&Fake::new(&[]), &damaged);
    assert!(problems[0].contains("damaged"), "{problems:?}");
    let nowhere = Config {
        record: "/no/such/folder/record".to_owned(),
        ..config("nowhere")
    };
    let fake = Fake::new(&[
        ("o/r/milestones", Ok(MILESTONES)),
        ("o/r/issues?state=all", Ok(ISSUES)),
        ("POST repos/g/r/issues --input", Ok(r#"{"number":5}"#)),
        ("o/r/labels", Ok(LABELS)),
    ]);
    let (_d, problems) = once(&fake, &nowhere);
    assert!(
        problems
            .iter()
            .any(|p| p.contains("cannot write the record"))
    );
}

#[test]
fn reads_phase_numbers() {
    assert_eq!(phase_number("Phase 10: Far"), Some(10));
    assert_eq!(phase_number("Later"), None);
    assert_eq!(phase_number("Phase x"), None);
}

#[test]
fn an_unreadable_record_stops_everything() {
    use std::os::unix::fs::PermissionsExt;
    let config = config("unreadable");
    std::fs::write(&config.record, "{}").unwrap();
    let locked = std::fs::Permissions::from_mode(0o000);
    std::fs::set_permissions(&config.record, locked).unwrap();
    let fake = Fake::new(&[]);
    let (_d, problems) = once(&fake, &config);
    assert!(
        problems[0].contains("cannot read the record"),
        "{problems:?}"
    );
    assert!(!fake.called("fetch"));
}

#[test]
fn a_problem_whose_alert_fails_is_raised_again() {
    let config = config("alert");
    let broken = Fake::new(&[
        ("fetch --quiet", Err("network down")),
        ("issues?state=open", Err("forgejo down")),
    ]);
    let (_d, problems) = once(&broken, &config);
    assert_eq!(problems, ["network down", "forgejo down"]);
    let fixed = Fake::new(&[
        ("fetch --quiet", Err("network down")),
        ("milestones?state=open", Ok(MILESTONES)),
        ("o/r/labels", Ok(LABELS)),
    ]);
    let _ = once(&fixed, &config);
    assert!(fixed.called(URGENT_TITLE));
}
