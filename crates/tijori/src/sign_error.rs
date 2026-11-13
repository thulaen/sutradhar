//! Why the signing core refused to sign (spec 0065 "Signing").

use std::fmt;

/// Why a request to sign was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignError {
    /// The caller's account is bound to no principal.
    UnknownCaller,
    /// The caller's principal has no active key for this purpose.
    NoActiveKey,
    /// The purpose is one another step signs, or one a later part adds
    /// (the list is `NOT_SIGNED_HERE` in `purposes`).
    NotThisStep,
    /// The bytes were last signed under a key since revoked, and this
    /// purpose is never signed again until the owner's recovery plan.
    RevokedRepeat,
    /// The bytes are longer than the limit.
    TooLarge {
        /// The largest number of bytes allowed.
        limit: usize,
    },
    /// The purpose name breaks the purpose-name rule.
    BadPurpose,
    /// The log cannot take another entry; signing stops and the owner
    /// is told.
    LogFull,
    /// The log and its indexes disagree; signing stops and the owner is
    /// told.
    OutOfStep,
}

impl fmt::Display for SignError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownCaller => {
                write!(f, "this account may not ask for signatures")
            }
            Self::NoActiveKey => {
                write!(f, "this caller has no active key for this purpose")
            }
            Self::NotThisStep => {
                write!(f, "this purpose is not signed through this step")
            }
            Self::RevokedRepeat => write!(
                f,
                "these bytes were signed under a revoked key and are not \
                 signed again until the owner's recovery plan has run"
            ),
            Self::TooLarge { limit } => {
                write!(f, "the bytes are longer than {limit} bytes")
            }
            Self::BadPurpose => write!(f, "the purpose name is not allowed"),
            Self::LogFull => write!(
                f,
                "the signature log cannot take another entry; signing has \
                 stopped and the owner must recover"
            ),
            Self::OutOfStep => write!(
                f,
                "the signature log and its indexes disagree; signing has \
                 stopped and the owner must recover"
            ),
        }
    }
}

impl std::error::Error for SignError {}
