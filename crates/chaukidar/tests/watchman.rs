//! The alarm and the `chaukidar` command.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::cell::RefCell;
use std::process::Command;

use chaukidar::alarm::{TITLE, raise};
use forge_sync::fetch::Config;
use forge_sync::run::Run;

/// Answers `labels` and `milestones` calls, fails calls that contain
/// `fail`, and keeps every call.
struct Fake {
    fail: &'static str,
    calls: RefCell<Vec<String>>,
}

impl Run for Fake {
    fn run(
        &self,
        program: &str,
        args: &[&str],
        input: &[u8],
    ) -> Result<String, String> {
        let call = format!(
            "{program} {} <<{}",
            args.join(" "),
            String::from_utf8_lossy(input)
        );
        self.calls.borrow_mut().push(call.clone());
        if !self.fail.is_empty() && call.contains(self.fail) {
            return Err(format!("{} failed", self.fail));
        }
        if call.contains("page=2") {
            return Ok("[]".to_owned());
        }
        if call.contains("labels") {
            return Ok(r#"[{"id":4,"name":"status/urgent"}]"#.to_owned());
        }
        if call.contains("milestones") {
            return Ok(r#"[{"id":9,"title":"Phase 1: Home"}]"#.to_owned());
        }
        Ok("{}".to_owned())
    }
}

fn config() -> Config {
    Config {
        forgejo: "http://forgejo/api/v1/repos/o/r".to_owned(),
        header_file: "/h".to_owned(),
        github: "g/r".to_owned(),
        mirror: String::new(),
        record: String::new(),
        github_user: String::new(),
        gitleaks: String::new(),
        private: Vec::new(),
    }
}

#[test]
fn the_alarm_goes_to_forgejo_and_github() {
    let fake = Fake {
        fail: "",
        calls: RefCell::new(Vec::new()),
    };
    raise(&fake, &config(), &["`door-hook` has changed".to_owned()]).unwrap();
    let calls = fake.calls.borrow();
    assert!(
        calls
            .iter()
            .any(|c| c.contains("o/r/issues") && c.contains(TITLE))
    );
    assert!(calls.iter().any(|c| c.contains("POST repos/g/r/issues")));
    assert!(calls.iter().any(|c| c.contains("\"milestone\":9")));
}

#[test]
fn one_failed_place_still_tries_the_other() {
    for (fail, tried) in [("POST repos/g/r/issues", 2), ("o/r/labels", 1)] {
        let fake = Fake {
            fail,
            calls: RefCell::new(Vec::new()),
        };
        let got = raise(&fake, &config(), &["x".to_owned()]);
        assert!(got.unwrap_err().contains("failed"), "{fail}");
        assert_eq!(
            fake.calls
                .borrow()
                .iter()
                .filter(|c| c.contains(TITLE))
                .count(),
            tried,
            "{fail}"
        );
    }
}

fn temp(name: &str) -> String {
    std::env::temp_dir()
        .join(format!("chaukidar-cmd-{name}-{}", std::process::id()))
        .display()
        .to_string()
}

fn watchman(args: &[&str]) -> Option<i32> {
    Command::new(env!("CARGO_BIN_EXE_chaukidar"))
        .args(args)
        .env_remove("CHAUKIDAR_FORGEJO")
        .env_remove("CHAUKIDAR_HEADER")
        .env_remove("CHAUKIDAR_GITHUB")
        .output()
        .unwrap()
        .status
        .code()
}

/// A watched file, a list naming it, and where the sums and state go,
/// with the sums recorded by the command.
fn recorded(name: &str) -> (String, String, String, String) {
    let watched = temp(name);
    std::fs::write(&watched, "1\n").unwrap();
    let list = temp(&format!("{name}list"));
    std::fs::write(&list, format!("file thing {watched}\n")).unwrap();
    let sums = temp(&format!("{name}sums"));
    let state = temp(&format!("{name}state"));
    assert_eq!(watchman(&["record", &list, &sums]), Some(0));
    (watched, list, sums, state)
}

#[test]
fn the_command_records_checks_and_raises_the_alarm() {
    let (watched, list, sums, state) = recorded("cmd");
    assert_eq!(watchman(&["check", &list, &sums, &state]), Some(0));
    std::fs::write(&watched, "2\n").unwrap();
    assert_eq!(watchman(&["check", &list, &sums, &state]), Some(2));
    std::fs::write(&state, "`thing` has changed").unwrap();
    assert_eq!(watchman(&["check", &list, &sums, &state]), Some(1));
}

#[test]
fn the_command_refuses_bad_use() {
    assert_eq!(watchman(&[]), Some(2));
    assert_eq!(watchman(&["record", "/no/list", "/no/sums"]), Some(2));
    let list = temp("badlist");
    std::fs::write(&list, "nonsense\n").unwrap();
    assert_eq!(
        watchman(&["check", &list, "/no/sums", "/no/state"]),
        Some(2)
    );
}

#[test]
fn a_half_raised_alarm_is_not_repeated() {
    let (watched, list, sums, state) = recorded("half");
    std::fs::write(&watched, "2\n").unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_chaukidar"))
        .args(["check", &list, &sums, &state])
        .env("CHAUKIDAR_FORGEJO", "http://127.0.0.1:1/x")
        .env("CHAUKIDAR_HEADER", "/no/header")
        .env("CHAUKIDAR_GITHUB", "nobody/nothing")
        .env("PATH", "/nonexistent:/usr/bin:/bin")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(2));
    let kept = std::fs::read_to_string(&state).unwrap();
    assert_eq!(kept, "`thing` has changed");
}

#[test]
fn a_named_alarm_needs_its_settings_and_one_line() {
    assert_eq!(watchman(&["raise"]), Some(2));
    assert_eq!(watchman(&["raise", ""]), Some(2));
    assert_eq!(watchman(&["raise", "two\nlines"]), Some(2));
    assert_eq!(
        watchman(&["raise", "the noisy-neighbour test failed"]),
        Some(2)
    );
}

#[test]
fn a_named_alarm_carries_its_own_text() {
    let fake = Fake {
        fail: "",
        calls: RefCell::new(Vec::new()),
    };
    chaukidar::alarm::raise_text(&fake, &config(), "the night failed").unwrap();
    let calls = fake.calls.borrow();
    let posts: Vec<&String> =
        calls.iter().filter(|c| c.contains(TITLE)).collect();
    assert_eq!(posts.len(), 2);
    assert!(posts.iter().all(|c| c.contains("the night failed")));
    assert!(!posts.iter().any(|c| c.contains("outside an install")));
}
