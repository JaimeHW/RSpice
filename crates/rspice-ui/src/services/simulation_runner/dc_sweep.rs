//! DC sweep analysis runner.

use super::error::{ensure_not_aborted, poll_periodically};
use super::{
    ServiceRunError, ServiceRunResult, build_engine_config, parse_runner_netlist_with_abort,
};
use rspice_core::Value;
use rspice_core::abort_signal::{AbortSignal, NoAbort};
use rspice_core::engine::Engine;
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
    run_dc_sweep_with_abort(netlist_text, source_name, start, stop, step, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run DC sweep analysis with cooperative cancellation.
pub fn run_dc_sweep_with_abort(
    netlist_text: &str,
    source_name: &str,
    start: Value,
    stop: Value,
    step: Value,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<DcSweepData> {
    run_dc_sweep_with_source_path_and_abort(
        netlist_text,
        source_name,
        start,
        stop,
        step,
        None,
        abort,
    )
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
    run_dc_sweep_with_source_path_and_abort(
        netlist_text,
        source_name,
        start,
        stop,
        step,
        source_path,
        &NoAbort,
    )
    .map_err(|error| error.to_string())
}

/// Run DC sweep analysis with source-path resolution and cooperative
/// cancellation through parsing, solving, and result transposition.
#[allow(clippy::too_many_arguments)]
pub fn run_dc_sweep_with_source_path_and_abort(
    netlist_text: &str,
    source_name: &str,
    start: Value,
    stop: Value,
    step: Value,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<DcSweepData> {
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    ensure_not_aborted(abort)?;

    // Create engine and run DC sweep
    let engine = Engine::new(build_engine_config(&netlist, None));
    let results = engine
        .run_dc_sweep_with_abort(&netlist, source_name, start, stop, step, abort)
        .map_err(|error| ServiceRunError::from_core("DC sweep error", error))?;

    dc_sweep_data_from_results(source_name, results, abort)
}

fn dc_sweep_data_from_results(
    source_name: &str,
    results: Vec<(Value, rspice_core::SimulationResult)>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<DcSweepData> {
    ensure_not_aborted(abort)?;
    // Extract sweep values and voltages
    let mut sweep_values = Vec::with_capacity(results.len());
    for (point_idx, (value, _)) in results.iter().enumerate() {
        poll_periodically(abort, point_idx)?;
        sweep_values.push(*value);
    }
    let num_points = sweep_values.len();

    // Build voltage vectors for each node
    let mut voltages = Vec::new();
    if !results.is_empty() {
        let num_nodes = results[0].1.node_voltages.len();
        let node_names = &results[0].1.node_names;

        for node_idx in 1..num_nodes {
            ensure_not_aborted(abort)?;
            // Skip ground (node 0)
            let mut values = Vec::with_capacity(results.len());
            for (point_idx, (_, result)) in results.iter().enumerate() {
                poll_periodically(abort, point_idx)?;
                values.push(result.node_voltages.get(node_idx).copied().unwrap_or(0.0));
            }

            let node_name = node_names
                .get(node_idx)
                .cloned()
                .unwrap_or_else(|| node_idx.to_string());
            voltages.push((format!("V({})", node_name), values));
        }
    }

    ensure_not_aborted(abort)?;
    Ok(DcSweepData {
        source_name: source_name.to_string(),
        sweep_values,
        voltages,
        num_points,
    })
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

    const DECK: &str = "DC sweep cancellation\n\
         Vin out 0 0\n\
         R1 out 0 1k\n\
         .end\n";

    #[test]
    fn dc_sweep_observes_counter_abort_before_solving() {
        let abort = AbortOnPoll::new(3);
        let result = run_dc_sweep_with_abort(DECK, "Vin", 0.0, 5.0, 0.1, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn dc_sweep_conversion_observes_counter_abort() {
        let mut point = rspice_core::SimulationResult::new(2, 0);
        point.node_names = vec!["0".to_string(), "in".to_string(), "out".to_string()];
        point.node_voltages = vec![0.0, 1.0, 0.5];
        let results = vec![(0.0, point.clone()), (1.0, point)];
        let abort = AbortOnPoll::new(2);

        let result = dc_sweep_data_from_results("Vin", results, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }
}
