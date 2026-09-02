//! Self-contained RSpice engine executor for the credentialless cloud worker
//! protocol. The binary in `main.rs` is the deliverable; this library surface
//! exists so integration tests and release tooling construct
//! byte-exact requests through the same code the executor validates with.

pub mod document;
pub mod execute;
pub mod measure;
pub mod result_document;
pub mod wire;
