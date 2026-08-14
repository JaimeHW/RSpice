//! Persisted reliability-result types.
//!
//! These types remain readable for historical projects. New reliability runs
//! are blocked until execution is backed by explicit PDK aging models; this
//! module deliberately contains no generic or hard-coded aging equations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
