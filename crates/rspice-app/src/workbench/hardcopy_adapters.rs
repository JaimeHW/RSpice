//! Resolve live application documents for hardcopy output and coordinate host printing.
//!
//! Frozen-source rendering belongs to `rspice-hardcopy`; persisted settings and
//! authenticated source contracts belong to `rspice-hardcopy-contract`.

pub(crate) mod print;
pub(crate) mod render;
pub(crate) mod sources;
