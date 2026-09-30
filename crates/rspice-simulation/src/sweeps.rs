//! Sweep configuration and bounded point preparation shared by execution hosts.

use crate::error::ServiceRunError;
use rspice_app_types::product::ProcessCorner;

mod config;
mod corner_points;
mod step_points;
mod worker;

pub use config::{
    CornerBaseMode, CornerFrequencySweep, CornerMetricLabel, CornerPoint, CornerRunConfig,
    TempRunConfig, validate_base_mode,
};
pub use step_points::{
    StepSweepExpandError, expand_step_sweep_values, expand_step_sweep_values_with_abort,
};

/// Expand the PVT tuples declared by a corner configuration using the default
/// batch limit. Callers validate the configuration before preparing tasks;
/// this bounded transformation neither executes nor authorizes a run.
pub fn expand_corner_pvt_points(
    config: &CornerRunConfig,
) -> Result<Vec<(ProcessCorner, rspice_core::Value, rspice_core::Value)>, ServiceRunError> {
    corner_points::expand_corner_points(
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
