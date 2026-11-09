//! The private-lists check (task #20; gate item GI-46; AGENTS.md section
//! 11). The owner's private lists live on Dell, outside the repository,
//! in one folder:
//!
//! - `deny-list.txt` and `xenforo-phrases.txt`: words, one a line, such
//!   as the owner's forum names and domain;
//! - `xenforo-index.txt`: XenForo's licensed files, one a line, as
//!   `KIND SHA256 PATH`;
//! - `credential-files.txt`: files on Dell that hold real passwords and
//!   tokens, one path a line; each file's value is a secret.
//!
//! A file, commit message or line added in history that holds a private
//! word or secret, or a file that is a licensed XenForo file, is
//! refused. A finding never shows what it matched, and a list or secret
//! that cannot be read fails the check: it never passes unseen.

use std::collections::HashSet;
use std::fmt::{self, Write as _};
use std::fs;
use std::path::Path;

use sha2::{Digest, Sha256};

use crate::Finding;

const CHECK: &str = "secrets.private-lists";

/// The lists the folder must hold.
const LISTS: [&str; 4] = [
    "deny-list.txt",
    "xenforo-phrases.txt",
    "xenforo-index.txt",
    "credential-files.txt",
];

/// The owner's private lists, read from their folder.
#[derive(Default)]
pub struct Private {
    words: Vec<String>,
    hashes: HashSet<String>,
    secrets: Vec<String>,
}

/// Shows only how many entries each list holds, never the entries.
impl fmt::Debug for Private {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Private {{ {} words, {} hashes, {} secrets }}",
            self.words.len(),
            self.hashes.len(),
            self.secrets.len()
        )
    }
}

fn read(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| {
        format!("cannot read the private list {}: {e}", path.display())
    })
}

fn entries(text: &str) -> impl Iterator<Item = &str> {
    text.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
}

/// The secret values in one secret file: each line, and the value part of
/// a `NAME=value` or `user:password` line. Values under 8 characters are
/// skipped, since they would match everyday text.
fn secret_values(text: &str) -> Vec<String> {
    const SHORTEST: usize = 8;
    let mut out = Vec::new();
    for line in text.lines().map(str::trim) {
        let after_equals = line.split_once('=').map(|(_, v)| v.trim());
        let after_colon = line.rsplit_once(':').map(|(_, v)| v.trim());
        for value in [Some(line), after_equals, after_colon]
            .into_iter()
            .flatten()
        {
            if value.len() >= SHORTEST && !out.iter().any(|v| v == value) {
                out.push(value.to_owned());
            }
        }
    }
    out
}

impl Private {
    /// Reads the four lists from `dir`, and every secret they name.
    ///
    /// # Errors
    /// When a list or a named secret file cannot be read: a gate that
    /// cannot see the lists must fail, never pass.
    pub fn load(dir: &Path) -> Result<Self, String> {
        let [deny, phrases, index, credentials] =
            LISTS.map(|name| read(&dir.join(name)));
        let words = entries(&deny?)
            .chain(entries(&phrases?))
            .map(str::to_lowercase)
            .collect();
        let hashes = entries(&index?)
            .filter_map(|l| l.split_whitespace().nth(1))
            .filter(|h| h.len() == crate::tools_list::HEX_LEN)
            .map(str::to_lowercase)
            .collect();
        let mut secrets = Vec::new();
        for path in entries(&credentials?) {
            let text = fs::read_to_string(path).map_err(|_| {
                "cannot read a secret file the private lists name".to_owned()
            })?;
            let before = secrets.len();
            secrets.extend(secret_values(&text));
            if secrets.len() == before && text.trim().is_empty() {
                return Err(
                    "a secret file the private lists name is empty".to_owned()
                );
            }
        }
        Ok(Self {
            words,
            hashes,
            secrets,
        })
    }

    /// The lists in the folder `AEGIS_PRIVATE` names, or `None` when it
    /// names none, as on a contributor's own computer.
    ///
    /// # Errors
    /// When the folder is named but a list cannot be read.
    pub fn from_env() -> Result<Option<Self>, String> {
        match std::env::var("AEGIS_PRIVATE") {
            Ok(dir) if !dir.is_empty() => Self::load(Path::new(&dir)).map(Some),
            _ => Ok(None),
        }
    }
}

/// The SHA-256 of `bytes`, in lower-case hex.
#[must_use]
pub fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes).iter().fold(
        String::with_capacity(crate::tools_list::HEX_LEN),
        |mut hex, b| {
            let _written = write!(hex, "{b:02x}");
            hex
        },
    )
}

/// Checks text, line by line, for a private word or secret.
#[must_use]
pub fn check_text(path: &str, text: &str, private: &Private) -> Vec<Finding> {
    let mut out = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let lower = line.to_lowercase();
        let word = private.words.iter().any(|w| lower.contains(w.as_str()));
        let secret = private.secrets.iter().any(|s| line.contains(s.as_str()));
        if word || secret {
            out.push(Finding::new(
                CHECK,
                path,
                index.saturating_add(1),
                "this line holds something from the owner's private lists \
                 (not shown here)",
            ));
        }
    }
    out
}

/// Checks one file's bytes: its text, and whether it is a licensed file.
#[must_use]
pub fn check(path: &str, bytes: &[u8], private: &Private) -> Vec<Finding> {
    let mut out = Vec::new();
    if private.hashes.contains(&sha256(bytes)) {
        out.push(Finding::new(
            CHECK,
            path,
            0,
            "this is a licensed XenForo file, which never enters the \
             repository",
        ));
    }
    out.extend(check_text(path, &String::from_utf8_lossy(bytes), private));
    out
}

/// The finding for lists that cannot be read.
#[must_use]
pub fn unreadable(problem: String) -> Finding {
    Finding::new(CHECK, "", 0, problem)
}

/// Checks a commit message against the lists `AEGIS_PRIVATE` names.
#[must_use]
pub fn check_message(message: &str) -> Vec<Finding> {
    match Private::from_env() {
        Ok(Some(lists)) => check_text("(commit message)", message, &lists),
        Ok(None) => Vec::new(),
        Err(e) => vec![unreadable(e)],
    }
}

#[cfg(test)]
#[path = "private_tests.rs"]
mod tests;
