//! Running a hand-written deck.
//!
//! Composes the source a Netlist-workspace run executes and queues the
//! analyses the deck itself declares, rather than the ones the Simulate
//! workspace configured.

use super::*;
use rspice_core::analysis::advanced::s_param;
use rspice_core::netlist::{
    AnalysisCommand, ElementKind, FreqVariation, Netlist, PoleZeroAnalysisType,
    PoleZeroTransferType, StepCommand, StepTarget,
};

use crate::services::simulation_runner::{
    CornerBaseMode, CornerFrequencySweep, TempRunConfig, expand_step_sweep_values,
};

pub(super) fn build_manual_deck_queue(
    state: &AppState,
    source: &str,
) -> Result<Vec<QueuedAnalysis>, Vec<String>> {
    let source = compose_manual_deck_source(source);
    let parsed =
        Netlist::parse(&source).map_err(|err| vec![format!("Netlist parse error: {err}")])?;
    if parsed.analyses.is_empty() {
        return Err(vec![
            "No analysis command in netlist. Add .op, .ac, .tran, or another supported analysis."
                .to_string(),
        ]);
    }

    let step_count = parsed
        .analyses
        .iter()
        .filter(|command| matches!(command, AnalysisCommand::Step(_)))
        .count();
    let mc_count = parsed
        .analyses
        .iter()
        .filter(|command| matches!(command, AnalysisCommand::MonteCarlo(_)))
        .count();
    let mut preflight_errors = Vec::new();
    if step_count > 1 {
        preflight_errors.push(format!(
            "Manual deck runs support one .step command per run; found {step_count}. Split independent sweeps into separate runs."
        ));
    }
    if mc_count > 1 {
        preflight_errors.push(format!(
            "Manual deck runs support one .mc command per run; found {mc_count}. Split independent Monte Carlo studies into separate runs."
        ));
    }
    if !preflight_errors.is_empty() {
        return Err(preflight_errors);
    }

    let temperature_plan = match build_temperature_plan(&parsed.analyses) {
        Ok(plan) => plan,
        Err(err) => return Err(vec![err]),
    };
    let parameter_step_skips = match parameter_step_context_skips(&parsed.analyses) {
        Ok(skips) => skips,
        Err(err) => return Err(vec![err]),
    };

    let mut queue = Vec::with_capacity(parsed.analyses.len());
    let mut errors = Vec::new();
    for (idx, command) in parsed.analyses.iter().enumerate() {
        if let Some(plan) = &temperature_plan
            && idx == plan.insert_index
        {
            queue.push(plan.item.clone());
        }
        if temperature_plan
            .as_ref()
            .is_some_and(|plan| plan.skip_indices.contains(&idx))
            || parameter_step_skips.contains(&idx)
        {
            continue;
        }
        if matches!(command, AnalysisCommand::Temp { .. }) {
            continue;
        }
        if let AnalysisCommand::Four {
            fundamental,
            outputs,
            num_harmonics,
        } = command
        {
            for output in outputs {
                match fourier_queue_item(&parsed, *fundamental, output, *num_harmonics) {
                    Ok(item) => queue.push(item),
                    Err(error) => errors.push(error),
                }
            }
            continue;
        }
        match command_to_queue_item(state, &parsed, command) {
            Ok(item) => queue.push(item),
            Err(err) => errors.push(err),
        }
    }

    if errors.is_empty() && queue.is_empty() {
        Err(vec![
            "No runnable analysis command in netlist. Add .op, .ac, .tran, .step, .mc, or another supported analysis."
                .to_string(),
        ])
    } else if errors.is_empty() {
        match finalize_manual_fourier_contracts(&mut queue) {
            Ok(()) => Ok(queue),
            Err(error) => Err(vec![error]),
        }
    } else {
        Err(errors)
    }
}

fn finalize_manual_fourier_contracts(queue: &mut [QueuedAnalysis]) -> Result<(), String> {
    let fourier_count = queue
        .iter()
        .filter(|item| matches!(&item.spec, AnalysisSpec::Fourier { .. }))
        .count();
    if fourier_count == 0 {
        return Ok(());
    }

    let transient_windows = queue
        .iter()
        .filter_map(|item| match &item.spec {
            AnalysisSpec::Transient {
                start_time,
                stop_time,
                ..
            } => Some((*start_time, *stop_time)),
            _ => None,
        })
        .collect::<Vec<_>>();
    if transient_windows.len() != 1 {
        return Err(format!(
            "Manual-deck .FOUR requires exactly one .TRAN analysis so its numerical trajectory is unambiguous; found {}",
            transient_windows.len()
        ));
    }
    let (transient_start, transient_stop) = transient_windows[0];
    for item in queue {
        if let AnalysisSpec::Fourier {
            start_time,
            stop_time,
            ..
        } = &mut item.spec
        {
            *start_time = transient_start;
            *stop_time = transient_stop;
        }
    }
    Ok(())
}

#[derive(Debug, Clone)]
struct PlannedQueueItem {
    insert_index: usize,
    skip_indices: Vec<usize>,
    item: QueuedAnalysis,
}

fn build_temperature_plan(
    analyses: &[AnalysisCommand],
) -> Result<Option<PlannedQueueItem>, String> {
    let step_temp = analyses
        .iter()
        .enumerate()
        .find_map(|(idx, command)| match command {
            AnalysisCommand::Step(step) if step.target == StepTarget::Temp => Some((idx, step)),
            _ => None,
        });
    let temp_indices: Vec<usize> = analyses
        .iter()
        .enumerate()
        .filter_map(|(idx, command)| matches!(command, AnalysisCommand::Temp { .. }).then_some(idx))
        .collect();

    if step_temp.is_none() && temp_indices.is_empty() {
        return Ok(None);
    }
    if step_temp.is_some() && !temp_indices.is_empty() {
        return Err(
            "Manual temperature sweeps must use either .step temp or .temp, not both".to_string(),
        );
    }

    let temperatures_c = if let Some((_, step)) = step_temp {
        expand_step_sweep_values(&step.sweep)
            .map_err(|err| format!("temperature .step is invalid: {err}"))?
    } else {
        unique_temp_directive_values(analyses)
    };
    if temperatures_c.is_empty() {
        return Err("Manual temperature sweep requires at least one temperature".to_string());
    }

    let mut supported_bases = Vec::new();
    let mut unsupported_bases = Vec::new();
    for (idx, command) in analyses.iter().enumerate() {
        if Some(idx) == step_temp.map(|(step_idx, _)| step_idx) || temp_indices.contains(&idx) {
            continue;
        }
        if is_temperature_base_command(command) {
            supported_bases.push((idx, command));
        } else if is_analysis_command(command) {
            unsupported_bases.push(command_name(command));
        }
    }

    if !unsupported_bases.is_empty() {
        unsupported_bases.sort_unstable();
        unsupported_bases.dedup();
        return Err(format!(
            "Manual temperature sweeps currently support .op, .dc, .tran, or .ac as the base analysis; found {}",
            unsupported_bases.join(", ")
        ));
    }
    if supported_bases.len() > 1 {
        let names = supported_bases
            .iter()
            .map(|(_, command)| command_name(command))
            .collect::<Vec<_>>()
            .join(", ");
        return Err(format!(
            "Manual temperature sweeps currently support one base analysis per run; found {names}"
        ));
    }
    if supported_bases.is_empty() && step_temp.is_none() {
        return Ok(None);
    }

    let (base_index, base_mode) = match supported_bases.first() {
        Some((idx, command)) => (*idx, temperature_base_mode(command)?),
        None => (
            step_temp
                .map(|(idx, _)| idx)
                .expect(".step temp should have an index"),
            CornerBaseMode::Op,
        ),
    };

    let mut skip_indices = temp_indices;
    if let Some((idx, _)) = step_temp {
        skip_indices.push(idx);
    }
    if Some(base_index) != step_temp.map(|(idx, _)| idx) {
        skip_indices.push(base_index);
    }
    skip_indices.sort_unstable();
    skip_indices.dedup();

    let insert_index = skip_indices.iter().copied().min().unwrap_or(base_index);
    let analysis_line = if step_temp.is_some() {
        ".step temp"
    } else {
        ".temp"
    }
    .to_string();

    Ok(Some(PlannedQueueItem {
        insert_index,
        skip_indices,
        item: QueuedAnalysis {
            spec: AnalysisSpec::Parametric,
            config: None,
            spec_options: SpecExecutionOptions {
                temp: Some(TempRunConfig {
                    temperatures_c,
                    base_mode,
                }),
                ..SpecExecutionOptions::default()
            },
            analysis_line,
        },
    }))
}

fn parameter_step_context_skips(analyses: &[AnalysisCommand]) -> Result<Vec<usize>, String> {
    let Some((_, step)) = analyses.iter().enumerate().find_map(|(idx, command)| {
        if let AnalysisCommand::Step(step) = command {
            (step.target != StepTarget::Temp).then_some((idx, step))
        } else {
            None
        }
    }) else {
        return Ok(Vec::new());
    };

    let mut op_indices = Vec::new();
    let mut unsupported = Vec::new();
    for (idx, command) in analyses.iter().enumerate() {
        match command {
            AnalysisCommand::Op => op_indices.push(idx),
            AnalysisCommand::Step(_) | AnalysisCommand::Temp { .. } => {}
            AnalysisCommand::MonteCarlo(_) => {}
            other if is_analysis_command(other) => unsupported.push(command_name(other)),
            _ => {}
        }
    }

    if !unsupported.is_empty() {
        unsupported.sort_unstable();
        unsupported.dedup();
        return Err(format!(
            "Manual parameter .step runs currently execute DC operating-point sweeps only; found unsupported paired analysis {} for target {}",
            unsupported.join(", "),
            step_target_name(step)
        ));
    }

    Ok(op_indices)
}

fn unique_temp_directive_values(analyses: &[AnalysisCommand]) -> Vec<f64> {
    let mut temperatures = Vec::new();
    for command in analyses {
        if let AnalysisCommand::Temp {
            temperatures: temps,
        } = command
        {
            for &temp in temps {
                if !temperatures
                    .iter()
                    .any(|existing: &f64| (*existing - temp).abs() < 1e-15)
                {
                    temperatures.push(temp);
                }
            }
        }
    }
    temperatures
}

fn is_analysis_command(command: &AnalysisCommand) -> bool {
    !matches!(command, AnalysisCommand::Temp { .. })
}

fn is_temperature_base_command(command: &AnalysisCommand) -> bool {
    matches!(
        command,
        AnalysisCommand::Op
            | AnalysisCommand::Dc { .. }
            | AnalysisCommand::Tran { .. }
            | AnalysisCommand::Ac { .. }
    )
}

fn temperature_base_mode(command: &AnalysisCommand) -> Result<CornerBaseMode, String> {
    match command {
        AnalysisCommand::Op => Ok(CornerBaseMode::Op),
        AnalysisCommand::Dc {
            source,
            start,
            stop,
            step,
            sweep2,
            mode: _,
        } => {
            if sweep2.is_some() {
                return Err(
                    "Manual temperature sweeps do not yet support nested .dc base sweeps"
                        .to_string(),
                );
            }
            Ok(CornerBaseMode::DcSweep {
                source_name: source.clone(),
                start: *start,
                stop: *stop,
                step: *step,
            })
        }
        AnalysisCommand::Tran {
            step,
            stop,
            start,
            max_step,
            uic,
        } => {
            if start.is_some() || max_step.is_some() || *uic {
                return Err(
                    "Manual temperature transient sweeps currently support tstep/tstop only"
                        .to_string(),
                );
            }
            Ok(CornerBaseMode::Transient {
                stop_time: *stop,
                step_time: *step,
            })
        }
        AnalysisCommand::Ac {
            variation,
            points,
            start_freq,
            stop_freq,
        } => Ok(CornerBaseMode::Ac {
            start_freq: *start_freq,
            stop_freq: *stop_freq,
            points_per_unit: *points,
            sweep: corner_frequency_sweep(*variation),
        }),
        _ => Err(format!(
            "{} cannot be used as a manual temperature sweep base analysis",
            command_name(command)
        )),
    }
}

fn corner_frequency_sweep(variation: FreqVariation) -> CornerFrequencySweep {
    match variation {
        FreqVariation::Lin => CornerFrequencySweep::Linear,
        FreqVariation::Oct => CornerFrequencySweep::Octave,
        FreqVariation::Dec => CornerFrequencySweep::Decade,
    }
}

fn step_target_name(step: &StepCommand) -> &'static str {
    match step.target {
        StepTarget::Param => "PARAM",
        StepTarget::Device => "device",
        StepTarget::Model => "MODEL",
        StepTarget::Temp => "TEMP",
    }
}

fn command_name(command: &AnalysisCommand) -> &'static str {
    match command {
        AnalysisCommand::Op => ".op",
        AnalysisCommand::Dc { .. } => ".dc",
        AnalysisCommand::Ac { .. } => ".ac",
        AnalysisCommand::AcData { .. } => ".ac data",
        AnalysisCommand::Hb { .. } => ".hb",
        AnalysisCommand::Sp { .. } => ".sp",
        AnalysisCommand::Stb { .. } => ".stb",
        AnalysisCommand::Disto { .. } => ".disto",
        AnalysisCommand::Tran { .. } => ".tran",
        AnalysisCommand::Noise { .. } => ".noise",
        AnalysisCommand::NoiseData { .. } => ".noise data",
        AnalysisCommand::PoleZero { .. } => ".pz",
        AnalysisCommand::Sensitivity { .. } => ".sens",
        AnalysisCommand::Tf { .. } => ".tf",
        AnalysisCommand::Four { .. } => ".four",
        AnalysisCommand::MonteCarlo(_) => ".mc",
        AnalysisCommand::Step(_) => ".step",
        AnalysisCommand::Temp { .. } => ".temp",
    }
}

pub(super) fn compose_manual_deck_source(source: &str) -> String {
    let has_end = source
        .lines()
        .any(|line| line.trim_start().eq_ignore_ascii_case(".end"));
    if has_end {
        source.to_string()
    } else if source.ends_with('\n') {
        format!("{source}.end\n")
    } else {
        format!("{source}\n.end\n")
    }
}

fn frequency_sweep(variation: FreqVariation) -> FrequencySweep {
    match variation {
        FreqVariation::Lin => FrequencySweep::Linear,
        FreqVariation::Oct => FrequencySweep::Octave,
        FreqVariation::Dec => FrequencySweep::Decade,
    }
}

fn ac_sweep(variation: FreqVariation) -> AcSweepType {
    match variation {
        FreqVariation::Lin => AcSweepType::Linear,
        FreqVariation::Oct => AcSweepType::Octave,
        FreqVariation::Dec => AcSweepType::Decade,
    }
}

/// Resolve the deck's `.SP` ports into the UI's port shape.
///
/// Discovery and validation live in `rspice_core`; this only projects the
/// shared port type onto the runner's own, so the GUI, the CLI, and the Python
/// bindings cannot disagree about which ports a deck declares.
fn collect_sparameter_ports(netlist: &Netlist) -> Result<Vec<SpPort>, String> {
    let ports = s_param::collect_ports(netlist).map_err(|error| error.to_string())?;
    Ok(ports
        .into_iter()
        .map(|port| SpPort {
            node_pos: port.node_pos,
            node_neg: port.node_neg,
            z0: Some(port.z0),
        })
        .collect())
}

fn pz_transfer_name(transfer_type: PoleZeroTransferType) -> String {
    match transfer_type {
        PoleZeroTransferType::Voltage => "VOL",
        PoleZeroTransferType::Current => "CUR",
    }
    .to_string()
}

fn pz_analysis_name(analysis_type: PoleZeroAnalysisType) -> String {
    match analysis_type {
        PoleZeroAnalysisType::PoleZero => "PZ",
        PoleZeroAnalysisType::PolesOnly => "POL",
        PoleZeroAnalysisType::ZerosOnly => "ZER",
    }
    .to_string()
}

fn pz_config_type(analysis_type: PoleZeroAnalysisType) -> PzAnalysisType {
    match analysis_type {
        PoleZeroAnalysisType::PoleZero => PzAnalysisType::PoleZero,
        PoleZeroAnalysisType::PolesOnly => PzAnalysisType::PolesOnly,
        PoleZeroAnalysisType::ZerosOnly => PzAnalysisType::ZerosOnly,
    }
}

fn ac_data_table_frequencies(netlist: &Netlist, table_name: &str) -> Result<Vec<f64>, String> {
    let table = netlist
        .data_tables
        .iter()
        .find(|table| table.name.eq_ignore_ascii_case(table_name))
        .ok_or_else(|| format!(".AC DATA table '{table_name}' not found"))?;
    let freq_column = table
        .params
        .iter()
        .position(|param| param.eq_ignore_ascii_case("FREQ"))
        .ok_or_else(|| format!(".AC DATA table '{}' must contain a FREQ column", table.name))?;

    if table.rows.is_empty() {
        return Err(format!(".AC DATA table '{}' has no rows", table.name));
    }

    let mut frequencies = Vec::with_capacity(table.rows.len());
    for (row_idx, row) in table.rows.iter().enumerate() {
        if row.len() != table.params.len() {
            return Err(format!(
                ".AC DATA table '{}' row {} has {} value(s), expected {}",
                table.name,
                row_idx + 1,
                row.len(),
                table.params.len()
            ));
        }
        let frequency = row[freq_column];
        if !frequency.is_finite() || frequency < 0.0 {
            return Err(format!(
                ".AC DATA table '{}' row {} has invalid frequency {}",
                table.name,
                row_idx + 1,
                frequency
            ));
        }
        frequencies.push(frequency);
    }
    Ok(frequencies)
}

fn command_to_queue_item(
    state: &AppState,
    netlist: &Netlist,
    command: &AnalysisCommand,
) -> Result<QueuedAnalysis, String> {
    let spec_options = SpecExecutionOptions::default();
    match command {
        AnalysisCommand::Op => Ok(QueuedAnalysis {
            spec: AnalysisSpec::dc_op(),
            config: Some(AnalysisConfig::dc_op()),
            spec_options,
            analysis_line: ".op".to_string(),
        }),
        AnalysisCommand::Dc {
            source,
            start,
            stop,
            step,
            sweep2,
            mode: _,
        } => {
            let (source2, start2, stop2, step2) = match sweep2 {
                Some(second) => (
                    Some(second.source.clone()),
                    Some(second.start),
                    Some(second.stop),
                    Some(second.step),
                ),
                None => (None, None, None, None),
            };
            let spec = AnalysisSpec::DcSweep {
                source_name: source.clone(),
                start: *start,
                stop: *stop,
                step: *step,
                source2: source2.clone(),
                start2,
                stop2,
                step2,
            };
            Ok(QueuedAnalysis {
                config: Some(AnalysisConfig::DcSweep(DcSweepConfig {
                    source: source.clone(),
                    start: *start,
                    stop: *stop,
                    step: *step,
                    source2,
                    start2,
                    stop2,
                    step2,
                })),
                analysis_line: ".dc".to_string(),
                spec,
                spec_options,
            })
        }
        AnalysisCommand::Ac {
            variation,
            points,
            start_freq,
            stop_freq,
        } => {
            let spec = AnalysisSpec::Ac {
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                points_per_unit: *points,
                sweep: frequency_sweep(*variation),
            };
            Ok(QueuedAnalysis {
                config: Some(AnalysisConfig::Ac(AcAnalysisConfig {
                    sweep_type: ac_sweep(*variation),
                    num_points: *points,
                    start_freq: *start_freq,
                    stop_freq: *stop_freq,
                })),
                analysis_line: ".ac".to_string(),
                spec,
                spec_options,
            })
        }
        AnalysisCommand::AcData { table_name } => Ok(QueuedAnalysis {
            spec: AnalysisSpec::AcData {
                table_name: table_name.clone(),
                frequencies: ac_data_table_frequencies(netlist, table_name)?,
            },
            config: None,
            spec_options,
            analysis_line: format!(".ac data={table_name}"),
        }),
        AnalysisCommand::Hb { frequencies } => {
            let defaults =
                rspice_core::analysis::HbConfig::new(frequencies.first().copied().unwrap_or(1.0));
            let order_for = |index: usize| {
                netlist
                    .options
                    .hb_num_frequencies
                    .get(index)
                    .copied()
                    .or_else(|| netlist.options.hb_num_frequencies.first().copied())
                    .unwrap_or(defaults.num_harmonics)
            };
            let tones = frequencies
                .iter()
                .enumerate()
                .map(|(index, frequency)| {
                    HbToneSpec::new(*frequency, order_for(index))
                        .with_name(format!("tone{}", index + 1))
                })
                .collect();
            let collocation_points = if frequencies.len() == 1
                && !netlist.options.hb_num_frequencies.is_empty()
            {
                Some(
                    order_for(0)
                        .checked_mul(2)
                        .and_then(|value| value.checked_add(1))
                        .ok_or_else(|| {
                            "HB harmonic count exceeds the addressable collocation grid".to_string()
                        })?,
                )
            } else {
                None
            };
            Ok(QueuedAnalysis {
                spec: AnalysisSpec::HarmonicBalance {
                    tones,
                    reltol: defaults.tolerance,
                    abstol: defaults.abstol,
                    max_iterations: defaults.max_iterations,
                    damping: defaults.damping,
                    oversample: defaults.oversample_factor,
                    collocation_points,
                    max_mixing_order: defaults.max_mixing_order,
                    use_krylov: defaults.use_krylov,
                    gmres_restart: defaults.gmres_restart,
                    source_stepping: defaults.source_stepping,
                    verbose: defaults.verbose,
                },
                config: None,
                spec_options,
                analysis_line: ".hb".to_string(),
            })
        }
        AnalysisCommand::Sp {
            variation,
            points,
            start_freq,
            stop_freq,
            do_noise: _,
        } => {
            let ports = collect_sparameter_ports(netlist)?;
            let z0 = ports.first().and_then(|port| port.z0).unwrap_or(50.0);
            Ok(QueuedAnalysis {
                spec: AnalysisSpec::SParameter {
                    start_freq: *start_freq,
                    stop_freq: *stop_freq,
                    points_per_unit: *points,
                    sweep: frequency_sweep(*variation),
                    z0,
                    ports,
                },
                config: None,
                spec_options,
                analysis_line: ".sp".to_string(),
            })
        }
        AnalysisCommand::Stb {
            variation,
            points,
            start_freq,
            stop_freq,
            probe,
        } => Ok(QueuedAnalysis {
            spec: AnalysisSpec::Stb {
                probe_node: probe.clone(),
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                sweep: frequency_sweep(*variation),
                points_per_decade: *points,
            },
            config: None,
            spec_options,
            analysis_line: format!(
                ".stb {} {} {} {} probe={}",
                frequency_sweep(*variation).runner_keyword(),
                points,
                start_freq,
                stop_freq,
                probe
            ),
        }),
        AnalysisCommand::Disto {
            variation,
            points,
            start_freq,
            stop_freq,
            f2_over_f1,
        } => Ok(QueuedAnalysis {
            spec: AnalysisSpec::Disto {
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                points_per_unit: *points,
                sweep: frequency_sweep(*variation),
                f2_over_f1: *f2_over_f1,
            },
            config: None,
            spec_options,
            analysis_line: ".disto".to_string(),
        }),
        AnalysisCommand::Tran {
            step,
            stop,
            start,
            max_step,
            uic,
        } => {
            let start_time = start.unwrap_or(0.0);
            let spec = AnalysisSpec::Transient {
                stop_time: *stop,
                step_time: *step,
                start_time,
                max_timestep: *max_step,
                uic: *uic,
            };
            Ok(QueuedAnalysis {
                config: Some(AnalysisConfig::Transient(TransientAnalysisConfig {
                    stop_time: *stop,
                    step_time: *step,
                    start_time,
                    max_timestep: *max_step,
                    uic: *uic,
                })),
                analysis_line: ".tran".to_string(),
                spec,
                spec_options,
            })
        }
        AnalysisCommand::Noise {
            output_node,
            reference_node,
            input_source,
            variation,
            points,
            start_freq,
            stop_freq,
        } => {
            let reference_node = reference_node.clone().unwrap_or_else(|| "0".to_string());
            let temperature = netlist
                .options
                .temp
                .unwrap_or(state.sim_setup.reference_pvt.temperature_celsius)
                + 273.15;
            let sweep = match variation {
                FreqVariation::Dec => NoiseSweepType::Decade,
                FreqVariation::Oct => NoiseSweepType::Octave,
                FreqVariation::Lin => NoiseSweepType::Linear,
            };
            let spec = AnalysisSpec::Noise {
                output_node: output_node.clone(),
                reference_node: reference_node.clone(),
                input_source: input_source.clone(),
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                points_per_decade: *points,
                sweep,
                explicit_frequencies: None,
                data_table_name: None,
                contribution_detail: crate::simulation::config::NoiseContributionDetail::Top50,
                integration_mode: crate::simulation::config::NoiseIntegrationMode::Enabled,
                temperature,
            };
            Ok(QueuedAnalysis {
                config: Some(AnalysisConfig::Noise(NoiseAnalysisConfig {
                    output_node: output_node.clone(),
                    reference_node,
                    input_source: input_source.clone(),
                    sweep_type: ac_sweep(*variation),
                    num_points: *points,
                    start_freq: *start_freq,
                    stop_freq: *stop_freq,
                    explicit_frequencies: None,
                    data_table_name: None,
                    contribution_detail: crate::simulation::config::NoiseContributionDetail::Top50,
                    integration_mode: crate::simulation::config::NoiseIntegrationMode::Enabled,
                    temperature_kelvin: temperature,
                })),
                analysis_line: ".noise".to_string(),
                spec,
                spec_options,
            })
        }
        AnalysisCommand::NoiseData {
            output_node,
            reference_node,
            input_source,
            table_name,
        } => {
            let reference_node = reference_node.clone().unwrap_or_else(|| "0".to_owned());
            let points = netlist
                .frequency_data_table_points(table_name)
                .map_err(|error| format!(".NOISE DATA {error}"))?;
            let frequencies = points
                .into_iter()
                .map(|point| point.frequency)
                .collect::<Vec<_>>();
            let start_freq = frequencies.first().copied().unwrap_or_default();
            let stop_freq = frequencies.last().copied().unwrap_or_default();
            let temperature = netlist
                .options
                .temp
                .unwrap_or(state.sim_setup.reference_pvt.temperature_celsius)
                + 273.15;
            let config = NoiseAnalysisConfig {
                output_node: output_node.clone(),
                reference_node: reference_node.clone(),
                input_source: input_source.clone(),
                sweep_type: AcSweepType::Decade,
                num_points: frequencies.len(),
                start_freq,
                stop_freq,
                explicit_frequencies: Some(frequencies.clone()),
                data_table_name: Some(table_name.clone()),
                contribution_detail: crate::simulation::config::NoiseContributionDetail::Top50,
                integration_mode: crate::simulation::config::NoiseIntegrationMode::Enabled,
                temperature_kelvin: temperature,
            };
            config.validate().map_err(|errors| errors.join("; "))?;
            Ok(QueuedAnalysis {
                spec: AnalysisSpec::Noise {
                    output_node: output_node.clone(),
                    reference_node,
                    input_source: input_source.clone(),
                    start_freq,
                    stop_freq,
                    points_per_decade: frequencies.len(),
                    sweep: NoiseSweepType::ExplicitFrequencyList,
                    explicit_frequencies: Some(frequencies),
                    data_table_name: Some(table_name.clone()),
                    contribution_detail: crate::simulation::config::NoiseContributionDetail::Top50,
                    integration_mode: crate::simulation::config::NoiseIntegrationMode::Enabled,
                    temperature,
                },
                config: Some(AnalysisConfig::Noise(config)),
                spec_options,
                analysis_line: format!(".noise data={table_name}"),
            })
        }
        AnalysisCommand::PoleZero {
            input_pos,
            input_neg,
            output_pos,
            output_neg,
            transfer_type,
            analysis_type,
        } => {
            let transfer_name = pz_transfer_name(*transfer_type);
            let analysis_name = pz_analysis_name(*analysis_type);
            let spec = AnalysisSpec::PoleZero {
                input_node: input_pos.clone(),
                input_ref: input_neg.clone(),
                output_node: output_pos.clone(),
                output_ref: output_neg.clone(),
                transfer_type: transfer_name.clone(),
                analysis_type: analysis_name.clone(),
            };
            Ok(QueuedAnalysis {
                config: Some(AnalysisConfig::PoleZero(PoleZeroConfig {
                    input_node: input_pos.clone(),
                    input_ref: input_neg.clone(),
                    output_node: output_pos.clone(),
                    output_ref: output_neg.clone(),
                    transfer_type: transfer_name,
                    analysis_type: pz_config_type(*analysis_type),
                })),
                analysis_line: ".pz".to_string(),
                spec,
                spec_options,
            })
        }
        AnalysisCommand::Sensitivity {
            output_node,
            reference_node,
            output_is_current,
            ac_sweep,
            ..
        } => {
            let output_var = if *output_is_current {
                format!("I({output_node})")
            } else {
                match reference_node {
                    Some(reference) => format!("V({output_node},{reference})"),
                    None => format!("V({output_node})"),
                }
            };
            let frequency = ac_sweep.as_ref().map(|sweep| sweep.start_freq);
            let spec = AnalysisSpec::Sensitivity {
                output_var: output_var.clone(),
                ac_mode: ac_sweep.is_some(),
                frequency,
            };
            Ok(QueuedAnalysis {
                config: Some(AnalysisConfig::Sensitivity(SensitivityConfig {
                    output_var,
                    ac_mode: ac_sweep.is_some(),
                    frequency,
                })),
                analysis_line: ".sens".to_string(),
                spec,
                spec_options,
            })
        }
        AnalysisCommand::Tf {
            output_node,
            reference_node,
            output_is_current,
            input_source,
        } => {
            let output_expression = if *output_is_current {
                format!("I({output_node})")
            } else if let Some(reference_node) = reference_node {
                format!("V({output_node},{reference_node})")
            } else {
                format!("V({output_node})")
            };

            Ok(QueuedAnalysis {
                config: None,
                analysis_line: ".tf".to_string(),
                spec: AnalysisSpec::Tf {
                    input_source: input_source.clone(),
                    output_expression,
                    // Classic SPICE .TF requests all three DC-linearized
                    // quantities as one indivisible analysis result.
                    transfer_gain: true,
                    input_resistance: true,
                    output_resistance: true,
                    // The directive has no normalization or solver-profile
                    // operands. Preserve raw SPICE semantics and use the
                    // product's standard numerical policy.
                    normalization: crate::simulation::multi_run::TfNormalization::None,
                    accuracy: crate::simulation::multi_run::TfAccuracy::Balanced,
                },
                spec_options,
            })
        }
        AnalysisCommand::Four {
            fundamental,
            outputs,
            num_harmonics,
        } => {
            let Some(output) = outputs.first() else {
                return Err(".four requires at least one output".to_string());
            };
            fourier_queue_item(netlist, *fundamental, output, *num_harmonics)
        }
        AnalysisCommand::MonteCarlo(_) => Ok(QueuedAnalysis {
            spec: AnalysisSpec::MonteCarlo,
            config: None,
            spec_options,
            analysis_line: ".mc".to_string(),
        }),
        AnalysisCommand::Step(_) => Ok(QueuedAnalysis {
            spec: AnalysisSpec::Parametric,
            config: None,
            spec_options,
            analysis_line: ".step".to_string(),
        }),
        AnalysisCommand::Temp { .. } => Err(
            ".temp directives must be planned as temperature sweeps before queueing".to_string(),
        ),
    }
}

fn fourier_queue_item(
    netlist: &Netlist,
    fundamental: f64,
    output: &str,
    num_harmonics: usize,
) -> Result<QueuedAnalysis, String> {
    let (output_node, output_ref) = parse_fourier_output(output)?;
    validate_manual_fourier_current_capability(netlist, &output_node)?;
    Ok(QueuedAnalysis {
        spec: AnalysisSpec::Fourier {
            fundamental_freq: fundamental,
            num_harmonics,
            output_node,
            output_ref,
            // Bound to the exact manual .TRAN window after the complete
            // directive list has been compiled.
            start_time: 0.0,
            stop_time: 0.0,
            // Classic .FOUR retains THD and dimensional Fourier components.
            compute_thd: true,
            normalize: false,
        },
        config: None,
        spec_options: SpecExecutionOptions::default(),
        analysis_line: ".four".to_string(),
    })
}

fn validate_manual_fourier_current_capability(
    netlist: &Netlist,
    output_node: &str,
) -> Result<(), String> {
    let Some(device) = output_node
        .strip_prefix("I(")
        .or_else(|| output_node.strip_prefix("i("))
        .and_then(|inner| inner.strip_suffix(')'))
        .map(str::trim)
    else {
        return Ok(());
    };

    let designator = device
        .rsplit(['.', ':'])
        .next()
        .and_then(|leaf| leaf.chars().next())
        .map(|value| value.to_ascii_uppercase());
    let supported = matches!(
        designator,
        Some('R' | 'C' | 'L' | 'V' | 'I' | 'E' | 'H' | 'B' | 'S' | 'W' | 'Y' | 'O' | 'T')
    );
    if !supported {
        return Err(format!(
            "Manual-deck .FOUR current output 'I({device})' is not an exact retained Transient branch. Use a voltage/current source, passive branch, supported controlled source, switch, memristor, or transmission-line branch; semiconductor terminal currents require a typed terminal-current selector."
        ));
    }

    // Top-level names are fully known at parse time. Hierarchical names are
    // resolved after subcircuit expansion, so their designator contract is the
    // strongest fail-closed check available at this stage.
    if !device.contains('.') && !device.contains(':') {
        let Some(element) = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(device))
        else {
            return Err(format!(
                "Manual-deck .FOUR current output 'I({device})' does not name a top-level circuit element"
            ));
        };
        if !matches!(
            element.kind,
            ElementKind::Resistor { .. }
                | ElementKind::Capacitor { .. }
                | ElementKind::Inductor { .. }
                | ElementKind::JilesAthertonInductor { .. }
                | ElementKind::VoltageSource(_)
                | ElementKind::VoltageSourceDeferred(_)
                | ElementKind::CurrentSource(_)
                | ElementKind::CurrentSourceDeferred(_)
                | ElementKind::Vcvs { .. }
                | ElementKind::Ccvs { .. }
                | ElementKind::BehavioralVoltage { .. }
                | ElementKind::BehavioralCurrent { .. }
                | ElementKind::VSwitch { .. }
                | ElementKind::ISwitch { .. }
                | ElementKind::GenericSwitch { .. }
                | ElementKind::XyceMemristor { .. }
                | ElementKind::TransmissionLine { .. }
        ) {
            return Err(format!(
                "Manual-deck .FOUR current output 'I({device})' does not map to an exact retained Transient branch"
            ));
        }
    }
    Ok(())
}

fn parse_fourier_output(output: &str) -> Result<(String, String), String> {
    let trimmed = output.trim();
    if trimmed.len() >= 4
        && (trimmed.starts_with("I(") || trimmed.starts_with("i("))
        && trimmed.ends_with(')')
    {
        let device = trimmed[2..trimmed.len() - 1].trim();
        if !device.is_empty() && !device.contains(',') {
            return Ok((format!("I({device})"), String::new()));
        }
        return Err(format!(
            "Manual-deck .FOUR current output '{trimmed}' must identify exactly one device"
        ));
    }
    if trimmed.len() < 4
        || !(trimmed.starts_with("V(") || trimmed.starts_with("v("))
        || !trimmed.ends_with(')')
    {
        return Err(format!(
            "Manual-deck .FOUR output '{trimmed}' is unsupported; use V(node), V(node+, node-), or I(device)"
        ));
    }
    let nodes = trimmed[2..trimmed.len() - 1]
        .split(',')
        .map(str::trim)
        .collect::<Vec<_>>();
    match nodes.as_slice() {
        [node] if !node.is_empty() => Ok(((*node).to_owned(), "0".to_owned())),
        [positive, reference] if !positive.is_empty() && !reference.is_empty() => {
            Ok(((*positive).to_owned(), (*reference).to_owned()))
        }
        _ => Err(format!(
            "Manual-deck .FOUR output '{trimmed}' must contain one node or one differential node pair"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::simulation_runner::CornerBaseMode;
    use crate::simulation::multi_run::FrequencySweep;

    fn specs_for(source: &str) -> Vec<AnalysisSpec> {
        let state = AppState::default();
        build_manual_deck_queue(&state, source)
            .expect("manual deck queue")
            .into_iter()
            .map(|q| q.spec)
            .collect()
    }

    #[test]
    fn manual_deck_preserves_common_analysis_order() {
        let specs = specs_for(
            "deck\nR1 out 0 1k\nV1 out 0 1 AC 1\n.op\n.ac dec 20 1 1g\n.tran 1n 1u\n.end\n",
        );

        assert!(matches!(specs[0], AnalysisSpec::DcOp { .. }));
        assert!(matches!(
            specs[1],
            AnalysisSpec::Ac {
                start_freq,
                stop_freq,
                points_per_unit: 20,
                sweep: FrequencySweep::Decade
            } if (start_freq - 1.0).abs() < 1e-12 && (stop_freq - 1e9).abs() < 1.0
        ));
        assert!(matches!(
            specs[2],
            AnalysisSpec::Transient {
                step_time,
                stop_time,
                start_time,
                max_timestep: None,
                uic: false
            } if (step_time - 1e-9).abs() < 1e-21
                && (stop_time - 1e-6).abs() < 1e-18
                && start_time == 0.0
        ));
    }

    #[test]
    fn manual_fourier_inherits_the_exact_transient_window() {
        let specs = specs_for(
            "Fourier deck\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.four 1k V(out)\n.tran 10u 5m 1m\n.end\n",
        );

        assert!(matches!(
            specs[0],
            AnalysisSpec::Fourier {
                start_time,
                stop_time,
                ..
            } if (start_time - 1.0e-3).abs() < 1.0e-15
                && (stop_time - 5.0e-3).abs() < 1.0e-15
        ));
    }

    #[test]
    fn manual_fourier_preserves_each_voltage_output_and_reference() {
        let specs = specs_for(
            "Fourier deck\nV1 in 0 SIN(0 1 1k)\nR1 in out 1k\nR2 out 0 1k\n.four 1k V(out) V(out,in)\n.tran 10u 5m\n.end\n",
        );

        assert_eq!(specs.len(), 3);
        assert!(matches!(
            &specs[0],
            AnalysisSpec::Fourier {
                output_node,
                output_ref,
                ..
            } if output_node.eq_ignore_ascii_case("out")
                && (output_ref == "0" || output_ref.eq_ignore_ascii_case("gnd"))
        ));
        assert!(matches!(
            &specs[1],
            AnalysisSpec::Fourier {
                output_node,
                output_ref,
                ..
            } if output_node.eq_ignore_ascii_case("out")
                && output_ref.eq_ignore_ascii_case("in")
        ));
    }

    #[test]
    fn manual_fourier_preserves_a_branch_current_output() {
        let specs = specs_for(
            "Fourier current deck\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.four 1k I(V1)\n.tran 10u 5m\n.end\n",
        );

        assert!(matches!(
            &specs[0],
            AnalysisSpec::Fourier {
                output_node,
                output_ref,
                ..
            } if output_node.eq_ignore_ascii_case("I(V1)") && output_ref.is_empty()
        ));
    }

    #[test]
    fn manual_fourier_preserves_an_explicit_harmonic_count() {
        let specs = specs_for(
            "Fourier harmonic count\nV1 out 0 SIN(0 1 60)\nR1 out 0 1k\n.four 60 15 I(V1)\n.tran 100u 50m\n.end\n",
        );

        assert!(matches!(
            &specs[0],
            AnalysisSpec::Fourier {
                num_harmonics: 15,
                output_node,
                ..
            } if output_node.eq_ignore_ascii_case("I(V1)")
        ));
    }

    #[test]
    fn manual_fourier_current_output_fails_closed_before_transient_when_unavailable() {
        let state = AppState::default();
        let semiconductor = build_manual_deck_queue(
            &state,
            "Fourier diode current\nV1 in 0 SIN(0 1 1k)\nD1 in 0 DMOD\n.model DMOD D\n.four 1k I(D1)\n.tran 10u 5m\n.end\n",
        )
        .expect_err("an unavailable semiconductor terminal current must fail preflight");
        assert!(
            semiconductor
                .join("; ")
                .contains("typed terminal-current selector"),
            "{semiconductor:?}"
        );

        let missing = build_manual_deck_queue(
            &state,
            "Fourier missing current\nV1 in 0 SIN(0 1 1k)\n.four 1k I(R404)\n.tran 10u 5m\n.end\n",
        )
        .expect_err("a missing branch identity must fail preflight");
        assert!(missing.join("; ").contains("does not name a top-level"));
    }

    #[test]
    fn manual_fourier_rejects_missing_or_ambiguous_transient_producers() {
        let state = AppState::default();
        let missing = build_manual_deck_queue(
            &state,
            "Fourier deck\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.four 1k V(out)\n.end\n",
        )
        .expect_err(".FOUR without .TRAN must fail closed");
        assert!(missing.join("; ").contains("exactly one .TRAN"));

        let ambiguous = build_manual_deck_queue(
            &state,
            "Fourier deck\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.tran 10u 1m\n.tran 20u 2m\n.four 1k V(out)\n.end\n",
        )
        .expect_err(".FOUR with multiple .TRAN producers must fail closed");
        assert!(ambiguous.join("; ").contains("found 2"));
    }

    #[test]
    fn manual_deck_maps_hbint_order_to_exact_collocation_grid() {
        let specs = specs_for(
            "HB deck\nV1 in 0 PULSE(0 1 0 1n 1n 4n 10n)\nR1 in 0 1k\n.hb 100meg\n.options hbint numfreq=12\n.end\n",
        );

        assert!(matches!(
            &specs[0],
            AnalysisSpec::HarmonicBalance {
                tones,
                collocation_points: Some(25),
                ..
            } if tones.len() == 1
                && tones[0].harmonics == 12
                && (tones[0].frequency - 100.0e6).abs() < 1.0
        ));
    }

    #[test]
    fn manual_deck_ac_data_uses_table_frequencies() {
        let state = AppState::default();
        let queue = build_manual_deck_queue(
            &state,
            "deck\n\
             I1 out 0 AC 1\n\
             R1 out 0 1k\n\
             .ac data=pts\n\
             .data pts\n\
             + FREQ\n\
             + 10\n\
             + 1\n\
             + 2.5\n\
             .enddata\n\
             .end\n",
        )
        .expect("manual AC DATA deck should queue");

        assert_eq!(queue.len(), 1);
        assert!(queue[0].config.is_none());
        assert!(queue[0].analysis_line.eq_ignore_ascii_case(".ac data=pts"));
        let AnalysisSpec::AcData {
            table_name,
            frequencies,
        } = &queue[0].spec
        else {
            panic!("expected AC DATA analysis");
        };
        assert!(table_name.eq_ignore_ascii_case("pts"));
        assert_eq!(frequencies.as_slice(), [10.0, 1.0, 2.5]);
    }

    #[test]
    fn manual_deck_ac_data_requires_freq_column() {
        let state = AppState::default();
        let err = build_manual_deck_queue(
            &state,
            "deck\n\
             I1 out 0 AC 1\n\
             R1 out 0 1k\n\
             .ac data=pts\n\
             .data pts\n\
             + TEMP\n\
             + 25\n\
             .enddata\n\
             .end\n",
        )
        .expect_err("AC DATA table without FREQ should be rejected");

        assert!(
            err.iter().any(|message| message.contains("FREQ")),
            "expected FREQ diagnostic, got {err:?}"
        );
    }

    #[test]
    fn manual_deck_sp_uses_rf_port_annotations() {
        let specs = specs_for(
            "deck\n\
             V2 out 0 dc 0 ac 1 portnum 2 z0 75\n\
             V1 in 0 dc 0 ac 1 portnum 1 z0 50\n\
             R1 in out 100\n\
             .sp lin 3 1Meg 3Meg\n\
             .end\n",
        );

        let AnalysisSpec::SParameter {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            z0,
            ports,
        } = &specs[0]
        else {
            panic!("expected S-parameter analysis");
        };

        assert_eq!(*points_per_unit, 3);
        assert_eq!(*sweep, FrequencySweep::Linear);
        assert!((*start_freq - 1.0e6).abs() < 1e-6);
        assert!((*stop_freq - 3.0e6).abs() < 1e-6);
        assert_eq!(*z0, 50.0);
        assert_eq!(ports.len(), 2);
        assert_eq!(ports[0].node_pos, "IN");
        assert_eq!(ports[0].node_neg, "0");
        assert_eq!(ports[0].z0, Some(50.0));
        assert_eq!(ports[1].node_pos, "OUT");
        assert_eq!(ports[1].node_neg, "0");
        assert_eq!(ports[1].z0, Some(75.0));
    }

    #[test]
    fn temp_command_queue_fallback_reports_error_without_panicking() {
        let state = AppState::default();
        let netlist = Netlist::default();
        let err = command_to_queue_item(
            &state,
            &netlist,
            &AnalysisCommand::Temp {
                temperatures: vec![25.0],
            },
        )
        .expect_err(".temp fallback should be a recoverable queueing error");

        assert!(err.contains(".temp"));
        assert!(err.contains("temperature sweeps"));
    }

    #[test]
    fn manual_deck_dc_and_noise_build_configs_without_dialog_state() {
        let state = AppState::default();
        let queue = build_manual_deck_queue(
            &state,
            "deck\nV1 in 0 0 AC 1\nR1 in out 1k\n.ac lin 5 1 5\n.noise v(out) V1 dec 10 1 1e6\n.end\n",
        )
        .expect("queue builds");

        assert!(matches!(queue[0].spec, AnalysisSpec::Ac { .. }));
        assert!(matches!(queue[1].config, Some(AnalysisConfig::Noise(_))));
    }

    #[test]
    fn manual_noise_data_preserves_authored_axis_and_executes_row_contexts() {
        let state = AppState::default();
        let source = "noise data deck\n\
.param rload=1k\n\
V1 in 0 AC 1\n\
R1 in out 1k\n\
Rload out 0 {rload}\n\
.noise V(out) V1 DATA=points\n\
.DATA points\n\
+ rload HERTZ\n\
+ 1000 10\n\
+ 2000 1\n\
.ENDDATA\n\
.end\n";
        let queue = build_manual_deck_queue(&state, source).expect("NOISE DATA queues");
        assert_eq!(queue.len(), 1);
        assert!(matches!(
            &queue[0].spec,
            AnalysisSpec::Noise {
                sweep: NoiseSweepType::ExplicitFrequencyList,
                explicit_frequencies: Some(frequencies),
                data_table_name: Some(table),
                ..
            } if frequencies == &[10.0, 1.0] && table.eq_ignore_ascii_case("points")
        ));
        let config = queue[0].config.as_ref().expect("exact config retained");
        let result = crate::simulation::EngineBridge::new()
            .run_with_abort(config, source, &rspice_core::abort_signal::NoAbort)
            .expect("NOISE DATA executes through the ordinary config path");
        assert!(matches!(
            result,
            crate::simulation::SimulationResult::Noise { frequencies, .. }
                if frequencies == vec![10.0, 1.0]
        ));
    }

    #[test]
    fn manual_deck_mc_and_step_are_runnable_specs() {
        let state = AppState::default();
        let queue = build_manual_deck_queue(
            &state,
            "deck\n.param rload=1k\nV1 in 0 1\nR1 in out {rload}\nR2 out 0 1k\n.step param rload 500 1500 500\n.mc 8 seed 7 dist uniform spread 0.05\n.end\n",
        )
        .expect("manual deck queue");

        assert_eq!(queue.len(), 2);
        assert!(matches!(queue[0].spec, AnalysisSpec::Parametric));
        assert!(queue[0].config.is_none());
        assert!(matches!(queue[1].spec, AnalysisSpec::MonteCarlo));
        assert!(queue[1].config.is_none());
    }

    #[test]
    fn manual_deck_tf_builds_classic_dc_transfer_spec() {
        let state = AppState::default();
        let queue = build_manual_deck_queue(
            &state,
            "deck\nV1 in 0 DC 1\nR1 in out 1k\nR2 out 0 1k\n.tf V(out) V1\n.end\n",
        )
        .expect("classic .tf should produce a runnable typed analysis");

        assert_eq!(queue.len(), 1);
        assert_eq!(queue[0].analysis_line, ".tf");
        assert!(queue[0].config.is_none());
        assert!(matches!(
            &queue[0].spec,
            AnalysisSpec::Tf {
                input_source,
                output_expression,
                transfer_gain: true,
                input_resistance: true,
                output_resistance: true,
                normalization: crate::simulation::multi_run::TfNormalization::None,
                accuracy: crate::simulation::multi_run::TfAccuracy::Balanced,
            } if input_source == "V1" && output_expression == "V(OUT)"
        ));
    }

    #[test]
    fn manual_deck_tf_preserves_differential_voltage_probe() {
        let state = AppState::default();
        let queue = build_manual_deck_queue(
            &state,
            "deck\nV1 in 0 DC 1\nR1 in out 1k\nR2 out ref 1k\nR3 ref 0 1k\n.tf V(out,ref) V1\n.end\n",
        )
        .expect("differential .tf should produce a runnable typed analysis");

        assert!(matches!(
            &queue[0].spec,
            AnalysisSpec::Tf {
                input_source,
                output_expression,
                ..
            } if input_source == "V1" && output_expression == "V(OUT,REF)"
        ));
    }

    #[test]
    fn manual_deck_tf_preserves_branch_current_probe() {
        let state = AppState::default();
        let queue = build_manual_deck_queue(
            &state,
            "deck\nV1 in 0 DC 1\nR1 in mid 1k\nVMEAS mid out DC 0\nR2 out 0 1k\n.tf I(VMEAS) V1\n.end\n",
        )
        .expect("branch-current .tf should produce a runnable typed analysis");

        assert!(matches!(
            &queue[0].spec,
            AnalysisSpec::Tf {
                input_source,
                output_expression,
                ..
            } if input_source == "V1" && output_expression == "I(VMEAS)"
        ));
    }

    #[test]
    fn manual_deck_rejects_multiple_step_commands() {
        let state = AppState::default();
        let err = build_manual_deck_queue(
            &state,
            "deck\n.param rload=1k cload=1p\nV1 out 0 1\nR1 out 0 {rload}\nC1 out 0 {cload}\n.step param rload 1k 2k 1k\n.step param cload 1p 2p 1p\n.end\n",
        )
        .expect_err("multiple step commands should be diagnosed");

        assert!(
            err.iter().any(|message| message.contains("one .step")),
            "expected duplicate .step diagnostic, got {err:?}"
        );
    }

    #[test]
    fn manual_deck_rejects_multiple_monte_carlo_commands() {
        let state = AppState::default();
        let err = build_manual_deck_queue(
            &state,
            "deck\n.param rload=1k\nV1 out 0 1\nR1 out 0 {rload}\n.mc 4 seed 1\n.mc 5 seed 2\n.end\n",
        )
        .expect_err("multiple Monte Carlo commands should be diagnosed");

        assert!(
            err.iter().any(|message| message.contains("one .mc")),
            "expected duplicate .mc diagnostic, got {err:?}"
        );
    }

    #[test]
    fn manual_deck_temp_directive_does_not_block_runnable_analysis() {
        let state = AppState::default();
        let queue = build_manual_deck_queue(&state, "deck\n.temp 125\nV1 out 0 1\n.op\n.end\n")
            .expect("manual deck queue");

        assert_eq!(queue.len(), 1);
        assert!(matches!(queue[0].spec, AnalysisSpec::Parametric));
        let temp = queue[0]
            .spec_options
            .temp
            .as_ref()
            .expect("temperature options");
        assert_eq!(temp.temperatures_c, vec![125.0]);
        assert!(matches!(temp.base_mode, CornerBaseMode::Op));
    }

    #[test]
    fn manual_deck_step_temp_uses_paired_transient_base_analysis() {
        let state = AppState::default();
        let queue = build_manual_deck_queue(
            &state,
            "deck\nV1 out 0 pulse(0 1 0 1n 1n 5n 10n)\nR1 out 0 1k\n.tran 1n 10n\n.step temp list 0 25 125\n.end\n",
        )
        .expect("manual deck queue");

        assert_eq!(queue.len(), 1);
        assert!(matches!(queue[0].spec, AnalysisSpec::Parametric));
        let temp = queue[0]
            .spec_options
            .temp
            .as_ref()
            .expect("temperature options");
        assert_eq!(temp.temperatures_c, vec![0.0, 25.0, 125.0]);
        assert!(matches!(
            temp.base_mode,
            CornerBaseMode::Transient {
                stop_time,
                step_time,
            } if (stop_time - 10e-9).abs() < 1e-21
                && (step_time - 1e-9).abs() < 1e-21
        ));
    }

    #[test]
    fn manual_deck_temp_list_uses_paired_op_base_analysis() {
        let state = AppState::default();
        let queue = build_manual_deck_queue(&state, "deck\n.temp 25 125\nV1 out 0 1\n.op\n.end\n")
            .expect("manual deck queue");

        assert_eq!(queue.len(), 1);
        assert!(matches!(queue[0].spec, AnalysisSpec::Parametric));
        let temp = queue[0]
            .spec_options
            .temp
            .as_ref()
            .expect("temperature options");
        assert_eq!(temp.temperatures_c, vec![25.0, 125.0]);
        assert!(matches!(temp.base_mode, CornerBaseMode::Op));
    }

    #[test]
    fn manual_deck_rejects_parameter_step_with_transient_base_analysis() {
        let state = AppState::default();
        let err = build_manual_deck_queue(
            &state,
            "deck\n.param rload=1k\nV1 out 0 pulse(0 1 0 1n 1n 5n 10n)\nR1 out 0 {rload}\n.tran 1n 10n\n.step param rload 1k 2k 1k\n.end\n",
        )
        .expect_err("parameter step with transient should be diagnosed");

        assert!(
            err.iter()
                .any(|message| message.contains("parameter .step")),
            "expected unsupported parameter .step diagnostic, got {err:?}"
        );
    }

    #[test]
    fn manual_deck_temp_only_reports_no_runnable_analysis() {
        let state = AppState::default();
        let err = build_manual_deck_queue(&state, "deck\n.temp 25 125\n.end\n")
            .expect_err("temperature directive alone should not run");

        assert!(
            err.iter()
                .any(|message| message.contains("No runnable analysis command"))
        );
    }

    #[test]
    fn manual_deck_source_does_not_append_analysis_lines() {
        let source = "deck\nR1 out 0 1k\n.op\n.end\n";
        assert_eq!(compose_manual_deck_source(source), source);
    }

    #[test]
    fn manual_deck_adds_end_only_when_missing() {
        assert_eq!(compose_manual_deck_source("deck\n.op"), "deck\n.op\n.end\n");
    }

    #[test]
    fn manual_deck_reports_no_analysis() {
        let state = AppState::default();
        let err = build_manual_deck_queue(&state, "deck\nR1 a 0 1k\n.end\n")
            .expect_err("no analysis should fail");
        assert!(
            err.iter()
                .any(|e| e.contains("No analysis command in netlist"))
        );
    }
}
