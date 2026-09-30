//! Intermediate sweep results and reference-deck markers.

use rspice_core::Value;

pub(crate) const REFERENCE_MODEL_BINDING_BEGIN: &str = "* RSPICE REFERENCE MODEL BINDING BEGIN";
pub(crate) const REFERENCE_MODEL_BINDING_END: &str = "* RSPICE REFERENCE MODEL BINDING END";

/// Parametric sweep data.
#[derive(Debug, Clone)]
pub struct ParametricData {
    pub target: String,
    pub sweep_values: Vec<Value>,
    pub voltages: Vec<(String, Vec<Value>)>,
    pub num_failures: usize,
}

/// One scalar per node at a single swept point, in a node order the caller
/// keeps identical across points: the mappers pair names and values by index.
#[derive(Debug, Clone)]
pub(crate) struct SweepPointResult {
    pub(crate) node_names: Vec<String>,
    pub(crate) node_values: Vec<Value>,
}
