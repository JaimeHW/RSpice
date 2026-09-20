//! Running a hand-written deck.
//!
//! Composes the source a Netlist-workspace run executes and queues the
//! analyses the deck itself declares, rather than the ones the Simulate
//! workspace configured.

use super::*;
use rspice_core::netlist::{
    AnalysisCommand, ElementKind, FreqVariation, Netlist, PoleZeroAnalysisType,
    PoleZeroTransferType, StepCommand, StepTarget,
};

use crate::services::simulation_runner::{
    CornerBaseMode, CornerFrequencySweep, TempRunConfig, expand_step_sweep_values,
};

mod periodic;
mod recorded_fft;

/// Apply the reviewed source contract before model binding or include parsing.
pub(super) fn adapt_owned_execution_profile<'a>(
    descriptor: Option<&crate::state::OwnedNetlistDescriptor>,
    source: &'a str,
) -> Result<std::borrow::Cow<'a, str>, String> {
    let Some(descriptor) = descriptor else {
        return Ok(std::borrow::Cow::Borrowed(source));
    };
    if descriptor.execution_profile_review_required() {
        return Err("Review this deck's execution profile before running it.".to_owned());
    }
    let Some(profile) = descriptor.execution_profile else {
        return Ok(std::borrow::Cow::Borrowed(source));
    };
    if profile.source_dialect() != descriptor.imported_dialect.unwrap_or_default() {
        return Err(
            "The deck's execution profile does not match its reviewed source dialect.".to_owned(),
        );
    }
    let adapted = profile.adapt_source(source)?;
    profile.validate_executable_source(&adapted)?;
    Ok(adapted)
}

/// Validate the sealed dependency closure and bind compatibility into the
/// executable source. All analysis drivers and workers resolve these options
/// through the same core configuration pipeline; the authored deck is retained.
pub(super) fn bind_execution_profile(
    profile: Option<crate::state::NetlistExecutionProfile>,
    source: String,
) -> Result<String, String> {
    let Some(profile) = profile else {
        return Ok(source);
    };
    profile.validate_resolved_source(&source)?;
    let parsed = Netlist::parse(&source).map_err(|error| error.to_string())?;
    profile.validate_parsed_netlist(&parsed)?;
    if let Some(diagnostic) = parsed.diagnostics.iter().find(|diagnostic| {
        crate::state::NetlistExecutionProfile::diagnostic_is_semantic_loss(&diagnostic.code)
    }) {
        return Err(format!(
            "Parser diagnostic {} at line {}: {}",
            diagnostic.code, diagnostic.line, diagnostic.message
        ));
    }
    rspice_core::netlist::validate_output_symbols(&parsed).map_err(|error| error.to_string())?;
    if profile == crate::state::NetlistExecutionProfile::RSpiceCanonicalV1 {
        return Ok(source);
    }
    let dialect = match profile.spice_dialect() {
        rspice_core::SpiceDialect::BestAvailable => "BEST_AVAILABLE",
        rspice_core::SpiceDialect::Ngspice => "NGSPICE",
        rspice_core::SpiceDialect::Xyce => "XYCE",
    };
    Ok(
        crate::services::simulation_runner::splice_before_terminal_end_card(
            &source,
            &format!(
                "* RSpice execution profile: {}\n.OPTIONS RSPICE_DIALECT={dialect}",
                profile.id()
            ),
        ),
    )
}

pub(super) fn build_manual_deck_queue(
    state: &AppState,
    source: &str,
) -> Result<Vec<QueuedAnalysis>, Vec<String>> {
    let source = compose_manual_deck_source(source);
    let parsed =
        Netlist::parse(&source).map_err(|err| vec![format!("Netlist parse error: {err}")])?;
    let periodic_tasks = periodic::parse_periodic_tasks(&parsed, &source)?;
    if parsed.analyses.is_empty() && periodic_tasks.is_empty() {
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
    let parameter_step_plan = match build_parameter_step_plan(&parsed.analyses) {
        Ok(plan) => plan,
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
        if let Some(plan) = &parameter_step_plan
            && idx == plan.insert_index
        {
            queue.push(plan.item.clone());
        }
        if temperature_plan
            .as_ref()
            .is_some_and(|plan| plan.skip_indices.contains(&idx))
            || parameter_step_plan
                .as_ref()
                .is_some_and(|plan| plan.skip_indices.contains(&idx))
        {
            continue;
        }
        if matches!(command, AnalysisCommand::Temp { .. }) {
            continue;
        }
        // The netlist parser owns `.PSS`, `.PAC` and `.PNOISE`, so they arrive
        // here as parsed cards as well as in the periodic reader's queue. Only
        // that reader can queue them: it is what binds each dependent card to
        // the deck's one PSS operating point and freezes its execution
        // options, and queueing them here too would run every periodic
        // analysis twice. `.ENVELOPE` is left to its refusal below — it has no
        // manual-deck route at all.
        if matches!(
            command,
            AnalysisCommand::Pss(_)
                | AnalysisCommand::Pac(_)
                | AnalysisCommand::Pxf(_)
                | AnalysisCommand::Pstb(_)
                | AnalysisCommand::Pnoise(_)
        ) {
            continue;
        }
        if let AnalysisCommand::Four { outputs, .. } = command {
            for output in outputs {
                match fourier_queue_item(&parsed, command, output) {
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

    let periodic_pss_count = periodic_tasks
        .iter()
        .filter(|task| matches!(task.spec, AnalysisSpec::Pss { .. }))
        .count();
    if periodic_pss_count > 0 {
        let op_count = queue
            .iter()
            .filter(|task| {
                matches!(
                    task.spec,
                    AnalysisSpec::LegacyDcOp | AnalysisSpec::DcOp { .. }
                )
            })
            .count();
        if op_count > 1 {
            errors.push(format!(
                "Manual-deck PSS requires one unambiguous operating-point seed; found {op_count} .OP analyses."
            ));
        } else if op_count == 0 {
            match command_to_queue_item(state, &parsed, &AnalysisCommand::Op) {
                Ok(mut item) => {
                    item.analysis_line = ".op (implicit PSS seed)".to_owned();
                    queue.push(item);
                }
                Err(error) => errors.push(format!("Implicit PSS operating point: {error}")),
            }
        }
    }
    queue.extend(periodic_tasks);
    // The deck already carries these cards and the engine already evaluates
    // them inside the transient; what was missing was the task that publishes
    // the spectrum, so that is all this adds.
    match recorded_fft::manual_fft_tasks(&parsed) {
        Ok(tasks) => queue.extend(tasks),
        Err(mut card_errors) => errors.append(&mut card_errors),
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
            *start_time = start_time.max(transient_start);
            if *stop_time == 0.0 {
                *stop_time = transient_stop;
            }
            if *stop_time > transient_stop {
                return Err(".FOUR TO exceeds the retained .TRAN stop time".to_owned());
            }
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

    let (base_index, base_mode) = match (supported_bases.first(), step_temp) {
        (Some((idx, command)), _) => (*idx, temperature_base_mode(command)?),
        (None, Some((idx, _))) => (idx, CornerBaseMode::Op),
        (None, None) => return Ok(None),
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
            numeric_override: None,
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

fn build_parameter_step_plan(
    analyses: &[AnalysisCommand],
) -> Result<Option<PlannedQueueItem>, String> {
    let Some((step_index, step)) = analyses.iter().enumerate().find_map(|(idx, command)| {
        if let AnalysisCommand::Step(step) = command {
            (step.target != StepTarget::Temp).then_some((idx, step))
        } else {
            None
        }
    }) else {
        return Ok(None);
    };

    let mut supported_bases = Vec::new();
    let mut unsupported = Vec::new();
    for (idx, command) in analyses.iter().enumerate() {
        match command {
            AnalysisCommand::Op
            | AnalysisCommand::Dc { .. }
            | AnalysisCommand::Tran { .. }
            | AnalysisCommand::Ac { .. } => supported_bases.push((idx, command)),
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
            "Manual parameter .step supports .op, .dc, .tran, or .ac as its paired base analysis; found unsupported analysis {} for target {}",
            unsupported.join(", "),
            step_target_name(step)
        ));
    }
    if supported_bases.len() > 1 {
        return Err(format!(
            "Manual parameter .step requires one unambiguous base analysis; found {}",
            supported_bases
                .iter()
                .map(|(_, command)| command_name(command))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    let (base_index, base_mode) = if let Some((index, command)) = supported_bases.first() {
        (*index, temperature_base_mode(command)?)
    } else {
        (step_index, CornerBaseMode::Op)
    };
    let mut skip_indices = vec![step_index];
    if base_index != step_index {
        skip_indices.push(base_index);
    }
    skip_indices.sort_unstable();
    skip_indices.dedup();

    Ok(Some(PlannedQueueItem {
        insert_index: skip_indices[0],
        skip_indices,
        item: QueuedAnalysis {
            numeric_override: None,
            spec: AnalysisSpec::Parametric,
            config: None,
            spec_options: SpecExecutionOptions {
                parametric_base: Some(base_mode),
                ..SpecExecutionOptions::default()
            },
            analysis_line: format!(".step {}", step_target_name(step)),
        },
    }))
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
            mode,
        } => {
            if let Some(second) = sweep2 {
                return Ok(CornerBaseMode::DcSweepNested {
                    modes: crate::simulation::config::DcSweepModes {
                        primary: mode.into(),
                        secondary: sweep2
                            .as_ref()
                            .map_or_else(Default::default, |second| (&second.mode).into()),
                    },
                    source_name: source.clone(),
                    start: *start,
                    stop: *stop,
                    step: *step,
                    source2: second.source.clone(),
                    start2: second.start,
                    stop2: second.stop,
                    step2: second.step,
                });
            }
            Ok(CornerBaseMode::DcSweep {
                modes: crate::simulation::config::DcSweepModes {
                    primary: mode.into(),
                    secondary: sweep2
                        .as_ref()
                        .map_or_else(Default::default, |second| (&second.mode).into()),
                },
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
                return Ok(CornerBaseMode::TransientWindow {
                    stop_time: *stop,
                    step_time: *step,
                    start_time: start.unwrap_or(0.0),
                    max_timestep: *max_step,
                    uic: *uic,
                });
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
        AnalysisCommand::Hb(_) => ".hb",
        AnalysisCommand::Qpss(_) => ".qpss",
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
        AnalysisCommand::Pss(_) => ".pss",
        AnalysisCommand::Pac(_) => ".pac",
        AnalysisCommand::Pxf(_) => ".pxf",
        AnalysisCommand::Pstb(_) => ".pstb",
        AnalysisCommand::Pnoise(_) => ".pnoise",
        AnalysisCommand::Envelope(_) => ".envelope",
        AnalysisCommand::DcMatch(_) => ".dcmatch",
    }
}

pub(super) fn compose_manual_deck_source(source: &str) -> String {
    let has_end = crate::services::simulation_runner::terminal_end_card_offset(source).is_some();
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

/// The band a hand-written `.SENS ... AC` card states, or `None` when the
/// card is the degenerate single frequency the form writes.
///
/// `DEC 1 f f` is what the Studio has always written for one frequency, and
/// the engine's grid function turns it back into exactly that one point. Any
/// other spelling — more points, or a stop above the start — is a band the
/// reader authored, and it reaches the run whole.
fn sensitivity_sweep_from_card(
    sweep: &rspice_core::netlist::SensitivityAcSweep,
) -> Option<crate::simulation::config::SensitivitySweep> {
    use crate::simulation::config::{AcSweepType, SensitivitySweep};
    use rspice_core::netlist::FreqVariation;

    if sweep.variation == FreqVariation::Dec
        && sweep.points == 1
        && sweep.start_freq == sweep.stop_freq
    {
        return None;
    }
    Some(SensitivitySweep {
        stop_frequency: sweep.stop_freq,
        points: u32::try_from(sweep.points).unwrap_or(u32::MAX),
        variation: match sweep.variation {
            FreqVariation::Dec => AcSweepType::Decade,
            FreqVariation::Oct => AcSweepType::Octave,
            FreqVariation::Lin => AcSweepType::Linear,
        },
    })
}

fn pz_config_type(analysis_type: PoleZeroAnalysisType) -> PzAnalysisType {
    match analysis_type {
        PoleZeroAnalysisType::PoleZero => PzAnalysisType::PoleZero,
        PoleZeroAnalysisType::PolesOnly => PzAnalysisType::PolesOnly,
        PoleZeroAnalysisType::ZerosOnly => PzAnalysisType::ZerosOnly,
    }
}

/// The frequency axis a `.AC DATA=` card refers to, resolved the way the
/// engine resolves it.
///
/// Delegated to `Netlist::frequency_data_table_points`, which is the resolver
/// `Engine::run_ac_data` itself uses, exactly as the `.NOISE DATA=` arm above
/// delegates to it. This function used to re-state the rule and stated it
/// differently: it accepted a `FREQ` column only, while the engine accepts
/// `FREQ` *or* `HERTZ`. A hand-written deck whose table said `HERTZ` therefore
/// ran in the engine and in the CLI and was refused here — and so did the
/// Studio's own generated table, which is written in the one spelling both
/// analyses share.
fn ac_data_table_frequencies(netlist: &Netlist, table_name: &str) -> Result<Vec<f64>, String> {
    Ok(netlist
        .frequency_data_table_points(table_name)
        .map_err(|error| format!(".AC DATA {error}"))?
        .into_iter()
        .map(|point| point.frequency)
        .collect())
}

/// The engine's own default transient-noise seed.
///
/// Duplicated from `rspice-core`'s `DEFAULT_NOISE_SEED` rather than exported
/// from it, because exporting a constant would widen that crate's public
/// surface to carry a number this crate can hold and prove instead. The proof
/// is `a_deck_without_a_noise_seed_carries_the_engine_resolved_default`, which
/// runs two decks — one silent about the seed, one stating this value — and
/// requires the realizations to be bit-identical. A drift in either crate
/// fails that test rather than silently re-pointing this card at a different
/// realization.
const ENGINE_DEFAULT_NOISE_SEED: u64 = 0x5EED_0001;

/// The seed the engine will actually play, resolved the way the engine
/// resolves it.
///
/// A hand-written deck is free to say nothing about the seed, and the run is
/// still reproducible — the engine falls back to `.OPTIONS SEED` and then to
/// its own constant. The resolved value is carried into the specification so
/// that the card the Studio would write for this analysis re-runs the same
/// realization the deck just ran. Leaving it unresolved would produce a plan
/// whose seed field disagreed with the run it came from.
const fn resolved_transient_noise_seed(
    noise: &rspice_core::netlist::TransientNoiseConfig,
    options_seed: Option<u64>,
) -> u64 {
    match (noise.seed, options_seed) {
        (Some(seed), _) => seed,
        (None, Some(seed)) => seed,
        (None, None) => ENGINE_DEFAULT_NOISE_SEED,
    }
}

fn command_to_queue_item(
    state: &AppState,
    netlist: &Netlist,
    command: &AnalysisCommand,
) -> Result<QueuedAnalysis, String> {
    let spec_options = SpecExecutionOptions::default();
    match command {
        AnalysisCommand::Op => Ok(QueuedAnalysis {
            numeric_override: None,
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
            mode,
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
                // LIST retains the card's visiting order, including retraces.
                hysteresis: false,
                modes: crate::simulation::config::DcSweepModes {
                    primary: mode.into(),
                    secondary: sweep2
                        .as_ref()
                        .map_or_else(Default::default, |second| (&second.mode).into()),
                },
            };
            Ok(QueuedAnalysis {
                numeric_override: None,
                config: Some(AnalysisConfig::DcSweep(DcSweepConfig {
                    source: source.clone(),
                    start: *start,
                    stop: *stop,
                    step: *step,
                    source2,
                    start2,
                    stop2,
                    step2,
                    hysteresis: false,
                    modes: crate::simulation::config::DcSweepModes {
                        primary: mode.into(),
                        secondary: sweep2
                            .as_ref()
                            .map_or_else(Default::default, |second| (&second.mode).into()),
                    },
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
                numeric_override: None,
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
            numeric_override: None,
            spec: AnalysisSpec::AcData {
                table_name: table_name.clone(),
                frequencies: ac_data_table_frequencies(netlist, table_name)?,
            },
            config: None,
            spec_options,
            analysis_line: format!(".ac data={table_name}"),
        }),
        AnalysisCommand::Qpss(card) => {
            let config = rspice_core::engine::QpssConfig::from_qpss_card(card)
                .map_err(|error| error.to_string())?;
            let analysis_line = config.to_spice().map_err(|error| error.to_string())?;
            Ok(QueuedAnalysis {
                numeric_override: None,
                spec: AnalysisSpec::from_driven_qpss_config(config),
                config: None,
                spec_options,
                analysis_line,
            })
        }
        AnalysisCommand::Hb(hb) => {
            let defaults = rspice_core::analysis::HbConfig::from_hb_card(hb, &netlist.options)
                .map_err(|error| error.to_string())?;
            let tones = if defaults.tones.is_empty() {
                vec![
                    HbToneSpec::new(defaults.fundamental_freq, defaults.num_harmonics)
                        .with_name("tone1"),
                ]
            } else {
                defaults
                    .tones
                    .iter()
                    .map(|tone| {
                        let mut spec = HbToneSpec::new(tone.frequency, tone.num_harmonics)
                            .with_name(&tone.name);
                        if let Some(source) = &tone.source_name {
                            spec = spec.with_source(source);
                        }
                        spec
                    })
                    .collect()
            };
            let collocation_points = defaults.collocation_points;
            Ok(QueuedAnalysis {
                numeric_override: None,
                spec: AnalysisSpec::HarmonicBalance {
                    tones,
                    reltol: defaults.tolerance,
                    abstol: defaults.abstol,
                    max_iterations: defaults.max_iterations,
                    damping: defaults.damping,
                    min_damping: defaults.min_damping,
                    oversample: defaults.oversample_factor,
                    collocation_points,
                    max_mixing_order: defaults.max_mixing_order,
                    use_krylov: defaults.use_krylov,
                    gmres_restart: defaults.gmres_restart,
                    source_stepping: defaults.source_stepping,
                    use_exact_jacobian: defaults.use_exact_jacobian,
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
            do_noise,
            ports,
        } => Ok(QueuedAnalysis {
            numeric_override: None,
            spec: AnalysisSpec::SParameter {
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                points_per_unit: *points,
                sweep: frequency_sweep(*variation),
                z0: 50.0,
                ports: ports
                    .iter()
                    .map(|port| crate::simulation::multi_run::SpPort {
                        node_pos: port.node_pos.clone(),
                        node_neg: port.node_neg.clone(),
                        z0: Some(port.z0),
                    })
                    .collect(),
                do_noise: *do_noise,
            },
            config: None,
            spec_options,
            analysis_line: ".sp".to_string(),
        }),
        AnalysisCommand::Stb {
            variation,
            points,
            start_freq,
            stop_freq,
            probe,
            compute_nyquist,
        } => Ok(QueuedAnalysis {
            numeric_override: None,
            spec: AnalysisSpec::Stb {
                probe_node: probe.clone(),
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                sweep: frequency_sweep(*variation),
                points_per_decade: *points,
                compute_nyquist: *compute_nyquist,
            },
            config: None,
            spec_options,
            analysis_line: format!(
                ".stb {} {} {} {} probe={} NYQUIST={}",
                frequency_sweep(*variation).runner_keyword(),
                points,
                start_freq,
                stop_freq,
                probe,
                if *compute_nyquist { "yes" } else { "no" }
            ),
        }),
        AnalysisCommand::Disto {
            variation,
            points,
            start_freq,
            stop_freq,
            f2_over_f1,
        } => Ok(QueuedAnalysis {
            numeric_override: None,
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
            // The noise keywords a `.TRAN` card carries are parsed into the
            // deck's options rather than onto the command, so a reader that
            // only looked at the command would plan an ordinary transient and
            // drop the whole noise request without a word. The deck asked for
            // a different analysis, and this is where it is recognized as one.
            let spec = match netlist.options.transient_noise {
                Some(noise) => AnalysisSpec::TransientNoise {
                    stop_time: *stop,
                    step_time: *step,
                    start_time,
                    // The engine derives its own bound when the card states
                    // none; the Studio's own drafts always state one, so the
                    // resolved bound is carried here rather than invented.
                    max_timestep: max_step.unwrap_or(*step),
                    seed: resolved_transient_noise_seed(&noise, netlist.options.seed),
                    noise_fmax: noise.fmax,
                    noise_fmin: noise.fmin,
                    scale: noise.scale,
                    uic: *uic,
                },
                None => AnalysisSpec::Transient {
                    stop_time: *stop,
                    step_time: *step,
                    start_time,
                    max_timestep: *max_step,
                    uic: *uic,
                },
            };
            Ok(QueuedAnalysis {
                numeric_override: None,
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
                numeric_override: None,
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
                numeric_override: None,
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
                numeric_override: None,
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
            filters,
            ac_sweep,
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
            // The card's own filter list reaches the run. A bare `.sens
            // V(out)` therefore means here exactly what it means to the
            // engine: every device and model parameter, no design parameter.
            let filter = filters.join(" ");
            // A card is "one frequency" only when it is exactly what the form
            // writes for one — `DEC 1 f f`. Anything else is a band, and it
            // is kept whole rather than collapsed to its lower edge.
            let sweep = ac_sweep.as_ref().and_then(sensitivity_sweep_from_card);
            let spec = AnalysisSpec::Sensitivity {
                output_var: output_var.clone(),
                ac_mode: ac_sweep.is_some(),
                frequency,
                filter: filter.clone(),
                sweep: sweep.map(crate::simulation::config::SensitivitySweep::to_spec),
            };
            Ok(QueuedAnalysis {
                numeric_override: None,
                config: Some(AnalysisConfig::Sensitivity(SensitivityConfig {
                    output_var,
                    ac_mode: ac_sweep.is_some(),
                    frequency,
                    filter,
                    sweep,
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
                numeric_override: None,
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
        AnalysisCommand::Four { outputs, .. } => {
            let Some(output) = outputs.first() else {
                return Err(".four requires at least one output".to_string());
            };
            fourier_queue_item(netlist, command, output)
        }
        AnalysisCommand::MonteCarlo(command) => Ok(QueuedAnalysis {
            numeric_override: None,
            spec: AnalysisSpec::MonteCarlo {
                variation_source: Default::default(),
                // The card's own `PARAMS` list, so a hand-written subset is the
                // same run identity as the authored one. The engine reads the
                // list off this deck; carrying it here is what keeps two
                // different subsets from digesting as one run.
                params: command.params.clone(),
            },
            config: None,
            spec_options,
            analysis_line: ".mc".to_string(),
        }),
        AnalysisCommand::Step(_) => Ok(QueuedAnalysis {
            numeric_override: None,
            spec: AnalysisSpec::Parametric,
            config: None,
            spec_options,
            analysis_line: ".step".to_string(),
        }),
        AnalysisCommand::Temp { .. } => Err(
            ".temp directives must be planned as temperature sweeps before queueing".to_string(),
        ),
        AnalysisCommand::Pss(_)
        | AnalysisCommand::Pac(_)
        | AnalysisCommand::Pxf(_)
        | AnalysisCommand::Pstb(_)
        | AnalysisCommand::Pnoise(_) => Err(format!(
            "{} is queued from the deck's own card by the periodic reader, which binds it to \
                 the PSS operating point; reaching this route means the deck walk did not skip it",
            command_name(command)
        )),
        // Every field of the card is already resolved by the parser — the
        // defaults among them — so the reader converts rather than
        // re-deriving what the deck asked for. The probe is rebuilt in the
        // spelling the engine itself echoes back, which is also the spelling
        // the `.TF` arm above uses.
        AnalysisCommand::DcMatch(card) => {
            let output_expression = if card.output_is_current {
                format!("I({})", card.output_node)
            } else if let Some(reference_node) = &card.reference_node {
                format!("V({},{reference_node})", card.output_node)
            } else {
                format!("V({})", card.output_node)
            };
            Ok(QueuedAnalysis {
                numeric_override: None,
                config: None,
                analysis_line: ".dcmatch".to_string(),
                spec: AnalysisSpec::DcMismatch {
                    output_expression,
                    sigma_multiplier: card.sigma_multiplier,
                    contributor_limit: card.contributor_limit,
                    include_process: card.process,
                    include_mismatch: card.mismatch,
                    // The card has no operand for the report basis — the
                    // engine always computes both the signed contribution and
                    // the share — so the product default applies, exactly as
                    // the `.TF` arm above takes the product's numerical
                    // policy for operands the directive does not carry.
                    normalized_contributions: true,
                    // The engine's own default threshold is exactly zero, so
                    // an unstated one and a `THRESHOLD=0` are the same card
                    // and read back as the same specification.
                    contribution_threshold: (card.threshold != 0.0).then_some(card.threshold),
                },
                spec_options,
            })
        }
        // `.ENVELOPE` stays refused: the queue carries an `AnalysisSpec`, and
        // an envelope run's modulation sources have none yet.
        AnalysisCommand::Envelope(_) => Err(format!(
            "{} has no manual-deck queue route in this build",
            command_name(command)
        )),
    }
}

fn fourier_queue_item(
    netlist: &Netlist,
    command: &AnalysisCommand,
    output: &str,
) -> Result<QueuedAnalysis, String> {
    let configured = rspice_core::analysis::fourier::FourierConfig::try_from(command)
        .map_err(|error| error.to_string())?;
    let (output_node, output_ref) = parse_fourier_output(output)?;
    validate_manual_fourier_current_capability(netlist, &output_node)?;
    Ok(QueuedAnalysis {
        numeric_override: None,
        spec: AnalysisSpec::Fourier {
            fundamental_freq: configured.fundamental_freq,
            num_harmonics: configured.num_harmonics,
            num_periods: configured.num_periods,
            output_node,
            output_ref,
            // The manual route fans one card's output list into one queued
            // analysis per output, so each of those carries exactly its own.
            additional_outputs: Vec::new(),
            // Bound to the exact manual .TRAN window after the complete
            // directive list has been compiled.
            start_time: configured.earliest_start.unwrap_or(0.0),
            stop_time: configured.window_stop.unwrap_or(0.0),
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
                | ElementKind::RfPortDeferred { .. }
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

/// Read one card output through the shared `.FOUR` accessor grammar, and say
/// where a refusal came from: a hand-written deck's own card.
fn parse_fourier_output(output: &str) -> Result<(String, String), String> {
    crate::services::simulation_runner::split_fourier_output(output).map_err(|error| {
        format!(
            "Manual-deck .FOUR output '{}' is unsupported: {error}",
            output.trim()
        )
    })
}

#[cfg(test)]
mod tests;
