use super::error::{ensure_not_aborted, poll_periodically};
use super::{
    ServiceRunError, ServiceRunResult, build_engine_config, generate_freq_points_with_abort,
    parse_runner_netlist_with_abort,
};
use num_complex::Complex64;
use rspice_core::Value;
use rspice_core::abort_signal::{AbortSignal, NoAbort};
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
        Self::from_results_with_abort(results, &NoAbort)
            .expect("NoAbort cannot cancel AC result conversion")
    }

    /// Create from engine results with cooperative cancellation during result
    /// transposition.
    pub fn from_results_with_abort(
        results: Vec<AcResult>,
        abort: &dyn AbortSignal,
    ) -> ServiceRunResult<Self> {
        ensure_not_aborted(abort)?;
        let mut frequencies = Vec::with_capacity(results.len());
        for (point_idx, result) in results.iter().enumerate() {
            poll_periodically(abort, point_idx)?;
            frequencies.push(result.frequency);
        }
        let num_points = frequencies.len();

        let mut responses = Vec::new();
        if let Some(first_result) = results.first() {
            for (ac_idx, name) in first_result.node_names.iter().enumerate() {
                ensure_not_aborted(abort)?;
                let mut values = Vec::with_capacity(results.len());
                for (point_idx, result) in results.iter().enumerate() {
                    poll_periodically(abort, point_idx)?;
                    if let Some(value) = result.voltages.get(ac_idx).copied() {
                        values.push(value);
                    }
                }
                if !values.is_empty() {
                    responses.push((format!("V({})", name), values));
                }
            }
        }

        ensure_not_aborted(abort)?;
        Ok(Self {
            frequencies,
            responses,
            num_points,
        })
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
    run_ac_analysis_with_abort(
        netlist_text,
        start_freq,
        stop_freq,
        num_points,
        sweep_type,
        &NoAbort,
    )
    .map_err(|error| error.to_string())
}

/// Run AC small-signal analysis with cooperative cancellation.
pub fn run_ac_analysis_with_abort(
    netlist_text: &str,
    start_freq: Value,
    stop_freq: Value,
    num_points: usize,
    sweep_type: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<AcData> {
    run_ac_analysis_with_source_path_and_abort(
        netlist_text,
        start_freq,
        stop_freq,
        num_points,
        sweep_type,
        None,
        abort,
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
    run_ac_analysis_with_source_path_and_abort(
        netlist_text,
        start_freq,
        stop_freq,
        num_points,
        sweep_type,
        source_path,
        &NoAbort,
    )
    .map_err(|error| error.to_string())
}

/// Run AC small-signal analysis with source-path resolution and cooperative
/// cancellation through parsing, sweep generation, solving, and conversion.
#[allow(clippy::too_many_arguments)]
pub fn run_ac_analysis_with_source_path_and_abort(
    netlist_text: &str,
    start_freq: Value,
    stop_freq: Value,
    num_points: usize,
    sweep_type: &str,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<AcData> {
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    let frequencies =
        generate_freq_points_with_abort(start_freq, stop_freq, num_points, sweep_type, abort)?;
    ensure_not_aborted(abort)?;
    let engine = Engine::new(build_engine_config(&netlist, None));
    let results = engine
        .run_ac_with_abort(&netlist, &frequencies, abort)
        .map_err(|error| ServiceRunError::from_core("AC analysis error", error))?;

    AcData::from_results_with_abort(results, abort)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

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

    #[test]
    fn ac_runner_observes_counter_abort_during_sweep_preparation() {
        let abort = AbortOnPoll::new(4);
        let result = run_ac_analysis_with_abort(RC_DECK, 1.0, 1.0e6, 100, "dec", &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn cancellation_precedes_invalid_ac_configuration() {
        let abort = AbortOnPoll::new(3);
        let result = run_ac_analysis_with_abort(RC_DECK, 0.0, 1.0e6, 0, "invalid", &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }
}
