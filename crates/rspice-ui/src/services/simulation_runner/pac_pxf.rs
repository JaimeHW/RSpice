//! Periodic AC and periodic transfer function analyses.
//!
//! Both linearize about a periodic steady state and share that setup, so the
//! shared half lives here alongside the two runners.

#![allow(clippy::type_complexity)]

mod pac;
mod pxf;
mod shared;

pub use pac::{
    PacFrequencySweep, PacRunConfig, run_pac_analysis_from_pss_with_source_path_and_abort,
};
pub use pxf::{
    PxfFrequencySweep, PxfRunConfig, run_pxf_analysis_from_pss_with_source_path_and_abort,
};
