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
                .map(|c| (c.device_name.clone(), c.percentage))
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
    let frequencies = generate_freq_points(start_freq, stop_freq, points_per_decade, "dec");

    // Run noise analysis
    let results = engine
        .run_noise(&netlist, output_idx, &frequencies, temperature)
        .map_err(|e| format!("Noise analysis error: {}", e))?;

    Ok(NoiseData::from_results(results))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::analysis::noise::{NoiseContribution, NoiseSourceType};
    use std::path::PathBuf;

    fn write_noise_include_fixture() -> (tempfile::TempDir, PathBuf, String) {
        let temp_dir = tempfile::tempdir().expect("temp dir should be created");
        let source_path = temp_dir.path().join("project").join("noise_top.rsch");
        std::fs::create_dir_all(
            source_path
                .parent()
                .expect("source path should have parent directory"),
        )
        .expect("project directory should be created");
        std::fs::create_dir_all(temp_dir.path().join("models"))
            .expect("models directory should be created");
        std::fs::write(
            temp_dir.path().join("models").join("noise_stage.inc"),
            "R1 in out 1k\nC1 out 0 1n\n",
        )
        .expect("include file should be written");

        let netlist = "* noise include fixture\n\
V1 in 0 DC 1 AC 1\n\
.include ../models/noise_stage.inc\n\
.end\n"
            .to_string();

        (temp_dir, source_path, netlist)
    }

    #[test]
    fn test_noise_data_from_results_empty() {
        let data = NoiseData::from_results(Vec::new());
        assert!(data.frequencies.is_empty());
        assert!(data.output_noise.is_empty());
        assert!(data.contributions.is_empty());
        assert_eq!(data.num_points, 0);
        assert_eq!(data.total_output_noise, 0.0);
    }

    #[test]
    fn test_noise_data_from_results_integrates_psd_and_copies_contributors() {
        let results = vec![
            NoiseResult {
                frequency: 10.0,
                output_noise_density: 4.0,
                input_referred_density: 1.0,
                contributions: vec![NoiseContribution {
                    device_name: "R1".to_string(),
                    noise_type: NoiseSourceType::Thermal,
                    output_contribution: 2.0,
                    percentage: 55.0,
                }],
            },
            NoiseResult {
                frequency: 110.0,
                output_noise_density: 9.0,
                input_referred_density: 2.0,
                contributions: vec![NoiseContribution {
                    device_name: "R2".to_string(),
                    noise_type: NoiseSourceType::Thermal,
                    output_contribution: 3.0,
                    percentage: 45.0,
                }],
            },
        ];

        let data = NoiseData::from_results(results);

        assert_eq!(data.num_points, 2);
        assert_eq!(data.frequencies, vec![10.0, 110.0]);
        assert_eq!(data.output_noise, vec![4.0, 9.0]);
        assert_eq!(data.contributions.len(), 1);
        assert_eq!(data.contributions[0], ("R1".to_string(), 55.0));
        let expected_integrated = ((4.0_f64 + 9.0_f64) * 0.5_f64 * 100.0_f64).sqrt();
        assert!((data.total_output_noise - expected_integrated).abs() < 1e-12);
    }

    #[test]
    fn test_run_noise_analysis_output_node_match_is_case_insensitive() {
        let netlist =
            "* noise case-insensitive\nV1 in 0 DC 1 AC 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";

        let lower = run_noise_analysis(netlist, "out", 1.0, 1e6, 5, 300.0)
            .expect("noise run with lower-case node should succeed");
        let mixed = run_noise_analysis(netlist, "OuT", 1.0, 1e6, 5, 300.0)
            .expect("noise run with mixed-case node should succeed");

        assert_eq!(lower.num_points, mixed.num_points);
        assert_eq!(lower.frequencies, mixed.frequencies);
        assert_eq!(lower.output_noise.len(), lower.frequencies.len());
        assert_eq!(mixed.output_noise.len(), mixed.frequencies.len());
    }

    #[test]
    fn test_run_noise_analysis_with_source_path_resolves_relative_include() {
        let (_temp_dir, source_path, netlist) = write_noise_include_fixture();

        let without_source = run_noise_analysis(&netlist, "out", 1.0, 1e6, 5, 300.0);
        assert!(
            without_source.is_err(),
            "relative include should fail without source path"
        );

        let with_source = run_noise_analysis_with_source_path(
            &netlist,
            "out",
            1.0,
            1e6,
            5,
            300.0,
            Some(&source_path),
        )
        .expect("source-aware noise analysis should resolve include");
        assert!(with_source.num_points > 0);
        assert_eq!(
            with_source.output_noise.len(),
            with_source.frequencies.len()
        );
    }
}
