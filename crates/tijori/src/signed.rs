//! The records the sign step builds: the signature it returns, the
//! entry it is about to log and what it keeps for a later repeat.

use crate::log::HASH_LEN;

/// A signature the core returned, with the key that made it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    /// The id of the key that signed.
    pub key_id: String,
    /// The Ed25519 signature over the purpose field and the bytes.
    pub signature: [u8; 64],
}

/// One length-prefixed field, as every signed encoding writes them.
pub(crate) fn push_field(out: &mut Vec<u8>, field: &[u8]) {
    let len = u64::try_from(field.len()).unwrap_or(u64::MAX);
    out.extend_from_slice(&len.to_be_bytes());
    out.extend_from_slice(field);
}

/// Joins `fields`, each length-prefixed, so different field lists never
/// give the same bytes.
pub(crate) fn fields(parts: &[&[u8]]) -> Vec<u8> {
    let mut out = Vec::new();
    for part in parts {
        push_field(&mut out, part);
    }
    out
}

/// One signature about to be logged.
pub(crate) struct Made<'a> {
    pub(crate) account: &'a str,
    pub(crate) principal: &'a str,
    pub(crate) purpose: &'a str,
    pub(crate) key_id: &'a str,
    pub(crate) bytes_hash: &'a [u8; HASH_LEN],
    pub(crate) signature: [u8; 64],
    pub(crate) repeat_key: Vec<u8>,
}

/// What the core keeps for each logged signature, so a repeat can check
/// that the entry it found is for the same request.
pub(crate) struct Logged {
    pub(crate) principal: String,
    pub(crate) purpose: String,
    pub(crate) bytes_hash: [u8; HASH_LEN],
    pub(crate) answer: Signature,
}
