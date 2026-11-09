//! chaukidar, the watchman. Agents have sudo, so the lock on Forgejo's
//! checks is a rule, not a wall (AGENTS.md section 11). The watchman
//! takes a fingerprint of everything that guards the project and raises
//! the alarm when any of it changes outside an install.
//!
//! The watch list is a private file. Each line is either
//! `file LABEL PATH` or `command LABEL SHELL-COMMAND`; blank lines and
//! lines starting with `#` are skipped. Labels are short public names,
//! such as `door-hook`, since they appear in alerts.

use forge_sync::run::Run;

pub mod alarm;

/// One thing the watchman looks at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Watched {
    /// A file, by path.
    File(String, String),
    /// The output of a shell command.
    Command(String, String),
}

impl Watched {
    /// Its public label.
    #[must_use]
    pub fn label(&self) -> &str {
        match self {
            Self::File(label, _) | Self::Command(label, _) => label,
        }
    }
}

/// Reads a watch list.
///
/// # Errors
/// Names the first line that is not `file` or `command` with a label and
/// a value, or a label used twice.
pub fn parse(list: &str) -> Result<Vec<Watched>, String> {
    let mut out: Vec<Watched> = Vec::new();
    for (index, line) in list.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let bad =
            || format!("watch list line {}: `{line}`", index.saturating_add(1));
        let (kind, rest) = line.split_once(' ').ok_or_else(bad)?;
        let (label, value) = rest.trim().split_once(' ').ok_or_else(bad)?;
        let (label, value) = (label.to_owned(), value.trim().to_owned());
        if !is_label(&label) {
            return Err(format!(
                "watch list line {}: a label is 1 to 40 small letters, digits \
                 or hyphens, since labels appear in public alarms",
                index.saturating_add(1)
            ));
        }
        if out.iter().any(|w| w.label() == label) {
            return Err(format!("the label `{label}` is used twice"));
        }
        out.push(match kind {
            "file" => Watched::File(label, value),
            "command" => Watched::Command(label, value),
            _ => return Err(bad()),
        });
    }
    Ok(out)
}

fn is_label(label: &str) -> bool {
    const LONGEST_LABEL: usize = 40;
    (1..=LONGEST_LABEL).contains(&label.len())
        && label
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}
/// The fingerprint (SHA-256) of one watched thing.
///
/// # Errors
/// When it cannot be read; the watchman then treats it as changed.
pub fn fingerprint(run: &dyn Run, watched: &Watched) -> Result<String, String> {
    let out = match watched {
        Watched::File(_, path) => run.run("sha256sum", &["--", path], &[])?,
        Watched::Command(_, command) => {
            let output =
                run.run("bash", &["-o", "pipefail", "-c", command], &[])?;
            if output.trim().is_empty() {
                return Err(format!("`{}` gave no output", watched.label()));
            }
            run.run("sha256sum", &[], output.as_bytes())?
        }
    };
    out.split_whitespace()
        .next()
        .map(str::to_owned)
        .ok_or_else(|| format!("no fingerprint for `{}`", watched.label()))
}

/// Every fingerprint, as `LABEL FINGERPRINT` lines.
///
/// # Errors
/// When any watched thing cannot be read.
pub fn record(run: &dyn Run, watched: &[Watched]) -> Result<String, String> {
    let mut lines = Vec::new();
    for w in watched {
        lines.push(format!("{} {}", w.label(), fingerprint(run, w)?));
    }
    Ok(format!("{}\n", lines.join("\n")))
}

/// What differs from the recorded fingerprints, one line each, sorted.
#[must_use]
pub fn changes(run: &dyn Run, watched: &[Watched], sums: &str) -> Vec<String> {
    let known: Vec<(&str, &str)> = sums
        .lines()
        .filter_map(|l| l.trim().split_once(' '))
        .collect();
    let mut out = Vec::new();
    for w in watched {
        let was = known.iter().find(|(l, _)| *l == w.label()).map(|(_, s)| *s);
        match (was, fingerprint(run, w)) {
            (None, _) => {
                out.push(format!(
                    "`{}` has no recorded fingerprint",
                    w.label()
                ));
            }
            (Some(_), Err(_)) => {
                out.push(format!("`{}` cannot be read", w.label()));
            }
            (Some(old), Ok(new)) if old != new => {
                out.push(format!("`{}` has changed", w.label()));
            }
            (Some(_), Ok(_)) => {}
        }
    }
    for (label, _) in &known {
        if !watched.iter().any(|w| w.label() == *label) {
            out.push(format!("`{label}` is no longer watched"));
        }
    }
    out.sort();
    out
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
