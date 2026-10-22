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
    /// Physical output/input unit, determined from the elaborated source/probe.
    pub gain_unit: crate::signal_unit::SignalUnit,
    /// Input impedance in Ohms
    pub input_impedance: Value,
    /// Output impedance in Ohms (Thevenin equivalent)
    pub output_impedance: Value,
}

impl TransferFunctionResult {
    /// Create a result with its known gain unit; source names do not determine units.
    pub fn new(
        output: &str,
        input: &str,
        gain: Value,
        gain_unit: crate::signal_unit::SignalUnit,
        zin: Value,
        zout: Value,
    ) -> Self {
        Self {
            output: output.to_string(),
            input: input.to_string(),
            gain,
            gain_unit,
            input_impedance: zin,
            output_impedance: zout,
        }
    }

    /// Gain in decibels relative to one unit of `gain_unit`.
    pub fn gain_db(&self) -> Value {
        20.0 * self.gain.abs().log10()
    }
}
