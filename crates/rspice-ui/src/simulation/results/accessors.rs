//! Reading a simulation result.
//!
//! Signal lookup by name, sweep-axis access, and the unit each carries.

use super::*;

impl SimulationResult {
    /// Get all waveform names
    #[cfg(test)]
    pub fn waveform_names(&self) -> Vec<&str> {
        match self {
            SimulationResult::DcSweep { waveforms, .. } => {
                waveforms.keys().map(|s| s.as_str()).collect()
            }
            SimulationResult::Transient { waveforms, .. } => {
                waveforms.keys().map(|s| s.as_str()).collect()
            }
            SimulationResult::Ac { waveforms, .. } => {
                waveforms.keys().map(|s| s.as_str()).collect()
            }
            SimulationResult::HarmonicBalance { waveforms, .. } => {
                waveforms.keys().map(|s| s.as_str()).collect()
            }
            SimulationResult::Parametric { waveforms, .. } => {
                waveforms.keys().map(|s| s.as_str()).collect()
            }
            SimulationResult::Corner { waveforms, .. } => {
                waveforms.keys().map(|s| s.as_str()).collect()
            }
            SimulationResult::Reliability { waveforms, .. } => {
                waveforms.keys().map(|s| s.as_str()).collect()
            }
            SimulationResult::Optimization { waveforms, .. } => {
                waveforms.keys().map(|s| s.as_str()).collect()
            }
            SimulationResult::Soa { waveforms, .. } => {
                waveforms.keys().map(|s| s.as_str()).collect()
            }
            SimulationResult::MonteCarlo { variables, .. } => {
                variables.iter().map(|v| v.name.as_str()).collect()
            }
            _ => vec![],
        }
    }

    /// Get a specific waveform by name
    #[cfg(test)]
    pub fn get_waveform(&self, name: &str) -> Option<&WaveformData> {
        match self {
            SimulationResult::DcSweep { waveforms, .. } => waveforms.get(name),
            SimulationResult::Transient { waveforms, .. } => waveforms.get(name),
            SimulationResult::Ac { waveforms, .. } => waveforms.get(name),
            SimulationResult::HarmonicBalance { waveforms, .. } => waveforms.get(name),
            SimulationResult::Parametric { waveforms, .. } => waveforms.get(name),
            SimulationResult::Corner { waveforms, .. } => waveforms.get(name),
            SimulationResult::Reliability { waveforms, .. } => waveforms.get(name),
            SimulationResult::Optimization { waveforms, .. } => waveforms.get(name),
            SimulationResult::Soa { waveforms, .. } => waveforms.get(name),
            _ => None,
        }
    }

    /// Check if this is a valid result with data
    #[cfg(test)]
    pub fn has_data(&self) -> bool {
        match self {
            SimulationResult::DcOp(op) => !op.node_voltages.is_empty(),
            SimulationResult::DcSweep { waveforms, .. } => !waveforms.is_empty(),
            SimulationResult::Transient { time, .. } => !time.is_empty(),
            SimulationResult::Ac { frequencies, .. } => !frequencies.is_empty(),
            SimulationResult::HarmonicBalance { frequencies, .. } => !frequencies.is_empty(),
            SimulationResult::Noise { frequencies, .. } => !frequencies.is_empty(),
            SimulationResult::PoleZero { gain, .. } => gain.is_finite(),
            SimulationResult::Sensitivity {
                output,
                sensitivities,
                normalized,
                ..
            } => {
                !output.trim().is_empty()
                    && sensitivities.len() == normalized.len()
                    && sensitivities
                        .keys()
                        .all(|parameter| normalized.contains_key(parameter))
            }
            SimulationResult::TransferFunction {
                input_source,
                output_expression,
                gain,
                input_resistance,
                output_resistance,
                ..
            } => {
                !input_source.trim().is_empty()
                    && !output_expression.trim().is_empty()
                    && (gain.is_some() || input_resistance.is_some() || output_resistance.is_some())
            }
            SimulationResult::MonteCarlo {
                runs_completed,
                variables,
                ..
            } => *runs_completed > 0 || !variables.is_empty(),
            SimulationResult::Parametric { sweep_values, .. } => !sweep_values.is_empty(),
            SimulationResult::Corner { x_values, .. } => !x_values.is_empty(),
            SimulationResult::Reliability {
                years, waveforms, ..
            } => !years.is_empty() && !waveforms.is_empty(),
            SimulationResult::Optimization {
                iterations,
                waveforms,
                ..
            } => !iterations.is_empty() && !waveforms.is_empty(),
            SimulationResult::Soa {
                time, waveforms, ..
            } => !time.is_empty() && !waveforms.is_empty(),
            SimulationResult::MeasurementsOnly { measurements } => !measurements.is_empty(),
        }
    }

    /// Get display name for the result type
    #[cfg(test)]
    pub fn type_name(&self) -> &'static str {
        match self {
            SimulationResult::DcOp(_) => "DC Operating Point",
            SimulationResult::DcSweep { .. } => "DC Sweep",
            SimulationResult::Transient { .. } => "Transient",
            SimulationResult::Ac { .. } => "AC Analysis",
            SimulationResult::HarmonicBalance { .. } => "Harmonic Balance",
            SimulationResult::Noise { .. } => "Noise Analysis",
            SimulationResult::PoleZero { .. } => "Pole-Zero",
            SimulationResult::Sensitivity { .. } => "Sensitivity",
            SimulationResult::TransferFunction { .. } => "Transfer Function",
            SimulationResult::MonteCarlo { .. } => "Monte Carlo",
            SimulationResult::Parametric { .. } => "Parametric",
            SimulationResult::Corner { .. } => "Corner",
            SimulationResult::Reliability { .. } => "Reliability",
            SimulationResult::Optimization { .. } => "Optimization",
            SimulationResult::Soa { .. } => "Safety (SOA)",
            SimulationResult::MeasurementsOnly { .. } => "Measurements Only",
        }
    }
}

#[cfg(test)]
mod transfer_function_tests {
    use super::*;
    use crate::simulation::multi_run::{TfAccuracy, TfNormalization};

    fn result(
        gain: Option<TransferFunctionScalar>,
        input_resistance: Option<TransferFunctionScalar>,
        output_resistance: Option<TransferFunctionScalar>,
    ) -> SimulationResult {
        SimulationResult::TransferFunction {
            input_source: "VIN".to_owned(),
            output_expression: "V(out)".to_owned(),
            input_quantity: TransferFunctionQuantity::Voltage,
            output_quantity: TransferFunctionQuantity::Voltage,
            input_unit: "V".to_owned(),
            output_unit: "V".to_owned(),
            normalization: TfNormalization::None,
            accuracy: TfAccuracy::Balanced,
            gain,
            input_resistance,
            output_resistance,
            nominal_input: None,
            nominal_output: None,
        }
    }

    #[test]
    fn scalar_tf_has_data_without_inventing_waveforms() {
        let tf = result(None, Some(TransferFunctionScalar::PositiveInfinity), None);

        assert!(tf.has_data());
        assert_eq!(tf.type_name(), "Transfer Function");
        assert!(tf.waveform_names().is_empty());
        assert!(tf.get_waveform("gain").is_none());
    }

    #[test]
    fn tf_without_any_retained_scalar_has_no_data() {
        assert!(!result(None, None, None).has_data());
    }
}
