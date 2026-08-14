//! Mapping sweep results back to their points.
//!
//! Each run returns results for one point; this labels them with the
//! parameter values that produced them so a plot can be swept by parameter
//! rather than by run index.

use super::super::error::{
    ServiceRunError, ServiceRunResult, ensure_not_aborted, poll_periodically,
};
use super::types::{CornerMetricLabel, CornerPoint, SweepPointResult};
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::netlist::{StepCommand, StepSweep, StepTarget};

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

/// Lay solved temperature points onto the axis a parametric plot reads.
///
/// Reachable outside the sweep runners for the same reason
/// [`map_corner_results`] is: a temperature step's family is assembled from the
/// results of its per-point tasks rather than from a solve of its own.
pub(crate) fn map_temperature_results(
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
        for (point_index, (_, result)) in results.iter().enumerate() {
            validate_sweep_point_shape(first, result, point_index, "temperature")?;
        }
        for node_idx in 1..first.node_values.len() {
            poll_periodically(abort, node_idx)?;
            let node_name = first.node_names[node_idx].clone();
            let mut values = Vec::with_capacity(results.len());
            for (point_index, (_, result)) in results.iter().enumerate() {
                poll_periodically(abort, point_index)?;
                values.push(result.node_values[node_idx]);
            }
            voltages.push((metric_label.format_trace_name(&node_name), values));
        }
    }

    ensure_not_aborted(abort)?;
    Ok((sweep_values, voltages))
}

/// Lay solved corner points onto the shared axis a corner plot reads.
///
/// Reachable outside the sweep runners because the corner family is assembled
/// from the results of the per-point tasks rather than from a solve of its
/// own; this is the single assembler both would otherwise have to agree with.
pub(crate) fn map_corner_results(
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
        for (point_index, (_, result)) in results.iter().enumerate() {
            validate_sweep_point_shape(first, result, point_index, "corner")?;
        }
        for node_idx in 1..first.node_values.len() {
            poll_periodically(abort, node_idx)?;
            let node_name = first.node_names[node_idx].clone();
            let mut values = Vec::with_capacity(results.len());
            for (point_index, (_, result)) in results.iter().enumerate() {
                poll_periodically(abort, point_index)?;
                values.push(result.node_values[node_idx]);
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

fn validate_sweep_point_shape(
    first: &SweepPointResult,
    point: &SweepPointResult,
    point_index: usize,
    family: &str,
) -> ServiceRunResult<()> {
    if first.node_names.len() != first.node_values.len() {
        return Err(ServiceRunError::Failure(format!(
            "{family} sweep reference point has {} node names but {} values",
            first.node_names.len(),
            first.node_values.len()
        )));
    }
    if point.node_names != first.node_names {
        return Err(ServiceRunError::Failure(format!(
            "{family} sweep point {} changed the solved node basis",
            point_index + 1
        )));
    }
    if point.node_values.len() != first.node_values.len() {
        return Err(ServiceRunError::Failure(format!(
            "{family} sweep point {} returned {} node values, expected {}",
            point_index + 1,
            point.node_values.len(),
            first.node_values.len()
        )));
    }
    if point.node_values.iter().any(|value| !value.is_finite()) {
        return Err(ServiceRunError::Failure(format!(
            "{family} sweep point {} returned a non-finite node value",
            point_index + 1
        )));
    }
    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::NoAbort;

    fn point(names: &[&str], values: &[Value]) -> SweepPointResult {
        SweepPointResult {
            node_names: names.iter().map(|name| (*name).to_owned()).collect(),
            node_values: values.to_vec(),
        }
    }

    #[test]
    fn temperature_mapping_rejects_missing_values_instead_of_filling_zero() {
        let results = vec![
            (25.0, point(&["0", "OUT"], &[0.0, 1.0])),
            (125.0, point(&["0", "OUT"], &[0.0])),
        ];

        let error = map_temperature_results(&results, CornerMetricLabel::Voltage, &NoAbort)
            .expect_err("shape mismatch must fail closed");

        assert!(error.to_string().contains("returned 1 node values"));
    }

    #[test]
    fn corner_mapping_rejects_node_basis_drift() {
        let results = vec![
            (
                CornerPoint {
                    process: super::super::types::CornerProcess::TT,
                    voltage: 1.0,
                    temperature_c: 25.0,
                },
                point(&["0", "OUT"], &[0.0, 1.0]),
            ),
            (
                CornerPoint {
                    process: super::super::types::CornerProcess::TT,
                    voltage: 1.0,
                    temperature_c: 125.0,
                },
                point(&["0", "OTHER"], &[0.0, 1.0]),
            ),
        ];

        let error = map_corner_results(&results, CornerMetricLabel::Voltage, &NoAbort)
            .expect_err("basis drift must fail closed");

        assert!(error.to_string().contains("changed the solved node basis"));
    }
}
