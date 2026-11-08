//! One sync run. Everything is read and checked first; only if every
//! check passes is anything sent. Any new problem raises one urgent task
//! in Forgejo.

use serde_json::{Value, json};

use crate::apply::{Target, apply};
use crate::fetch::{self, Config, forgejo_list};
use crate::guard;
use crate::plan::{Action, Rules, URGENT_TITLE, outgoing, plan};
use crate::record::Record;
use crate::run::Run;

/// Runs the sync once. Returns the changes made and the problems found.
#[must_use]
pub fn once(run: &dyn Run, config: &Config) -> (Vec<String>, Vec<String>) {
    let mut record = match Record::load(&config.record) {
        Ok(r) => r,
        Err(e) => return (Vec::new(), vec![e]),
    };
    let (done, mut problems) = match checked_run(run, config, &mut record) {
        Ok(done) => (done, Vec::new()),
        Err(failure) => failure,
    };
    let new: Vec<String> = problems
        .iter()
        .filter(|p| !record.reported.contains(p))
        .cloned()
        .collect();
    let raised = new.is_empty() || {
        match raise_urgent(run, config, &new) {
            Ok(()) => true,
            Err(e) => {
                problems.push(e);
                false
            }
        }
    };
    if raised {
        record.reported.clone_from(&problems);
    }
    if let Err(e) = record.save(&config.record) {
        problems.push(e);
    }
    (done, problems)
}

type Failure = (Vec<String>, Vec<String>);

fn checked_run(
    run: &dyn Run,
    config: &Config,
    record: &mut Record,
) -> Result<Vec<String>, Failure> {
    let fail = |e: String| (Vec::new(), vec![e]);
    if config.private.iter().all(|w| w.trim().is_empty()) {
        return Err(fail(
            "the private word list is empty, so nothing was sent".to_owned(),
        ));
    }
    fetch::fetch_code(run, config).map_err(fail)?;
    let sides = fetch::sides(run, config).map_err(fail)?;
    let rules = Rules {
        github_user: &config.github_user,
        private: &config.private,
        record,
    };
    let actions = plan(&sides, rules).map_err(|p| (Vec::new(), p))?;
    let texts: Vec<String> = actions.iter().flat_map(outgoing).collect();
    guard::text(run, config, &texts).map_err(fail)?;
    guard::code(run, config).map_err(fail)?;
    let sent = fetch::push_code(run, config).map_err(fail)?;
    // A release may name only a tag that is now on GitHub.
    let actions: Vec<Action> = actions
        .into_iter()
        .filter(|a| {
            !matches!(a, Action::CreateRelease(r) if !sent.contains(&r.tag))
        })
        .collect();
    let later = sides
        .forgejo_milestones
        .iter()
        .find(|m| m.title == "Later")
        .map(|m| m.number)
        .ok_or_else(|| fail("Forgejo has no `Later` milestone".to_owned()))?;
    let mut target = Target {
        run,
        config,
        later,
        milestones: sides
            .github_milestones
            .iter()
            .map(|m| (m.title.clone(), m.number))
            .collect(),
        record,
    };
    let (mut done, problems) = apply(&mut target, &actions);
    done.insert(0, "copied main and its tags".to_owned());
    if problems.is_empty() {
        Ok(done)
    } else {
        Err((done, problems))
    }
}

fn raise_urgent(
    run: &dyn Run,
    config: &Config,
    problems: &[String],
) -> Result<(), String> {
    let text = format!(
        "The sync to GitHub found these problems:\n\n- {}\n",
        problems.join("\n- ")
    );
    let open: Value = serde_json::from_str(&forgejo_list(
        run,
        config,
        "issues?state=open&type=issues",
    )?)
    .map_err(|e| e.to_string())?;
    let existing = open.as_array().and_then(|all| {
        all.iter()
            .find(|i| {
                i.get("title").and_then(Value::as_str) == Some(URGENT_TITLE)
            })
            .and_then(|i| i.get("number").and_then(Value::as_u64))
    });
    if let Some(n) = existing {
        return post(
            run,
            config,
            &format!("issues/{n}/comments"),
            &json!({"body": text}),
        );
    }
    let (label, milestone) = urgent_home(run, config)?;
    let body = json!({"title": URGENT_TITLE, "body": text,
        "labels": [label], "milestone": milestone});
    post(run, config, "issues", &body)
}

/// The `status/urgent` label and the current phase: the open phase
/// milestone with the lowest number.
///
/// # Errors
/// When Forgejo cannot be read, or has no such label or phase.
pub fn urgent_home(
    run: &dyn Run,
    config: &Config,
) -> Result<(u64, u64), String> {
    let labels: Value =
        serde_json::from_str(&forgejo_list(run, config, "labels?")?)
            .map_err(|e| e.to_string())?;
    let label = labels
        .as_array()
        .and_then(|all| {
            all.iter()
                .find(|l| {
                    l.get("name").and_then(Value::as_str)
                        == Some("status/urgent")
                })
                .and_then(|l| l.get("id").and_then(Value::as_u64))
        })
        .ok_or_else(|| "Forgejo has no `status/urgent` label".to_owned())?;
    let open = crate::model::milestones(
        &forgejo_list(run, config, "milestones?state=open")?,
        "id",
    )?;
    let phase = open
        .iter()
        .filter_map(|m| phase_number(&m.title).map(|p| (p, m.number)))
        .min()
        .map(|(_, id)| id)
        .ok_or_else(|| "Forgejo has no open phase".to_owned())?;
    Ok((label, phase))
}

/// The number in a title such as `Phase 10: Something`.
#[must_use]
pub fn phase_number(title: &str) -> Option<u64> {
    let rest = title.strip_prefix("Phase ")?;
    let digits: String =
        rest.chars().take_while(char::is_ascii_digit).collect();
    digits.parse().ok()
}

fn post(
    run: &dyn Run,
    config: &Config,
    path: &str,
    body: &Value,
) -> Result<(), String> {
    fetch::forgejo_post(run, config, path, body)
}
