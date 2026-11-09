//! The checks run on everything before it leaves Dell: gitleaks, the
//! secrets check and the private words (AGENTS.md section 9.1). If a
//! check cannot run, nothing is sent.

use crate::fetch::Config;
use crate::plan::is_private;
use crate::run::Run;

/// Checks text that would go to GitHub.
///
/// # Errors
/// When the text looks private or gitleaks finds a secret, or cannot run.
pub fn text(
    run: &dyn Run,
    config: &Config,
    texts: &[String],
) -> Result<(), String> {
    if is_private(texts, &config.private) {
        return Err(
            "text going out looks private, so nothing was sent".to_owned()
        );
    }
    let all = texts.join("\n");
    run.run(
        &config.gitleaks,
        &["stdin", "--no-banner", "--redact"],
        all.as_bytes(),
    )
    .map(|_| ())
    .map_err(|e| {
        format!("gitleaks did not pass the text going out: {}", steady(&e))
    })
}

/// The tool's report without its colours, clock times and byte counts,
/// so the same problem reads the same on every run and is reported in
/// the urgent task once, not once a minute.
#[must_use]
pub fn steady(report: &str) -> String {
    strip_colours(report)
        .lines()
        .map(|l| {
            // A count of what was scanned changes every run; the words
            // before it, such as `... failed:`, stay.
            let l = l.find("failed:").map_or(l, |at| {
                if l.contains("scanned") {
                    l.get(..at.saturating_add(7)).unwrap_or(l)
                } else {
                    l
                }
            });
            let words: Vec<&str> =
                l.split_whitespace().filter(|w| !is_clock(w)).collect();
            if words.iter().any(|w| w.contains("scanned")) {
                String::new()
            } else {
                words.join(" ")
            }
        })
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// The text without terminal colour codes.
fn strip_colours(report: &str) -> String {
    let mut plain = String::new();
    let mut in_code = false;
    for c in report.chars() {
        if c == '\u{1b}' {
            in_code = true;
        } else if in_code {
            in_code = !c.is_ascii_alphabetic();
        } else {
            plain.push(c);
        }
    }
    plain
}

fn is_clock(word: &str) -> bool {
    let time = word.trim_end_matches("AM").trim_end_matches("PM");
    time.len() < word.len()
        && time
            .split(':')
            .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
}

/// Runs gitleaks over the new commits.
fn scan_commits(
    run: &dyn Run,
    config: &Config,
    log_opts: &str,
) -> Result<(), String> {
    let m = config.mirror.as_str();
    let ignore = ignore_list(run, config)?;
    run.run(
        &config.gitleaks,
        &[
            "git",
            "--no-banner",
            "--redact",
            "--ignore-gitleaks-allow",
            &format!("--gitleaks-ignore-path={ignore}"),
            log_opts,
            m,
        ],
        &[],
    )
    .map_err(|e| {
        format!("gitleaks did not pass the new commits: {}", steady(&e))
    })
    .map(|_| ())
}

/// The repository's reviewed list of false alarms (`.gitleaksignore` on
/// `main`), saved next to the record for gitleaks to read. An empty list
/// when `main` has none.
fn ignore_list(run: &dyn Run, config: &Config) -> Result<String, String> {
    let m = config.mirror.as_str();
    let list = run
        .run("git", &["-C", m, "show", "main:.gitleaksignore"], &[])
        .unwrap_or_default();
    let path = format!("{}.gitleaksignore", config.record);
    std::fs::write(&path, list)
        .map_err(|e| format!("cannot save the list of false alarms: {e}"))?;
    Ok(path)
}

/// Checks every commit and tag that would go to GitHub: what GitHub's
/// `main` lacks, or the whole history if GitHub has none.
///
/// # Errors
/// When anything looks private, or a check cannot run.
pub fn code(run: &dyn Run, config: &Config) -> Result<(), String> {
    let m = config.mirror.as_str();
    let theirs = run.run(
        "git",
        &["-C", m, "ls-remote", "github", "refs/heads/main"],
        &[],
    )?;
    let known = theirs.split_whitespace().next().filter(|id| {
        run.run("git", &["-C", m, "cat-file", "-e", id], &[])
            .is_ok()
    });
    let range =
        known.map_or_else(|| "main".to_owned(), |id| format!("{id}..main"));
    let log_opts = format!("--log-opts=--diff-merges=first-parent {range}");
    scan_commits(run, config, &log_opts)?;
    let history = run.run(
        "git",
        &[
            "-C",
            m,
            "log",
            "-p",
            "--diff-merges=first-parent",
            "--format=%an %ae %cn %ce%n%B",
            "--end-of-options",
            &range,
        ],
        &[],
    )?;
    let tags = run.run(
        "git",
        &[
            "-C",
            m,
            "for-each-ref",
            "refs/tags",
            "--format=%(refname) %(taggername) %(taggeremail) %(contents)",
        ],
        &[],
    )?;
    let _scanned = run
        .run(
            &config.gitleaks,
            &["stdin", "--no-banner", "--redact"],
            tags.as_bytes(),
        )
        .map_err(|e| {
            format!("gitleaks did not pass the tag messages: {}", steady(&e))
        })?;
    if is_private(&[history, tags], &config.private) {
        return Err(
            "a new commit or tag looks private, so nothing was sent".to_owned()
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::steady;

    #[test]
    fn the_same_problem_reads_the_same_on_every_run() {
        let at = |time: &str, bytes: &str| {
            format!(
                "/t/gitleaks git failed: \u{1b}[90m{time}\u{1b}[0m \
                 \u{1b}[32mINF\u{1b}[0m \u{1b}[1m4 commits scanned.\u{1b}[0m\n\
                 \u{1b}[90m{time}\u{1b}[0m INF scanned ~{bytes} bytes\n\
                 \u{1b}[90m{time}\u{1b}[0m \u{1b}[33mWRN\u{1b}[0m \
                 leaks found: 2\n"
            )
        };
        let first = steady(&at("1:14AM", "18730"));
        assert_eq!(first, steady(&at("12:42PM", "83804")));
        assert_eq!(first, "/t/gitleaks git failed: WRN leaks found: 2");
    }
}
