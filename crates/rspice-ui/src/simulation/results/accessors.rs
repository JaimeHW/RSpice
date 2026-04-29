use super::*;

impl SimulationResult {
    /// Get all waveform names
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
    pub fn get_waveform(&self, name: &str) -> Option<&WaveformData> {
        match self {
            SimulationResult::DcSweep { waveforms, .. } => waveforms.get(name),
            SimulationResult::Transient { waveforms, .. } => waveforms.get(name),
            SimulationResult::Ac { waveforms, .. } => waveforms.get(name),
            SimulationResult::Parametric { waveforms, .. } => waveforms.get(name),
            SimulationResult::Corner { waveforms, .. } => waveforms.get(name),
            SimulationResult::Reliability { waveforms, .. } => waveforms.get(name),
            SimulationResult::Optimization { waveforms, .. } => waveforms.get(name),
            SimulationResult::Soa { waveforms, .. } => waveforms.get(name),
            _ => None,
        }
    }

    /// Check if this is a valid result with data
    pub fn has_data(&self) -> bool {
        match self {
            SimulationResult::DcOp(op) => !op.node_voltages.is_empty(),
            SimulationResult::DcSweep { waveforms, .. } => !waveforms.is_empty(),
            SimulationResult::Transient { time, .. } => !time.is_empty(),
            SimulationResult::Ac { frequencies, .. } => !frequencies.is_empty(),
            SimulationResult::Noise { frequencies, .. } => !frequencies.is_empty(),
            SimulationResult::PoleZero { poles, zeros, .. } => {
                !poles.is_empty() || !zeros.is_empty()
            }
            SimulationResult::Sensitivity { sensitivities, .. } => !sensitivities.is_empty(),
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
            SimulationResult::MeasurementsOnly { .. } => false,
        }
    }

    /// Get display name for the result type
    pub fn type_name(&self) -> &'static str {
        match self {
            SimulationResult::DcOp(_) => "DC Operating Point",
            SimulationResult::DcSweep { .. } => "DC Sweep",
            SimulationResult::Transient { .. } => "Transient",
            SimulationResult::Ac { .. } => "AC Analysis",
            SimulationResult::Noise { .. } => "Noise Analysis",
            SimulationResult::PoleZero { .. } => "Pole-Zero",
            SimulationResult::Sensitivity { .. } => "Sensitivity",
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
