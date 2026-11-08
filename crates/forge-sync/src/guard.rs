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
    .map_err(|e| format!("gitleaks did not pass the text going out: {e}"))
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
    let _out = run
        .run(
            &config.gitleaks,
            &["git", "--no-banner", "--redact", &log_opts, m],
            &[],
        )
        .map_err(|e| format!("gitleaks did not pass the new commits: {e}"))?;
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
        .map_err(|e| format!("gitleaks did not pass the tag messages: {e}"))?;
    if is_private(&[history, tags], &config.private) {
        return Err(
            "a new commit or tag looks private, so nothing was sent".to_owned()
        );
    }
    Ok(())
}
