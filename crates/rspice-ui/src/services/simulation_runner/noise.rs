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
use std::collections::HashMap;
use std::path::Path;

/// Complete configuration for a noise-analysis service run.
#[derive(Debug, Clone, Copy)]
pub struct NoiseRunSpec<'a> {
    pub output_node: &'a str,
    pub output_reference: Option<&'a str>,
    pub input_source: &'a str,
    pub start_freq: Value,
    pub stop_freq: Value,
    pub points_per_decade: usize,
    pub temperature: Value,
}

impl<'a> NoiseRunSpec<'a> {
    pub const fn new(
        output_node: &'a str,
        output_reference: Option<&'a str>,
        input_source: &'a str,
        start_freq: Value,
        stop_freq: Value,
        points_per_decade: usize,
        temperature: Value,
    ) -> Self {
        Self {
            output_node,
            output_reference,
            input_source,
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
    /// Input-referred noise spectral density, present only when the solver
    /// validated and applied an independent input-source normalization.
    pub input_noise: Option<Vec<Value>>,
    /// Total integrated output noise (V RMS)
    pub total_output_noise: Value,
    /// Total integrated input-referred noise (V RMS), when normalized.
    pub total_input_noise: Option<Value>,
    /// Band-integrated noise contribution percentages by device/mechanism,
    /// ranked from largest to smallest.
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
        Self::from_results_with_normalization(results, false, abort)
    }

    /// Convert results produced by an input-referred solver invocation.
    /// Keeping this distinct from `from_results_with_abort` prevents an
    /// output-only result from being mislabeled as input-referred noise.
    pub fn from_input_referred_results_with_abort(
        results: Vec<NoiseResult>,
        abort: &dyn AbortSignal,
    ) -> ServiceRunResult<Self> {
        Self::from_results_with_normalization(results, true, abort)
    }

    fn from_results_with_normalization(
        results: Vec<NoiseResult>,
        input_is_normalized: bool,
        abort: &dyn AbortSignal,
    ) -> ServiceRunResult<Self> {
        ensure_not_aborted(abort)?;
        let mut frequencies = Vec::with_capacity(results.len());
        let mut output_noise = Vec::with_capacity(results.len());
        let mut input_noise = input_is_normalized.then(|| Vec::with_capacity(results.len()));
        for (point_index, result) in results.iter().enumerate() {
            poll_periodically(abort, point_index)?;
            frequencies.push(result.frequency);
            output_noise.push(result.output_noise_density);
            if let Some(input_noise) = input_noise.as_mut() {
                input_noise.push(result.input_referred_density);
            }
        }
        let num_points = frequencies.len();

        let total_output_noise = integrate_noise_density(&frequencies, &output_noise, abort)?;
        let total_input_noise = input_noise
            .as_ref()
            .map(|values| integrate_noise_density(&frequencies, values, abort))
            .transpose()?;
        let contributions = ranked_integrated_contributions(&results, abort)?;

        ensure_not_aborted(abort)?;
        Ok(Self {
            frequencies,
            output_noise,
            input_noise,
            total_output_noise,
            total_input_noise,
            contributions,
            num_points,
        })
    }
}

fn integrate_noise_density(
    frequencies: &[Value],
    density: &[Value],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Value> {
    if frequencies.len() < 2 {
        return Ok(0.0);
    }

    let mut integrated = 0.0;
    for index in 1..frequencies.len() {
        poll_periodically(abort, index)?;
        let df = frequencies[index] - frequencies[index - 1];
        integrated += 0.5 * (density[index] + density[index - 1]) * df;
    }
    Ok(integrated.max(0.0).sqrt())
}

fn ranked_integrated_contributions(
    results: &[NoiseResult],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<(String, Value)>> {
    if results.len() < 2 {
        return Ok(Vec::new());
    }

    let mut series: HashMap<String, Vec<Value>> = HashMap::new();
    for (point_index, result) in results.iter().enumerate() {
        poll_periodically(abort, point_index)?;
        for contribution in &result.contributions {
            ensure_not_aborted(abort)?;
            let mechanism = contribution
                .identity
                .mechanism
                .as_deref()
                .unwrap_or_else(|| contribution.noise_type.label());
            let key = format!("{} · {}", contribution.identity.device, mechanism);
            series
                .entry(key)
                .or_insert_with(|| vec![0.0; results.len()])[point_index] +=
                contribution.output_contribution;
        }
    }

    let frequencies = results
        .iter()
        .map(|result| result.frequency)
        .collect::<Vec<_>>();
    let mut powers = Vec::with_capacity(series.len());
    for (name, density) in series {
        ensure_not_aborted(abort)?;
        let rms = integrate_noise_density(&frequencies, &density, abort)?;
        powers.push((name, rms * rms));
    }
    let total_power = powers.iter().map(|(_, power)| *power).sum::<Value>();
    for (_, power) in &mut powers {
        *power = if total_power > 0.0 {
            100.0 * *power / total_power
        } else {
            0.0
        };
    }
    powers.sort_by(|(left_name, left), (right_name, right)| {
        right
            .total_cmp(left)
            .then_with(|| {
                left_name
                    .to_ascii_lowercase()
                    .cmp(&right_name.to_ascii_lowercase())
            })
            .then_with(|| left_name.cmp(right_name))
    });
    Ok(powers)
}

/// Run noise analysis.
pub fn run_noise_analysis(
    netlist_text: &str,
    output_node: &str,
    output_reference: Option<&str>,
    input_source: &str,
    start_freq: Value,
    stop_freq: Value,
    points_per_decade: usize,
    temperature: Value, // Kelvin, default 300K
) -> Result<NoiseData, String> {
    let spec = NoiseRunSpec::new(
        output_node,
        output_reference,
        input_source,
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
    output_reference: Option<&str>,
    input_source: &str,
    start_freq: Value,
    stop_freq: Value,
    points_per_decade: usize,
    temperature: Value,
    source_path: Option<&Path>,
) -> Result<NoiseData, String> {
    let spec = NoiseRunSpec::new(
        output_node,
        output_reference,
        input_source,
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
    let input_source = spec.input_source.trim();
    if input_source.is_empty() {
        return Err(ServiceRunError::Failure(
            "Noise input source is required for input-referred noise".to_string(),
        ));
    }
    let output_reference = spec.output_reference.and_then(nonempty_trimmed);

    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;

    // Create engine
    ensure_not_aborted(abort)?;
    let engine = Engine::new(build_engine_config(&netlist, None));

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
        .run_noise_named_with_input_source_and_abort(
            &netlist,
            output_node,
            output_reference,
            input_source,
            &frequencies,
            spec.temperature,
            abort,
        )
        .map_err(|error| ServiceRunError::from_core("Noise analysis error", error))?;

    NoiseData::from_input_referred_results_with_abort(results, abort)
}

fn nonempty_trimmed(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then_some(trimmed)
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
        let error = run_noise_analysis(
            NOISY_DIVIDER_DECK,
            "out",
            Some("0"),
            "V1",
            0.0,
            1.0e6,
            10,
            300.15,
        )
        .expect_err("zero start frequency must be invalid");

        assert!(error.contains("frequency"));
    }

    #[test]
    fn noise_runner_preserves_typed_abort_before_parse_or_validation() {
        let spec = NoiseRunSpec::new("", None, "", 0.0, -1.0, 0, -1.0);
        let result = run_noise_analysis_with_abort("not a netlist", spec, &ImmediateAbort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn noise_runner_preserves_reference_and_validates_input_source() {
        let spec = NoiseRunSpec::new("out", Some("in"), "missing", 1.0e3, 2.0e3, 2, 300.15);
        let error = run_noise_analysis_with_abort(NOISY_DIVIDER_DECK, spec, &NoAbort)
            .expect_err("missing input source must fail closed");
        assert!(error.to_string().contains("missing"));

        let exact = NoiseRunSpec::new("out", Some("in"), "v1", 1.0e3, 2.0e3, 2, 300.15);
        let data = run_noise_analysis_with_abort(NOISY_DIVIDER_DECK, exact, &NoAbort)
            .expect("named differential noise analysis runs");
        assert_eq!(data.num_points, 2);
        assert_eq!(
            data.input_noise.as_ref().map(Vec::len),
            Some(data.num_points)
        );
        assert!(data.total_input_noise.is_some());
    }

    #[test]
    fn output_only_conversion_does_not_publish_input_referred_values() {
        let results = vec![NoiseResult {
            frequency: 1.0,
            node_names: Vec::new(),
            branch_names: Vec::new(),
            voltages: Vec::new(),
            currents: Vec::new(),
            output_noise_density: 2.0,
            input_referred_density: 99.0,
            input_gain_squared: 1.0,
            contribution_catalog: Vec::new(),
            contributions: Vec::new(),
        }];

        let data = NoiseData::from_results(results);

        assert_eq!(data.input_noise, None);
        assert_eq!(data.total_input_noise, None);
    }

    #[test]
    fn integrated_contributor_policy_is_ranked_and_deterministic() {
        use rspice_core::analysis::advanced::NoiseContribution;
        use rspice_core::analysis::{NoiseSourceIdentity, NoiseSourceType};

        let make_point = |frequency| NoiseResult {
            frequency,
            node_names: Vec::new(),
            branch_names: Vec::new(),
            voltages: Vec::new(),
            currents: Vec::new(),
            output_noise_density: 4.0,
            input_referred_density: 1.0,
            input_gain_squared: 4.0,
            contribution_catalog: Vec::new(),
            contributions: vec![
                NoiseContribution {
                    identity: NoiseSourceIdentity::mechanism("R1", "rd"),
                    noise_type: NoiseSourceType::Thermal,
                    output_contribution: 2.0,
                    input_contribution: 0.5,
                    percentage: 0.0,
                },
                NoiseContribution {
                    identity: NoiseSourceIdentity::mechanism("R1", "rs"),
                    noise_type: NoiseSourceType::Thermal,
                    output_contribution: 1.0,
                    input_contribution: 0.25,
                    percentage: 0.0,
                },
                NoiseContribution {
                    identity: NoiseSourceIdentity::mechanism("M1", "flicker"),
                    noise_type: NoiseSourceType::Flicker,
                    output_contribution: 1.0,
                    input_contribution: 0.25,
                    percentage: 0.0,
                },
            ],
        };

        let data = NoiseData::from_input_referred_results_with_abort(
            vec![make_point(100.0), make_point(200.0)],
            &NoAbort,
        )
        .expect("conversion succeeds");

        assert_eq!(data.contributions.len(), 3);
        assert_eq!(data.contributions[0].0, "R1 · rd");
        assert!((data.contributions[0].1 - 50.0).abs() < 1.0e-12);
        assert_eq!(data.contributions[1].0, "M1 · flicker");
        assert!((data.contributions[1].1 - 25.0).abs() < 1.0e-12);
        assert_eq!(data.contributions[2].0, "R1 · rs");
        assert!((data.contributions[2].1 - 25.0).abs() < 1.0e-12);
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
