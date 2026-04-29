use std::collections::HashMap;

use crate::services::simulation_runner as svc_runner;
use crate::simulation::multi_run::AnalysisSpec;
use crate::simulation::results::{MonteCarloVariableResult, SimulationResult, WaveformData};
use crate::simulation::runner::{SimulationError, SpecExecutionOptions};

pub(super) fn run_sweep_spec(
    spec: AnalysisSpec,
    options: SpecExecutionOptions,
    netlist: &str,
) -> Result<SimulationResult, SimulationError> {
    match spec {
        AnalysisSpec::MonteCarlo => run_monte_carlo(netlist),
        AnalysisSpec::Parametric => run_parametric(netlist, options),
        AnalysisSpec::Corner => run_corner(netlist, options),
        _ => unreachable!("non-sweep spec routed to sweep runner"),
    }
}

fn run_monte_carlo(netlist: &str) -> Result<SimulationResult, SimulationError> {
    let data =
        svc_runner::run_monte_carlo_analysis(netlist).map_err(SimulationError::InvalidConfig)?;
    let variables = data
        .variables
        .into_iter()
        .map(|var| MonteCarloVariableResult {
            name: var.name,
            mean: var.mean,
            std_dev: var.std_dev,
            min: var.min,
            max: var.max,
            histogram: var.histogram,
            bin_edges: var.bin_edges,
        })
        .collect();

    Ok(SimulationResult::MonteCarlo {
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
) -> Result<SimulationResult, SimulationError> {
    let data = if let Some(temp_cfg) = options.temp {
        svc_runner::run_parametric_analysis_with_config(netlist, &temp_cfg)
            .map_err(SimulationError::InvalidConfig)?
    } else {
        svc_runner::run_parametric_analysis(netlist).map_err(SimulationError::InvalidConfig)?
    };
    let sweep_values = data.sweep_values;
    let waveforms: HashMap<String, WaveformData> = data
        .voltages
        .into_iter()
        .map(|(name, values)| {
            (
                name.clone(),
                WaveformData::new_time_domain(name, sweep_values.clone(), values),
            )
        })
        .collect();

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
) -> Result<SimulationResult, SimulationError> {
    let data = if let Some(corner_cfg) = options.corner {
        svc_runner::run_corner_analysis_with_config(netlist, &corner_cfg)
            .map_err(SimulationError::InvalidConfig)?
    } else {
        svc_runner::run_corner_analysis(netlist).map_err(SimulationError::InvalidConfig)?
    };
    let x_values = data.x_values;
    let x_label = data.x_label;
    let x_unit = data.x_unit;
    let temperatures_c = data.temperatures_c;
    let corner_labels = data.corner_labels;
    let waveforms: HashMap<String, WaveformData> = data
        .voltages
        .into_iter()
        .map(|(name, values)| {
            let waveform = WaveformData {
                name: name.clone(),
                x_values: x_values.clone(),
                y_values: values,
                y_unit: "V".to_string(),
                x_unit: x_unit.clone(),
                is_complex: false,
                y_imag: None,
            };
            (name.clone(), waveform)
        })
        .collect();

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
