//! Which purposes the sign step refuses or treats apart (spec 0065
//! "Signing" items 2 and 3, "Release guards" item 1).

/// Purposes `sign` refuses: the release purposes, which only
/// `sign_release` signs; `log-head`, which only the instance key signs;
/// `head-confirmation`, which only the relay check signs; the purposes
/// the owner signs after approving in person, a later part; and
/// `site-message`, whose own first-line rule is a later part.
pub(crate) const NOT_SIGNED_HERE: [&str; 11] = [
    "release-record",
    "release-manifest",
    "log-head",
    "head-confirmation",
    "waiver",
    "policy-change",
    "policy-record-reset",
    "site-key-record-reset",
    "site-key-change",
    "recovery",
    "site-message",
];

/// Purposes that bind a key to the release step.
pub(crate) const RELEASE_PURPOSES: [&str; 2] =
    ["release-record", "release-manifest"];

/// Purposes whose revoked-key repeats are refused until recovery.
pub(crate) const NO_CLEAN_REPEAT: [&str; 3] =
    ["runner-result", "evidence-bundle", "review-result"];

/// The longest purpose name this part accepts; the policy's own limit
/// replaces it when the policy file is read.
pub(crate) const PURPOSE_LEN: usize = 64;
