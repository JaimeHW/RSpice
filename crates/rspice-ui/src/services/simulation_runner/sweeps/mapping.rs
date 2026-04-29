use super::types::{CornerMetricLabel, CornerPoint, SweepPointResult};
use rspice_core::Value;
use rspice_core::netlist::{StepCommand, StepTarget};
use rspice_core::solver::SimulationResult as CoreSimulationResult;

pub(super) fn describe_step_target(step_cmd: &StepCommand) -> String {
    match step_cmd.target {
        StepTarget::Param => format!("PARAM {}", step_cmd.name),
        StepTarget::Device => match step_cmd.param_name.as_deref() {
            Some(param) => format!("DEVICE {}.{}", step_cmd.name, param),
            None => format!("DEVICE {}", step_cmd.name),
        },
        StepTarget::Model => {
            let param = step_cmd.param_name.as_deref().unwrap_or("PARAM");
            format!("MODEL {}.{}", step_cmd.name, param)
        }
        StepTarget::Temp => "TEMP".to_string(),
    }
}

pub(super) fn map_dc_sweep_results(
    results: &[(Value, CoreSimulationResult)],
) -> (Vec<Value>, Vec<(String, Vec<Value>)>) {
    let sweep_values: Vec<Value> = results.iter().map(|(value, _)| *value).collect();
    let mut voltages = Vec::new();

    if let Some((_, first)) = results.first() {
        for node_idx in 1..first.node_voltages.len() {
            let node_name = first
                .node_names
                .get(node_idx)
                .cloned()
                .unwrap_or_else(|| node_idx.to_string());
            let values: Vec<Value> = results
                .iter()
                .map(|(_, result)| result.node_voltages.get(node_idx).copied().unwrap_or(0.0))
                .collect();
            voltages.push((format!("V({})", node_name), values));
        }
    }

    (sweep_values, voltages)
}

pub(super) fn map_temperature_results(
    results: &[(Value, SweepPointResult)],
    metric_label: CornerMetricLabel,
) -> (Vec<Value>, Vec<(String, Vec<Value>)>) {
    let sweep_values: Vec<Value> = results.iter().map(|(temp_c, _)| *temp_c).collect();
    let mut voltages = Vec::new();

    if let Some((_, first)) = results.first() {
        for node_idx in 1..first.node_values.len() {
            let node_name = first
                .node_names
                .get(node_idx)
                .cloned()
                .unwrap_or_else(|| node_idx.to_string());
            let values: Vec<Value> = results
                .iter()
                .map(|(_, result)| result.node_values.get(node_idx).copied().unwrap_or(0.0))
                .collect();
            voltages.push((metric_label.format_trace_name(&node_name), values));
        }
    }

    (sweep_values, voltages)
}

pub(super) fn map_corner_results(
    results: &[(CornerPoint, SweepPointResult)],
    metric_label: CornerMetricLabel,
) -> (
    Vec<Value>,
    String,
    String,
    Vec<Value>,
    Vec<String>,
    Vec<(String, Vec<Value>)>,
) {
    let temperatures_c: Vec<Value> = results
        .iter()
        .map(|(point, _)| point.temperature_c)
        .collect();
    let (x_values, x_label, x_unit) = corner_axis_from_points(results, &temperatures_c);
    let corner_labels: Vec<String> = results.iter().map(|(point, _)| point.label()).collect();
    let mut voltages = Vec::new();

    if let Some((_, first)) = results.first() {
        for node_idx in 1..first.node_values.len() {
            let node_name = first
                .node_names
                .get(node_idx)
                .cloned()
                .unwrap_or_else(|| node_idx.to_string());
            let values: Vec<Value> = results
                .iter()
                .map(|(_, result)| result.node_values.get(node_idx).copied().unwrap_or(0.0))
                .collect();
            voltages.push((metric_label.format_trace_name(&node_name), values));
        }
    }

    (
        x_values,
        x_label,
        x_unit,
        temperatures_c,
        corner_labels,
        voltages,
    )
}

fn corner_axis_from_points(
    results: &[(CornerPoint, SweepPointResult)],
    temperatures_c: &[Value],
) -> (Vec<Value>, String, String) {
    if results.is_empty() {
        return (Vec::new(), "Corner Index".to_string(), String::new());
    }

    let first_point = &results[0].0;
    let single_process = results
        .iter()
        .all(|(point, _)| point.process == first_point.process);
    let single_voltage = results
        .iter()
        .all(|(point, _)| (point.voltage - first_point.voltage).abs() < 1e-15);

    let mut seen_temps = std::collections::HashSet::with_capacity(temperatures_c.len());
    let has_duplicate_temp = temperatures_c
        .iter()
        .any(|temperature| !seen_temps.insert(temperature.to_bits()));

    if single_process && single_voltage && !has_duplicate_temp {
        return (
            temperatures_c.to_vec(),
            "Temperature".to_string(),
            "C".to_string(),
        );
    }

    (
        (0..results.len()).map(|index| index as Value).collect(),
        "Corner Index".to_string(),
        String::new(),
    )
}
