use super::{build_engine_config, generate_freq_points, parse_runner_netlist};
use num_complex::Complex64;
use rspice_core::Value;
use rspice_core::analysis::ac::AcResult;
use rspice_core::engine::Engine;
use std::path::Path;

/// AC small-signal analysis data for Bode plots
#[derive(Debug, Clone)]
pub struct AcData {
    /// Frequency points (Hz)
    pub frequencies: Vec<Value>,
    /// Node responses: (node_name, complex values)
    pub responses: Vec<(String, Vec<Complex64>)>,
    /// Number of frequency points
    pub num_points: usize,
}

impl AcData {
    /// Get magnitude in dB for a response
    pub fn magnitude_db(&self, response_idx: usize) -> Vec<Value> {
        self.responses
            .get(response_idx)
            .map(|(_, vals)| vals.iter().map(|c| 20.0 * c.norm().log10()).collect())
            .unwrap_or_default()
    }

    /// Get phase in degrees for a response
    pub fn phase_deg(&self, response_idx: usize) -> Vec<Value> {
        self.responses
            .get(response_idx)
            .map(|(_, vals)| vals.iter().map(|c| c.arg().to_degrees()).collect())
            .unwrap_or_default()
    }

    /// Create from engine AcResult vector
    pub fn from_results(results: Vec<AcResult>) -> Self {
        let frequencies: Vec<Value> = results.iter().map(|result| result.frequency).collect();
        let num_points = frequencies.len();

        let mut responses = Vec::new();
        if let Some(first_result) = results.first() {
            for (ac_idx, name) in first_result.node_names.iter().enumerate() {
                let values: Vec<Complex64> = results
                    .iter()
                    .filter_map(|result| result.voltages.get(ac_idx).copied())
                    .collect();
                if !values.is_empty() {
                    responses.push((format!("V({})", name), values));
                }
            }
        }

        Self {
            frequencies,
            responses,
            num_points,
        }
    }
}

/// Run AC small-signal analysis
pub fn run_ac_analysis(
    netlist_text: &str,
    start_freq: Value,
    stop_freq: Value,
    num_points: usize,
    sweep_type: &str, // "dec", "oct", or "lin"
) -> Result<AcData, String> {
    run_ac_analysis_with_source_path(
        netlist_text,
        start_freq,
        stop_freq,
        num_points,
        sweep_type,
        None,
    )
}

/// Run AC small-signal analysis with a source path used to resolve relative
/// includes and model file references.
pub fn run_ac_analysis_with_source_path(
    netlist_text: &str,
    start_freq: Value,
    stop_freq: Value,
    num_points: usize,
    sweep_type: &str, // "dec", "oct", or "lin"
    source_path: Option<&Path>,
) -> Result<AcData, String> {
    let netlist = parse_runner_netlist(netlist_text, source_path)?;
    let frequencies = generate_freq_points(start_freq, stop_freq, num_points, sweep_type)?;
    let engine = Engine::new(build_engine_config(&netlist, None));
    let results = engine
        .run_ac(&netlist, &frequencies)
        .map_err(|e| format!("AC analysis error: {}", e))?;

    Ok(AcData::from_results(results))
}

#[cfg(test)]
mod tests {
    use super::*;

    const RC_DECK: &str = "\
V1 in 0 DC 1 AC 1
R1 in out 1k
C1 out 0 1n
.end
";

    #[test]
    fn ac_runner_rejects_empty_frequency_sweep_instead_of_empty_success() {
        let error = run_ac_analysis(RC_DECK, 0.0, 1.0e6, 10, "dec")
            .expect_err("zero start frequency must be invalid");

        assert!(error.contains("frequency"));
    }

    #[test]
    fn ac_runner_rejects_unknown_sweep_type_instead_of_linear_fallback() {
        let error = run_ac_analysis(RC_DECK, 1.0, 1.0e6, 10, "banana")
            .expect_err("unknown AC sweep keyword must be invalid");

        assert!(error.contains("sweep"));
        assert!(error.contains("banana"));
    }
}
