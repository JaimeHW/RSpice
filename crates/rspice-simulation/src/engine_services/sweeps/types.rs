//! Intermediate sweep results.

use rspice_core::Value;

/// Parametric sweep data.
#[derive(Debug, Clone)]
pub struct ParametricData {
    pub target: String,
    pub sweep_values: Vec<Value>,
    pub voltages: Vec<(String, Vec<Value>)>,
    pub num_failures: usize,
}
