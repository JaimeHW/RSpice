//! Self-contained RSpice engine executor for the credentialless cloud worker
//! protocol. The binary in `main.rs` is the deliverable; this library surface
//! exists so integration tests and release tooling construct
//! byte-exact requests through the same code the executor validates with.
//!
//! Result documents are `rspice_core::execution::AnalysisResultDocument`. The
//! adapter owns no result schema of its own except the transient `.FFT`
//! bundle in [`fft_result_document`], and the STEP/TEMP orchestration record
//! in [`axis_execution_document`], which references the shared documents
//! rather than restating them.

pub mod axis_execution_document;
pub mod document;
pub mod execute;
mod failure;
pub mod family;
pub mod fft_result_document;
pub mod measure;
pub mod result_artifact;
pub mod wire;
