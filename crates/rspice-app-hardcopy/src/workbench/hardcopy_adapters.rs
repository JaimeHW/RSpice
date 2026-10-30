//! Resolve live application documents for hardcopy output and coordinate host printing.
//!
//! Frozen-source rendering belongs to `rspice-hardcopy`; persisted settings and
//! authenticated source contracts belong to `rspice-hardcopy-contract`. The
//! sealed publication snapshot compiles the same resolved sources into the
//! scenes `rspice-publication-contract` carries.

pub mod print;
pub mod publication_snapshot;
pub mod sources;

#[cfg(any(test, feature = "test-support"))]
pub mod tests;
