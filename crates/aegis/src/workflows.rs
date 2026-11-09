//! The workflow check (docs/specs/0060-runner.md). A workflow may not
//! choose its own container or its options, add side services, or use
//! ready-made actions, since each would get past the runner's sealed,
//! capped container or bring in a language runtime. Forgejo workflows
//! run only on the `dell` runner.
//!
//! The file is read with a full YAML reader and every key is checked,
//! however it is written. So that the reader and Forgejo's own cannot
//! disagree about where a line or a key ends, a workflow may hold only
//! plain printable ASCII and `\n` line breaks, one document, and no
//! tags (a tag such as `!!binary` can make a key mean other than its
//! text). The reader follows aliases itself, so a refused key behind
//! one is found.

use yaml_rust2::parser::{MarkedEventReceiver, Parser};
use yaml_rust2::scanner::Marker;
use yaml_rust2::{Event, Yaml, YamlLoader};

use crate::Finding;

const CHECK: &str = "workflows.sealed";

/// Keys a workflow may never use, anywhere.
const REFUSED: [&str; 4] = ["container", "services", "uses", "options"];

/// The folders that hold workflows.
const FOLDERS: [&str; 3] = [
    ".forgejo/workflows/",
    ".gitea/workflows/",
    ".github/workflows/",
];

/// Whether `path` is a workflow file.
#[must_use]
pub fn is_workflow(path: &str) -> bool {
    FOLDERS.iter().any(|f| path.starts_with(f))
}

/// Whether Forgejo runs this workflow, so it must run on `dell`.
fn on_forgejo(path: &str) -> bool {
    path.starts_with(".forgejo/") || path.starts_with(".gitea/")
}

/// The first line holding a character other than printable ASCII.
fn odd_character(text: &str) -> Option<usize> {
    text.lines()
        .position(|l| !l.chars().all(|c| (' '..='~').contains(&c)))
        .map(|i| i.saturating_add(1))
        .or_else(|| text.contains('\r').then_some(0))
}

/// Notes the line of the first tagged node. A tag such as `!!binary` can
/// make a key mean something other than its text, so tags are refused.
struct FirstTag(Option<usize>);

impl MarkedEventReceiver for FirstTag {
    fn on_event(&mut self, event: Event, mark: Marker) {
        let tagged = matches!(
            event,
            Event::Scalar(_, _, _, Some(_))
                | Event::SequenceStart(_, Some(_))
                | Event::MappingStart(_, Some(_))
        );
        if tagged && self.0.is_none() {
            self.0 = Some(mark.line());
        }
    }
}

/// Where a workflow runs.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Host {
    Forgejo,
    Elsewhere,
}

/// Walks one node, adding a problem for each refused key or value.
fn walk(node: &Yaml, host: Host, out: &mut Vec<String>) {
    match node {
        Yaml::Hash(map) => {
            for (key, value) in map {
                let Yaml::String(name) = key else {
                    out.push("every key must be a plain name".to_owned());
                    continue;
                };
                if REFUSED.contains(&name.as_str()) {
                    out.push(format!(
                        "`{name}` is not allowed: jobs run only in the \
                         runner's own sealed container, with plain shell \
                         steps"
                    ));
                }
                let dell = matches!(value, Yaml::String(v) if v == "dell");
                if host == Host::Forgejo && name == "runs-on" && !dell {
                    out.push("a Forgejo workflow runs only on `dell`".into());
                }
                walk(value, host, out);
            }
        }
        Yaml::Array(items) => {
            for item in items {
                walk(item, host, out);
            }
        }
        Yaml::BadValue => out.push("a value cannot be read".to_owned()),
        _ => {}
    }
}

/// Checks one workflow file.
#[must_use]
pub fn check(path: &str, text: &str) -> Vec<Finding> {
    let found = |line: usize, message: String| {
        vec![Finding::new(CHECK, path, line, message)]
    };
    if let Some(line) = odd_character(text) {
        return found(
            line,
            "a workflow may hold only plain ASCII text and \\n line breaks"
                .to_owned(),
        );
    }
    let mut tags = FirstTag(None);
    if let Err(e) = Parser::new_from_str(text).load(&mut tags, true) {
        return found(0, format!("this is not valid YAML: {e}"));
    }
    if let Some(line) = tags.0 {
        return found(line, "tags such as `!!str` are not allowed".to_owned());
    }
    let docs = match YamlLoader::load_from_str(text) {
        Ok(docs) => docs,
        Err(e) => return found(0, format!("this is not valid YAML: {e}")),
    };
    let [doc] = docs.as_slice() else {
        return found(0, "a workflow must be exactly one document".to_owned());
    };
    let mut problems = Vec::new();
    let host = if on_forgejo(path) {
        Host::Forgejo
    } else {
        Host::Elsewhere
    };
    walk(doc, host, &mut problems);
    problems
        .into_iter()
        .map(|message| Finding::new(CHECK, path, 0, message))
        .collect()
}

#[cfg(test)]
#[path = "workflows_tests.rs"]
mod tests;
