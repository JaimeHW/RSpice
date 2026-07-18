//! Parametric and corner sweep runners.

#![allow(clippy::needless_range_loop, clippy::type_complexity)]

use super::ServiceRunError;

mod corner;
mod execution;
mod mapping;
mod netlist_mutation;
mod parametric;
mod sweep_points;
mod types;

pub(crate) use sweep_points::expand_step_sweep_values;
pub(crate) use types::{REFERENCE_MODEL_BINDING_BEGIN, REFERENCE_MODEL_BINDING_END};

/// Expand exactly the PVT tuples consumed by the corner runner without
/// exposing its internal result type. Preflight uses this same expansion so
/// its point count cannot drift from execution semantics.
pub(crate) fn expand_corner_pvt_points(
    config: &CornerRunConfig,
) -> Result<Vec<(CornerProcess, rspice_core::Value, rspice_core::Value)>, ServiceRunError> {
    execution::expand_corner_points(
        config,
        rspice_core::ResourceLimits::default().max_batch_runs,
    )
    .map(|points| {
        points
            .into_iter()
            .map(|point| (point.process, point.voltage, point.temperature_c))
            .collect()
    })
}

pub use corner::{
    run_corner_analysis, run_corner_analysis_with_abort, run_corner_analysis_with_config,
    run_corner_analysis_with_config_and_abort, run_corner_analysis_with_config_and_source_path,
    run_corner_analysis_with_config_and_source_path_and_abort,
    run_corner_analysis_with_source_path, run_corner_analysis_with_source_path_and_abort,
};
pub use parametric::{
    run_parametric_analysis, run_parametric_analysis_with_abort,
    run_parametric_analysis_with_config, run_parametric_analysis_with_config_and_abort,
    run_parametric_analysis_with_config_and_source_path,
    run_parametric_analysis_with_config_and_source_path_and_abort,
    run_parametric_analysis_with_source_path, run_parametric_analysis_with_source_path_and_abort,
};
pub use types::{
    CornerBaseMode, CornerData, CornerFrequencySweep, CornerModelBinding, CornerProcess,
    CornerRunConfig, ParametricData, TempRunConfig,
};
