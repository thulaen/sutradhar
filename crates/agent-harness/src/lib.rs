//! The agent harness (spec 0155): the program that holds the tokens and
//! acts for locked agent accounts, so an agent never holds a token.
//!
//! This crate holds the pieces that need no network and no tokens: the
//! strict parsing and checking of one request, and the rules that every
//! later step relies on.

#[macro_use]
mod refusal;

pub mod agent;
pub mod git_read;
mod git_run;
pub mod limits;
pub mod log_chain;
mod log_entry;
pub mod mailbox_rules;
pub mod push_check;
pub mod request;
mod request_check;
mod request_parts;
pub mod summary;
pub mod task_rules;
