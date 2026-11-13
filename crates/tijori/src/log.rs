//! The signature log's hash chain (spec 0065 "The signature log" items 1
//! and 2).
//!
//! Each entry holds its counter, time, kind, the previous entry's hash and
//! a body, each a field prefixed with its byte length as an unsigned
//! 64-bit big-endian integer. Its hash is the SHA-256 of those fields, so
//! changing any earlier entry changes every head after it. This part is
//! pure: the caller supplies the clock and does the durable write.

use std::fmt;

use sha2::{Digest, Sha256};

/// Bytes in a SHA-256 digest, the size of every hash in the log.
pub const HASH_LEN: usize = 32;

/// The previous-entry hash the first entry names.
const NO_PREVIOUS: [u8; HASH_LEN] = [0; HASH_LEN];

/// The kind of a log entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// A signature the core made.
    Signature,
    /// A release request waiting for its guards.
    ReleasePending,
    /// A policy the core applied.
    PolicyApplied,
    /// An owner-signed reset of the policy-version record.
    PolicyRecordReset,
    /// A change to the service's site-link keys.
    SiteKeyChange,
    /// An owner-signed reset of the site-key record.
    SiteKeyRecordReset,
    /// A private key the core destroyed.
    KeyDestroyed,
    /// An owner-signed recovery step.
    Recovery,
}

impl Kind {
    /// The kind's name as it is written in the entry.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Signature => "signature",
            Self::ReleasePending => "release-pending",
            Self::PolicyApplied => "policy-applied",
            Self::PolicyRecordReset => "policy-record-reset",
            Self::SiteKeyChange => "site-key-change",
            Self::SiteKeyRecordReset => "site-key-record-reset",
            Self::KeyDestroyed => "key-destroyed",
            Self::Recovery => "recovery",
        }
    }
}

/// The head of the log: the latest entry's counter and hash.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Head {
    /// The latest entry's counter.
    pub counter: u64,
    /// The latest entry's hash.
    pub hash: [u8; HASH_LEN],
}

/// One entry of the log.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// The entry's counter, from 1 upwards.
    pub counter: u64,
    /// The entry's time, never earlier than the entry before it.
    pub time: u64,
    /// The entry's kind.
    pub kind: Kind,
    /// The hash of the entry before it, or zeros for the first entry.
    pub previous: [u8; HASH_LEN],
    /// The kind's own fields, already encoded.
    pub body: Vec<u8>,
}

/// Why an entry could not be added or the log could not be checked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogError {
    /// The counter has reached its largest value; signing stops and the
    /// owner is told.
    CounterExhausted,
    /// An entry's counter does not follow the one before it.
    CounterGap {
        /// The counter the entry should have had.
        expected: u64,
    },
    /// An entry does not name the hash of the entry before it.
    BrokenChain {
        /// The counter of the entry that does not link.
        counter: u64,
    },
    /// An entry's time is earlier than the entry before it.
    TimeWentBack {
        /// The counter of the entry whose time went back.
        counter: u64,
    },
}

impl fmt::Display for LogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CounterExhausted => write!(
                f,
                "the log counter has reached its largest value; signing has \
                 stopped and the owner must recover"
            ),
            Self::CounterGap { expected } => {
                write!(f, "the log skips or repeats entry {expected}")
            }
            Self::BrokenChain { counter } => write!(
                f,
                "log entry {counter} does not link to the entry before it"
            ),
            Self::TimeWentBack { counter } => {
                write!(
                    f,
                    "log entry {counter} is dated before the entry before it"
                )
            }
        }
    }
}

impl std::error::Error for LogError {}

fn push_field(out: &mut Vec<u8>, field: &[u8]) {
    let len = u64::try_from(field.len()).unwrap_or(u64::MAX);
    out.extend_from_slice(&len.to_be_bytes());
    out.extend_from_slice(field);
}

impl Entry {
    /// The bytes the entry's hash covers.
    #[must_use]
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        push_field(&mut out, &self.counter.to_be_bytes());
        push_field(&mut out, &self.time.to_be_bytes());
        push_field(&mut out, self.kind.name().as_bytes());
        push_field(&mut out, &self.previous);
        push_field(&mut out, &self.body);
        out
    }

    /// The entry's hash.
    #[must_use]
    pub fn hash(&self) -> [u8; HASH_LEN] {
        Sha256::digest(self.encode()).into()
    }
}

/// The growing end of the log.
#[derive(Debug, Clone, Default)]
pub struct Chain {
    head: Option<Head>,
    last_time: u64,
}

impl Chain {
    /// An empty log.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Carries on a log whose latest entry has `head` and `last_time`.
    #[must_use]
    pub const fn resume(head: Head, last_time: u64) -> Self {
        Self {
            head: Some(head),
            last_time,
        }
    }

    /// The current head, if any entry exists.
    #[must_use]
    pub const fn head(&self) -> Option<Head> {
        self.head
    }

    /// Makes the next entry. Its time is the later of `clock` and the
    /// previous entry's time.
    ///
    /// # Errors
    ///
    /// Refuses when the counter has reached its largest value; it never
    /// wraps round.
    pub fn append(
        &mut self,
        kind: Kind,
        clock: u64,
        body: Vec<u8>,
    ) -> Result<Entry, LogError> {
        let (counter, previous) = match self.head {
            None => (1, NO_PREVIOUS),
            Some(head) => (
                head.counter
                    .checked_add(1)
                    .ok_or(LogError::CounterExhausted)?,
                head.hash,
            ),
        };
        let time = clock.max(self.last_time);
        let entry = Entry {
            counter,
            time,
            kind,
            previous,
            body,
        };
        self.head = Some(Head {
            counter,
            hash: entry.hash(),
        });
        self.last_time = time;
        Ok(entry)
    }
}

/// Checks `entries`, from the first, and returns the head they reach.
/// It proves only that the entries link up: a log cut short or rewritten
/// from the start also links up, so the caller must compare the head it
/// returns with a head the owner's instance has confirmed.
///
/// # Errors
///
/// Refuses a gap or repeat in the counters, an entry that does not link
/// to the one before it, and a time that goes back.
pub fn check(entries: &[Entry]) -> Result<Option<Head>, LogError> {
    let mut head: Option<Head> = None;
    let mut last_time = 0;
    for entry in entries {
        let (expected, previous) = match head {
            None => (1, NO_PREVIOUS),
            Some(h) => (
                h.counter.checked_add(1).ok_or(LogError::CounterExhausted)?,
                h.hash,
            ),
        };
        if entry.counter != expected {
            return Err(LogError::CounterGap { expected });
        }
        if entry.previous != previous {
            return Err(LogError::BrokenChain {
                counter: entry.counter,
            });
        }
        if entry.time < last_time {
            return Err(LogError::TimeWentBack {
                counter: entry.counter,
            });
        }
        last_time = entry.time;
        head = Some(Head {
            counter: entry.counter,
            hash: entry.hash(),
        });
    }
    Ok(head)
}
