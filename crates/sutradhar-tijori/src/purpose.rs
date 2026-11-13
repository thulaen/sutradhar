//! The purpose field (spec 0065 "Signing purposes", spec 0064 "Evidence
//! records and groups").
//!
//! Every signed encoding starts with fields, each prefixed with its byte
//! length as an unsigned 64-bit big-endian integer. The first field is
//! the signing purpose. The core writes it; the caller's bytes start at
//! the next field.

use std::fmt;

/// Bytes in the length prefix of each field.
const PREFIX_LEN: usize = 8;

/// A checked purpose name: one to `max_len` bytes, each a lower-case
/// ASCII letter, a digit or a hyphen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Purpose(String);

/// Why a purpose or a signed encoding was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PurposeError {
    /// The name is empty or longer than the policy allows.
    BadLength {
        /// The name's length in bytes.
        len: usize,
        /// The policy's largest purpose length.
        max: usize,
    },
    /// The name holds a byte other than `a`-`z`, `0`-`9` or `-`.
    BadCharacter {
        /// Where the first bad byte is.
        at: usize,
    },
    /// The bytes end before the length prefix or the field it names.
    Truncated,
    /// The first field names another purpose than the one expected.
    WrongPurpose {
        /// The purpose the caller expected.
        expected: String,
    },
}

impl fmt::Display for PurposeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadLength { len, max } => {
                write!(
                    f,
                    "the purpose name is {len} bytes; it must be 1 to {max}"
                )
            }
            Self::BadCharacter { at } => write!(
                f,
                "the purpose name has a character at byte {at} that is not \
                 a lower-case letter, a digit or a hyphen"
            ),
            Self::Truncated => {
                write!(f, "the signed bytes end before the purpose field does")
            }
            Self::WrongPurpose { expected } => {
                write!(f, "the signed bytes are not for the purpose {expected}")
            }
        }
    }
}

impl std::error::Error for PurposeError {}

impl Purpose {
    /// Checks `name` against the purpose-name rule, with `max_len` the
    /// policy's largest purpose length.
    ///
    /// # Errors
    ///
    /// Refuses an empty or over-long name, or one with any byte other
    /// than a lower-case ASCII letter, a digit or a hyphen.
    pub fn new(name: &str, max_len: usize) -> Result<Self, PurposeError> {
        let len = name.len();
        if len == 0 || len > max_len {
            return Err(PurposeError::BadLength { len, max: max_len });
        }
        if let Some(at) = name.bytes().position(|b| {
            !(b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        }) {
            return Err(PurposeError::BadCharacter { at });
        }
        Ok(Self(name.to_owned()))
    }

    /// The purpose name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The bytes the core signs: the purpose field, then the caller's
    /// bytes unchanged.
    #[must_use]
    pub fn prefix(&self, caller_bytes: &[u8]) -> Vec<u8> {
        let name = self.0.as_bytes();
        // A purpose name is at most the policy's length, so it always fits
        // in a u64; the capacity is only a hint, so saturating is safe.
        let len = u64::try_from(name.len()).unwrap_or(u64::MAX);
        let mut out = Vec::with_capacity(
            PREFIX_LEN
                .saturating_add(name.len())
                .saturating_add(caller_bytes.len()),
        );
        out.extend_from_slice(&len.to_be_bytes());
        out.extend_from_slice(name);
        out.extend_from_slice(caller_bytes);
        out
    }

    /// Reads the purpose field from the front of `signed`, refuses it
    /// unless it is exactly this purpose, and returns the caller's bytes
    /// that follow.
    ///
    /// # Errors
    ///
    /// Refuses bytes too short for the prefix or the field, and a field
    /// that names any other purpose.
    pub fn strip<'a>(
        &self,
        signed: &'a [u8],
    ) -> Result<&'a [u8], PurposeError> {
        let (len_bytes, rest) = signed
            .split_first_chunk::<PREFIX_LEN>()
            .ok_or(PurposeError::Truncated)?;
        let len = usize::try_from(u64::from_be_bytes(*len_bytes))
            .map_err(|_| PurposeError::Truncated)?;
        if len > rest.len() {
            return Err(PurposeError::Truncated);
        }
        let (field, caller_bytes) = rest.split_at(len);
        if field != self.0.as_bytes() {
            return Err(PurposeError::WrongPurpose {
                expected: self.0.clone(),
            });
        }
        Ok(caller_bytes)
    }
}
