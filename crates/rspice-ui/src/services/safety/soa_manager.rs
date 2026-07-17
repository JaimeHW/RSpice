//! Safe Operating Area (SOA) Manager
//!
//! Provides commercial-grade safety checking for circuit devices.
//! Monitors configured terminal-voltage magnitudes against device limits.
//!
//! # Features
//!
//! - Multi-parameter limit checking (Vgs, Vds, Vbe, Vce)
//! - Warning, violation, and critical threshold classification
//! - Complete sampled-rule coverage and exact worst-point retention
//! - Integration with Schematic and Waveform viewers for visual alerts

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// SOA Limits
// =============================================================================

/// Type of SOA limit parameter
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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

/// Verdict for one completely evaluated SOA rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SoARuleVerdict {
    Pass,
    Warning,
    Violation,
    Critical,
}

/// Worst observed point and coverage for one device/parameter rule.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SoAEvaluation {
    pub device_id: String,
    pub parameter: SoAParameter,
    pub limit_value: f64,
    pub worst_actual_value: f64,
    pub worst_time: f64,
    pub sample_count: u64,
    pub unit: String,
    pub description: String,
    pub verdict: SoARuleVerdict,
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
    /// Complete evaluated-rule coverage, keyed by stable device/parameter identity.
    evaluations: HashMap<(String, SoAParameter), SoAEvaluation>,
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
            evaluations: HashMap::new(),
        }
    }

    /// Register SOA limits for a device
    pub fn register_device(
        &mut self,
        device_id: impl Into<String>,
        def: SoADefinition,
    ) -> Result<(), String> {
        let device_id = device_id.into();
        if device_id.trim().is_empty() {
            return Err("SOA device identity is empty".to_owned());
        }
        if def.limits.is_empty() {
            return Err(format!("SOA device '{device_id}' has no enabled rules"));
        }
        let mut parameters = std::collections::HashSet::new();
        for limit in &def.limits {
            if !parameters.insert(limit.parameter) {
                return Err(format!(
                    "SOA device '{device_id}' has duplicate rules for {:?}",
                    limit.parameter
                ));
            }
            if !limit.max_value.is_finite() || limit.max_value <= 0.0 {
                return Err(format!(
                    "SOA device '{device_id}' has an invalid {:?} limit",
                    limit.parameter
                ));
            }
            if limit.unit.trim().is_empty() || limit.description.trim().is_empty() {
                return Err(format!(
                    "SOA device '{device_id}' has incomplete {:?} rule metadata",
                    limit.parameter
                ));
            }
        }
        if self.device_defs.contains_key(&device_id) {
            return Err(format!("SOA device '{device_id}' was registered twice"));
        }
        self.device_defs.insert(device_id, def);
        Ok(())
    }

    /// Clear all violations
    pub fn clear_violations(&mut self) {
        self.violations.clear();
        self.evaluations.clear();
    }

    /// Check a single measurement point for all registered devices
    pub fn check_point(
        &mut self,
        time: f64,
        values: &HashMap<String, HashMap<SoAParameter, f64>>,
    ) -> Result<(), String> {
        if !time.is_finite() || time < 0.0 {
            return Err("SOA sample time must be finite and nonnegative".to_owned());
        }
        for (device_id, device_values) in values {
            if let Some(def) = self.device_defs.get(device_id) {
                for limit in &def.limits {
                    if let Some(&actual) = device_values.get(&limit.parameter) {
                        if !actual.is_finite() || actual < 0.0 {
                            return Err(format!(
                                "SOA device '{device_id}' has an invalid {:?} sample",
                                limit.parameter
                            ));
                        }
                        let verdict = rule_verdict(actual, limit.max_value);
                        let key = (device_id.clone(), limit.parameter);
                        let evaluation =
                            self.evaluations
                                .entry(key)
                                .or_insert_with(|| SoAEvaluation {
                                    device_id: device_id.clone(),
                                    parameter: limit.parameter,
                                    limit_value: limit.max_value,
                                    worst_actual_value: actual,
                                    worst_time: time,
                                    sample_count: 0,
                                    unit: limit.unit.clone(),
                                    description: limit.description.clone(),
                                    verdict,
                                });
                        evaluation.sample_count =
                            evaluation.sample_count.checked_add(1).ok_or_else(|| {
                                format!("SOA sample count overflow for device '{device_id}'")
                            })?;
                        if actual > evaluation.worst_actual_value {
                            evaluation.worst_actual_value = actual;
                            evaluation.worst_time = time;
                            evaluation.verdict = verdict;
                        }
                        let severity = match verdict {
                            SoARuleVerdict::Pass => None,
                            SoARuleVerdict::Warning => Some(ViolationSeverity::Warning),
                            SoARuleVerdict::Violation => Some(ViolationSeverity::Violation),
                            SoARuleVerdict::Critical => Some(ViolationSeverity::Critical),
                        };
                        if let Some(severity) = severity {
                            self.violations.push(SoAViolation {
                                device_id: device_id.clone(),
                                parameter: limit.parameter,
                                limit_value: limit.max_value,
                                actual_value: actual,
                                time,
                                severity,
                            });
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Get all detected violations
    pub fn violations(&self) -> &[SoAViolation] {
        &self.violations
    }

    /// Iterate the complete evaluated-rule set. Callers that persist or
    /// transport these records must impose canonical ordering.
    pub fn evaluations(&self) -> impl Iterator<Item = &SoAEvaluation> {
        self.evaluations.values()
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

fn rule_verdict(actual: f64, maximum: f64) -> SoARuleVerdict {
    if actual > maximum * 1.2 {
        SoARuleVerdict::Critical
    } else if actual > maximum {
        SoARuleVerdict::Violation
    } else if actual > maximum * 0.9 {
        SoARuleVerdict::Warning
    } else {
        SoARuleVerdict::Pass
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manager_retains_complete_rule_coverage_and_exact_worst_point() {
        let mut manager = SoAManager::new();
        manager
            .register_device(
                "M1",
                SoADefinition {
                    limits: vec![SoALimit {
                        parameter: SoAParameter::Vds,
                        max_value: 10.0,
                        unit: "V".to_owned(),
                        description: "Maximum drain-source voltage".to_owned(),
                    }],
                },
            )
            .expect("valid SOA rule registers");
        assert!(
            manager
                .register_device(
                    "M1",
                    SoADefinition {
                        limits: vec![SoALimit {
                            parameter: SoAParameter::Vds,
                            max_value: 1.0,
                            unit: "V".to_owned(),
                            description: "Conflicting duplicate".to_owned(),
                        }],
                    },
                )
                .is_err()
        );

        for (time, actual) in [(0.0, 5.0), (1.0, 9.5), (2.0, 12.5)] {
            manager
                .check_point(
                    time,
                    &HashMap::from([(
                        "M1".to_owned(),
                        HashMap::from([(SoAParameter::Vds, actual)]),
                    )]),
                )
                .expect("finite SOA sample evaluates");
        }

        let evaluations = manager.evaluations().collect::<Vec<_>>();
        assert_eq!(evaluations.len(), 1);
        assert_eq!(evaluations[0].sample_count, 3);
        assert_eq!(evaluations[0].limit_value, 10.0);
        assert_eq!(evaluations[0].worst_actual_value, 12.5);
        assert_eq!(evaluations[0].worst_time, 2.0);
        assert_eq!(evaluations[0].verdict, SoARuleVerdict::Critical);
        assert_eq!(manager.violations().len(), 2);
        assert_eq!(manager.violations()[0].severity, ViolationSeverity::Warning);
        assert_eq!(
            manager.violations()[1].severity,
            ViolationSeverity::Critical
        );

        manager.clear_violations();
        assert!(manager.violations().is_empty());
        assert_eq!(manager.evaluations().count(), 0);
    }
}
