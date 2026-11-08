//! The stand-in for curl, gh, git and gitleaks, shared by the sync tests.
#![allow(dead_code, reason = "each test file uses some of the helpers")]

use std::cell::RefCell;

use forge_sync::fetch::Config;
use forge_sync::run::Run;

/// One rule: if a call contains the text, give this answer.
pub type Pair = (&'static str, Result<&'static str, &'static str>);

/// Answers each call from a list of rules, first match wins; page 2 and
/// later of a list are empty, and anything else gets `[]`. Keeps every
/// call made.
pub struct Fake {
    pub rules: Vec<Pair>,
    pub calls: RefCell<Vec<String>>,
    pub endless: bool,
}

impl Fake {
    pub fn new(rules: &[Pair]) -> Self {
        Self {
            rules: rules.to_vec(),
            calls: RefCell::new(Vec::new()),
            endless: false,
        }
    }

    pub fn called(&self, needle: &str) -> bool {
        self.calls.borrow().iter().any(|c| c.contains(needle))
    }
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
        if call.contains("page=2") && !self.endless {
            return Ok("[]".to_owned());
        }
        self.rules
            .iter()
            .find(|(needle, _)| call.contains(needle))
            .map_or_else(
                || Ok("[]".to_owned()),
                |(_, answer)| answer.map(str::to_owned).map_err(str::to_owned),
            )
    }
}

/// Settings with a fresh record file for each test.
pub fn config(name: &str) -> Config {
    let record = std::env::temp_dir()
        .join(format!("forge-sync-{name}-{}", std::process::id()));
    let _ = std::fs::remove_file(&record);
    Config {
        forgejo: "http://forgejo/api/v1/repos/o/r".to_owned(),
        header_file: "/h".to_owned(),
        github: "g/r".to_owned(),
        mirror: "/m".to_owned(),
        record: record.display().to_string(),
        github_user: "owner".to_owned(),
        gitleaks: "gitleaks".to_owned(),
        private: vec!["myforum".to_owned()],
    }
}

pub const MILESTONES: &str = r#"[
    {"id":16,"title":"Later","description":"","state":"open"},
    {"id":9,"title":"Phase 1: Forgejo home","description":"","state":"open"},
    {"id":10,"title":"Phase 10: Far","description":"","state":"open"}]"#;
pub const ISSUES: &str = r#"[{"number":3,"title":"Task","body":"b",
    "state":"closed","labels":[],"milestone":{"title":"Later"}}]"#;
pub const RELEASES: &str =
    r#"[{"tag_name":"v0.1.0","name":"v0.1.0","body":"notes"}]"#;
pub const LABELS: &str = r#"[{"id":4,"name":"status/urgent"}]"#;
