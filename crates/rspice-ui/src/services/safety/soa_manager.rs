//! Safe Operating Area (SOA) Manager
//!
//! Provides commercial-grade safety checking for circuit devices.
//! Monitors voltages, currents, and power dissipation against device limits.
//!
//! # Features
//!
//! - Multi-parameter limit checking (Vgs, Vds, Vbe, Vce, Id, Pdiss)
//! - Dynamic limit calculation based on temperature/operating conditions
//! - Violation tracking with peak/duration metrics
//! - Integration with Schematic and Waveform viewers for visual alerts

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// SOA Limits
// =============================================================================

/// Type of SOA limit parameter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SoAParameter {
    Vgs,
    Vds,
    Vgd,
    Vbe,
    Vce,
    Vbc,
    Id,
    Ic,
    Pdiss,
    Temp,
}

/// A specific limit definition for a device type or model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoALimit {
    pub parameter: SoAParameter,
    pub max_value: f64,
    pub min_value: Option<f64>,
    /// Duration allowed at violation (seconds) before fatal failure
    pub max_duration: Option<f64>,
    pub unit: String,
    pub description: String,
}

/// Collection of limits for a specific device instance or model
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SoADefinition {
    pub limits: Vec<SoALimit>,
}

impl SoADefinition {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_limit(&mut self, limit: SoALimit) {
        self.limits.push(limit);
    }
}

// =============================================================================
// SOA Violations
// =============================================================================

/// Details of a safety limit violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoAViolation {
    pub device_id: String,
    pub parameter: SoAParameter,
    pub limit_value: f64,
    pub actual_value: f64,
    pub time: f64, // Simulation time of violation
    pub severity: ViolationSeverity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ViolationSeverity {
    /// Near limit (e.g. > 90%)
    Warning,
    /// Limit exceeded
    Violation,
    /// Significant exceedence / Fatal path
    Critical,
}

// =============================================================================
// SOA Manager
// =============================================================================

/// Manager for tracking and checking Safe Operating Area limits
pub struct SoAManager {
    /// Device instance ID -> SOA Definition
    device_defs: HashMap<String, SoADefinition>,
    /// Accumulated violations from most recent check
    violations: Vec<SoAViolation>,
}

impl Default for SoAManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SoAManager {
    pub fn new() -> Self {
        Self {
            device_defs: HashMap::new(),
            violations: Vec::new(),
        }
    }

    /// Register SOA limits for a device
    pub fn register_device(&mut self, device_id: impl Into<String>, def: SoADefinition) {
        self.device_defs.insert(device_id.into(), def);
    }

    /// Clear all violations
    pub fn clear_violations(&mut self) {
        self.violations.clear();
    }

    /// Check a single measurement point for all registered devices
    pub fn check_point(&mut self, time: f64, values: &HashMap<String, HashMap<SoAParameter, f64>>) {
        for (device_id, device_values) in values {
            if let Some(def) = self.device_defs.get(device_id) {
                for limit in &def.limits {
                    if let Some(&actual) = device_values.get(&limit.parameter) {
                        if actual > limit.max_value {
                            let severity = if actual > limit.max_value * 1.2 {
                                ViolationSeverity::Critical
                            } else {
                                ViolationSeverity::Violation
                            };

                            self.violations.push(SoAViolation {
                                device_id: device_id.clone(),
                                parameter: limit.parameter,
                                limit_value: limit.max_value,
                                actual_value: actual,
                                time,
                                severity,
                            });
                        } else if actual > limit.max_value * 0.9 {
                            self.violations.push(SoAViolation {
                                device_id: device_id.clone(),
                                parameter: limit.parameter,
                                limit_value: limit.max_value,
                                actual_value: actual,
                                time,
                                severity: ViolationSeverity::Warning,
                            });
                        }
                    }
                }
            }
        }
    }

    /// Get all detected violations
    pub fn violations(&self) -> &[SoAViolation] {
        &self.violations
    }

    /// Group violations by device ID
    pub fn violations_by_device(&self) -> HashMap<String, Vec<&SoAViolation>> {
        let mut map: HashMap<String, Vec<&SoAViolation>> = HashMap::new();
        for v in &self.violations {
            map.entry(v.device_id.clone()).or_default().push(v);
        }
        map
    }
}

// =============================================================================
// Tests
// =============================================================================

