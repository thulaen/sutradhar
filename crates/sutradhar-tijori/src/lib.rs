//! tijori's check library (spec 0065). It holds no private key.
//! `purpose` reads and writes the purpose field that spec 0064 puts
//! first in every signed encoding; `verify` is the strict Ed25519 check;
//! `check` checks a signature against the signer registry in force;
//! `policy` decides whether a policy may be put in force and answers
//! checks from the one accepted copy.

pub mod check;
pub mod policy;
pub mod purpose;
pub mod verify;
