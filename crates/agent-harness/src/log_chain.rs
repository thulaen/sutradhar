//! The harness's log: append-only, each line holding the hash of the
//! line before, so a cut or a changed line shows (spec 0155, "The log").
//! Plain text in, plain text out; the file, its space and the owner-only
//! copy of the newest line are the service's.
//!
//! A line is `SEQ PREV HASH ENTRY`: its number from 1, the hash of the
//! line before (64 zeros for the first), its own hash, and the entry.
//! An entry is built from fixed parts, so it can hold no free text.

use std::fmt::{self, Write as _};

use sha2::{Digest, Sha256};

pub use crate::log_entry::{Done, Entry, Kind, Outcome, Summary, entry};

/// The `PREV` of the first line.
const START: &str =
    "0000000000000000000000000000000000000000000000000000000000000000";
/// The number of parts in a line: number, link, hash and entry.
const PARTS: usize = 4;

/// How a log failed to check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChainError {
    /// A line (counted from 1) is not in the format.
    BadFormat(u64),
    /// A line's number is not the next one.
    BadSequence(u64),
    /// A line does not hold the hash of the line before.
    BadLink(u64),
    /// A line's hash is not the hash of its own text.
    BadHash(u64),
    /// The log ends before the owner's copy of the newest line.
    TailCut,
    /// The log's line at the copy's number is not the copied line.
    HistoryChanged,
}

impl fmt::Display for ChainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadFormat(n) => write!(f, "line {n} is not in the format"),
            Self::BadSequence(n) => write!(f, "line {n} is out of order"),
            Self::BadLink(n) => {
                write!(f, "line {n} does not follow the line before")
            }
            Self::BadHash(n) => write!(f, "line {n} was changed"),
            Self::TailCut => {
                f.write_str("the log ends before the owner's copy")
            }
            Self::HistoryChanged => {
                f.write_str("the log differs from the owner's copy")
            }
        }
    }
}

/// The newest line of a log: its number and hash. The owner keeps a copy
/// outside the agents' reach.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Head {
    /// How many lines the log has.
    pub count: u64,
    /// The hash of the newest line; 64 zeros for an empty log.
    pub hash: String,
}

impl Head {
    /// Checks a log against this copy of its newest line: the log must
    /// check, must reach the copy's number and must hold the copied line
    /// there. A log that has grown since the copy still matches.
    ///
    /// # Errors
    /// The log's own error, or [`ChainError::TailCut`] or
    /// [`ChainError::HistoryChanged`].
    pub fn matches_copy(&self, lines: &[String]) -> Result<(), ChainError> {
        let head = verify(lines)?;
        if head.count < self.count {
            return Err(ChainError::TailCut);
        }
        if self.count == 0 {
            return Ok(());
        }
        let index =
            usize::try_from(self.count).map_err(|_| ChainError::TailCut)?;
        let line = lines
            .get(index.saturating_sub(1))
            .ok_or(ChainError::TailCut)?;
        match parse(line, self.count) {
            Ok(parts) if parts.hash == self.hash => Ok(()),
            _ => Err(ChainError::HistoryChanged),
        }
    }
}

/// A log being written.
#[derive(Debug, Clone)]
pub struct Chain {
    count: u64,
    last: String,
}

impl Default for Chain {
    fn default() -> Self {
        Self::new()
    }
}

impl Chain {
    /// An empty log.
    #[must_use]
    pub fn new() -> Self {
        Self {
            count: 0,
            last: START.to_owned(),
        }
    }

    /// The newest line so far.
    #[must_use]
    pub fn head(&self) -> Head {
        Head {
            count: self.count,
            hash: self.last.clone(),
        }
    }

    /// Adds an entry and gives its line. The owner's copy of the newest
    /// line must be written with every append: a log grown past the copy
    /// still matches it, so a tail cut and rewritten after the copy shows
    /// only once the copy catches up.
    pub fn append(&mut self, entry: &Entry) -> String {
        self.count = self.count.saturating_add(1);
        let text = entry.as_str();
        let hash = hash_of(self.count, &self.last, text);
        let line = format!("{} {} {hash} {text}", self.count, self.last);
        self.last = hash;
        line
    }
}

fn hash_of(number: u64, prev: &str, entry: &str) -> String {
    let digest = Sha256::digest(format!("{number} {prev} {entry}").as_bytes());
    let mut hex = String::with_capacity(START.len());
    for byte in digest {
        let _written = write!(hex, "{byte:02x}");
    }
    hex
}

struct Parts<'a> {
    number: &'a str,
    prev: &'a str,
    hash: &'a str,
    entry: &'a str,
}

fn is_hash(text: &str) -> bool {
    text.len() == START.len()
        && text.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

fn parse(line: &str, number: u64) -> Result<Parts<'_>, ChainError> {
    let bad = ChainError::BadFormat(number);
    if line.contains('\n') || line.contains('\r') {
        return Err(bad);
    }
    let mut parts = line.splitn(PARTS, ' ');
    let (Some(number), Some(prev), Some(hash), Some(entry)) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return Err(bad);
    };
    if !is_hash(prev) || !is_hash(hash) || entry.is_empty() {
        return Err(bad);
    }
    Ok(Parts {
        number,
        prev,
        hash,
        entry,
    })
}

/// Checks a whole log and gives its newest line.
///
/// # Errors
/// The first line that is not in the format, out of order, not linked
/// to the line before, or changed.
pub fn verify(lines: &[String]) -> Result<Head, ChainError> {
    let mut last = START.to_owned();
    let mut count: u64 = 0;
    for line in lines {
        count = count.saturating_add(1);
        let parts = parse(line, count)?;
        // The number is written in one plain form, so `01` and `+1` are
        // not the line number 1.
        if parts.number != count.to_string() {
            return Err(ChainError::BadSequence(count));
        }
        if parts.prev != last {
            return Err(ChainError::BadLink(count));
        }
        if hash_of(count, parts.prev, parts.entry) != parts.hash {
            return Err(ChainError::BadHash(count));
        }
        parts.hash.clone_into(&mut last);
    }
    Ok(Head { count, hash: last })
}
