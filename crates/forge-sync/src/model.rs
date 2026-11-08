//! The few fields the sync reads from Forgejo and GitHub. Only these are
//! ever copied (AGENTS.md section 9.1: an allow-list).

use serde_json::Value;

/// A milestone, by title.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Milestone {
    /// Its number on that side.
    pub number: u64,
    /// The title, which matches the two sides.
    pub title: String,
    /// The description.
    pub description: String,
    /// `open` or `closed`.
    pub state: String,
}

/// A task (an issue).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Issue {
    /// Its number on that side.
    pub number: u64,
    /// The title.
    pub title: String,
    /// The body.
    pub body: String,
    /// `open` or `closed`.
    pub state: String,
    /// Label names, sorted.
    pub labels: Vec<String>,
    /// The milestone's title, if any.
    pub milestone: Option<String>,
    /// Who opened it (only read on GitHub).
    pub author: String,
}

/// A release, by tag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Release {
    /// The tag, such as `v1.0.0`.
    pub tag: String,
    /// The title.
    pub name: String,
    /// The notes.
    pub body: String,
    /// True for a test release such as `v1.0.0-rc.1`.
    pub prerelease: bool,
}

/// A comment on a GitHub issue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
    /// Its id.
    pub id: u64,
    /// The issue it is on.
    pub issue: u64,
    /// Who wrote it.
    pub author: String,
    /// What it says.
    pub body: String,
}
fn text(v: &Value, key: &str) -> String {
    v.get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned()
}

fn number(v: &Value, key: &str) -> u64 {
    v.get(key).and_then(Value::as_u64).unwrap_or_default()
}

fn list(json: &str) -> Result<Vec<Value>, String> {
    match serde_json::from_str::<Value>(json) {
        Ok(Value::Array(items)) => Ok(items),
        Ok(_) => Err("expected a list".to_owned()),
        Err(e) => Err(format!("not JSON: {e}")),
    }
}

/// Reads a list of milestones, using `key` for their number (`id` on
/// Forgejo, `number` on GitHub).
///
/// # Errors
/// When the text is not a JSON list.
pub fn milestones(json: &str, key: &str) -> Result<Vec<Milestone>, String> {
    Ok(list(json)?
        .iter()
        .map(|m| Milestone {
            number: number(m, key),
            title: text(m, "title"),
            description: text(m, "description"),
            state: text(m, "state"),
        })
        .collect())
}

/// Reads a list of issues. Pull requests are left out.
///
/// # Errors
/// When the text is not a JSON list.
pub fn issues(json: &str) -> Result<Vec<Issue>, String> {
    Ok(list(json)?
        .iter()
        .filter(|i| i.get("pull_request").is_none_or(Value::is_null))
        .map(|i| {
            let mut labels: Vec<String> = i
                .get("labels")
                .and_then(Value::as_array)
                .map(|ls| ls.iter().map(|l| text(l, "name")).collect())
                .unwrap_or_default();
            labels.sort();
            Issue {
                number: number(i, "number"),
                title: text(i, "title"),
                body: text(i, "body"),
                state: text(i, "state"),
                labels,
                milestone: i
                    .get("milestone")
                    .filter(|m| !m.is_null())
                    .map(|m| text(m, "title")),
                author: i
                    .get("user")
                    .map(|u| text(u, "login"))
                    .unwrap_or_default(),
            }
        })
        .collect())
}

/// Reads a list of releases. Drafts are left out.
///
/// # Errors
/// When the text is not a JSON list.
pub fn releases(json: &str) -> Result<Vec<Release>, String> {
    Ok(list(json)?
        .iter()
        .filter(|r| r.get("draft").and_then(Value::as_bool) != Some(true))
        .map(|r| Release {
            tag: text(r, "tag_name"),
            name: text(r, "name"),
            body: text(r, "body"),
            prerelease: r.get("prerelease").and_then(Value::as_bool)
                == Some(true),
        })
        .collect())
}

/// Reads a list of GitHub issue comments.
///
/// # Errors
/// When the text is not a JSON list.
pub fn comments(json: &str) -> Result<Vec<Comment>, String> {
    Ok(list(json)?
        .iter()
        .map(|c| Comment {
            id: number(c, "id"),
            issue: text(c, "issue_url")
                .rsplit('/')
                .next()
                .and_then(|n| n.parse().ok())
                .unwrap_or_default(),
            author: c.get("user").map(|u| text(u, "login")).unwrap_or_default(),
            body: text(c, "body"),
        })
        .collect())
}
#[cfg(test)]
mod tests {
    use super::{issues, milestones, releases};

    #[test]
    fn reads_milestones() {
        let json =
            r#"[{"id":9,"title":"Phase 1","description":"d","state":"open"}]"#;
        let got = milestones(json, "id").unwrap();
        assert_eq!(got[0].number, 9);
        assert_eq!(got[0].title, "Phase 1");
    }

    #[test]
    fn reads_issues_and_leaves_out_pull_requests() {
        let json = r#"[
          {"number":1,"title":"t","body":null,"state":"open",
           "labels":[{"name":"b"},{"name":"a"}],
           "milestone":{"title":"Later"},"user":{"login":"x"}},
          {"number":2,"title":"pr","pull_request":{"url":"u"}},
          {"number":3,"title":"t3","milestone":null,"pull_request":null}
        ]"#;
        let got = issues(json).unwrap();
        assert_eq!(got.len(), 2);
        assert_eq!(got[0].labels, ["a", "b"]);
        assert_eq!(got[0].milestone.as_deref(), Some("Later"));
        assert_eq!(got[0].author, "x");
        assert_eq!(got[0].body, "");
        assert_eq!(got[1].milestone, None);
    }

    #[test]
    fn reads_releases_and_leaves_out_drafts() {
        let json = r#"[{"tag_name":"v1.0.0","name":"n","body":"b",
            "prerelease":true},
            {"tag_name":"v2","draft":true}]"#;
        let got = releases(json).unwrap();
        assert_eq!(got.len(), 1);
        assert!(got[0].prerelease);
    }

    #[test]
    fn reads_comments() {
        let json = r#"[{"id":7,"issue_url":"https://x/issues/12",
            "user":{"login":"pat"},"body":"hi"}]"#;
        let got = super::comments(json).unwrap();
        assert_eq!((got[0].id, got[0].issue), (7, 12));
        assert_eq!(got[0].author, "pat");
    }

    #[test]
    fn refuses_text_that_is_not_a_list() {
        assert!(issues("{}").is_err());
        assert!(issues("nope").is_err());
    }
}
