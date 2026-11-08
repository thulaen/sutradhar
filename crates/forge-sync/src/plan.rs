//! Working out what to change on GitHub, and what to bring back, from
//! what each side holds. Nothing here talks to the network.

use crate::model::{Comment, Issue, Milestone, Release};
use crate::record::Record;

/// The hidden note at the end of each GitHub copy, naming its task.
pub const MARKER: &str = "<!-- forgejo-task: ";
/// The hidden note on a Forgejo task brought in from GitHub.
pub const FROM_GITHUB: &str = "<!-- github-issue: ";
/// The label put on a GitHub issue once it is brought into Forgejo.
pub const COPIED: &str = "copied-to-forgejo";
/// The title of the sync's own urgent task, which is never copied.
pub const URGENT_TITLE: &str = "forge-sync: the copy to GitHub had problems";

/// One change to make.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Make a milestone on GitHub.
    CreateMilestone(Milestone),
    /// Bring a GitHub milestone, by its GitHub number, up to date.
    UpdateMilestone(u64, Milestone),
    /// Make a copy of a task on GitHub.
    CreateIssue(Issue),
    /// Bring a GitHub copy, by its GitHub number, up to date.
    UpdateIssue(u64, Issue),
    /// Make a release on GitHub.
    CreateRelease(Release),
    /// Bring a GitHub issue into Forgejo.
    Import(Issue),
    /// Bring a GitHub comment into the given Forgejo task.
    ImportComment(u64, Comment),
}

/// What each side holds.
#[derive(Debug, Default)]
pub struct Sides {
    /// Forgejo's milestones.
    pub forgejo_milestones: Vec<Milestone>,
    /// GitHub's milestones.
    pub github_milestones: Vec<Milestone>,
    /// Forgejo's tasks.
    pub forgejo_issues: Vec<Issue>,
    /// GitHub's issues.
    pub github_issues: Vec<Issue>,
    /// GitHub's issue comments.
    pub github_comments: Vec<Comment>,
    /// Forgejo's releases.
    pub forgejo_releases: Vec<Release>,
    /// GitHub's releases.
    pub github_releases: Vec<Release>,
}

/// Who and what the plan must trust or hold back.
#[derive(Debug, Clone, Copy)]
pub struct Rules<'a> {
    /// The sync's own GitHub account; only its issues can be copies.
    pub github_user: &'a str,
    /// Words that must never leave Dell.
    pub private: &'a [String],
    /// What the sync has done before.
    pub record: &'a Record,
}

/// Every change needed, or, if anything that would leave looks private,
/// why nothing may be sent this run.
///
/// # Errors
/// Lists each private item; then no change may be made at all.
pub fn plan(
    sides: &Sides,
    rules: Rules<'_>,
) -> Result<Vec<Action>, Vec<String>> {
    let mut out = Vec::new();
    for m in &sides.forgejo_milestones {
        out.extend(milestone(m, &sides.github_milestones));
    }
    for i in sides.forgejo_issues.iter().filter(|i| belongs_on_github(i)) {
        out.extend(issue(i, sides, rules));
    }
    for r in &sides.forgejo_releases {
        if !sides.github_releases.iter().any(|g| g.tag == r.tag) {
            out.push(Action::CreateRelease(r.clone()));
        }
    }
    out.extend(imports(sides, rules));
    let blocked: Vec<String> = out
        .iter()
        .filter(|a| is_private(&outgoing(a), rules.private))
        .map(|a| format!("{} looks private, so nothing was sent", name(a)))
        .collect();
    if blocked.is_empty() {
        Ok(out)
    } else {
        Err(blocked)
    }
}

/// Tasks that came from GitHub, and the sync's urgent task, stay on
/// Forgejo.
fn belongs_on_github(i: &Issue) -> bool {
    i.title != URGENT_TITLE && !i.body.contains(FROM_GITHUB)
}

fn milestone(m: &Milestone, github: &[Milestone]) -> Option<Action> {
    match github.iter().find(|g| g.title == m.title) {
        None => Some(Action::CreateMilestone(m.clone())),
        Some(g) if g.description != m.description || g.state != m.state => {
            Some(Action::UpdateMilestone(g.number, m.clone()))
        }
        Some(_) => None,
    }
}

fn issue(i: &Issue, sides: &Sides, rules: Rules<'_>) -> Option<Action> {
    let copy = Issue {
        body: format!("{}\n\n{MARKER}{} -->", i.body, i.number),
        ..i.clone()
    };
    let found = sides
        .github_issues
        .iter()
        .find(|g| copy_of(g, rules.github_user) == Some(i.number));
    match found {
        None if rules.record.copied.contains(&i.number) => None,
        None => Some(Action::CreateIssue(copy)),
        Some(g) if !same_issue(g, &copy) => {
            Some(Action::UpdateIssue(g.number, copy))
        }
        Some(_) => None,
    }
}

fn same_issue(a: &Issue, b: &Issue) -> bool {
    a.title == b.title
        && a.body.trim_end() == b.body.trim_end()
        && a.state == b.state
        && a.labels == b.labels
        && a.milestone == b.milestone
}

/// The task a GitHub issue is a copy of. Only issues the sync's own
/// account opened can be copies; anyone can write a marker.
#[must_use]
pub fn copy_of(g: &Issue, github_user: &str) -> Option<u64> {
    if g.author != github_user {
        return None;
    }
    let (_, rest) = g.body.trim_end().rsplit_once(MARKER)?;
    rest.strip_suffix(" -->")?.parse().ok()
}

fn imports(sides: &Sides, rules: Rules<'_>) -> Vec<Action> {
    let mut out = Vec::new();
    for g in &sides.github_issues {
        let is_copy = copy_of(g, rules.github_user).is_some();
        let done = g.labels.iter().any(|l| l == COPIED)
            || rules.record.imported.contains(&g.number);
        if !is_copy && !done {
            out.push(Action::Import(g.clone()));
        }
    }
    for c in &sides.github_comments {
        if c.author == rules.github_user
            || rules.record.comments.contains(&c.id)
        {
            continue;
        }
        if let Some(task) = task_for(sides, rules, c.issue) {
            out.push(Action::ImportComment(task, c.clone()));
        }
    }
    out
}

/// The Forgejo task that a GitHub issue belongs to, if it has one yet.
fn task_for(sides: &Sides, rules: Rules<'_>, issue: u64) -> Option<u64> {
    let note = format!("{FROM_GITHUB}{issue} -->");
    sides
        .github_issues
        .iter()
        .find(|g| g.number == issue)
        .and_then(|g| copy_of(g, rules.github_user))
        .or_else(|| {
            sides
                .forgejo_issues
                .iter()
                .find(|f| {
                    f.body.contains(&note)
                        && f.title
                            .starts_with(&format!("[From GitHub #{issue}]"))
                })
                .map(|f| f.number)
        })
}

/// The text a change would send to GitHub. Changes that only bring text
/// into Forgejo send nothing out.
#[must_use]
pub fn outgoing(action: &Action) -> Vec<String> {
    match action {
        Action::CreateMilestone(m) | Action::UpdateMilestone(_, m) => {
            vec![m.title.clone(), m.description.clone()]
        }
        Action::CreateIssue(i) | Action::UpdateIssue(_, i) => {
            let mut texts = vec![i.title.clone(), i.body.clone()];
            texts.extend(i.labels.iter().cloned());
            texts.extend(i.milestone.iter().cloned());
            texts
        }
        Action::CreateRelease(r) => {
            vec![r.tag.clone(), r.name.clone(), r.body.clone()]
        }
        Action::Import(_) | Action::ImportComment(..) => Vec::new(),
    }
}

fn name(action: &Action) -> String {
    match action {
        Action::CreateMilestone(m) | Action::UpdateMilestone(_, m) => {
            format!("milestone `{}`", m.title)
        }
        Action::CreateIssue(i) | Action::UpdateIssue(_, i) => {
            format!("task #{}", i.number)
        }
        Action::CreateRelease(r) => format!("release {}", r.tag),
        Action::Import(i) => format!("GitHub #{}", i.number),
        Action::ImportComment(_, c) => format!("GitHub comment {}", c.id),
    }
}

/// True when any text holds a secret, a private path, or a private word.
#[must_use]
pub fn is_private(texts: &[String], private: &[String]) -> bool {
    texts.iter().any(|t| {
        let lower = t.to_lowercase();
        !aegis::secrets::check("sync", t).is_empty()
            || private
                .iter()
                .filter(|w| !w.trim().is_empty())
                .any(|w| lower.contains(&w.trim().to_lowercase()))
    })
}

#[cfg(test)]
#[path = "plan_tests.rs"]
mod tests;
