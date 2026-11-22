//! The sync watch. After a push, the sync must copy Forgejo's `main` to
//! GitHub (AGENTS.md section 8, step 11). Each check reads both heads.
//! The alarm is raised once when GitHub's head differs from Forgejo's, or
//! cannot be read, and has not moved for longer than the grace: a sync
//! that keeps copying, even behind steady pushes, never raises it.

use serde_json::Value;

use forge_sync::fetch::Config;
use forge_sync::run::Run;

/// How long GitHub's head may stay still while it differs from Forgejo's
/// before the alarm (proposed). The sync runs every minute and the
/// watchman every five.
pub const GRACE_SECONDS: u64 = 600;

/// A clock whose last check is older than this (proposed) was left by a
/// stopped timer or a machine that was off. Its waiting time starts
/// again rather than counting the gap as lag.
pub const STALE_SECONDS: u64 = 900;

/// The longest one read of a head may take (proposed), so a hung site
/// cannot hold up the watchman for long.
pub const READ_SECONDS: &str = "20";

/// How many digits of a commit id the alarm shows.
const SHORT_ID: usize = 12;

/// The clock's word for a GitHub head that could not be read.
const UNREADABLE: &str = "unreadable";

/// The head of `main` on Forgejo and on GitHub, each as a full commit id
/// or the reason it could not be read.
pub fn heads(
    run: &dyn Run,
    config: &Config,
) -> (Result<String, String>, Result<String, String>) {
    let header = format!("@{}", config.header_file);
    let forgejo_url = format!("{}/branches/main", config.forgejo);
    let forgejo = run
        .run(
            "curl",
            &[
                "-sS",
                "--fail",
                "--max-time",
                READ_SECONDS,
                "-H",
                &header,
                &forgejo_url,
            ],
            &[],
        )
        .and_then(|text| commit(&text, "id"));
    let github_url = format!("repos/{}/branches/main", config.github);
    let github = run
        .run("timeout", &[READ_SECONDS, "gh", "api", &github_url], &[])
        .and_then(|text| commit(&text, "sha"));
    (forgejo, github)
}

fn commit(text: &str, key: &str) -> Result<String, String> {
    let value: Value = serde_json::from_str(text)
        .map_err(|_| "the branch reply is not JSON".to_owned())?;
    let id = value
        .get("commit")
        .and_then(|c| c.get(key))
        .and_then(Value::as_str)
        .unwrap_or_default();
    // Git commit ids are 40 hex digits, or 64 in a SHA-256 repository.
    if matches!(id.len(), 40 | 64) && id.bytes().all(|b| b.is_ascii_hexdigit())
    {
        Ok(id.to_owned())
    } else {
        Err("the branch reply holds no commit id".to_owned())
    }
}

/// What the watch remembers between checks, in `STATE.sync`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clock {
    /// GitHub's head when it last moved, or `unreadable`.
    pub github: String,
    /// When GitHub's head was last seen to move, in seconds.
    pub since: u64,
    /// When the last check ran, in seconds.
    pub last: u64,
    /// Whether the alarm has been raised for this lag.
    pub raised: bool,
}

impl Clock {
    /// Reads a clock written by [`Clock::render`]; anything else is none.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        let mut parts = text.split_whitespace();
        let github = parts.next()?.to_owned();
        let since = parts.next()?.parse().ok()?;
        let last = parts.next()?.parse().ok()?;
        let raised = match parts.next()? {
            "raised" => true,
            "waiting" => false,
            _ => return None,
        };
        parts.next().is_none().then_some(Self {
            github,
            since,
            last,
            raised,
        })
    }

    /// One line for `STATE.sync`.
    #[must_use]
    pub fn render(&self) -> String {
        let word = if self.raised { "raised" } else { "waiting" };
        format!("{} {} {} {word}\n", self.github, self.since, self.last)
    }
}

/// Decides one check at time `now`. Gives the clock to keep (`None` once
/// the heads match) and the alarm's text when it is due. Only GitHub's
/// side can be stuck: when Forgejo cannot be read, nothing is judged and
/// the clock is kept as it was. The alarm is due once per lag: GitHub's
/// head moving, or the heads matching, starts over, and a clock that is
/// from the future or stale restarts its waiting time but remembers that
/// the alarm was raised for the same GitHub head.
#[must_use]
pub fn judge(
    forgejo: &Result<String, String>,
    github: &Result<String, String>,
    clock: Option<Clock>,
    now: u64,
) -> (Option<Clock>, Option<String>) {
    let Ok(f) = forgejo else {
        return (clock, None);
    };
    if github.as_ref().is_ok_and(|g| g == f) {
        return (None, None);
    }
    let seen = github.as_ref().ok();
    let head = seen.cloned().unwrap_or_else(|| UNREADABLE.to_owned());
    let mut next = match clock {
        // GitHub could not be read before and now can: the same lag,
        // now with its head known.
        Some(c) if c.github == UNREADABLE && seen.is_some() => {
            Clock { github: head, ..c }
        }
        // GitHub's head moved: a new lag, if any.
        Some(c) if seen.is_some_and(|g| *g != c.github) => fresh(head, now),
        Some(c) => {
            let fresh_time = c.since > now
                || c.last > now
                || now.saturating_sub(c.last) > STALE_SECONDS;
            if fresh_time {
                Clock { since: now, ..c }
            } else {
                c
            }
        }
        None => fresh(head, now),
    };
    next.last = now;
    if next.raised || now.saturating_sub(next.since) < GRACE_SECONDS {
        return (Some(next), None);
    }
    next.raised = true;
    let what = if github.is_ok() {
        "has not moved while it differs from Forgejo's"
    } else {
        "could not be read"
    };
    let body = format!(
        "GitHub's `main` {what} for over {} minutes (Forgejo {}, GitHub \
         {}).\n\nRead \
         forge-sync's log on Dell (`journalctl -u forge-sync`); if the \
         sync refused a push, its own urgent task says why. A sync paused \
         for its campaign (AGENTS.md section 9.1) is not a failure: say so \
         here and close this task.",
        GRACE_SECONDS / 60,
        short(forgejo),
        short(github)
    );
    (Some(next), Some(body))
}

fn fresh(github: String, now: u64) -> Clock {
    Clock {
        github,
        since: now,
        last: now,
        raised: false,
    }
}

fn short(head: &Result<String, String>) -> String {
    match head {
        Ok(id) => id.chars().take(SHORT_ID).collect(),
        Err(_) => UNREADABLE.to_owned(),
    }
}
