//! Dispatch for swept and statistical analyses.

use std::collections::HashMap;
use std::path::Path;

use rspice_core::abort_signal::AbortSignal;

use crate::services::simulation_runner as svc_runner;
use crate::simulation::multi_run::AnalysisSpec;
use crate::simulation::results::{MonteCarloVariableResult, SimulationResult, WaveformData};
use crate::simulation::runner::{
    AnalysisExecutionEnvironment, SimulationError, SpecExecutionOptions,
};

pub(super) fn run_sweep_spec(
    spec: AnalysisSpec,
    options: SpecExecutionOptions,
    netlist: &str,
    source_path: Option<&Path>,
    environment: Option<AnalysisExecutionEnvironment>,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    super::ensure_not_aborted(abort)?;
    match spec {
        AnalysisSpec::MonteCarlo { variation_source } => {
            run_monte_carlo(variation_source, netlist, source_path, environment, abort)
        }
        AnalysisSpec::Parametric => run_parametric(netlist, options, source_path, abort),
        // A corner declaration is expanded into one task per declared point
        // before the run is authorized, and its plotting family is assembled
        // from those results. Nothing solves the declaration itself, so a
        // corner spec arriving here is a routing fault, not a request.
        other => Err(super::misrouted_spec_error("sweep", &other)),
    }
}

fn run_monte_carlo(
    variation_source: crate::simulation::dialog::McVariationSource,
    netlist: &str,
    source_path: Option<&Path>,
    environment: Option<AnalysisExecutionEnvironment>,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    use crate::simulation::dialog::McVariationSource;

    let (temperature, supply, nominal_supply, supply_source_names) = environment.map_or_else(
        || (None, None, None, Vec::new()),
        |environment| {
            (
                Some(environment.temperature_celsius),
                environment.supply_voltage,
                environment.nominal_supply_voltage,
                environment.supply_source_names,
            )
        },
    );

    let data = super::run_abort_aware_service(abort, || match variation_source {
        McVariationSource::ParameterTolerance => {
            svc_runner::run_monte_carlo_analysis_with_environment_and_source_path_and_abort(
                netlist,
                source_path,
                temperature,
                supply,
                nominal_supply,
                &supply_source_names,
                abort,
            )
        }
        McVariationSource::DeckStatistics => {
            svc_runner::run_statistical_monte_carlo_with_environment_and_source_path_and_abort(
                netlist,
                source_path,
                temperature,
                supply,
                nominal_supply,
                &supply_source_names,
                abort,
            )
        }
    })?;
    let mut variables = Vec::with_capacity(data.variables.len());
    for variable in data.variables {
        super::ensure_not_aborted(abort)?;
        variables.push(MonteCarloVariableResult {
            name: variable.name,
            samples: variable.samples,
            mean: variable.mean,
            std_dev: variable.std_dev,
            min: variable.min,
            max: variable.max,
            histogram: variable.histogram,
            bin_edges: variable.bin_edges,
        });
    }

    Ok(SimulationResult::MonteCarlo {
        seed: data.seed,
        runs_requested: data.runs_requested,
        runs_completed: data.runs_completed,
        num_failures: data.num_failures,
        all_converged: data.all_converged,
        variables,
    })
}

/// Step a design parameter through the `.STEP` command the deck declares.
///
/// A temperature step is not this. It declares a PVT axis, expands into one
/// task per temperature before the run is authorized, and has its family
/// assembled from those results — so a temperature contract arriving here means
/// a declaration reached an executor instead of being expanded.
fn run_parametric(
    netlist: &str,
    options: SpecExecutionOptions,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    if options.temp.is_some() {
        return Err(SimulationError::InvalidConfig(
            "sweep runner received a temperature step, which is solved one declared point at a time"
                .to_owned(),
        ));
    }
    let data = super::run_abort_aware_service(abort, || {
        if let Some(base_mode) = options.parametric_base.as_ref() {
            svc_runner::run_parametric_analysis_with_base_and_source_path_and_abort(
                netlist,
                source_path,
                base_mode,
                abort,
            )
        } else {
            svc_runner::run_parametric_analysis_with_source_path_and_abort(
                netlist,
                source_path,
                abort,
            )
        }
    })?;
    let sweep_values = data.sweep_values;
    let mut waveforms = HashMap::with_capacity(data.voltages.len());
    for (name, values) in data.voltages {
        super::ensure_not_aborted(abort)?;
        waveforms.insert(
            name.clone(),
            WaveformData::new_time_domain(name, sweep_values.clone(), values),
        );
    }

    Ok(SimulationResult::Parametric {
        target: data.target,
        sweep_values,
        waveforms,
        num_failures: data.num_failures,
    })
}
