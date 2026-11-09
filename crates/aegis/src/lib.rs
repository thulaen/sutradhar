//! aegis: the gate. Every check Sutradhar runs at commit, push and
//! release. The rules it enforces are in `AGENTS.md`.

pub mod catalogue;
pub mod commit_msg;
pub mod docs;
pub mod file_kinds;
pub mod finding;
pub mod history;
pub mod leaks;
pub mod limits;
pub mod markers;
pub mod metrics;
pub mod names;
pub mod packages;
pub mod patch;
pub mod private;
pub mod proofs;
pub mod red_green;
pub mod repo;
pub mod rule_file;
pub mod rust_tokens;
pub mod secrets;
pub mod shell;
pub mod tdd;
pub mod text;
pub mod tiers;
pub mod tool;
pub mod workflows;

pub use finding::Finding;
