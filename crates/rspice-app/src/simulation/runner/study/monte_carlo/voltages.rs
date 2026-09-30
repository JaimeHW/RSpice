//! All-node OP observations share the configured-study continuation machinery.
use super::*;
use rspice_simulation::study::monte_carlo::prepare_voltages;

pub(crate) fn run(
    source: &str,
    source_path: Option<&Path>,
    variation_source: McVariationSource,
    histogram_bins: usize,
    environment: Option<AnalysisExecutionEnvironment>,
    abort: &dyn AbortSignal,
    continuation: Option<MonteCarloContinuation<'_>>,
) -> Result<services::MonteCarloData, SimulationError> {
    let (circuit, engine, study, basis) = prepare_voltages(
        source,
        source_path,
        variation_source,
        histogram_bins,
        environment,
        abort,
    )?
    .into_parts();
    run_prepared(
        circuit,
        study,
        engine,
        basis.identity(),
        abort,
        continuation,
        |engine, trial, abort| {
            let result = engine
                .run_dc_op_with_abort(trial, abort)
                .map_err(|error| EngineBridge::new().translate_error(error))?;
            basis.observe(result)
        },
    )
}

#[cfg(test)]
mod tests;
