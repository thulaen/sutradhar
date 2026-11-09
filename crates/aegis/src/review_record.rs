//! Reading and judging one review record (see `reviews`). A record is
//! read from git's own copy, never through a link; it must be a plain
//! file of at most 64 KiB, and text from it is cleaned before it is
//! shown.

use serde::Deserialize;

use super::{Entry, FOLDER, REQUIRED};
use crate::Finding;

const CHECK: &str = "reviews.required";
/// The largest record read, in bytes.
pub const LARGEST: usize = 64 * 1024;
const MIN_SUMMARY_WORDS: usize = 4;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Record {
    skill: String,
    change: String,
    verdict: String,
    agent: String,
    summary: String,
    #[serde(default)]
    finding: Vec<Note>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Note {
    what: String,
    fixed: bool,
}

/// Text from a record with control characters removed, so it cannot
/// rewrite the screen it is shown on.
fn clean(text: &str) -> String {
    let hidden = |c: &char| {
        matches!(
            *c,
            '\u{ad}'
                | '\u{200b}'..='\u{200f}'
                | '\u{202a}'..='\u{202e}'
                | '\u{2060}'..='\u{2064}'
                | '\u{2066}'..='\u{2069}'
                | '\u{feff}'
                | '\u{61c}'
                | '\u{2028}'..='\u{2029}'
                | '\u{fe00}'..='\u{fe0f}'
                | '\u{e0000}'..='\u{e007f}'
                | '\u{e0100}'..='\u{e01ef}'
        )
    };
    text.chars()
        .filter(|c| !c.is_control() && !hidden(c))
        .collect()
}

fn words(text: &str) -> usize {
    text.split_whitespace()
        .filter(|w| w.chars().any(char::is_alphanumeric))
        .count()
}

/// Why a record does not pass for `change`, if it does not.
fn judge(skill: &str, change: &str, record: &Record) -> Option<String> {
    let agent_ok = record.agent.chars().any(|c| c.is_ascii_alphanumeric())
        && record
            .agent
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || " -_".contains(c));
    let open = record.finding.iter().filter(|n| !n.fixed).count();
    let blank = record.finding.iter().any(|n| words(&n.what) == 0);
    if record.skill != skill {
        Some(format!(
            "the record says it is from `{}`",
            clean(&record.skill)
        ))
    } else if record.change != change {
        Some(format!(
            "the review was made for other content; this is {change}"
        ))
    } else if record.verdict != "pass" {
        Some(format!(
            "the review did not pass: `{}`",
            clean(&record.verdict)
        ))
    } else if !agent_ok || words(&record.summary) < MIN_SUMMARY_WORDS {
        Some(
            "the record must name its agent and say what was reviewed"
                .to_owned(),
        )
    } else if blank {
        Some("a finding does not say what was wrong".to_owned())
    } else if open > 0 {
        Some(format!("{open} finding(s) are not fixed"))
    } else {
        None
    }
}

/// Gives a path's content from git: `None` when it is not there, or why
/// it was not read.
pub type Reader<'a> = dyn Fn(&str) -> Option<Result<Vec<u8>, String>> + 'a;

/// Checks every required record. `entries` are the tree's files, which
/// show each record's mode; `read` gives a path's content from git.
#[must_use]
pub fn check(
    change: &str,
    entries: &[Entry],
    read: &Reader<'_>,
    at: &str,
) -> Vec<Finding> {
    const MOST_SHOWN: usize = 200;
    let mut out = Vec::new();
    for skill in REQUIRED {
        let path = format!("{FOLDER}{skill}.toml");
        let problem =
            |why: String| Finding::new(CHECK, &path, 0, format!("{at}: {why}"));
        let mode = entries
            .iter()
            .find(|(_, _, p)| p.as_slice() == path.as_bytes())
            .map(|(m, _, _)| m.as_str());
        let why = match (mode, read(&path)) {
            (None, _) | (_, None) => Some(format!(
                "no `{skill}` review; follow .claude/skills/{skill}/SKILL.md"
            )),
            (Some(m), _) if m != "100644" => {
                Some("the record must be a plain file".to_owned())
            }
            (_, Some(Err(why))) => Some(clean(&why)),
            (_, Some(Ok(bytes))) if bytes.len() > LARGEST => {
                Some("the record is too large".to_owned())
            }
            (_, Some(Ok(bytes))) => match std::str::from_utf8(&bytes)
                .map(toml::from_str::<Record>)
            {
                Ok(Ok(record)) => judge(skill, change, &record),
                Ok(Err(e)) => {
                    let why: String =
                        clean(e.message()).chars().take(MOST_SHOWN).collect();
                    Some(format!("the record cannot be read: {why}"))
                }
                Err(_) => Some("the record is not UTF-8 text".to_owned()),
            },
        };
        out.extend(why.map(problem));
    }
    out
}
