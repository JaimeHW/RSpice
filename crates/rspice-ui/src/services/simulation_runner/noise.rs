//! Noise analysis runner.

use super::error::{ensure_not_aborted, poll_periodically};
use super::{
    ServiceRunError, ServiceRunResult, build_engine_config, generate_freq_points_with_abort,
    parse_runner_netlist_with_abort,
};
use rspice_core::Value;
use rspice_core::abort_signal::{AbortSignal, NoAbort};
use rspice_core::analysis::noise::NoiseResult;
use rspice_core::engine::Engine;
use std::path::Path;

/// Complete configuration for a noise-analysis service run.
#[derive(Debug, Clone, Copy)]
pub struct NoiseRunSpec<'a> {
    pub output_node: &'a str,
    pub start_freq: Value,
    pub stop_freq: Value,
    pub points_per_decade: usize,
    pub temperature: Value,
}

impl<'a> NoiseRunSpec<'a> {
    pub const fn new(
        output_node: &'a str,
        start_freq: Value,
        stop_freq: Value,
        points_per_decade: usize,
        temperature: Value,
    ) -> Self {
        Self {
            output_node,
            start_freq,
            stop_freq,
            points_per_decade,
            temperature,
        }
    }
}

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
        Self::from_results_with_abort(results, &NoAbort)
            .expect("NoAbort cannot cancel noise result conversion")
    }

    /// Create from engine results with cooperative cancellation during
    /// extraction, integration, and contribution summarization.
    pub fn from_results_with_abort(
        results: Vec<NoiseResult>,
        abort: &dyn AbortSignal,
    ) -> ServiceRunResult<Self> {
        ensure_not_aborted(abort)?;
        let mut frequencies = Vec::with_capacity(results.len());
        let mut output_noise = Vec::with_capacity(results.len());
        for (point_index, result) in results.iter().enumerate() {
            poll_periodically(abort, point_index)?;
            frequencies.push(result.frequency);
            output_noise.push(result.output_noise_density);
        }
        let num_points = frequencies.len();

        // Integrate noise: approximate with trapezoidal rule
        let total_output_noise = if frequencies.len() >= 2 {
            let mut integrated = 0.0;
            for i in 1..frequencies.len() {
                poll_periodically(abort, i)?;
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
            let mut contributions = Vec::with_capacity(first.contributions.len());
            for (contribution_index, contribution) in first.contributions.iter().enumerate() {
                poll_periodically(abort, contribution_index)?;
                contributions.push((
                    contribution.identity.device.clone(),
                    contribution.percentage,
                ));
            }
            contributions
        } else {
            vec![]
        };

        ensure_not_aborted(abort)?;
        Ok(Self {
            frequencies,
            output_noise,
            total_output_noise,
            contributions,
            num_points,
        })
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
    let spec = NoiseRunSpec::new(
        output_node,
        start_freq,
        stop_freq,
        points_per_decade,
        temperature,
    );
    run_noise_analysis_with_abort(netlist_text, spec, &NoAbort).map_err(|error| error.to_string())
}

/// Run noise analysis with cooperative cancellation.
pub fn run_noise_analysis_with_abort(
    netlist_text: &str,
    spec: NoiseRunSpec<'_>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<NoiseData> {
    run_noise_analysis_with_source_path_and_abort(netlist_text, spec, None, abort)
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
    let spec = NoiseRunSpec::new(
        output_node,
        start_freq,
        stop_freq,
        points_per_decade,
        temperature,
    );
    run_noise_analysis_with_source_path_and_abort(netlist_text, spec, source_path, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run noise analysis with source-path resolution and cooperative cancellation
/// through parsing, operating-point setup, sweep generation, solving, and
/// result conversion.
pub fn run_noise_analysis_with_source_path_and_abort(
    netlist_text: &str,
    spec: NoiseRunSpec<'_>,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<NoiseData> {
    ensure_not_aborted(abort)?;
    let output_node = spec.output_node.trim();
    if output_node.is_empty() {
        return Err(ServiceRunError::Failure(
            "Noise output node is required".to_string(),
        ));
    }
    if !spec.temperature.is_finite() || spec.temperature <= 0.0 {
        return Err(ServiceRunError::Failure(
            "Noise temperature must be finite and greater than zero Kelvin".to_string(),
        ));
    }

    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;

    // Create engine
    ensure_not_aborted(abort)?;
    let engine = Engine::new(build_engine_config(&netlist, None));

    // Run DC OP to get node names and find output node index
    let dc_result = engine
        .run_dc_op_with_abort(&netlist, abort)
        .map_err(|error| ServiceRunError::from_core("DC OP error (required for noise)", error))?;

    // Find output node index by name (case-insensitive)
    let mut output_idx = None;
    for (node_index, node_name) in dc_result.node_names.iter().enumerate() {
        poll_periodically(abort, node_index)?;
        if node_name.eq_ignore_ascii_case(output_node) {
            output_idx = Some(node_index);
            break;
        }
    }
    let output_idx = output_idx.ok_or_else(|| {
        ServiceRunError::Failure(format!("Output node '{output_node}' not found"))
    })?;

    // Generate frequency points (always log-spaced for noise)
    let frequencies = generate_freq_points_with_abort(
        spec.start_freq,
        spec.stop_freq,
        spec.points_per_decade,
        "dec",
        abort,
    )?;

    // Run noise analysis
    let results = engine
        .run_noise_with_abort(&netlist, output_idx, &frequencies, spec.temperature, abort)
        .map_err(|error| ServiceRunError::from_core("Noise analysis error", error))?;

    NoiseData::from_results_with_abort(results, abort)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use rspice_core::abort_signal::{AbortSignal, ImmediateAbort};

    use super::*;

    struct AbortOnPoll {
        abort_on: usize,
        polls: AtomicUsize,
    }

    impl AbortOnPoll {
        fn new(abort_on: usize) -> Self {
            Self {
                abort_on,
                polls: AtomicUsize::new(0),
            }
        }
    }

    impl AbortSignal for AbortOnPoll {
        fn is_aborted(&self) -> bool {
            self.polls.fetch_add(1, Ordering::Relaxed) + 1 >= self.abort_on
        }
    }

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

    #[test]
    fn noise_runner_preserves_typed_abort_before_parse_or_validation() {
        let spec = NoiseRunSpec::new("", 0.0, -1.0, 0, -1.0);
        let result = run_noise_analysis_with_abort("not a netlist", spec, &ImmediateAbort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn noise_result_conversion_polls_inside_point_loops() {
        let template = NoiseResult {
            frequency: 1.0,
            node_names: Vec::new(),
            branch_names: Vec::new(),
            voltages: Vec::new(),
            currents: Vec::new(),
            output_noise_density: 1.0,
            input_referred_density: 1.0,
            input_gain_squared: 1.0,
            contribution_catalog: Vec::new(),
            contributions: Vec::new(),
        };
        let mut results = vec![template; 129];
        for (index, result) in results.iter_mut().enumerate() {
            result.frequency = index as Value + 1.0;
        }
        let abort = AbortOnPoll::new(3);

        let result = NoiseData::from_results_with_abort(results, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }
}
