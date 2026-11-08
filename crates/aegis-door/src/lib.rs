//! aegis-door: Forgejo's door. Forgejo's pre-receive hook sends each push
//! of `main` here, and the push is accepted only if every check passes
//! (AGENTS.md sections 7 and 8).

pub mod gate;
pub mod http;
pub mod serve;
