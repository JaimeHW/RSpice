//! Transient analysis.
//!
//! Time-domain integration from an initial condition, returning node
//! voltage waveforms.

use super::error::{ensure_not_aborted, poll_periodically};
use super::{
    ServiceRunError, ServiceRunResult, build_engine_config, parse_runner_netlist_with_abort,
};
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::diagnostics::ConvergenceQuality;
use rspice_core::engine::{Engine, TransientResult};
use std::collections::HashSet;
use std::path::Path;

// Reached only by the legacy `cfg(test)` whole-deck path; see [`SimulationResult`].
#[cfg(test)]
use super::now_ms;
#[cfg(test)]
use rspice_core::abort_signal::NoAbort;
#[cfg(test)]
use rspice_core::netlist::AnalysisCommand;

/// Result of a whole-deck simulation run in the legacy shape, where an
/// ordinary parse or solver failure is carried in `success`/`error` rather
/// than returned as an error.
///
/// Superseded: whole-deck execution ships through
/// [`crate::simulation::runner`], which dispatches per analysis and returns
/// each analysis' own typed data. Retained under `cfg(test)` only to pin the
/// contract that cancellation stays a typed error in this shape; delete the
/// legacy family once that contract is asserted against the shipping path.
#[cfg(test)]
#[derive(Debug, Clone)]
pub struct SimulationResult {
    /// Whether the simulation succeeded
    pub success: bool,

    /// Waveform data from transient analysis
    pub transient: Option<TransientData>,

    /// DC operating point voltages (node_name, voltage)
    pub dc_op: Option<Vec<(String, Value)>>,

    /// Error message if simulation failed
    pub error: Option<String>,

    /// Simulation statistics
    pub stats: SimulationStats,
}

/// Transient analysis waveform data
#[derive(Debug, Clone)]
pub struct TransientData {
    /// Time points
    pub time: Vec<Value>,

    /// Node voltages: (node_name, values)
    pub voltages: Vec<(String, Vec<Value>)>,

    /// What the solver had to do to produce these waveforms.
    ///
    /// Carried alongside the data because it qualifies it: force-accepted
    /// points are samples the solver could not converge and kept anyway, so a
    /// run that looks smooth can still be untrustworthy at those times.
    pub convergence: ConvergenceQuality,
}

impl TransientData {
    /// Validate and move retained engine voltage waveforms with cooperative
    /// cancellation. Empty vectors identify signals omitted by output selection.
    pub fn from_result_with_abort(
        result: TransientResult,
        node_names: &[String],
        abort: &dyn AbortSignal,
    ) -> ServiceRunResult<Self> {
        ensure_not_aborted(abort)?;
        let invalid_time_axis = || {
            ServiceRunError::Failure(
                "Transient engine returned an empty, non-finite, or non-increasing time axis"
                    .to_owned(),
            )
        };
        if result.time.is_empty() {
            return Err(invalid_time_axis());
        }
        for (index, &time) in result.time.iter().enumerate() {
            poll_periodically(abort, index)?;
            if !time.is_finite() || (index > 0 && time <= result.time[index - 1]) {
                return Err(invalid_time_axis());
            }
        }
        if node_names != result.node_names || node_names.len() != result.voltages.len() {
            return Err(ServiceRunError::Failure(format!(
                "Transient engine returned {} node names for {} voltage waveforms",
                node_names.len(),
                result.voltages.len()
            )));
        }

        let mut voltages = Vec::with_capacity(node_names.len());
        let mut seen_names = HashSet::with_capacity(node_names.len());

        for (name, samples) in node_names.iter().zip(result.voltages) {
            ensure_not_aborted(abort)?;
            let normalized = name.trim().to_ascii_lowercase();
            if normalized.is_empty() || !seen_names.insert(normalized) {
                return Err(ServiceRunError::Failure(
                    "Transient engine returned an empty or duplicate node identity".to_owned(),
                ));
            }
            if samples.is_empty() {
                continue;
            }
            if samples.len() != result.time.len() {
                return Err(ServiceRunError::Failure(format!(
                    "Transient node '{name}' returned an invalid sample vector"
                )));
            }
            for (sample_index, sample) in samples.iter().enumerate() {
                poll_periodically(abort, sample_index)?;
                if !sample.is_finite() {
                    return Err(ServiceRunError::Failure(format!(
                        "Transient node '{name}' returned an invalid sample vector"
                    )));
                }
            }
            if name == "0" || name.eq_ignore_ascii_case("gnd") {
                continue;
            }

            voltages.push((format!("V({name})"), samples));
        }
        if voltages.is_empty() {
            return Err(ServiceRunError::Failure(
                "Transient engine returned no non-ground voltage waveforms".to_owned(),
            ));
        }

        ensure_not_aborted(abort)?;
        Ok(Self {
            time: result.time,
            voltages,
            // The engine is not in scope here; callers that have it overwrite
            // this from `Engine::convergence_quality`.
            convergence: ConvergenceQuality::default(),
        })
    }
}

/// Timing and point-count statistics for a legacy [`SimulationResult`].
#[cfg(test)]
#[derive(Debug, Clone, Default)]
pub struct SimulationStats {
    /// Parse time in milliseconds
    pub parse_time_ms: f64,

    /// Simulation time in milliseconds
    pub sim_time_ms: f64,

    /// Number of time points
    pub num_points: usize,
}

/// Run a whole deck in the legacy [`SimulationResult`] shape with cooperative
/// cancellation. Test-only; see [`SimulationResult`].
#[cfg(test)]
pub fn run_simulation_with_abort(
    netlist_text: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<SimulationResult> {
    run_simulation_with_options_and_source_path_and_abort(netlist_text, None, None, abort)
}

/// Canonical legacy whole-deck entry point, with explicit options and
/// source-path resolution.
///
/// This legacy result shape carries ordinary parse/solver failures in its
/// `success`/`error` fields. Cooperative cancellation is the sole typed error
/// so callers cannot mistake it for a failed simulation. Test-only; see
/// [`SimulationResult`].
#[cfg(test)]
pub fn run_simulation_with_options_and_source_path_and_abort(
    netlist_text: &str,
    options: Option<&crate::simulation::dialog::SimulationOptions>,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<SimulationResult> {
    ensure_not_aborted(abort)?;
    let mut stats = SimulationStats::default();

    // Parse the netlist
    let parse_start = now_ms();
    let netlist = match parse_runner_netlist_with_abort(netlist_text, source_path, abort) {
        Ok(nl) => nl,
        Err(ServiceRunError::Aborted) => return Err(ServiceRunError::Aborted),
        Err(error @ ServiceRunError::ResourceLimit(_)) => return Err(error),
        Err(error) => {
            return Ok(failed_simulation_result(error.to_string(), stats));
        }
    };
    stats.parse_time_ms = now_ms() - parse_start;
    ensure_not_aborted(abort)?;

    // Create engine config with proper precedence:
    // default/UI base < netlist .OPTIONS < explicit UI overrides.
    let config = build_engine_config(&netlist, options);
    let engine = Engine::new(config);

    // Extract transient parameters from analyses
    let mut tran_params = None;
    for analysis in &netlist.analyses {
        ensure_not_aborted(abort)?;
        if tran_params.is_none()
            && let AnalysisCommand::Tran { step, stop, .. } = analysis
        {
            tran_params = Some((*step, *stop));
        }
    }

    let sim_start = now_ms();

    // Run transient if requested
    let transient = if let Some((tstep, tstop)) = tran_params {
        match engine.run_tran_with_abort(&netlist, tstop, tstep, abort) {
            Ok(tran_result) => {
                stats.num_points = tran_result.time.len();

                // Use actual node names from the simulation result
                // These are populated from the circuit's node_map (e.g., "N001", "N002")
                let node_names = tran_result.node_names.clone();
                Some(TransientData::from_result_with_abort(
                    tran_result,
                    &node_names,
                    abort,
                )?)
            }
            Err(error) => {
                let error = ServiceRunError::from_core("Transient error", error);
                if !matches!(error, ServiceRunError::Failure(_)) {
                    return Err(error);
                }
                return Ok(failed_simulation_result(error.to_string(), stats));
            }
        }
    } else {
        None
    };

    // Always run DC OP (operating point is always available)
    // This enables DC annotation display for any simulation type.
    // Note: DC OP failure is only fatal if we have no transient results.
    let dc_op = match engine.run_dc_op_with_abort(&netlist, abort) {
        Ok(result) => {
            let mut ops = Vec::new();

            for (idx, &v) in result.node_voltages.iter().enumerate() {
                ensure_not_aborted(abort)?;
                if idx > 0 {
                    let node_name = result
                        .node_names
                        .get(idx)
                        .cloned()
                        .unwrap_or_else(|| idx.to_string());
                    ops.push((node_name, v));
                }
            }
            log::info!("DC operating point computed: {} node voltages", ops.len());
            Some(ops)
        }
        Err(error) => {
            let error = ServiceRunError::from_core("DC OP error", error);
            if !matches!(error, ServiceRunError::Failure(_)) {
                return Err(error);
            }
            log::warn!(
                "DC OP computation failed: {} (continuing with transient if available)",
                error
            );
            // DC OP failure is fatal only if we have no transient
            if transient.is_none() {
                return Ok(failed_simulation_result(error.to_string(), stats));
            }
            None
        }
    };

    stats.sim_time_ms = now_ms() - sim_start;
    ensure_not_aborted(abort)?;

    Ok(SimulationResult {
        success: true,
        transient,
        dc_op,
        error: None,
        stats,
    })
}

#[cfg(test)]
fn failed_simulation_result(error: String, stats: SimulationStats) -> SimulationResult {
    SimulationResult {
        success: false,
        transient: None,
        dc_op: None,
        error: Some(error),
        stats,
    }
}

/// Run transient analysis with explicit parameters and cooperative
/// cancellation.
///
/// Test-only. The shipping path is
/// [`run_transient_analysis_with_source_path_and_abort`].
#[cfg(test)]
pub fn run_transient_analysis_with_abort(
    netlist_text: &str,
    stop_time: Value,
    step_time: Value,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<TransientData> {
    run_transient_analysis_with_source_path_and_abort(
        netlist_text,
        stop_time,
        step_time,
        None,
        abort,
    )
}

/// Run transient analysis with explicit parameters, source-path resolution,
/// and cooperative cancellation.
pub fn run_transient_analysis_with_source_path_and_abort(
    netlist_text: &str,
    stop_time: Value,
    step_time: Value,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<TransientData> {
    ensure_not_aborted(abort)?;
    let validation = validate_transient_parameters(stop_time, step_time);
    ensure_not_aborted(abort)?;
    validation.map_err(ServiceRunError::Failure)?;

    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    ensure_not_aborted(abort)?;

    let engine = Engine::new(build_engine_config(&netlist, None));
    let result = engine
        .run_tran_with_abort(&netlist, stop_time, step_time, abort)
        .map_err(|error| ServiceRunError::from_core("Transient analysis error", error))?;

    let node_names = result.node_names.clone();
    let mut data = TransientData::from_result_with_abort(result, &node_names, abort)?;
    data.convergence = engine.convergence_quality();
    Ok(data)
}

fn validate_transient_parameters(stop_time: Value, step_time: Value) -> Result<(), String> {
    if !stop_time.is_finite() || stop_time <= 0.0 {
        return Err("Transient stop_time must be finite and > 0".to_string());
    }
    if !step_time.is_finite() || step_time <= 0.0 {
        return Err("Transient step_time must be finite and > 0".to_string());
    }
    if step_time > stop_time {
        return Err("Transient step_time must be <= stop_time".to_string());
    }
    Ok(())
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

    const DECK: &str = "Transient cancellation\n\
         V1 out 0 1\n\
         R1 out 0 1k\n\
         .end\n";

    #[test]
    fn transient_runner_threads_abort_into_the_core_solver() {
        let abort = AbortOnPoll::new(6);
        let result = run_transient_analysis_with_abort(DECK, 1.0e-6, 1.0e-9, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn cancellation_precedes_invalid_transient_configuration() {
        let abort = AbortOnPoll::new(2);
        let result = run_transient_analysis_with_abort(DECK, -1.0, 0.0, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn legacy_simulation_shape_still_returns_typed_cancellation() {
        let abort = AbortOnPoll::new(3);
        let result = run_simulation_with_abort(DECK, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn legacy_simulation_shape_reports_success_in_band() {
        const TRAN_DECK: &str = "Legacy simulation shape\n\
             V1 out 0 1\n\
             R1 out 0 1k\n\
             .tran 1n 10n\n\
             .end\n";

        let result = run_simulation_with_abort(TRAN_DECK, &NoAbort).expect("not cancelled");

        // The legacy shape carries outcome in-band: success and error are the
        // status, not the Result arm.
        assert!(result.success);
        assert!(result.error.is_none());

        let transient = result.transient.expect("`.tran` produces waveforms");
        assert!(!transient.time.is_empty());
        assert_eq!(result.stats.num_points, transient.time.len());

        let dc_op = result.dc_op.expect("operating point is always attempted");
        assert_eq!(dc_op.len(), 1, "one non-ground node");
        assert_eq!(dc_op[0].1, 1.0, "V1 holds the node at 1 V");
    }

    fn conversion_result() -> TransientResult {
        TransientResult {
            time: vec![0.0, 1.0],
            step_sizes: vec![0.0, 1.0],
            voltages: vec![vec![1.0, 1.0], vec![0.5, 0.5]],
            branch_currents: Vec::new(),
            num_nodes: 2,
            node_names: vec!["out".to_string(), "sense".to_string()],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            digital_buses: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        }
    }

    #[test]
    fn transient_conversion_observes_counter_abort() {
        let result = conversion_result();
        let names = result.node_names.clone();
        let abort = AbortOnPoll::new(2);

        let converted = TransientData::from_result_with_abort(result, &names, &abort);

        assert!(matches!(converted, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn transient_runner_preserves_selected_voltage_outputs() {
        for (selection, expected) in [("out", 0.5), ("in", 1.0)] {
            let deck = format!(
                "Selected transient output\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.save V({selection})\n.end\n"
            );
            let result = run_transient_analysis_with_source_path_and_abort(
                &deck, 10e-6, 1e-6, None, &NoAbort,
            )
            .expect("unselected nodes must not invalidate a retained voltage waveform");
            assert_eq!(result.voltages.len(), 1);
            let (name, values) = &result.voltages[0];
            assert!(name.eq_ignore_ascii_case(&format!("V({selection})")));
            assert_eq!(values.len(), result.time.len());
            assert!(values.iter().all(|value| (value - expected).abs() < 1e-12));
            assert_eq!(result.time.last().copied(), Some(10e-6));
        }
    }

    #[test]
    fn transient_conversion_rejects_malformed_retained_data() {
        let mut cases = Vec::new();
        for time in [
            vec![],
            vec![0.0, Value::NAN],
            vec![0.0, Value::INFINITY],
            vec![0.0, 0.0],
            vec![1.0, 0.0],
        ] {
            let mut result = conversion_result();
            result.time = time;
            cases.push(result);
        }
        for samples in [vec![0.5], vec![0.5, Value::NAN], vec![0.5, Value::INFINITY]] {
            let mut result = conversion_result();
            result.voltages[1] = samples;
            cases.push(result);
        }
        for name in ["", "OUT"] {
            let mut result = conversion_result();
            result.node_names[1] = name.to_owned();
            // Omitted traces still participate in node-identity validation.
            result.voltages[1].clear();
            cases.push(result);
        }
        let mut missing_vector = conversion_result();
        missing_vector.voltages.pop();
        cases.push(missing_vector);
        let mut no_retained_voltage = conversion_result();
        no_retained_voltage.voltages.iter_mut().for_each(Vec::clear);
        cases.push(no_retained_voltage);
        for (index, result) in cases.into_iter().enumerate() {
            let names = result.node_names.clone();
            assert!(
                matches!(
                    TransientData::from_result_with_abort(result, &names, &NoAbort),
                    Err(ServiceRunError::Failure(_))
                ),
                "malformed case {index} must fail"
            );
        }
        let result = conversion_result();
        let mut names = result.node_names.clone();
        names.swap(0, 1);
        assert!(matches!(
            TransientData::from_result_with_abort(result, &names, &NoAbort),
            Err(ServiceRunError::Failure(_))
        ));
    }

    #[test]
    fn transient_conversion_polls_during_time_and_sample_validation() {
        let mut result = conversion_result();
        result.time = (0..256).map(Value::from).collect();
        result.time[255] = Value::NAN;
        let names = result.node_names.clone();
        assert!(matches!(
            TransientData::from_result_with_abort(result, &names, &AbortOnPoll::new(3)),
            Err(ServiceRunError::Aborted)
        ));

        let mut result = conversion_result();
        result.time = (0..256).map(Value::from).collect();
        result.voltages = vec![vec![1.0; 256], vec![0.5; 256]];
        result.voltages[0][255] = Value::NAN;
        let names = result.node_names.clone();
        // Entry, four time-axis polls, then the first node and its samples.
        assert!(matches!(
            TransientData::from_result_with_abort(result, &names, &AbortOnPoll::new(8)),
            Err(ServiceRunError::Aborted)
        ));
    }
}
