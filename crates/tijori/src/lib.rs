//! tijori's signing core (spec 0065). Only this program holds private
//! keys. `log` and `index` are the signature log and its two indexes;
//! `core` is the sign step that writes them, refusing the purposes
//! `purposes` lists; `signed` holds the records it builds and returns;
//! `sign_error` says why a request was refused.

pub mod core;
pub mod index;
pub mod log;
mod purposes;
pub mod sign_error;
pub mod signed;
