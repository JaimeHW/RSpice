//! Simulation Runner
//!
//! Async wrapper around rspice-core for running simulations from the GUI.

use rspice_core::engine::{Engine, SimulationConfig, TransientResult};
use rspice_core::netlist::AnalysisCommand;
use rspice_core::Value;

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
            // Skip ground node
            if name == "0" || name.eq_ignore_ascii_case("gnd") {
                continue;
            }

            // Get waveform for this node (idx is 0-based, but voltages array is also 0-based)
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
    use std::time::Instant;

    let mut stats = SimulationStats::default();

    // Parse the netlist
    let parse_start = Instant::now();
    let netlist = match rspice_core::netlist::parse_netlist(netlist_text) {
        Ok(nl) => nl,
        Err(e) => {
            return SimulationResult {
                success: false,
                transient: None,
                dc_op: None,
                error: Some(format!("Parse error: {}", e)),
                stats,
            };
        }
    };
    stats.parse_time_ms = parse_start.elapsed().as_secs_f64() * 1000.0;

    // Create engine with default config
    let engine = Engine::new(SimulationConfig::default());

    // Extract transient parameters from analyses
    let tran_params = netlist.analyses.iter().find_map(|a| {
        if let AnalysisCommand::Tran { step, stop, .. } = a {
            Some((*step, *stop))
        } else {
            None
        }
    });

    let has_op = netlist
        .analyses
        .iter()
        .any(|a| matches!(a, AnalysisCommand::Op));

    let sim_start = Instant::now();

    // Run transient if requested
    let transient = if let Some((tstep, tstop)) = tran_params {
        match engine.run_tran(&netlist, tstop, tstep) {
            Ok(result) => {
                stats.num_points = result.time.len();

                // Get node names from the circuit (we need to build it to get names)
                // For now, generate generic names based on node count
                let node_names: Vec<String> =
                    (1..=result.num_nodes).map(|i| i.to_string()).collect();

                Some(TransientData::from_result(result, &node_names))
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

    // Run DC OP if requested or as fallback
    let dc_op = if has_op || transient.is_none() {
        match engine.run_dc_op(&netlist) {
            Ok(result) => {
                let mut ops = Vec::new();
                for (idx, &v) in result.node_voltages.iter().enumerate() {
                    if idx > 0 {
                        // Skip ground
                        ops.push((format!("V({})", idx), v));
                    }
                }
                Some(ops)
            }
            Err(e) => {
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
        }
    } else {
        None
    };

    stats.sim_time_ms = sim_start.elapsed().as_secs_f64() * 1000.0;

    SimulationResult {
        success: true,
        transient,
        dc_op,
        error: None,
        stats,
    }
}
