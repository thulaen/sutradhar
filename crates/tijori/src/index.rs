//! The log's two indexes (spec 0065 "The signature log" items 3 and 4).
//!
//! One maps principal, purpose and bytes hash to a counter, for repeats.
//! The other maps key id and signature to a counter, for the
//! before-revocation answer. Each keeps a running hash over its entries
//! in order, so the core can check at start-up that an index matches the
//! running hash its latest `signature` entry recorded.

use std::collections::HashMap;
use std::hash::Hash;

use sha2::{Digest, Sha256};

/// Bytes in a SHA-256 digest.
const DIGEST_LEN: usize = 32;

/// A counter that would move the index backwards.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CounterWentBack {
    /// The latest counter the index holds.
    pub latest: u64,
    /// The counter that was offered.
    pub counter: u64,
}

impl std::fmt::Display for CounterWentBack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "index entry {} is not after the latest entry, {}",
            self.counter, self.latest
        )
    }
}

impl std::error::Error for CounterWentBack {}

/// One index with its running hash.
#[derive(Debug, Clone)]
pub struct Index<K> {
    map: HashMap<K, u64>,
    running: [u8; DIGEST_LEN],
    latest: u64,
}

impl<K> Default for Index<K> {
    fn default() -> Self {
        Self {
            map: HashMap::new(),
            running: [0; DIGEST_LEN],
            latest: 0,
        }
    }
}

impl<K: Eq + Hash + AsRef<[u8]>> Index<K> {
    /// An empty index.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds `key` at `counter`. A later entry for the same key replaces
    /// the earlier one, as a revoked-key repeat does. The running hash
    /// covers the previous running hash, the key and the counter.
    ///
    /// # Errors
    ///
    /// Refuses a counter that is not above the latest one, so the index
    /// only moves forward with the log.
    pub fn insert(
        &mut self,
        key: K,
        counter: u64,
    ) -> Result<(), CounterWentBack> {
        if counter <= self.latest {
            return Err(CounterWentBack {
                latest: self.latest,
                counter,
            });
        }
        let key_bytes = key.as_ref();
        let len = u64::try_from(key_bytes.len()).unwrap_or(u64::MAX);
        let mut fields = Vec::new();
        fields.extend_from_slice(&self.running);
        fields.extend_from_slice(&len.to_be_bytes());
        fields.extend_from_slice(key_bytes);
        fields.extend_from_slice(&counter.to_be_bytes());
        self.running = Sha256::digest(&fields).into();
        self.latest = counter;
        let _previous = self.map.insert(key, counter);
        Ok(())
    }

    /// The counter recorded for `key`, without scanning the log.
    #[must_use]
    pub fn get(&self, key: &K) -> Option<u64> {
        self.map.get(key).copied()
    }

    /// The counter of the latest entry added.
    #[must_use]
    pub const fn latest(&self) -> u64 {
        self.latest
    }

    /// The running hash after the latest entry.
    #[must_use]
    pub const fn running(&self) -> [u8; DIGEST_LEN] {
        self.running
    }

    /// Whether this index matches what the latest `signature` entry
    /// recorded. A mismatch stops signing until the owner's index-rebuild
    /// step.
    #[must_use]
    pub fn matches(&self, latest: u64, running: &[u8; DIGEST_LEN]) -> bool {
        self.latest == latest && &self.running == running
    }
}
