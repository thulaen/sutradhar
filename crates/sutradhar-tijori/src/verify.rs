//! Strict Ed25519 checks (spec 0065 "Checking signatures" item 5).
//!
//! The check needs no private key. It refuses a public key that is not a
//! point on the curve or has small order, and a signature that is not in
//! canonical form or whose `R` has small order.

use std::fmt;

use ed25519_compact::{Error, PublicKey, Signature};

/// Why a signature was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyError {
    /// The public key is not a valid point on the curve.
    BadKey,
    /// The public key has small order, so it would accept forged
    /// signatures.
    WeakKey,
    /// The signature does not match the key and bytes, or is not in
    /// canonical form.
    BadSignature,
}

impl fmt::Display for VerifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadKey => write!(f, "the public key is not a valid key"),
            Self::WeakKey => write!(
                f,
                "the public key is weak and could accept forged signatures"
            ),
            Self::BadSignature => {
                write!(f, "the signature does not match the signed bytes")
            }
        }
    }
}

impl std::error::Error for VerifyError {}

/// Checks `signature` over `signed` with `public_key`, under the strict
/// rules.
///
/// # Errors
///
/// Refuses an invalid or weak key, and any signature that does not
/// match or is not in canonical form.
pub fn verify_strict(
    public_key: &[u8; 32],
    signed: &[u8],
    signature: &[u8; 64],
) -> Result<(), VerifyError> {
    let key =
        PublicKey::from_slice(public_key).map_err(|_| VerifyError::BadKey)?;
    let signature = Signature::from_slice(signature)
        .map_err(|_| VerifyError::BadSignature)?;
    key.verify(signed, &signature).map_err(|why| match why {
        Error::WeakPublicKey => VerifyError::WeakKey,
        Error::InvalidPublicKey => VerifyError::BadKey,
        _ => VerifyError::BadSignature,
    })
}
