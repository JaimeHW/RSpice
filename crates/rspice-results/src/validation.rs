//! Shared checks for exact retained result evidence.

use serde::{Deserialize, Serialize};

pub fn require_non_empty(value: &str, label: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        Err(format!("{label} is empty"))
    } else {
        Ok(())
    }
}

pub fn require_finite_values(values: &[f64], label: &str) -> Result<(), String> {
    if values.iter().any(|value| !value.is_finite()) {
        Err(format!("{label} contain a non-finite value"))
    } else {
        Ok(())
    }
}

pub fn strictly_increasing(values: &[f64]) -> bool {
    values
        .windows(2)
        .all(|pair| normalized_f64(pair[0]) < normalized_f64(pair[1]))
}

pub fn normalized_f64(value: f64) -> f64 {
    if value == 0.0 { 0.0 } else { value }
}

/// The exact retained-result schema disagreement reported across execution boundaries.
///
/// Serialized rather than mirrored into a separate worker-side type: a result
/// that fails its own schema is the same report on both sides of the browser
/// worker boundary, and the two registries are what a report consists of.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultSchemaMismatch {
    /// The analysis whose result failed its schema.
    pub analysis: String,
    /// The sweep, frequency, or time point, where the failure has one.
    pub coordinate: Option<String>,
    /// The family of signals whose registry and payload disagree.
    pub signal_family: String,
    /// Both registries in their original order, because the order is part of
    /// the contract that was broken.
    pub expected_names: Vec<String>,
    pub actual_names: Vec<String>,
    pub expected_value_count: usize,
    pub actual_value_count: usize,
}

impl std::fmt::Display for ResultSchemaMismatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Result schema mismatch for {} analysis", self.analysis)?;
        if let Some(coordinate) = &self.coordinate {
            write!(f, " at {coordinate}")?;
        }
        write!(
            f,
            " in {}: expected names {:?} with {} value(s), got names {:?} with {} value(s)",
            self.signal_family,
            self.expected_names,
            self.expected_value_count,
            self.actual_names,
            self.actual_value_count
        )
    }
}
