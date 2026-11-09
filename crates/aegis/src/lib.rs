//! aegis: the gate. Every check Sutradhar runs at commit, push and
//! release. The rules it enforces are in `AGENTS.md`.

pub mod analysis;
pub mod catalogue;
pub mod commit_msg;
pub mod copies;
pub mod docs;
pub mod file_kinds;
pub mod finding;
pub mod git_env;
pub mod history;
pub mod images;
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
pub mod reviews;
pub mod rule_file;
pub mod run_outcome;
pub mod rust_steps;
pub mod rust_tokens;
pub mod sbom;
pub mod score;
pub mod secrets;
pub mod shell;
pub mod tdd;
pub mod text;
pub mod tiers;
pub mod tool;
pub mod tools_list;
pub mod workflows;
pub mod yaml_safe;

pub use finding::Finding;
