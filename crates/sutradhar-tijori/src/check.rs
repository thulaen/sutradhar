//! The signature check (spec 0065 "Checking signatures" items 2 and 4).
//!
//! The library never reads files itself. It gets the signer registry and
//! the policy in force through the [`Trust`] trait the caller passes in.

use std::collections::BTreeMap;
use std::fmt;

use crate::purpose::{Purpose, PurposeError};
use crate::verify::{VerifyError, verify_strict};

/// A key's state in the registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    /// The key may sign, and its signatures pass.
    Active,
    /// The key was replaced by rotation; its private half is destroyed,
    /// and its signatures still pass.
    Retired,
    /// The key was revoked; its signatures never pass this check.
    Revoked,
}

/// One key in the signer registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEntry {
    /// The key's public half.
    pub public_key: [u8; 32],
    /// The principal the key belongs to.
    pub principal: String,
    /// Each purpose the key may sign for, with the scopes its owning
    /// spec defines. The library returns the scopes and never reads them.
    pub purposes: BTreeMap<String, Vec<String>>,
    /// The key's state.
    pub state: KeyState,
}

/// What the policy in force says. The caller's trust source reads and
/// checks the policy file; the library only asks it questions.
pub trait Trust {
    /// One answer from one copy of the policy in force: its digest and
    /// the registry entry for `key_id`, if any. Both come from the same
    /// copy, so a reload between them cannot mix two policies. `None`
    /// when the policy or its version record cannot be read, in which
    /// case every check is refused.
    fn lookup(&self, key_id: &str) -> Option<([u8; 32], Option<KeyEntry>)>;
}

/// What a passing check returns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checked {
    /// The principal that signed.
    pub principal: String,
    /// The key's scopes for this purpose.
    pub scopes: Vec<String>,
}

/// Why a check was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckError {
    /// The policy or its version record cannot be read.
    NoPolicy,
    /// The caller expects another policy than the one in force.
    PolicyNotInForce,
    /// No key has this id.
    UnknownKey,
    /// The key was revoked.
    Revoked,
    /// The key may not sign for this purpose.
    WrongPurpose,
    /// The signed bytes do not start with this purpose's field: they are
    /// cut short, carry an impossible length or name another purpose.
    BadPurposeField(PurposeError),
    /// The signature, or the key, failed the strict check.
    BadSignature(VerifyError),
}

impl fmt::Display for CheckError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoPolicy => write!(
                f,
                "the signing policy cannot be read, so no signature is checked"
            ),
            Self::PolicyNotInForce => {
                write!(f, "the caller expects a signing policy not in force")
            }
            Self::UnknownKey => {
                write!(f, "no key in the signing policy has this id")
            }
            Self::Revoked => write!(f, "the key was revoked"),
            Self::WrongPurpose => {
                write!(f, "the key may not sign for this purpose")
            }
            Self::BadPurposeField(why) => write!(f, "{why}"),
            Self::BadSignature(why) => write!(f, "{why}"),
        }
    }
}

impl std::error::Error for CheckError {}

/// One signature to check.
#[derive(Debug, Clone, Copy)]
pub struct Signed<'a> {
    /// The id of the key that signed.
    pub key_id: &'a str,
    /// The purpose the caller expects the signature to be for.
    pub purpose: &'a Purpose,
    /// The signed bytes: the purpose field, then the caller's bytes.
    pub bytes: &'a [u8],
    /// The signature.
    pub signature: &'a [u8; 64],
}

/// Checks `signed` under the policy in force, which must be the one the
/// caller expects.
///
/// # Errors
///
/// Refuses when the policy cannot be read or is not the one the caller
/// expects, when the key is unknown, revoked or not allowed `purpose`,
/// when the purpose field is not `purpose`, and when the signature fails
/// the strict check.
pub fn check(
    trust: &impl Trust,
    signed: &Signed<'_>,
    expected_policy: &[u8; 32],
) -> Result<Checked, CheckError> {
    let Signed {
        key_id,
        purpose,
        bytes,
        signature,
    } = *signed;
    let (in_force, entry) = trust.lookup(key_id).ok_or(CheckError::NoPolicy)?;
    if &in_force != expected_policy {
        return Err(CheckError::PolicyNotInForce);
    }
    let key = entry.ok_or(CheckError::UnknownKey)?;
    if key.state == KeyState::Revoked {
        return Err(CheckError::Revoked);
    }
    let scopes = key
        .purposes
        .get(purpose.as_str())
        .ok_or(CheckError::WrongPurpose)?;
    let _caller_bytes =
        purpose.strip(bytes).map_err(CheckError::BadPurposeField)?;
    verify_strict(&key.public_key, bytes, signature)
        .map_err(CheckError::BadSignature)?;
    Ok(Checked {
        principal: key.principal,
        scopes: scopes.clone(),
    })
}
