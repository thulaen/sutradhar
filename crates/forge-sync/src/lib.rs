//! forge-sync: copies Forgejo to GitHub one way, sending only allowed
//! fields, and brings new GitHub issues and comments back into Forgejo
//! (AGENTS.md section 9.1).

pub mod apply;
pub mod fetch;
pub mod guard;
pub mod model;
pub mod plan;
pub mod record;
pub mod run;
pub mod sync;
