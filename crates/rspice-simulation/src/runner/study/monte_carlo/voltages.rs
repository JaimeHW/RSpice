//! All-node OP observations share the configured-study continuation machinery.
use super::*;
#[cfg(test)]
use crate::study::monte_carlo::prepare_voltages;
use crate::study::monte_carlo::prepare_voltages_with_context;

pub(crate) fn run(
    source: &str,
    variation_source: McVariationSource,
    histogram_bins: usize,
    environment: Option<AnalysisExecutionEnvironment>,
    context: ServiceContext<'_>,
    continuation: Option<MonteCarloContinuation<'_>>,
) -> Result<services::MonteCarloData, SimulationError> {
    let abort = context.abort;
    let (circuit, engine, study, basis) = prepare_voltages_with_context(
        source,
        variation_source,
        histogram_bins,
        environment,
        context,
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
