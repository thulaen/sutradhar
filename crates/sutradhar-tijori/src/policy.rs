//! Accepting a policy and answering checks from it (spec 0065 "Checking
//! signatures" items 2 and 3).
//!
//! The caller reads the policy file and its version record and decodes
//! the instance and version; this module decides whether that policy may
//! be put in force, and then answers every check from the one accepted
//! copy. It never reads a file and never writes the record.

use std::collections::HashMap;
use std::fmt;

use sha2::{Digest, Sha256};

use crate::check::{KeyEntry, Trust};
use crate::purpose::{Purpose, PurposeError};
use crate::verify::{VerifyError, verify_strict};

/// Bytes in a policy digest.
const DIGEST_LEN: usize = 32;

/// The purpose the owner signs a policy for.
const POLICY_PURPOSE: &str = "policy-change";

/// The length limit passed to `Purpose::new` when naming the
/// `policy-change` purpose; long enough for that name.
const PURPOSE_LEN: usize = 64;

/// The record of the highest policy version accepted on an instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Record {
    version: u64,
    digest: [u8; DIGEST_LEN],
}

impl Record {
    /// A record of `version` with `digest`.
    #[must_use]
    pub const fn new(version: u64, digest: [u8; DIGEST_LEN]) -> Self {
        Self { version, digest }
    }

    /// The highest version accepted.
    #[must_use]
    pub const fn version(&self) -> u64 {
        self.version
    }

    /// That version's digest.
    #[must_use]
    pub const fn digest(&self) -> [u8; DIGEST_LEN] {
        self.digest
    }
}

/// A policy as read from its file: the instance and version decoded by
/// the caller, the exact signed bytes and the owner's signature. The
/// caller must decode the instance and version from `signed` itself,
/// never from anywhere else, because the signature covers only `signed`.
#[derive(Debug, Clone, Copy)]
pub struct Policy<'a> {
    /// The instance the policy names.
    pub instance: &'a str,
    /// The policy's version.
    pub version: u64,
    /// The signed bytes: the `policy-change` purpose field, then the body.
    pub signed: &'a [u8],
    /// The owner's signature over `signed`.
    pub signature: &'a [u8; 64],
}

/// Why a policy was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyError {
    /// The record of the highest version accepted is missing; nothing is
    /// accepted until the owner resets it.
    NoRecord,
    /// The policy names another instance.
    OtherInstance,
    /// The policy is older than the one in force.
    Older {
        /// The version in force.
        in_force: u64,
    },
    /// The policy has the version in force but other bytes.
    SameVersionOtherBytes,
    /// The signed bytes do not start with the `policy-change` purpose
    /// field: they are cut short or name another purpose.
    NotPolicyChange(PurposeError),
    /// The owner's signature over these bytes does not check.
    NotOwnerSigned(VerifyError),
}

impl fmt::Display for PolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoRecord => write!(
                f,
                "the record of the policy version in force is missing; \
                 the owner must reset it"
            ),
            Self::OtherInstance => {
                write!(f, "the policy is for another instance")
            }
            Self::Older { in_force } => write!(
                f,
                "the policy is older than version {in_force}, which is in force"
            ),
            Self::SameVersionOtherBytes => write!(
                f,
                "the policy has the version in force but different contents"
            ),
            Self::NotPolicyChange(why) => {
                write!(f, "the policy is not signed as a policy change: {why}")
            }
            Self::NotOwnerSigned(why) => {
                write!(f, "the owner did not sign this policy: {why}")
            }
        }
    }
}

impl std::error::Error for PolicyError {}

/// Decides whether `policy` may be in force on `instance`, given the
/// record of the highest version accepted. Returns the record to keep.
///
/// The owner's signature covers only `policy.signed`. The caller must
/// decode `policy.instance` and `policy.version` from those signed bytes
/// and from nowhere else: this function cannot tell a mislabelled policy
/// from a real one until spec 0065 sets the policy's byte encoding and
/// the decoding moves in here.
///
/// # Errors
///
/// Refuses a missing record, a policy for another instance, an older
/// version, the same version with other bytes, and a newer version the
/// owner did not sign for `policy-change` with `owner_key`.
pub fn accept(
    instance: &str,
    record: Option<Record>,
    policy: &Policy<'_>,
    owner_key: &[u8; 32],
) -> Result<Record, PolicyError> {
    let record = record.ok_or(PolicyError::NoRecord)?;
    if policy.instance != instance {
        return Err(PolicyError::OtherInstance);
    }
    let digest: [u8; DIGEST_LEN] = Sha256::digest(policy.signed).into();
    if policy.version < record.version {
        return Err(PolicyError::Older {
            in_force: record.version,
        });
    }
    if policy.version == record.version {
        return if digest == record.digest {
            Ok(record)
        } else {
            Err(PolicyError::SameVersionOtherBytes)
        };
    }
    let purpose = Purpose::new(POLICY_PURPOSE, PURPOSE_LEN)
        .map_err(PolicyError::NotPolicyChange)?;
    // The strip checks that the signed bytes start with the
    // policy-change purpose field; the body after it is not needed here.
    let _body = purpose
        .strip(policy.signed)
        .map_err(PolicyError::NotPolicyChange)?;
    verify_strict(owner_key, policy.signed, policy.signature)
        .map_err(PolicyError::NotOwnerSigned)?;
    Ok(Record::new(policy.version, digest))
}

/// The policy in force, held in memory once accepted, answering every
/// check from that one copy.
#[derive(Debug, Clone)]
pub struct PolicyTrust {
    digest: [u8; DIGEST_LEN],
    keys: HashMap<String, KeyEntry>,
}

impl PolicyTrust {
    /// The accepted policy with `digest` and its signer registry.
    #[must_use]
    pub const fn new(
        digest: [u8; DIGEST_LEN],
        keys: HashMap<String, KeyEntry>,
    ) -> Self {
        Self { digest, keys }
    }
}

impl Trust for PolicyTrust {
    fn lookup(
        &self,
        key_id: &str,
    ) -> Option<([u8; DIGEST_LEN], Option<KeyEntry>)> {
        Some((self.digest, self.keys.get(key_id).cloned()))
    }
}
