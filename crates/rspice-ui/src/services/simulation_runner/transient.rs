use super::error::{ensure_not_aborted, poll_periodically};
use super::{
    ServiceRunError, ServiceRunResult, build_engine_config, now_ms, parse_runner_netlist_with_abort,
};
use rspice_core::Value;
use rspice_core::abort_signal::{AbortSignal, NoAbort};
use rspice_core::engine::{Engine, TransientResult};
use rspice_core::netlist::AnalysisCommand;
use std::path::Path;

/// Result of a simulation run
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
}

impl TransientData {
    /// Create from Engine's TransientResult and node names
    pub fn from_result(result: TransientResult, node_names: &[String]) -> Self {
        Self::from_result_with_abort(result, node_names, &NoAbort)
            .expect("NoAbort cannot cancel transient result conversion")
    }

    /// Create from an engine transient result with cooperative cancellation
    /// during waveform transposition.
    pub fn from_result_with_abort(
        result: TransientResult,
        node_names: &[String],
        abort: &dyn AbortSignal,
    ) -> ServiceRunResult<Self> {
        ensure_not_aborted(abort)?;
        let mut voltages = Vec::new();

        for (idx, name) in node_names.iter().enumerate() {
            ensure_not_aborted(abort)?;
            if name == "0" || name.eq_ignore_ascii_case("gnd") {
                continue;
            }

            if idx < result.voltages.len() {
                let samples = &result.voltages[idx];
                let mut copied_samples = Vec::with_capacity(samples.len());
                for (sample_idx, sample) in samples.iter().enumerate() {
                    poll_periodically(abort, sample_idx)?;
                    copied_samples.push(*sample);
                }
                voltages.push((format!("V({})", name), copied_samples));
            }
        }

        ensure_not_aborted(abort)?;
        Ok(Self {
            time: result.time,
            voltages,
        })
    }
}

/// Simulation statistics
#[derive(Debug, Clone, Default)]
pub struct SimulationStats {
    /// Parse time in milliseconds
    pub parse_time_ms: f64,

    /// Simulation time in milliseconds
    pub sim_time_ms: f64,

    /// Number of time points
    pub num_points: usize,
}

/// Run a simulation from netlist text
///
/// This function parses the netlist, runs requested analyses, and extracts results.
pub fn run_simulation(netlist_text: &str) -> SimulationResult {
    compatibility_simulation_result(run_simulation_with_abort(netlist_text, &NoAbort))
}

/// Run a simulation from netlist text with cooperative cancellation.
pub fn run_simulation_with_abort(
    netlist_text: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<SimulationResult> {
    run_simulation_with_options_and_source_path_and_abort(netlist_text, None, None, abort)
}

/// Run a simulation from netlist text with a source path used to resolve
/// relative includes and model file references.
pub fn run_simulation_with_source_path(
    netlist_text: &str,
    source_path: Option<&Path>,
) -> SimulationResult {
    compatibility_simulation_result(run_simulation_with_source_path_and_abort(
        netlist_text,
        source_path,
        &NoAbort,
    ))
}

/// Run a simulation with source-path resolution and cooperative cancellation.
pub fn run_simulation_with_source_path_and_abort(
    netlist_text: &str,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<SimulationResult> {
    run_simulation_with_options_and_source_path_and_abort(netlist_text, None, source_path, abort)
}

/// Run a simulation with custom simulation options
///
/// Allows passing UI-configured SimulationOptions to control the solver behavior.
pub fn run_simulation_with_options(
    netlist_text: &str,
    options: Option<&crate::simulation::dialog::SimulationOptions>,
) -> SimulationResult {
    compatibility_simulation_result(run_simulation_with_options_and_abort(
        netlist_text,
        options,
        &NoAbort,
    ))
}

/// Run a simulation with custom options and cooperative cancellation.
pub fn run_simulation_with_options_and_abort(
    netlist_text: &str,
    options: Option<&crate::simulation::dialog::SimulationOptions>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<SimulationResult> {
    run_simulation_with_options_and_source_path_and_abort(netlist_text, options, None, abort)
}

/// Run a simulation with custom simulation options and a source path used to
/// resolve relative includes and model file references.
pub fn run_simulation_with_options_and_source_path(
    netlist_text: &str,
    options: Option<&crate::simulation::dialog::SimulationOptions>,
    source_path: Option<&Path>,
) -> SimulationResult {
    compatibility_simulation_result(run_simulation_with_options_and_source_path_and_abort(
        netlist_text,
        options,
        source_path,
        &NoAbort,
    ))
}

/// Canonical cancellable simulation entry point with explicit options and
/// source-path resolution.
///
/// This legacy result shape carries ordinary parse/solver failures in its
/// `success`/`error` fields. Cooperative cancellation is the sole typed error
/// so callers cannot mistake it for a failed simulation.
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

fn compatibility_simulation_result(result: ServiceRunResult<SimulationResult>) -> SimulationResult {
    result.unwrap_or_else(|error| {
        failed_simulation_result(error.to_string(), SimulationStats::default())
    })
}

fn failed_simulation_result(error: String, stats: SimulationStats) -> SimulationResult {
    SimulationResult {
        success: false,
        transient: None,
        dc_op: None,
        error: Some(error),
        stats,
    }
}

/// Run transient analysis with explicit parameters.
///
/// Unlike `run_simulation`, this path does not depend on `.tran` directives
/// embedded in the netlist text.
pub fn run_transient_analysis(
    netlist_text: &str,
    stop_time: Value,
    step_time: Value,
) -> Result<TransientData, String> {
    run_transient_analysis_with_abort(netlist_text, stop_time, step_time, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run transient analysis with explicit parameters and cooperative
/// cancellation.
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

/// Run transient analysis with explicit parameters and a source path used to
/// resolve relative includes and model file references.
pub fn run_transient_analysis_with_source_path(
    netlist_text: &str,
    stop_time: Value,
    step_time: Value,
    source_path: Option<&Path>,
) -> Result<TransientData, String> {
    run_transient_analysis_with_source_path_and_abort(
        netlist_text,
        stop_time,
        step_time,
        source_path,
        &NoAbort,
    )
    .map_err(|error| error.to_string())
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
    TransientData::from_result_with_abort(result, &node_names, abort)
}

fn validate_transient_parameters(stop_time: Value, step_time: Value) -> Result<(), String> {
    if stop_time <= 0.0 {
        return Err("Transient stop_time must be > 0".to_string());
    }
    if step_time <= 0.0 {
        return Err("Transient step_time must be > 0".to_string());
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
    fn transient_conversion_observes_counter_abort() {
        let result = TransientResult {
            time: vec![0.0, 1.0],
            voltages: vec![vec![1.0, 1.0], vec![0.5, 0.5]],
            branch_currents: Vec::new(),
            num_nodes: 2,
            node_names: vec!["out".to_string(), "sense".to_string()],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
        };
        let names = result.node_names.clone();
        let abort = AbortOnPoll::new(2);

        let converted = TransientData::from_result_with_abort(result, &names, &abort);

        assert!(matches!(converted, Err(ServiceRunError::Aborted)));
    }
}
