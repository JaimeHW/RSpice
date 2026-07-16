use std::collections::HashMap;
use std::path::Path;

use rspice_core::abort_signal::AbortSignal;

use crate::services::simulation_runner as svc_runner;
use crate::simulation::multi_run::AnalysisSpec;
use crate::simulation::results::{MonteCarloVariableResult, SimulationResult, WaveformData};
use crate::simulation::runner::{SimulationError, SpecExecutionOptions};

pub(super) fn run_sweep_spec(
    spec: AnalysisSpec,
    options: SpecExecutionOptions,
    netlist: &str,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    super::ensure_not_aborted(abort)?;
    match spec {
        AnalysisSpec::MonteCarlo => run_monte_carlo(netlist, source_path, abort),
        AnalysisSpec::Parametric => run_parametric(netlist, options, source_path, abort),
        AnalysisSpec::Corner => run_corner(netlist, options, source_path, abort),
        other => Err(super::misrouted_spec_error("sweep", &other)),
    }
}

fn run_monte_carlo(
    netlist: &str,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let data = super::run_abort_aware_service(abort, || {
        svc_runner::run_monte_carlo_analysis_with_source_path_and_abort(netlist, source_path, abort)
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

fn run_parametric(
    netlist: &str,
    options: SpecExecutionOptions,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let data = if let Some(temp_cfg) = options.temp {
        super::run_abort_aware_service(abort, || {
            svc_runner::run_parametric_analysis_with_config_and_source_path_and_abort(
                netlist,
                &temp_cfg,
                source_path,
                abort,
            )
        })?
    } else {
        super::run_abort_aware_service(abort, || {
            svc_runner::run_parametric_analysis_with_source_path_and_abort(
                netlist,
                source_path,
                abort,
            )
        })?
    };
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

fn run_corner(
    netlist: &str,
    options: SpecExecutionOptions,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let data = if let Some(corner_cfg) = options.corner {
        super::run_abort_aware_service(abort, || {
            svc_runner::run_corner_analysis_with_config_and_source_path_and_abort(
                netlist,
                &corner_cfg,
                source_path,
                abort,
            )
        })?
    } else {
        super::run_abort_aware_service(abort, || {
            svc_runner::run_corner_analysis_with_source_path_and_abort(netlist, source_path, abort)
        })?
    };
    let x_values = data.x_values;
    let x_label = data.x_label;
    let x_unit = data.x_unit;
    let temperatures_c = data.temperatures_c;
    let corner_labels = data.corner_labels;
    let mut waveforms = HashMap::with_capacity(data.voltages.len());
    for (name, values) in data.voltages {
        super::ensure_not_aborted(abort)?;
        let waveform = WaveformData {
            name: name.clone(),
            x_values: x_values.clone(),
            y_values: values,
            y_unit: "V".to_string(),
            x_unit: x_unit.clone(),
            is_complex: false,
            y_imag: None,
        };
        waveforms.insert(name, waveform);
    }

    Ok(SimulationResult::Corner {
        x_values,
        x_label,
        x_unit,
        temperatures_c,
        corner_labels,
        waveforms,
        num_failures: data.num_failures,
    })
}
