//! Smith chart viewer state.
//!
//! The chart itself is drawn by `workbench::documents::result_document::smith`,
//! which owns the geometry and the Γ → Z conversion at the cursor. This module
//! is only the data behind it: the reference impedance and the S-parameter
//! traces a run produced.
//!
//! It used to also carry chart modes, markers, VSWR circles, constant-R/X
//! circle math, and its own complex-arithmetic and impedance/admittance types
//! — a full RF chart toolkit, none of it reachable. The complex type in
//! particular duplicated `num_complex`, which the rest of the crate already
//! uses; the survivors here are on `Complex64` now.

pub(crate) mod state;

pub use state::SmithChartState;
