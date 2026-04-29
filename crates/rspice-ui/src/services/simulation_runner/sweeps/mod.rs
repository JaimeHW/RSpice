//! Parametric and corner sweep runners.

#![allow(clippy::needless_range_loop, clippy::type_complexity)]

mod corner;
mod execution;
mod mapping;
mod netlist_mutation;
mod parametric;
mod sweep_points;
mod types;

pub use corner::{
    run_corner_analysis, run_corner_analysis_with_config,
    run_corner_analysis_with_config_and_source_path, run_corner_analysis_with_source_path,
};
pub use parametric::{
    run_parametric_analysis, run_parametric_analysis_with_config,
    run_parametric_analysis_with_config_and_source_path, run_parametric_analysis_with_source_path,
};
pub use types::{
    CornerBaseMode, CornerData, CornerFrequencySweep, CornerProcess, CornerRunConfig,
    ParametricData, TempRunConfig,
};
