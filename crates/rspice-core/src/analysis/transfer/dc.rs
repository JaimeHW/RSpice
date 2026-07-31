use super::*;

/// Result of transfer function analysis
#[derive(Debug, Clone)]
pub struct TransferFunctionResult {
    /// Output variable (e.g., "V(out)")
    pub output: String,
    /// Input source name (e.g., "Vin")
    pub input: String,
    /// Transfer gain (output/input ratio)
    pub gain: Value,
    /// Input impedance in Ohms
    pub input_impedance: Value,
    /// Output impedance in Ohms (Thevenin equivalent)
    pub output_impedance: Value,
}

impl TransferFunctionResult {
    /// Create a new transfer function result
    pub fn new(output: &str, input: &str, gain: Value, zin: Value, zout: Value) -> Self {
        Self {
            output: output.to_string(),
            input: input.to_string(),
            gain,
            input_impedance: zin,
            output_impedance: zout,
        }
    }

    /// Get gain in decibels
    pub fn gain_db(&self) -> Value {
        20.0 * self.gain.abs().log10()
    }
}

/// Configuration for transfer function analysis
#[derive(Debug, Clone)]
pub struct TransferFunctionConfig {
    /// Output node or variable (e.g., "out" or "V(out)")
    pub output_node: String,
    /// Reference node for output (None = ground)
    pub output_ref: Option<String>,
    /// Input source name
    pub input_source: String,
    /// Whether input is current source (vs voltage source)
    pub input_is_current: bool,
    /// Whether output is current (vs voltage)
    pub output_is_current: bool,
    /// Current measurement element (if output_is_current)
    pub output_element: Option<String>,
}

impl TransferFunctionConfig {

    /// Create config for voltage-to-current transfer function
    ///
    /// Example: `.TF I(Rload) Vin`
    pub fn transconductance(output_element: &str, input_source: &str) -> Self {
        Self {
            output_node: String::new(),
            output_ref: None,
            input_source: input_source.to_string(),
            input_is_current: false,
            output_is_current: true,
            output_element: Some(output_element.to_string()),
        }
    }

}

