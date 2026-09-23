//! Periodic AC and periodic transfer function analyses.
//!
//! Both linearize about a periodic steady state and share that setup, so the
//! shared half lives here alongside the two runners.

#![allow(clippy::type_complexity)]

mod pac;
mod pxf;
mod shared;
pub(crate) use pac::{PacData, run_pac_analysis_on_materialized_with_abort};
pub(crate) use pxf::{PxfData, run_pxf_analysis_on_materialized_with_abort};

pub use pac::{PacFrequencySweep, PacRunConfig};
pub use pxf::{PxfFrequencySweep, PxfRunConfig};
#[cfg(test)]
pub use pac::run_pac_analysis_from_hb_with_source_path_and_abort;
#[cfg(test)]
pub use pxf::run_pxf_analysis_from_hb_with_source_path_and_abort;
