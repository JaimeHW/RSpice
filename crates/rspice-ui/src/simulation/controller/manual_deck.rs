use super::*;
use rspice_core::netlist::{
    AnalysisCommand, FreqVariation, Netlist, PoleZeroAnalysisType, PoleZeroTransferType,
    StepCommand, StepTarget,
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
        match command_to_queue_item(state, command) {
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
        Ok(queue)
    } else {
        Err(errors)
    }
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
        AnalysisCommand::Stb { .. } => ".stb",
        AnalysisCommand::Disto { .. } => ".disto",
        AnalysisCommand::Tran { .. } => ".tran",
        AnalysisCommand::Noise { .. } => ".noise",
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

fn command_to_queue_item(
    state: &AppState,
    command: &AnalysisCommand,
) -> Result<QueuedAnalysis, String> {
    let spec_options = SpecExecutionOptions::default();
    match command {
        AnalysisCommand::Op => Ok(QueuedAnalysis {
            spec: AnalysisSpec::DcOp,
            config: Some(AnalysisConfig::DcOp),
            spec_options,
            analysis_line: ".op".to_string(),
        }),
        AnalysisCommand::Dc {
            source,
            start,
            stop,
            step,
            sweep2,
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
            let spec = AnalysisSpec::Noise {
                output_node: output_node.clone(),
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                points_per_decade: *points,
                temperature: state.sim_setup.options.temp + 273.15,
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
                })),
                analysis_line: ".noise".to_string(),
                spec,
                spec_options,
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
            ac_sweep,
        } => {
            let output_var = match reference_node {
                Some(reference) => format!("V({output_node},{reference})"),
                None => format!("V({output_node})"),
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
        AnalysisCommand::Tf { .. } => Err(
            "Manual deck classic .tf is a DC transfer-function analysis; the current XF runner is frequency-sweep based, so .tf decks are rejected until DC .tf execution is wired through without invented sweep defaults"
                .to_string(),
        ),
        AnalysisCommand::Four {
            fundamental,
            outputs,
            num_harmonics,
        } => {
            let Some(output) = outputs.first() else {
                return Err(".four requires at least one output".to_string());
            };
            Ok(QueuedAnalysis {
                spec: AnalysisSpec::Fourier {
                    fundamental_freq: *fundamental,
                    num_harmonics: *num_harmonics,
                    output_node: output.clone(),
                    output_ref: "0".to_string(),
                    start_time: 0.0,
                    stop_time: 0.0,
                },
                config: None,
                spec_options,
                analysis_line: ".four".to_string(),
            })
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
        AnalysisCommand::Temp { .. } => {
            Err(".temp directives must be planned as temperature sweeps before queueing".to_string())
        }
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

        assert!(matches!(specs[0], AnalysisSpec::DcOp));
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
    fn temp_command_queue_fallback_reports_error_without_panicking() {
        let state = AppState::default();
        let err = command_to_queue_item(
            &state,
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
    fn manual_deck_tf_rejects_invented_ac_sweep_fallback() {
        let state = AppState::default();
        let err = build_manual_deck_queue(
            &state,
            "deck\nV1 in 0 DC 1\nR1 in out 1k\nR2 out 0 1k\n.tf V(out) V1\n.end\n",
        )
        .expect_err("classic .tf must not be mapped to default AC/XF sweep");

        assert!(
            err.iter().any(|message| message.contains("classic .tf")),
            "expected manual .tf diagnostic, got {err:?}"
        );
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
