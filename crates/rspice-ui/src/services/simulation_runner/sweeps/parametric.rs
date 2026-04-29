use super::execution::run_temperature_sweep;
use super::mapping::{describe_step_target, map_dc_sweep_results, map_temperature_results};
use super::sweep_points::expand_step_sweep_values;
use super::types::{CornerBaseMode, ParametricData, TempRunConfig};
use rspice_core::engine::Engine;
use rspice_core::netlist::{AnalysisCommand, StepTarget};
use std::path::Path;

/// Run parametric analysis by executing the first `.STEP` command in the netlist.
pub fn run_parametric_analysis(netlist_text: &str) -> Result<ParametricData, String> {
    run_parametric_analysis_with_source_path(netlist_text, None)
}

/// Run parametric analysis by executing the first `.STEP` command in the
/// netlist, resolving relative includes from the source path when provided.
pub fn run_parametric_analysis_with_source_path(
    netlist_text: &str,
    source_path: Option<&Path>,
) -> Result<ParametricData, String> {
    let netlist = super::super::parse_runner_netlist(netlist_text, source_path)?;

    let step_cmd = netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::Step(step) => Some(step),
            _ => None,
        })
        .ok_or_else(|| "Parametric analysis requires a .STEP command in the netlist".to_string())?;

    let values = expand_step_sweep_values(&step_cmd.sweep).map_err(|err| err.to_string())?;
    if values.is_empty() {
        return Err("Parametric analysis has no sweep points to execute".to_string());
    }

    let results = if step_cmd.target == StepTarget::Temp {
        let cfg = TempRunConfig {
            temperatures_c: values.clone(),
            base_mode: CornerBaseMode::Op,
        };
        return run_parametric_analysis_with_netlist_and_config(&netlist, &cfg, "TEMP");
    } else {
        let engine = Engine::new(super::super::build_engine_config(&netlist, None));
        engine
            .run_step_command(&netlist, step_cmd, &values)
            .map_err(|e| format!("Parametric analysis error: {}", e))?
    };

    if results.is_empty() {
        return Err("Parametric analysis produced no converged sweep points".to_string());
    }

    let num_failures = values.len().saturating_sub(results.len());
    let (sweep_values, voltages) = map_dc_sweep_results(&results);

    Ok(ParametricData {
        target: describe_step_target(step_cmd),
        num_points: sweep_values.len(),
        sweep_values,
        voltages,
        num_failures,
    })
}

/// Run temperature sweep analysis with explicit base-mode configuration.
pub fn run_parametric_analysis_with_config(
    netlist_text: &str,
    config: &TempRunConfig,
) -> Result<ParametricData, String> {
    run_parametric_analysis_with_config_and_source_path(netlist_text, config, None)
}

/// Run temperature sweep analysis with explicit base-mode configuration and a
/// source path used to resolve relative includes and model file references.
pub fn run_parametric_analysis_with_config_and_source_path(
    netlist_text: &str,
    config: &TempRunConfig,
    source_path: Option<&Path>,
) -> Result<ParametricData, String> {
    let netlist = super::super::parse_runner_netlist(netlist_text, source_path)?;
    run_parametric_analysis_with_netlist_and_config(&netlist, config, "TEMP")
}

fn run_parametric_analysis_with_netlist_and_config(
    netlist: &rspice_core::Netlist,
    config: &TempRunConfig,
    target: &str,
) -> Result<ParametricData, String> {
    config.validate()?;

    let results = run_temperature_sweep(netlist, &config.temperatures_c, &config.base_mode)?;
    if results.is_empty() {
        return Err("Parametric analysis produced no converged sweep points".to_string());
    }

    let num_failures = config.temperatures_c.len().saturating_sub(results.len());
    let metric_label = config.base_mode.metric_label();
    let (sweep_values, voltages) = map_temperature_results(&results, metric_label);

    Ok(ParametricData {
        target: target.to_string(),
        num_points: sweep_values.len(),
        sweep_values,
        voltages,
        num_failures,
    })
}
