//! Raising the alarm: an urgent task in Forgejo and an alert issue on
//! GitHub, which the sync never brings back (AGENTS.md section 11).

use serde_json::json;

use forge_sync::fetch::{Config, forgejo_post};
use forge_sync::run::Run;
use forge_sync::sync::urgent_home;

/// The title of the watchman's alarms, which the sync never copies.
pub const TITLE: &str = forge_sync::plan::ALARM_TITLE;

/// Raises the alarm for these changes, in both places. Both are tried
/// even if the first fails.
///
/// # Errors
/// Lists every place the alarm could not be raised.
pub fn raise(
    run: &dyn Run,
    config: &Config,
    changes: &[String],
) -> Result<(), String> {
    let text = format!(
        "These changed outside an install:\n\n- {}\n\nCheck who changed \
         them and why before any further push.",
        changes.join("\n- ")
    );
    let forgejo = urgent_home(run, config).and_then(|(label, milestone)| {
        let body = json!({"title": TITLE, "body": text,
            "labels": [label], "milestone": milestone});
        forgejo_post(run, config, "issues", &body)
    });
    let url = format!("repos/{}/issues", config.github);
    let body = json!({"title": TITLE, "body": text}).to_string();
    let github = run
        .run(
            "gh",
            &["api", "-X", "POST", &url, "--input", "-"],
            body.as_bytes(),
        )
        .map(|_| ());
    let failed: Vec<String> = [forgejo, github]
        .into_iter()
        .filter_map(Result::err)
        .collect();
    if failed.is_empty() {
        Ok(())
    } else {
        Err(failed.join("; "))
    }
}
