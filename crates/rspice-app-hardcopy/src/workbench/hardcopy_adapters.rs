//! Resolve live application documents for hardcopy output and coordinate host printing.
//!
//! Frozen-source rendering belongs to `rspice-hardcopy`; persisted settings and
//! authenticated source contracts belong to `rspice-hardcopy-contract`. The
//! sealed publication snapshot compiles the same resolved sources into the
//! scenes `rspice-publication-contract` carries.

pub(crate) mod print;
pub(crate) mod publication_snapshot;
pub(crate) mod sources;

#[cfg(test)]
pub(crate) mod tests;
