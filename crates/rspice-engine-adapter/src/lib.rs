//! Self-contained RSpice engine executor for the credentialless cloud worker
//! protocol. The binary in `main.rs` is the deliverable; this library surface
//! exists so integration tests and the qualification-corpus tooling construct
//! byte-exact requests through the same code the executor validates with.

pub mod document;
pub mod execute;
pub mod measure;
pub mod wire;
