//! The sign step of tijori's signing core (spec 0065 "Signing" items 2
//! and 3, "Callers and their keys" items 1 and 2).
//!
//! This part holds the keys, the accounts bound to principals and the
//! log in memory. Durable writes, the socket and sealed key files are
//! later parts; here a signature is written to the log and both indexes
//! in one step before it is returned, and nothing changes if any part
//! of that step fails.

use std::collections::HashMap;

use ed25519_compact::KeyPair;
use sha2::{Digest, Sha256};
use sutradhar_tijori::check::{KeyEntry, KeyState};
use sutradhar_tijori::policy::PolicyTrust;
use sutradhar_tijori::purpose::Purpose;

pub use crate::sign_error::SignError;
pub use crate::signed::Signature;

use crate::index::Index;
use crate::log::{Chain, Entry, HASH_LEN, Kind};
use crate::purposes::{
    NO_CLEAN_REPEAT, NOT_SIGNED_HERE, PURPOSE_LEN, RELEASE_PURPOSES,
};
use crate::signed::{Logged, Made, fields};

/// The signing core of one instance, held in memory.
pub struct Core {
    policy: [u8; HASH_LEN],
    max_bytes: usize,
    accounts: HashMap<String, String>,
    keys: HashMap<String, KeyEntry>,
    secrets: HashMap<String, KeyPair>,
    chain: Chain,
    entries: Vec<Entry>,
    repeats: Index<Vec<u8>>,
    by_signature: Index<Vec<u8>>,
    issued: HashMap<u64, Logged>,
}

impl Core {
    /// An empty core under the policy with digest `policy`, refusing
    /// bytes longer than `max_bytes`.
    #[must_use]
    pub fn new(policy: [u8; HASH_LEN], max_bytes: usize) -> Self {
        Self {
            policy,
            max_bytes,
            accounts: HashMap::new(),
            keys: HashMap::new(),
            secrets: HashMap::new(),
            chain: Chain::new(),
            entries: Vec::new(),
            repeats: Index::new(),
            by_signature: Index::new(),
            issued: HashMap::new(),
        }
    }

    /// Binds an operating-system account to a principal.
    pub fn bind_account(&mut self, account: &str, principal: &str) {
        let _previous = self
            .accounts
            .insert(account.to_owned(), principal.to_owned());
    }

    /// Adds a key to the registry with its private half.
    pub fn add_key(&mut self, key_id: &str, entry: KeyEntry, pair: KeyPair) {
        let _previous = self.keys.insert(key_id.to_owned(), entry);
        let _previous = self.secrets.insert(key_id.to_owned(), pair);
    }

    /// Sets a key's state, as a policy change does.
    pub fn set_state(&mut self, key_id: &str, state: KeyState) {
        if let Some(entry) = self.keys.get_mut(key_id) {
            entry.state = state;
        }
    }

    /// The trust source a check reads, built from this core's registry.
    #[must_use]
    pub fn trust(&self) -> PolicyTrust {
        PolicyTrust::new(self.policy, self.keys.clone())
    }

    /// The log, oldest entry first.
    #[must_use]
    pub fn log(&self) -> &[Entry] {
        &self.entries
    }

    /// The caller's active key for `purpose`, the lowest id if several. A
    /// key bound to a release purpose is the release key, which signs only
    /// through `sign_release`, so it is never chosen here.
    fn active_key(&self, principal: &str, purpose: &str) -> Option<String> {
        let mut found: Vec<&String> = self
            .keys
            .iter()
            .filter(|(_, k)| {
                k.principal == principal
                    && k.state == KeyState::Active
                    && k.purposes.contains_key(purpose)
                    && !RELEASE_PURPOSES
                        .iter()
                        .any(|r| k.purposes.contains_key(*r))
            })
            .map(|(id, _)| id)
            .collect();
        found.sort();
        found.first().map(|id| (*id).clone())
    }

    /// Signs `bytes` for `purpose` on behalf of `account` at time `now`,
    /// writing the log entry and both indexes before returning.
    ///
    /// # Errors
    ///
    /// Refuses bytes over the limit, an unknown account, a purpose this
    /// step does not sign, a bad purpose name, a caller with no active
    /// key for the purpose, a revoked-key repeat of a purpose that is
    /// never signed clean again, a log that disagrees with its indexes,
    /// and a full log.
    pub fn sign(
        &mut self,
        account: &str,
        purpose_name: &str,
        bytes: &[u8],
        now: u64,
    ) -> Result<Signature, SignError> {
        if bytes.len() > self.max_bytes {
            return Err(SignError::TooLarge {
                limit: self.max_bytes,
            });
        }
        let principal = self
            .accounts
            .get(account)
            .cloned()
            .ok_or(SignError::UnknownCaller)?;
        if NOT_SIGNED_HERE.contains(&purpose_name) {
            return Err(SignError::NotThisStep);
        }
        let purpose = Purpose::new(purpose_name, PURPOSE_LEN)
            .map_err(|_| SignError::BadPurpose)?;
        let bytes_hash: [u8; HASH_LEN] = Sha256::digest(bytes).into();
        let repeat_key = fields(&[
            principal.as_bytes(),
            purpose_name.as_bytes(),
            &bytes_hash,
        ]);
        let active = self.active_key(&principal, purpose_name);
        if let Some(earlier) = self.repeats.get(&repeat_key)
            && let Some(answer) = self.repeat(
                earlier,
                (&principal, purpose_name, &bytes_hash),
                active.as_deref(),
            )?
        {
            return Ok(answer);
        }
        let key_id = active.ok_or(SignError::NoActiveKey)?;
        let pair = self.secrets.get(&key_id).ok_or(SignError::NoActiveKey)?;
        let signature = *pair.sk.sign(purpose.prefix(bytes), None);
        self.record(
            Made {
                account,
                principal: &principal,
                purpose: purpose_name,
                key_id: &key_id,
                bytes_hash: &bytes_hash,
                signature,
                repeat_key,
            },
            now,
        )?;
        Ok(Signature { key_id, signature })
    }

    /// The answer to a repeat of the request logged at `earlier`: the
    /// logged signature, `None` to sign afresh, or a refusal. It checks,
    /// in this order, that the caller may still ask, that the
    /// logged entry's principal, purpose and bytes hash match `request`,
    /// and that the logged key is still active or retired, still the
    /// caller's and still lists the purpose; otherwise, or if the key is
    /// no longer listed, the request takes the revoked path.
    fn repeat(
        &self,
        earlier: u64,
        request: (&str, &str, &[u8; HASH_LEN]),
        active: Option<&str>,
    ) -> Result<Option<Signature>, SignError> {
        let (principal, purpose, bytes_hash) = request;
        if active.is_none() {
            return Err(SignError::NoActiveKey);
        }
        let logged = self.issued.get(&earlier).ok_or(SignError::OutOfStep)?;
        if logged.principal != principal
            || logged.purpose != purpose
            || &logged.bytes_hash != bytes_hash
        {
            return Err(SignError::OutOfStep);
        }
        let still_good =
            self.keys.get(&logged.answer.key_id).is_some_and(|k| {
                matches!(k.state, KeyState::Active | KeyState::Retired)
                    && k.principal == principal
                    && k.purposes.contains_key(purpose)
            });
        if still_good {
            return Ok(Some(logged.answer.clone()));
        }
        if NO_CLEAN_REPEAT.contains(&purpose) {
            Err(SignError::RevokedRepeat)
        } else {
            Ok(None)
        }
    }

    /// Writes the signature entry and both indexes in one step: every
    /// part that can fail is checked first, on the indexes' own `next`
    /// and a copy of the chain head, and nothing changes unless all of
    /// it worked.
    fn record(&mut self, made: Made<'_>, now: u64) -> Result<(), SignError> {
        let Made {
            account,
            principal,
            purpose,
            key_id,
            bytes_hash,
            signature,
            repeat_key,
        } = made;
        let counter = match self.chain.head() {
            None => 1,
            Some(head) => {
                head.counter.checked_add(1).ok_or(SignError::LogFull)?
            }
        };
        let next_repeat = self
            .repeats
            .next(&repeat_key, counter)
            .map_err(|_| SignError::OutOfStep)?;
        let signature_key = fields(&[key_id.as_bytes(), &signature]);
        let next_signature = self
            .by_signature
            .next(&signature_key, counter)
            .map_err(|_| SignError::OutOfStep)?;
        let body = fields(&[
            account.as_bytes(),
            principal.as_bytes(),
            purpose.as_bytes(),
            key_id.as_bytes(),
            bytes_hash,
            &signature,
            &next_repeat.running(),
            &next_signature.running(),
        ]);
        let mut chain = self.chain.clone();
        let entry = chain
            .append(Kind::Signature, now, body)
            .map_err(|_| SignError::LogFull)?;
        self.chain = chain;
        self.repeats.apply(repeat_key, next_repeat);
        self.by_signature.apply(signature_key, next_signature);
        self.entries.push(entry);
        let answer = Signature {
            key_id: key_id.to_owned(),
            signature,
        };
        let logged = Logged {
            principal: principal.to_owned(),
            purpose: purpose.to_owned(),
            bytes_hash: *bytes_hash,
            answer,
        };
        let _previous = self.issued.insert(counter, logged);
        Ok(())
    }
}
