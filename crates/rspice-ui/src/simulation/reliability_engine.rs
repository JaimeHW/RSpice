//! Reliability requests and persisted historical result types.
//!
//! Historical result types remain readable. New requests carry explicit model
//! packs and mission profiles; circuit dispatch stays blocked until extraction
//! and aged re-simulation are connected to the characterized core aging clocks.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod study;
pub use study::{
    ReliabilityBinding, ReliabilityMissionPhase, ReliabilityStudy, ReliabilityTransientWindow,
};

#[cfg(test)]
pub(crate) mod tests;

/// Accumulated stress metrics for a device
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StressMetrics {
    /// Average Vgs stress (effective)
    pub avg_vgs_stress: f64,
    /// Average Vds stress
    pub avg_vds_stress: f64,
    /// Effective temperature during stress (Kelvin)
    pub avg_temp: f64,
    /// Total stress duration (seconds)
    pub duration: f64,
}

/// Shifted parameters due to aging
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParamShift {
    pub vth_shift: f64,
    pub mobility_shift: f64,
    pub rds_shift: f64,
}

/// Reliability analysis output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReliabilityResult {
    pub device_id: String,
    pub stress: StressMetrics,
    /// Parameter shifts at specific time intervals (e.g., "10y" -> ParamShift)
    pub shifts: HashMap<String, ParamShift>,
}
