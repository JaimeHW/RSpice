//! Measurement results exchanged with the simulation worker.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerMeasurement {
    pub name: String,
    pub value: Option<f64>,
    #[serde(default)]
    pub raw_value: Option<f64>,
    pub error: Option<String>,
    pub passed: bool,
    pub expected: Option<f64>,
    pub tolerance: Option<f64>,
    #[serde(default)]
    pub failure_limit: Option<f64>,
    #[serde(default)]
    pub failure_limit_exceeded: bool,
    pub event_axis: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub units: Option<rspice_core::analysis::MeasurementUnits>,
}

impl WorkerMeasurement {
    /// Reject malformed current-protocol evidence before publishing a result.
    pub fn validate_current_evidence(&self, prefix: &str) -> Result<(), String> {
        if let Some(units) = &self.units {
            units
                .validate()
                .map_err(|error| format!("{prefix}.units: {error}"))?;
        }
        if self.name.trim().is_empty() || self.name.chars().any(char::is_control) {
            return Err(format!("{prefix} has an invalid measurement name"));
        }
        if [
            self.value,
            self.raw_value,
            self.expected,
            self.tolerance,
            self.failure_limit,
            self.event_axis,
        ]
        .into_iter()
        .flatten()
        .any(|value| !value.is_finite())
            || self.tolerance.is_some_and(|tolerance| tolerance < 0.0)
        {
            return Err(format!(
                "{prefix} contains non-finite measurement evidence or a negative tolerance"
            ));
        }
        if self.value.is_some() != self.raw_value.is_some() {
            return Err(format!(
                "{prefix}.raw_value must be present exactly when value is present"
            ));
        }
        let expected_exceeded = match (self.raw_value, self.failure_limit) {
            (Some(raw_value), Some(limit)) => raw_value.abs() >= limit,
            _ => false,
        };
        if self.failure_limit_exceeded != expected_exceeded {
            return Err(format!(
                "{prefix}.failure_limit_exceeded does not match abs(raw_value) >= failure_limit"
            ));
        }
        if self.failure_limit_exceeded && self.passed {
            return Err(format!(
                "{prefix} cannot pass after its FAILVALUE limit was reached"
            ));
        }
        if self.passed && (self.value.is_none() || self.error.is_some()) {
            return Err(format!(
                "{prefix} has contradictory passing measurement evidence"
            ));
        }
        Ok(())
    }
}

impl From<rspice_core::MeasureResult> for WorkerMeasurement {
    fn from(value: rspice_core::MeasureResult) -> Self {
        Self {
            name: value.name,
            value: value.value,
            raw_value: value.raw_value,
            error: value.error,
            passed: value.passed,
            expected: value.expected,
            tolerance: value.tolerance,
            failure_limit: value.failure_limit,
            failure_limit_exceeded: value.failure_limit_exceeded,
            event_axis: value.event_axis,
            units: value.units,
        }
    }
}

impl From<WorkerMeasurement> for rspice_core::MeasureResult {
    fn from(value: WorkerMeasurement) -> Self {
        Self {
            name: value.name,
            value: value.value,
            raw_value: value.raw_value,
            error: value.error,
            passed: value.passed,
            expected: value.expected,
            tolerance: value.tolerance,
            failure_limit: value.failure_limit,
            failure_limit_exceeded: value.failure_limit_exceeded,
            event_axis: value.event_axis,
            units: value.units,
        }
    }
}
