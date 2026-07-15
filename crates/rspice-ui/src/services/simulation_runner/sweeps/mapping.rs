use super::super::error::{ServiceRunResult, ensure_not_aborted, poll_periodically};
use super::types::{CornerMetricLabel, CornerPoint, SweepPointResult};
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::netlist::{StepCommand, StepSweep, StepTarget};
use rspice_core::solver::SimulationResult as CoreSimulationResult;

pub(super) fn describe_step_target(step_cmd: &StepCommand) -> String {
    if let StepSweep::Data { table_name } = &step_cmd.sweep {
        return format!("DATA {table_name}");
    }

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

pub(super) fn map_dc_sweep_results_with_abort(
    results: &[(Value, CoreSimulationResult)],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<(Vec<Value>, Vec<(String, Vec<Value>)>)> {
    let mut sweep_values = Vec::with_capacity(results.len());
    for (index, (value, _)) in results.iter().enumerate() {
        poll_periodically(abort, index)?;
        sweep_values.push(*value);
    }
    let mut voltages = Vec::new();

    if let Some((_, first)) = results.first() {
        for node_idx in 1..first.node_voltages.len() {
            poll_periodically(abort, node_idx)?;
            let node_name = first
                .node_names
                .get(node_idx)
                .cloned()
                .unwrap_or_else(|| node_idx.to_string());
            let mut values = Vec::with_capacity(results.len());
            for (point_index, (_, result)) in results.iter().enumerate() {
                poll_periodically(abort, point_index)?;
                values.push(result.node_voltages.get(node_idx).copied().unwrap_or(0.0));
            }
            voltages.push((format!("V({})", node_name), values));
        }
    }

    ensure_not_aborted(abort)?;
    Ok((sweep_values, voltages))
}

pub(super) fn map_temperature_results(
    results: &[(Value, SweepPointResult)],
    metric_label: CornerMetricLabel,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<(Vec<Value>, Vec<(String, Vec<Value>)>)> {
    let mut sweep_values = Vec::with_capacity(results.len());
    for (index, (temp_c, _)) in results.iter().enumerate() {
        poll_periodically(abort, index)?;
        sweep_values.push(*temp_c);
    }
    let mut voltages = Vec::new();

    if let Some((_, first)) = results.first() {
        for node_idx in 1..first.node_values.len() {
            poll_periodically(abort, node_idx)?;
            let node_name = first
                .node_names
                .get(node_idx)
                .cloned()
                .unwrap_or_else(|| node_idx.to_string());
            let mut values = Vec::with_capacity(results.len());
            for (point_index, (_, result)) in results.iter().enumerate() {
                poll_periodically(abort, point_index)?;
                values.push(result.node_values.get(node_idx).copied().unwrap_or(0.0));
            }
            voltages.push((metric_label.format_trace_name(&node_name), values));
        }
    }

    ensure_not_aborted(abort)?;
    Ok((sweep_values, voltages))
}

pub(super) fn map_corner_results(
    results: &[(CornerPoint, SweepPointResult)],
    metric_label: CornerMetricLabel,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<(
    Vec<Value>,
    String,
    String,
    Vec<Value>,
    Vec<String>,
    Vec<(String, Vec<Value>)>,
)> {
    let mut temperatures_c = Vec::with_capacity(results.len());
    let mut corner_labels = Vec::with_capacity(results.len());
    for (index, (point, _)) in results.iter().enumerate() {
        poll_periodically(abort, index)?;
        temperatures_c.push(point.temperature_c);
        corner_labels.push(point.label());
    }
    let (x_values, x_label, x_unit) = corner_axis_from_points(results, &temperatures_c, abort)?;
    let mut voltages = Vec::new();

    if let Some((_, first)) = results.first() {
        for node_idx in 1..first.node_values.len() {
            poll_periodically(abort, node_idx)?;
            let node_name = first
                .node_names
                .get(node_idx)
                .cloned()
                .unwrap_or_else(|| node_idx.to_string());
            let mut values = Vec::with_capacity(results.len());
            for (point_index, (_, result)) in results.iter().enumerate() {
                poll_periodically(abort, point_index)?;
                values.push(result.node_values.get(node_idx).copied().unwrap_or(0.0));
            }
            voltages.push((metric_label.format_trace_name(&node_name), values));
        }
    }

    ensure_not_aborted(abort)?;
    Ok((
        x_values,
        x_label,
        x_unit,
        temperatures_c,
        corner_labels,
        voltages,
    ))
}

fn corner_axis_from_points(
    results: &[(CornerPoint, SweepPointResult)],
    temperatures_c: &[Value],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<(Vec<Value>, String, String)> {
    if results.is_empty() {
        return Ok((Vec::new(), "Corner Index".to_string(), String::new()));
    }

    let first_point = &results[0].0;
    let mut single_process = true;
    let mut single_voltage = true;
    for (index, (point, _)) in results.iter().enumerate() {
        poll_periodically(abort, index)?;
        single_process &= point.process == first_point.process;
        single_voltage &= (point.voltage - first_point.voltage).abs() < 1e-15;
    }

    let mut seen_temps = std::collections::HashSet::with_capacity(temperatures_c.len());
    let mut has_duplicate_temp = false;
    for (index, temperature) in temperatures_c.iter().enumerate() {
        poll_periodically(abort, index)?;
        if !seen_temps.insert(temperature.to_bits()) {
            has_duplicate_temp = true;
            break;
        }
    }

    if single_process && single_voltage && !has_duplicate_temp {
        return Ok((
            temperatures_c.to_vec(),
            "Temperature".to_string(),
            "C".to_string(),
        ));
    }

    let mut indices = Vec::with_capacity(results.len());
    for index in 0..results.len() {
        poll_periodically(abort, index)?;
        indices.push(index as Value);
    }
    ensure_not_aborted(abort)?;
    Ok((indices, "Corner Index".to_string(), String::new()))
}
