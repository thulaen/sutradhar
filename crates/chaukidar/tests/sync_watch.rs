//! The sync watch: GitHub's `main` must keep following Forgejo's.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use chaukidar::sync_watch::{
    Clock, GRACE_SECONDS, STALE_SECONDS, heads, judge,
};
use forge_sync::fetch::Config;
use forge_sync::run::Run;

/// Answers the two branch reads with fixed heads; `None` fails the read.
struct Fake {
    forgejo: Option<&'static str>,
    github: Option<&'static str>,
}

impl Run for Fake {
    fn run(
        &self,
        program: &str,
        args: &[&str],
        _input: &[u8],
    ) -> Result<String, String> {
        let call = format!("{program} {}", args.join(" "));
        let (head, key) = if call.contains("o/r/branches/main") {
            assert!(call.contains("--max-time"), "{call}");
            (self.forgejo, "id")
        } else if call.contains("repos/g/r/branches/main") {
            assert!(call.starts_with("timeout "), "{call}");
            (self.github, "sha")
        } else {
            return Err(format!("unexpected call {call}"));
        };
        head.map(|h| format!(r#"{{"commit":{{"{key}":"{h}"}}}}"#))
            .ok_or_else(|| "down".to_owned())
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

const A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const C: &str = "cccccccccccccccccccccccccccccccccccccccc";
const STEP: u64 = 300;

fn read(forgejo: Option<&'static str>, github: Option<&'static str>) -> Heads {
    heads(&Fake { forgejo, github }, &config())
}

type Heads = (Result<String, String>, Result<String, String>);

/// Runs checks five minutes apart from `start`, carrying the clock, and
/// gives each check's alarm text.
fn run(
    reads: &[Heads],
    clock: Option<Clock>,
    start: u64,
) -> Vec<Option<String>> {
    let mut clock = clock;
    let mut out = Vec::new();
    for (n, (f, g)) in reads.iter().enumerate() {
        let now = STEP
            .checked_mul(u64::try_from(n).unwrap())
            .and_then(|t| t.checked_add(start))
            .unwrap();
        let (next, body) = judge(f, g, clock, now);
        clock = next;
        out.push(body);
    }
    out
}

#[test]
fn matching_heads_clear_the_clock() {
    let (f, g) = read(Some(A), Some(A));
    let old = Clock {
        github: B.to_owned(),
        since: 5,
        last: 9_000,
        raised: true,
    };
    assert_eq!(judge(&f, &g, Some(old), 10_000), (None, None));
}

#[test]
fn a_sync_that_keeps_following_steady_pushes_never_raises() {
    // Forgejo is always one push ahead, but GitHub moves every check.
    let reads = [
        read(Some(B), Some(A)),
        read(Some(C), Some(B)),
        read(Some(A), Some(C)),
        read(Some(B), Some(A)),
        read(Some(C), Some(B)),
    ];
    assert!(run(&reads, None, 1_000).iter().all(Option::is_none));
}

#[test]
fn a_stuck_sync_raises_once_after_the_grace() {
    let stuck = read(Some(B), Some(A));
    let reads = vec![stuck.clone(); 6];
    let alarms = run(&reads, None, 1_000);
    let due = usize::try_from(GRACE_SECONDS / STEP).unwrap();
    for (n, alarm) in alarms.iter().enumerate() {
        assert_eq!(alarm.is_some(), n == due, "check {n}");
    }
    let text = alarms[due].as_ref().unwrap();
    assert!(text.contains("bbbbbbbbbbbb") && text.contains("aaaaaaaaaaaa"));
    assert!(text.contains("campaign"), "{text}");
    assert!(text.contains("differs"), "{text}");
    // GitHub coming back with the same stuck head is the same lag.
    let reads = [
        read(Some(B), None),
        read(Some(B), None),
        read(Some(B), None),
        read(Some(B), Some(A)),
        read(Some(B), Some(A)),
    ];
    let alarms = run(&reads, None, 1_000);
    assert_eq!(alarms.iter().filter(|a| a.is_some()).count(), 1);
    assert!(alarms[2].as_ref().unwrap().contains("could not be read"));
    assert!(!text.contains("outside an install"), "{text}");
}

#[test]
fn a_github_read_that_comes_and_goes_does_not_repeat_the_alarm() {
    let up = read(Some(B), Some(A));
    let down = read(Some(B), None);
    let reads = [up.clone(), down.clone(), up.clone(), down, up];
    let raised = run(&reads, None, 1_000);
    assert_eq!(raised.iter().filter(|a| a.is_some()).count(), 1);
}

#[test]
fn an_unreadable_side_counts_as_a_lag() {
    let reads = vec![read(Some(A), None); 3];
    let alarms = run(&reads, None, 1_000);
    assert!(alarms[2].as_ref().unwrap().contains("unreadable"));
}

#[test]
fn a_clock_from_the_future_or_a_stopped_timer_starts_over() {
    let (f, g) = read(Some(B), Some(A));
    let now: u64 = 100_000;
    for clock in [
        Clock {
            github: A.to_owned(),
            since: 150_000,
            last: 150_000,
            raised: false,
        },
        Clock {
            github: A.to_owned(),
            since: 1,
            last: now
                .checked_sub(STALE_SECONDS)
                .and_then(|t| t.checked_sub(1))
                .unwrap(),
            raised: false,
        },
    ] {
        let (kept, body) = judge(&f, &g, Some(clock), now);
        assert!(body.is_none());
        assert_eq!(kept.unwrap().since, now);
    }
}

#[test]
fn the_clock_reads_back_what_it_wrote() {
    let clock = Clock {
        github: A.to_owned(),
        since: 7,
        last: 9,
        raised: true,
    };
    assert_eq!(Clock::parse(&clock.render()), Some(clock));
    for bad in ["", "a 1 2", "a 1 2 maybe", "a x 2 raised", "a 1 2 raised x"] {
        assert_eq!(Clock::parse(bad), None, "{bad}");
    }
}

#[test]
fn a_head_that_is_not_a_commit_id_is_unreadable() {
    let (f, _g) = read(Some("not-a-sha"), Some(A));
    assert!(f.is_err());
    let sha256 = "d".repeat(64);
    let leaked: &'static str = Box::leak(sha256.into_boxed_str());
    let (f, _g) = read(Some(leaked), Some(A));
    assert_eq!(f.unwrap().len(), 64);
}

#[test]
fn the_check_logs_an_unreadable_forgejo_and_passes_an_unchanged_list() {
    let dir = std::env::temp_dir()
        .join(format!("chaukidar-clock-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let watched = dir.join("thing");
    std::fs::write(&watched, "1\n").unwrap();
    let list = dir.join("list");
    std::fs::write(&list, format!("file thing {}\n", watched.display()))
        .unwrap();
    let sums = dir.join("sums");
    let state = dir.join("state");
    let me = env!("CARGO_BIN_EXE_chaukidar");
    let status = std::process::Command::new(me)
        .arg("record")
        .args([&list, &sums])
        .status()
        .unwrap();
    assert!(status.success());
    // Neither site answers. Forgejo cannot be read, so nothing is
    // judged, the reason is logged, and the unchanged watch list passes.
    let out = std::process::Command::new(me)
        .arg("check")
        .args([&list, &sums, &state])
        .env("CHAUKIDAR_FORGEJO", "http://127.0.0.1:1/x")
        .env("CHAUKIDAR_HEADER", "/no/header")
        .env("CHAUKIDAR_GITHUB", "nobody/nothing")
        .env("GH_HOST", "127.0.0.1:1")
        .env("GH_TOKEN", "none")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0));
    let clock = std::fs::read_to_string(dir.join("state.sync")).unwrap();
    assert_eq!(clock, "");
    let said = String::from_utf8_lossy(&out.stderr);
    assert!(said.contains("cannot read Forgejo's main"), "{said}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn an_unreadable_forgejo_judges_nothing_and_keeps_the_clock() {
    let (f, g) = read(None, Some(A));
    let kept = Clock {
        github: A.to_owned(),
        since: 1_000,
        last: 1_300,
        raised: false,
    };
    let late = 1_000 + GRACE_SECONDS * 3;
    assert_eq!(judge(&f, &g, Some(kept.clone()), late), (Some(kept), None));
    assert_eq!(judge(&f, &g, None, late), (None, None));
}

#[test]
fn a_restarted_clock_remembers_the_alarm_for_the_same_head() {
    let (f, g) = read(Some(B), Some(A));
    let now: u64 = 100_000;
    let raised = Clock {
        github: A.to_owned(),
        since: 1,
        last: now.checked_sub(STALE_SECONDS * 2).unwrap(),
        raised: true,
    };
    let (kept, body) = judge(&f, &g, Some(raised), now);
    assert!(body.is_none());
    let kept = kept.unwrap();
    assert!(kept.raised);
    let later = now.checked_add(GRACE_SECONDS * 2).unwrap();
    assert_eq!(judge(&f, &g, Some(kept), later).1, None);
}

#[test]
fn the_sync_alarm_has_its_own_title_that_the_sync_never_copies() {
    let title = forge_sync::plan::SYNC_ALARM_TITLE;
    assert_ne!(title, chaukidar::alarm::TITLE);
    assert!(forge_sync::plan::is_local_only(title));
    let fake = Fake {
        forgejo: None,
        github: None,
    };
    let refused =
        chaukidar::alarm::raise_titled(&fake, &config(), "copied", "x");
    assert!(refused.unwrap_err().contains("would copy"));
}
