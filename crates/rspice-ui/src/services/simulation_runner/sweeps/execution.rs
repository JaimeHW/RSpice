use super::super::error::{
    ServiceRunError, ServiceRunResult, ensure_not_aborted, poll_periodically,
};
use super::netlist_mutation::apply_voltage_corner;
use super::types::{
    CornerBaseMode, CornerFrequencySweep, CornerPoint, CornerRunConfig, SweepPointResult,
};
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::engine::{Engine, TransientResult};
use rspice_core::solver::SimulationResult as CoreSimulationResult;

pub(super) fn expand_corner_points(
    config: &CornerRunConfig,
    max_batch_runs: usize,
) -> ServiceRunResult<Vec<CornerPoint>> {
    expand_corner_points_impl(config, max_batch_runs, None)
}

pub(super) fn expand_corner_points_with_abort(
    config: &CornerRunConfig,
    max_batch_runs: usize,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<CornerPoint>> {
    expand_corner_points_impl(config, max_batch_runs, Some(abort))
}

fn expand_corner_points_impl(
    config: &CornerRunConfig,
    max_batch_runs: usize,
    abort: Option<&dyn AbortSignal>,
) -> ServiceRunResult<Vec<CornerPoint>> {
    if let Some(abort) = abort {
        ensure_not_aborted(abort)?;
    }
    if config.process_corners.is_empty()
        || config.voltages.is_empty()
        || config.temperatures_c.is_empty()
    {
        return Err(ServiceRunError::Failure(
            "Corner expansion requires non-empty process, voltage, and temperature axes"
                .to_string(),
        ));
    }
    let requested = if config.full_matrix {
        config
            .process_corners
            .len()
            .checked_mul(config.voltages.len())
            .and_then(|count| count.checked_mul(config.temperatures_c.len()))
            .unwrap_or(usize::MAX)
    } else {
        config
            .process_corners
            .len()
            .max(config.voltages.len())
            .max(config.temperatures_c.len())
    };
    if requested > max_batch_runs {
        return Err(ServiceRunError::resource_limit(
            rspice_core::ResourceKind::BatchRuns,
            requested,
            max_batch_runs,
        ));
    }
    let mut points = Vec::new();
    points.try_reserve_exact(requested).map_err(|error| {
        ServiceRunError::Failure(format!(
            "Corner expansion allocation for {requested} points failed: {error}"
        ))
    })?;

    if config.full_matrix {
        for (process_index, process) in config.process_corners.iter().enumerate() {
            if let Some(abort) = abort {
                poll_periodically(abort, process_index)?;
            }
            for (voltage_index, &voltage) in config.voltages.iter().enumerate() {
                if let Some(abort) = abort {
                    poll_periodically(abort, voltage_index)?;
                }
                for (temperature_index, &temperature_c) in config.temperatures_c.iter().enumerate()
                {
                    if let Some(abort) = abort {
                        poll_periodically(abort, temperature_index)?;
                    }
                    points.push(CornerPoint {
                        process: *process,
                        voltage,
                        temperature_c,
                    });
                }
            }
        }
        if let Some(abort) = abort {
            ensure_not_aborted(abort)?;
        }
        return Ok(points);
    }

    for idx in 0..requested {
        if let Some(abort) = abort {
            poll_periodically(abort, idx)?;
        }
        points.push(CornerPoint {
            process: config.process_corners[idx % config.process_corners.len()],
            voltage: config.voltages[idx % config.voltages.len()],
            temperature_c: config.temperatures_c[idx % config.temperatures_c.len()],
        });
    }
    if let Some(abort) = abort {
        ensure_not_aborted(abort)?;
    }
    Ok(points)
}

pub(super) fn run_corner_sweep(
    netlist: &rspice_core::Netlist,
    points: &[CornerPoint],
    config: &CornerRunConfig,
    nominal_voltage: Value,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<(CornerPoint, SweepPointResult)>> {
    ensure_not_aborted(abort)?;
    if !nominal_voltage.is_finite() || nominal_voltage <= 0.0 {
        return Err(ServiceRunError::Failure(
            "Corner analysis nominal voltage must be a positive finite value".to_string(),
        ));
    }

    let base_sim_config = super::super::build_engine_config(netlist, None);
    if points.len() > base_sim_config.resource_limits.max_batch_runs {
        return Err(ServiceRunError::resource_limit(
            rspice_core::ResourceKind::BatchRuns,
            points.len(),
            base_sim_config.resource_limits.max_batch_runs,
        ));
    }
    let mut results = Vec::new();
    results.try_reserve_exact(points.len()).map_err(|error| {
        ServiceRunError::Failure(format!(
            "Corner result allocation for {} points failed: {error}",
            points.len()
        ))
    })?;

    for (point_index, point) in points.iter().enumerate() {
        poll_periodically(abort, point_index)?;
        if !point.voltage.is_finite() || point.voltage <= 0.0 {
            return Err(ServiceRunError::Failure(format!(
                "Corner voltage must be positive and finite (got {})",
                point.voltage
            )));
        }
        if !point.temperature_c.is_finite() {
            return Err(ServiceRunError::Failure(format!(
                "Corner temperature must be finite (got {})",
                point.temperature_c
            )));
        }

        ensure_not_aborted(abort)?;
        let mut corner_netlist = netlist.clone();
        ensure_not_aborted(abort)?;
        apply_voltage_corner(&mut corner_netlist, point.voltage, nominal_voltage, abort)?;

        let mut sim_config = base_sim_config.clone();
        sim_config.temperature = point.temperature_c + 273.15;
        let engine = Engine::new(sim_config);

        match run_base_mode_point(&engine, &corner_netlist, &config.base_mode, abort) {
            Ok(result) => results.push((point.clone(), result)),
            Err(ServiceRunError::Aborted) => return Err(ServiceRunError::Aborted),
            Err(error @ ServiceRunError::ResourceLimit(_)) => return Err(error),
            Err(ServiceRunError::Failure(error)) => {
                log::warn!(
                    "Corner {} ({}) failed: {}",
                    point.label(),
                    config.base_mode.display_name(),
                    error
                );
            }
        }
    }

    ensure_not_aborted(abort)?;
    Ok(results)
}

pub(super) fn run_temperature_sweep(
    netlist: &rspice_core::Netlist,
    temperatures_c: &[Value],
    base_mode: &CornerBaseMode,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<(Value, SweepPointResult)>> {
    ensure_not_aborted(abort)?;
    let base_config = super::super::build_engine_config(netlist, None);
    if temperatures_c.len() > base_config.resource_limits.max_batch_runs {
        return Err(ServiceRunError::resource_limit(
            rspice_core::ResourceKind::BatchRuns,
            temperatures_c.len(),
            base_config.resource_limits.max_batch_runs,
        ));
    }
    let mut results = Vec::new();
    results
        .try_reserve_exact(temperatures_c.len())
        .map_err(|error| {
            ServiceRunError::Failure(format!(
                "Temperature result allocation for {} points failed: {error}",
                temperatures_c.len()
            ))
        })?;

    for (point_index, &temp_c) in temperatures_c.iter().enumerate() {
        poll_periodically(abort, point_index)?;
        if !temp_c.is_finite() {
            return Err(ServiceRunError::Failure(
                "Temperature sweep contains non-finite value".to_string(),
            ));
        }

        let mut config = base_config.clone();
        config.temperature = temp_c + 273.15;
        let engine = Engine::new(config);

        match run_base_mode_point(&engine, netlist, base_mode, abort) {
            Ok(point_result) => results.push((temp_c, point_result)),
            Err(ServiceRunError::Aborted) => return Err(ServiceRunError::Aborted),
            Err(error @ ServiceRunError::ResourceLimit(_)) => return Err(error),
            Err(ServiceRunError::Failure(error)) => {
                log::warn!(
                    "Temperature corner {}C ({}) failed: {}",
                    temp_c,
                    base_mode.display_name(),
                    error
                );
            }
        }
    }

    ensure_not_aborted(abort)?;
    Ok(results)
}

fn run_base_mode_point(
    engine: &Engine,
    netlist: &rspice_core::Netlist,
    base_mode: &CornerBaseMode,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<SweepPointResult> {
    ensure_not_aborted(abort)?;
    match base_mode {
        CornerBaseMode::Op => engine
            .run_dc_op_with_abort(netlist, abort)
            .map_err(|error| ServiceRunError::from_core("DC operating point error", error))
            .and_then(|dc| sweep_point_result_from_dc(&dc, abort)),
        CornerBaseMode::DcSweep {
            source_name,
            start,
            stop,
            step,
        } => {
            let results = engine
                .run_dc_sweep_with_abort(netlist, source_name, *start, *stop, *step, abort)
                .map_err(|error| ServiceRunError::from_core("DC sweep error", error))?;
            let (_, terminal) = results.last().ok_or_else(|| {
                ServiceRunError::Failure("DC sweep produced no points".to_string())
            })?;
            sweep_point_result_from_dc(terminal, abort)
        }
        CornerBaseMode::Transient {
            stop_time,
            step_time,
        } => {
            let result = engine
                .run_tran_with_abort(netlist, *stop_time, *step_time, abort)
                .map_err(|error| ServiceRunError::from_core("Transient analysis error", error))?;
            sweep_point_result_from_transient(result, abort)
        }
        CornerBaseMode::Ac {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
        } => run_base_mode_ac_point(
            engine,
            netlist,
            *start_freq,
            *stop_freq,
            *points_per_unit,
            *sweep,
            abort,
        ),
    }
}

fn sweep_point_result_from_dc(
    result: &CoreSimulationResult,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<SweepPointResult> {
    let mut node_names = Vec::with_capacity(result.node_names.len());
    for (index, name) in result.node_names.iter().enumerate() {
        poll_periodically(abort, index)?;
        node_names.push(name.clone());
    }
    let mut node_values = Vec::with_capacity(result.node_voltages.len());
    for (index, value) in result.node_voltages.iter().copied().enumerate() {
        poll_periodically(abort, index)?;
        node_values.push(value);
    }
    ensure_not_aborted(abort)?;
    Ok(SweepPointResult {
        node_names,
        node_values,
    })
}

fn sweep_point_result_from_transient(
    result: TransientResult,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<SweepPointResult> {
    ensure_not_aborted(abort)?;
    if result.time.is_empty() {
        return Err(ServiceRunError::Failure(
            "Transient analysis produced no time points".to_string(),
        ));
    }
    if result.node_names.is_empty() {
        return Err(ServiceRunError::Failure(
            "Transient analysis returned no node names".to_string(),
        ));
    }

    let mut node_values = Vec::with_capacity(result.node_names.len());
    for (idx, node_name) in result.node_names.iter().enumerate() {
        poll_periodically(abort, idx)?;
        let Some(waveform) = result.voltages.get(idx) else {
            return Err(ServiceRunError::Failure(format!(
                "Transient result missing waveform for node '{}'",
                node_name
            )));
        };
        let Some(value) = waveform.last().copied() else {
            return Err(ServiceRunError::Failure(format!(
                "Transient waveform for node '{}' contains no samples",
                node_name
            )));
        };
        node_values.push(value);
    }

    ensure_not_aborted(abort)?;
    Ok(SweepPointResult {
        node_names: result.node_names,
        node_values,
    })
}

fn run_base_mode_ac_point(
    engine: &Engine,
    netlist: &rspice_core::Netlist,
    start_freq: Value,
    stop_freq: Value,
    points_per_unit: usize,
    sweep: CornerFrequencySweep,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<SweepPointResult> {
    let frequencies = super::super::generate_freq_points_with_abort(
        start_freq,
        stop_freq,
        points_per_unit,
        sweep.as_keyword(),
        abort,
    )?;

    let dc_result = engine
        .run_dc_op_with_abort(netlist, abort)
        .map_err(|error| ServiceRunError::from_core("DC OP error (required for AC)", error))?;
    let node_names = dc_result.node_names;

    let ac_results = engine
        .run_ac_with_abort(netlist, &frequencies, abort)
        .map_err(|error| ServiceRunError::from_core("AC analysis error", error))?;
    let terminal = ac_results
        .last()
        .ok_or_else(|| ServiceRunError::Failure("AC analysis produced no points".to_string()))?;

    let mut node_values = vec![0.0; node_names.len()];
    for node_idx in 1..node_names.len() {
        poll_periodically(abort, node_idx)?;
        let ac_idx = node_idx.saturating_sub(1);
        node_values[node_idx] = terminal
            .voltages
            .get(ac_idx)
            .map(|value| value.norm())
            .unwrap_or(0.0);
    }

    ensure_not_aborted(abort)?;
    Ok(SweepPointResult {
        node_names,
        node_values,
    })
}

#[cfg(test)]
mod tests {
    use super::super::types::CornerProcess;
    use super::*;

    #[test]
    fn full_corner_matrix_obeys_configured_batch_limit_before_allocation() {
        let config = CornerRunConfig {
            process_corners: vec![CornerProcess::TT, CornerProcess::FF],
            voltages: vec![0.9, 1.1],
            temperatures_c: vec![-40.0, 125.0],
            ..Default::default()
        };

        let result = expand_corner_points(&config, 7);

        assert!(matches!(
            result,
            Err(ServiceRunError::ResourceLimit(
                rspice_core::ResourceLimitError {
                    resource: rspice_core::ResourceKind::BatchRuns,
                    requested: 8,
                    limit: 7,
                }
            ))
        ));
    }

    #[test]
    fn diagonal_corner_expansion_uses_largest_axis_as_run_count() {
        let config = CornerRunConfig {
            process_corners: vec![CornerProcess::TT],
            voltages: vec![0.9, 1.0],
            temperatures_c: vec![-40.0, 25.0, 125.0],
            full_matrix: false,
            ..Default::default()
        };

        let points = expand_corner_points(&config, 3).expect("three diagonal points");

        assert_eq!(points.len(), 3);
        assert_eq!(points[2].voltage, 0.9);
        assert_eq!(points[2].temperature_c, 125.0);
    }
}
