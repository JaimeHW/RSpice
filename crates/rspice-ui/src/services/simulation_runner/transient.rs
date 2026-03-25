use super::{build_engine_config, now_ms, parse_runner_netlist};
use rspice_core::engine::{Engine, TransientResult};
use rspice_core::netlist::AnalysisCommand;
use rspice_core::Value;
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
        let mut voltages = Vec::new();

        for (idx, name) in node_names.iter().enumerate() {
            if name == "0" || name.eq_ignore_ascii_case("gnd") {
                continue;
            }

            if idx < result.voltages.len() {
                voltages.push((format!("V({})", name), result.voltages[idx].clone()));
            }
        }

        Self {
            time: result.time,
            voltages,
        }
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
    run_simulation_with_options_and_source_path(netlist_text, None, None)
}

/// Run a simulation from netlist text with a source path used to resolve
/// relative includes and model file references.
pub fn run_simulation_with_source_path(
    netlist_text: &str,
    source_path: Option<&Path>,
) -> SimulationResult {
    run_simulation_with_options_and_source_path(netlist_text, None, source_path)
}

/// Run a simulation with custom simulation options
///
/// Allows passing UI-configured SimulationOptions to control the solver behavior.
pub fn run_simulation_with_options(
    netlist_text: &str,
    options: Option<&crate::simulation::dialog::SimulationOptions>,
) -> SimulationResult {
    run_simulation_with_options_and_source_path(netlist_text, options, None)
}

/// Run a simulation with custom simulation options and a source path used to
/// resolve relative includes and model file references.
pub fn run_simulation_with_options_and_source_path(
    netlist_text: &str,
    options: Option<&crate::simulation::dialog::SimulationOptions>,
    source_path: Option<&Path>,
) -> SimulationResult {
    let mut stats = SimulationStats::default();

    // Parse the netlist
    let parse_start = now_ms();
    let netlist = match parse_runner_netlist(netlist_text, source_path) {
        Ok(nl) => nl,
        Err(error) => {
            return SimulationResult {
                success: false,
                transient: None,
                dc_op: None,
                error: Some(error),
                stats,
            };
        }
    };
    stats.parse_time_ms = now_ms() - parse_start;

    // Create engine config with proper precedence:
    // default/UI base < netlist .OPTIONS < explicit UI overrides.
    let config = build_engine_config(&netlist, options);
    let engine = Engine::new(config);

    // Extract transient parameters from analyses
    let tran_params = netlist.analyses.iter().find_map(|analysis| {
        if let AnalysisCommand::Tran { step, stop, .. } = analysis {
            Some((*step, *stop))
        } else {
            None
        }
    });

    let _has_op = netlist
        .analyses
        .iter()
        .any(|analysis| matches!(analysis, AnalysisCommand::Op));

    let sim_start = now_ms();

    // Run transient if requested
    let transient = if let Some((tstep, tstop)) = tran_params {
        match engine.run_tran(&netlist, tstop, tstep) {
            Ok(tran_result) => {
                stats.num_points = tran_result.time.len();

                // Use actual node names from the simulation result
                // These are populated from the circuit's node_map (e.g., "N001", "N002")
                Some(TransientData::from_result(
                    tran_result.clone(),
                    &tran_result.node_names,
                ))
            }
            Err(e) => {
                return SimulationResult {
                    success: false,
                    transient: None,
                    dc_op: None,
                    error: Some(format!("Transient error: {}", e)),
                    stats,
                };
            }
        }
    } else {
        None
    };

    // Always run DC OP (operating point is always available)
    // This enables DC annotation display for any simulation type.
    // Note: DC OP failure is only fatal if we have no transient results.
    let dc_op = match engine.run_dc_op(&netlist) {
        Ok(result) => {
            let mut ops = Vec::new();

            for (idx, &v) in result.node_voltages.iter().enumerate() {
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
        Err(e) => {
            log::warn!(
                "DC OP computation failed: {} (continuing with transient if available)",
                e
            );
            // DC OP failure is fatal only if we have no transient
            if transient.is_none() {
                return SimulationResult {
                    success: false,
                    transient: None,
                    dc_op: None,
                    error: Some(format!("DC OP error: {}", e)),
                    stats,
                };
            }
            None
        }
    };

    stats.sim_time_ms = now_ms() - sim_start;

    SimulationResult {
        success: true,
        transient,
        dc_op,
        error: None,
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
    run_transient_analysis_with_source_path(netlist_text, stop_time, step_time, None)
}

/// Run transient analysis with explicit parameters and a source path used to
/// resolve relative includes and model file references.
pub fn run_transient_analysis_with_source_path(
    netlist_text: &str,
    stop_time: Value,
    step_time: Value,
    source_path: Option<&Path>,
) -> Result<TransientData, String> {
    if stop_time <= 0.0 {
        return Err("Transient stop_time must be > 0".to_string());
    }
    if step_time <= 0.0 {
        return Err("Transient step_time must be > 0".to_string());
    }
    if step_time > stop_time {
        return Err("Transient step_time must be <= stop_time".to_string());
    }

    let netlist = parse_runner_netlist(netlist_text, source_path)?;

    let engine = Engine::new(build_engine_config(&netlist, None));
    let result = engine
        .run_tran(&netlist, stop_time, step_time)
        .map_err(|e| format!("Transient analysis error: {}", e))?;

    let node_names = result.node_names.clone();
    Ok(TransientData::from_result(result, &node_names))
}
