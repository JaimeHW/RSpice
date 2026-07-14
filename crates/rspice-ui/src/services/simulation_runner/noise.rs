//! Noise analysis runner.

use super::{build_engine_config, generate_freq_points, parse_runner_netlist};
use rspice_core::Value;
use rspice_core::analysis::noise::NoiseResult;
use rspice_core::engine::Engine;
use std::path::Path;

/// Noise analysis data for spectral density plots
#[derive(Debug, Clone)]
pub struct NoiseData {
    /// Frequency points (Hz)
    pub frequencies: Vec<Value>,
    /// Output noise spectral density (V^2/Hz)
    pub output_noise: Vec<Value>,
    /// Total integrated output noise (V RMS)
    pub total_output_noise: Value,
    /// Noise contributions by device
    pub contributions: Vec<(String, Value)>,
    /// Number of frequency points
    pub num_points: usize,
}

impl NoiseData {
    /// Create from engine NoiseResult vector
    pub fn from_results(results: Vec<NoiseResult>) -> Self {
        let frequencies: Vec<Value> = results.iter().map(|r| r.frequency).collect();
        let output_noise: Vec<Value> = results.iter().map(|r| r.output_noise_density).collect();
        let num_points = frequencies.len();

        // Integrate noise: approximate with trapezoidal rule
        let total_output_noise = if frequencies.len() >= 2 {
            let mut integrated = 0.0;
            for i in 1..frequencies.len() {
                let df = frequencies[i] - frequencies[i - 1];
                let avg_noise = (output_noise[i] + output_noise[i - 1]) / 2.0;
                integrated += avg_noise * df;
            }
            integrated.sqrt() // RMS = sqrt(integral of PSD)
        } else {
            0.0
        };

        // Summarize contributions from the first frequency point
        let contributions = if let Some(first) = results.first() {
            first
                .contributions
                .iter()
                .map(|c| (c.identity.device.clone(), c.percentage))
                .collect()
        } else {
            vec![]
        };

        Self {
            frequencies,
            output_noise,
            total_output_noise,
            contributions,
            num_points,
        }
    }
}

/// Run noise analysis.
pub fn run_noise_analysis(
    netlist_text: &str,
    output_node: &str,
    start_freq: Value,
    stop_freq: Value,
    points_per_decade: usize,
    temperature: Value, // Kelvin, default 300K
) -> Result<NoiseData, String> {
    run_noise_analysis_with_source_path(
        netlist_text,
        output_node,
        start_freq,
        stop_freq,
        points_per_decade,
        temperature,
        None,
    )
}

/// Run noise analysis with a source path used to resolve relative includes and
/// model file references.
pub fn run_noise_analysis_with_source_path(
    netlist_text: &str,
    output_node: &str,
    start_freq: Value,
    stop_freq: Value,
    points_per_decade: usize,
    temperature: Value,
    source_path: Option<&Path>,
) -> Result<NoiseData, String> {
    let netlist = parse_runner_netlist(netlist_text, source_path)?;

    // Create engine
    let engine = Engine::new(build_engine_config(&netlist, None));

    // Run DC OP to get node names and find output node index
    let dc_result = engine
        .run_dc_op(&netlist)
        .map_err(|e| format!("DC OP error (required for noise): {}", e))?;

    // Find output node index by name (case-insensitive)
    let output_idx = dc_result
        .node_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case(output_node))
        .ok_or_else(|| format!("Output node '{}' not found", output_node))?;

    // Generate frequency points (always log-spaced for noise)
    let frequencies = generate_freq_points(start_freq, stop_freq, points_per_decade, "dec")?;

    // Run noise analysis
    let results = engine
        .run_noise(&netlist, output_idx, &frequencies, temperature)
        .map_err(|e| format!("Noise analysis error: {}", e))?;

    Ok(NoiseData::from_results(results))
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOISY_DIVIDER_DECK: &str = "\
V1 in 0 DC 1
R1 in out 1k
R2 out 0 1k
.end
";

    #[test]
    fn noise_runner_rejects_empty_frequency_sweep_instead_of_empty_success() {
        let error = run_noise_analysis(NOISY_DIVIDER_DECK, "out", 0.0, 1.0e6, 10, 300.15)
            .expect_err("zero start frequency must be invalid");

        assert!(error.contains("frequency"));
    }
}
