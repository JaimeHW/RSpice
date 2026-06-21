use super::netlist_mutation::{apply_process_corner, apply_voltage_corner};
use super::types::{
    CornerBaseMode, CornerFrequencySweep, CornerPoint, CornerRunConfig, SweepPointResult,
};
use rspice_core::Value;
use rspice_core::engine::{Engine, TransientResult};
use rspice_core::solver::SimulationResult as CoreSimulationResult;

pub(super) fn expand_corner_points(config: &CornerRunConfig) -> Vec<CornerPoint> {
    if config.full_matrix {
        let mut points = Vec::with_capacity(
            config.process_corners.len() * config.voltages.len() * config.temperatures_c.len(),
        );
        for process in &config.process_corners {
            for &voltage in &config.voltages {
                for &temperature_c in &config.temperatures_c {
                    points.push(CornerPoint {
                        process: *process,
                        voltage,
                        temperature_c,
                    });
                }
            }
        }
        return points;
    }

    let n = config
        .process_corners
        .len()
        .max(config.voltages.len())
        .max(config.temperatures_c.len());
    let mut points = Vec::with_capacity(n);
    for idx in 0..n {
        points.push(CornerPoint {
            process: config.process_corners[idx % config.process_corners.len()],
            voltage: config.voltages[idx % config.voltages.len()],
            temperature_c: config.temperatures_c[idx % config.temperatures_c.len()],
        });
    }
    points
}

pub(super) fn run_corner_sweep(
    netlist: &rspice_core::Netlist,
    points: &[CornerPoint],
    config: &CornerRunConfig,
    nominal_voltage: Value,
) -> Result<Vec<(CornerPoint, SweepPointResult)>, String> {
    if !nominal_voltage.is_finite() || nominal_voltage <= 0.0 {
        return Err("Corner analysis nominal voltage must be a positive finite value".to_string());
    }

    let mut results = Vec::with_capacity(points.len());

    for point in points {
        if !point.voltage.is_finite() || point.voltage <= 0.0 {
            return Err(format!(
                "Corner voltage must be positive and finite (got {})",
                point.voltage
            ));
        }
        if !point.temperature_c.is_finite() {
            return Err(format!(
                "Corner temperature must be finite (got {})",
                point.temperature_c
            ));
        }

        let mut corner_netlist = netlist.clone();
        apply_process_corner(&mut corner_netlist, point.process);
        apply_voltage_corner(&mut corner_netlist, point.voltage, nominal_voltage)?;

        let mut sim_config = super::super::build_engine_config(&corner_netlist, None);
        sim_config.temperature = point.temperature_c + 273.15;
        let engine = Engine::new(sim_config);

        match run_base_mode_point(&engine, &corner_netlist, &config.base_mode) {
            Ok(result) => results.push((point.clone(), result)),
            Err(e) => {
                log::warn!(
                    "Corner {} ({}) failed: {}",
                    point.label(),
                    config.base_mode.display_name(),
                    e
                );
            }
        }
    }

    Ok(results)
}

pub(super) fn run_temperature_sweep(
    netlist: &rspice_core::Netlist,
    temperatures_c: &[Value],
    base_mode: &CornerBaseMode,
) -> Result<Vec<(Value, SweepPointResult)>, String> {
    let mut results = Vec::with_capacity(temperatures_c.len());

    for &temp_c in temperatures_c {
        if !temp_c.is_finite() {
            return Err("Temperature sweep contains non-finite value".to_string());
        }

        let mut config = super::super::build_engine_config(netlist, None);
        config.temperature = temp_c + 273.15;
        let engine = Engine::new(config);

        match run_base_mode_point(&engine, netlist, base_mode) {
            Ok(point_result) => results.push((temp_c, point_result)),
            Err(e) => {
                log::warn!(
                    "Temperature corner {}C ({}) failed: {}",
                    temp_c,
                    base_mode.display_name(),
                    e
                );
            }
        }
    }

    Ok(results)
}

fn run_base_mode_point(
    engine: &Engine,
    netlist: &rspice_core::Netlist,
    base_mode: &CornerBaseMode,
) -> Result<SweepPointResult, String> {
    match base_mode {
        CornerBaseMode::Op => engine
            .run_dc_op(netlist)
            .map(|dc| sweep_point_result_from_dc(&dc))
            .map_err(|e| format!("DC operating point error: {}", e)),
        CornerBaseMode::DcSweep {
            source_name,
            start,
            stop,
            step,
        } => {
            let results = engine
                .run_dc_sweep(netlist, source_name, *start, *stop, *step)
                .map_err(|e| format!("DC sweep error: {}", e))?;
            let (_, terminal) = results
                .last()
                .ok_or_else(|| "DC sweep produced no points".to_string())?;
            Ok(sweep_point_result_from_dc(terminal))
        }
        CornerBaseMode::Transient {
            stop_time,
            step_time,
        } => {
            let result = engine
                .run_tran(netlist, *stop_time, *step_time)
                .map_err(|e| format!("Transient analysis error: {}", e))?;
            sweep_point_result_from_transient(result)
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
        ),
    }
}

fn sweep_point_result_from_dc(result: &CoreSimulationResult) -> SweepPointResult {
    SweepPointResult {
        node_names: result.node_names.clone(),
        node_values: result.node_voltages.clone(),
    }
}

fn sweep_point_result_from_transient(result: TransientResult) -> Result<SweepPointResult, String> {
    if result.time.is_empty() {
        return Err("Transient analysis produced no time points".to_string());
    }
    if result.node_names.is_empty() {
        return Err("Transient analysis returned no node names".to_string());
    }

    let mut node_values = Vec::with_capacity(result.node_names.len());
    for (idx, node_name) in result.node_names.iter().enumerate() {
        let Some(waveform) = result.voltages.get(idx) else {
            return Err(format!(
                "Transient result missing waveform for node '{}'",
                node_name
            ));
        };
        let Some(value) = waveform.last().copied() else {
            return Err(format!(
                "Transient waveform for node '{}' contains no samples",
                node_name
            ));
        };
        node_values.push(value);
    }

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
) -> Result<SweepPointResult, String> {
    let frequencies = super::super::generate_freq_points(
        start_freq,
        stop_freq,
        points_per_unit,
        sweep.as_keyword(),
    )?;

    let dc_result = engine
        .run_dc_op(netlist)
        .map_err(|e| format!("DC OP error (required for AC): {}", e))?;
    let node_names = dc_result.node_names;

    let ac_results = engine
        .run_ac(netlist, &frequencies)
        .map_err(|e| format!("AC analysis error: {}", e))?;
    let terminal = ac_results
        .last()
        .ok_or_else(|| "AC analysis produced no points".to_string())?;

    let mut node_values = vec![0.0; node_names.len()];
    for node_idx in 1..node_names.len() {
        let ac_idx = node_idx.saturating_sub(1);
        node_values[node_idx] = terminal
            .voltages
            .get(ac_idx)
            .map(|value| value.norm())
            .unwrap_or(0.0);
    }

    Ok(SweepPointResult {
        node_names,
        node_values,
    })
}
