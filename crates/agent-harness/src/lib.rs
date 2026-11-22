//! The agent harness (spec 0155): the program that holds the tokens and
//! acts for locked agent accounts, so an agent never holds a token.
//!
//! This crate holds the pieces that need no network and no tokens: the
//! strict parsing and checking of one request, and the rules that every
//! later step relies on.

pub mod request;
mod request_check;
mod request_parts;
