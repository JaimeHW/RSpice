use super::execution::{expand_corner_points, run_corner_sweep};
use super::mapping::map_corner_results;
use super::netlist_mutation::infer_nominal_supply_voltage;
use super::sweep_points::extract_temp_points;
use super::types::{CornerData, CornerRunConfig};
use std::path::Path;

/// Run corner analysis from `.TEMP` commands in the netlist.
///
/// This compatibility entry point executes temperature-only TT/nominal sweeps.
pub fn run_corner_analysis(netlist_text: &str) -> Result<CornerData, String> {
    run_corner_analysis_with_source_path(netlist_text, None)
}

/// Run corner analysis from `.TEMP` commands in the netlist, resolving relative
/// includes from the source path when provided.
pub fn run_corner_analysis_with_source_path(
    netlist_text: &str,
    source_path: Option<&Path>,
) -> Result<CornerData, String> {
    let netlist = super::super::parse_runner_netlist(netlist_text, source_path)?;
    let temperatures = extract_temp_points(&netlist);

    if temperatures.is_empty() {
        return Err("Corner analysis requires at least one .TEMP command".to_string());
    }

    let config = CornerRunConfig {
        temperatures_c: temperatures,
        ..Default::default()
    };
    run_corner_analysis_with_netlist(&netlist, &config)
}

/// Run corner analysis with explicit process/voltage/temperature configuration.
pub fn run_corner_analysis_with_config(
    netlist_text: &str,
    config: &CornerRunConfig,
) -> Result<CornerData, String> {
    run_corner_analysis_with_config_and_source_path(netlist_text, config, None)
}

/// Run corner analysis with explicit process/voltage/temperature configuration
/// and a source path used to resolve relative includes and model file
/// references.
pub fn run_corner_analysis_with_config_and_source_path(
    netlist_text: &str,
    config: &CornerRunConfig,
    source_path: Option<&Path>,
) -> Result<CornerData, String> {
    let netlist = super::super::parse_runner_netlist(netlist_text, source_path)?;
    run_corner_analysis_with_netlist(&netlist, config)
}

fn run_corner_analysis_with_netlist(
    netlist: &rspice_core::Netlist,
    config: &CornerRunConfig,
) -> Result<CornerData, String> {
    config.validate()?;
    let points = expand_corner_points(config);
    if points.is_empty() {
        return Err("Corner analysis produced no corner points".to_string());
    }

    let nominal_voltage = config
        .nominal_voltage
        .or_else(|| infer_nominal_supply_voltage(netlist))
        .unwrap_or(1.0);
    let results = run_corner_sweep(netlist, &points, config, nominal_voltage)?;
    if results.is_empty() {
        return Err("Corner analysis produced no converged corner points".to_string());
    }

    let num_failures = points.len().saturating_sub(results.len());
    let metric = config.base_mode.metric_label();
    let (x_values, x_label, x_unit, temperatures_c, corner_labels, voltages) =
        map_corner_results(&results, metric);

    Ok(CornerData {
        x_values,
        x_label,
        x_unit,
        num_points: temperatures_c.len(),
        temperatures_c,
        corner_labels,
        voltages,
        num_failures,
    })
}
