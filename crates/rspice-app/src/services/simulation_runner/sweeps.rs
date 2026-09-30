//! Parametric and corner sweep runners.

#![allow(clippy::needless_range_loop, clippy::type_complexity)]

mod mapping;
mod parametric;
mod types;

pub(crate) use mapping::{map_corner_results, map_temperature_results};
pub(crate) use types::SweepPointResult;

pub use parametric::{
    run_parametric_analysis_with_base_and_source_path_and_abort,
    run_parametric_analysis_with_source_path_and_abort,
};
