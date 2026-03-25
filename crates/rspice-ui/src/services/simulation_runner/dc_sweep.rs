//! DC sweep analysis runner.

use super::{build_engine_config, parse_runner_netlist};
use rspice_core::engine::Engine;
use rspice_core::Value;
use std::path::Path;

/// DC sweep analysis data
#[derive(Debug, Clone)]
pub struct DcSweepData {
    /// Source name being swept
    pub source_name: String,
    /// Sweep values (x-axis)
    pub sweep_values: Vec<Value>,
    /// Node voltages: (node_name, values at each sweep point)
    pub voltages: Vec<(String, Vec<Value>)>,
    /// Number of sweep points
    pub num_points: usize,
}

/// Run DC sweep analysis
pub fn run_dc_sweep(
    netlist_text: &str,
    source_name: &str,
    start: Value,
    stop: Value,
    step: Value,
) -> Result<DcSweepData, String> {
    run_dc_sweep_with_source_path(netlist_text, source_name, start, stop, step, None)
}

/// Run DC sweep analysis with a source path used to resolve relative includes
/// and model file references.
pub fn run_dc_sweep_with_source_path(
    netlist_text: &str,
    source_name: &str,
    start: Value,
    stop: Value,
    step: Value,
    source_path: Option<&Path>,
) -> Result<DcSweepData, String> {
    let netlist = parse_runner_netlist(netlist_text, source_path)?;

    // Create engine and run DC sweep
    let engine = Engine::new(build_engine_config(&netlist, None));
    let results = engine
        .run_dc_sweep(&netlist, source_name, start, stop, step)
        .map_err(|e| format!("DC sweep error: {}", e))?;

    // Extract sweep values and voltages
    let sweep_values: Vec<Value> = results.iter().map(|(v, _)| *v).collect();
    let num_points = sweep_values.len();

    // Build voltage vectors for each node
    let mut voltages = Vec::new();
    if !results.is_empty() {
        let num_nodes = results[0].1.node_voltages.len();
        let node_names = &results[0].1.node_names;

        for node_idx in 1..num_nodes {
            // Skip ground (node 0)
            let values: Vec<Value> = results
                .iter()
                .map(|(_, result)| result.node_voltages.get(node_idx).copied().unwrap_or(0.0))
                .collect();

            let node_name = node_names
                .get(node_idx)
                .cloned()
                .unwrap_or_else(|| node_idx.to_string());
            voltages.push((format!("V({})", node_name), values));
        }
    }

    Ok(DcSweepData {
        source_name: source_name.to_string(),
        sweep_values,
        voltages,
        num_points,
    })
}
